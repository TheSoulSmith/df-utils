# Patterns and worked samples

Six real templates supplied by the user, explained as algorithms. Their codes are in `assets/samples/` (regression fixtures: `scripts/selftest.py`).

## 10. The five sample scripts, as algorithms

Samples 1–5 are `func` templates; sample 6 is an event line. `scripts/df_tools.py` decompiles any of them; the descriptions below are derived from that output.

### 10.1 `cpu_limit(limiter.max_cpu: num)` — cooperative CPU limiter

```
cpu_limit(max_cpu):
    if now_us() >= deadline:                         # deadline: LOCAL var
        wait 1 tick                                  # yield so the plot isn't over-budget  (Wait, Ticks)
        players  = min(player_count, 8)              # line var
        deadline = now_us() + (40 + 6*(players-1)) * 6.25 * max_cpu     # microseconds
```
Intended use: call it inside a heavy loop; it yields only when the time slice is exhausted. Lore on the icon says it can be off by about ±2 iterations because it only checks when called. A tick is 50 ms; computed budgets:

| players (cap 8) | max_cpu 50 | max_cpu 100 |
|---|---|---|
| 1 | 12.5 ms | 25 ms |
| 2 | 14.4 ms | 28.8 ms |
| 4 | 18.1 ms | 36.3 ms |
| 8+ | 25.6 ms | 51.3 ms |

Reading: base slice = 25 ms at 100 (half a tick), growing by 3.75 ms per extra player up to 8 players. The "why" is inferred, not stated in the sample **[K]**.

### 10.2 `xorshift(Output:var, Input:num)` — 32-bit xorshift PRNG step

```
xorshift(x):
    x &= 0xFFFFFFFF ; out = x
    x = (x << 13) & 0xFFFFFFFF ; out ^= x
    x = (out >> 17) & 0xFFFFFFFF ; out ^= x
    x = (out << 5)  & 0xFFFFFFFF ; out ^= x
    return out
```
This is the standard xorshift32 (13, 17, 5). The `& 0xFFFFFFFF` after each left shift is needed because DF numbers are stored as **64-bit signed integers** (fixed-point ×1000; `Bit Precision`: `Default` = long/1000, `64-bit` = raw long), so a left shift does not wrap at 32 bits on its own **[A]**. Operators available: `\|`, `&`, `~` (no 2nd operand), `^`, `<<`, `>>`, `>>>`.

### 10.3 `Voronoi(Output:var, Location:loc, Frequency:num, Seed:num)` — nearest feature point (cellular noise)

```
Voronoi(L, F, S):
    x = L.x * F ; y = L.z * F                     # via %index(Location,1) and (Location,3)  -- Y is skipped
    cx = floor(x/16) ; cy = floor(y/16)
    best = 64 ; (ox, oy) = none
    for ccx in cx-1 .. cx+1:
      for ccy in cy-1 .. cy+1:                    # 3x3 neighbouring cells
        sx = ccx ^ 53 ; sy = ccy ^ 71             # hash the cell coords
        sx = sx * S ; sy = sy * S                 # mix in the seed
        sx = sx * sy
        sx = sx & 0xFFFFFFFF
        sy = sx & 15                              # low 4 bits -> offset 0..15
        sx = (sx >> 4) & 15                       # next 4 bits -> offset 0..15
        sx += ccx*16 ; sy += ccy*16               # feature point inside this 16x16 cell
        d = len2(sx - x, sy - y)                  # call_func len2 (NOT included in the samples)
        if d < best: best = d ; (ox, oy) = (sx, sy)
    Output = Location with x = ox/F, z = oy/F     # SetAllCoords, y untouched
```
Notes: cell size is `16 / Frequency` blocks in world space. `len2` is called but its definition isn't provided — I assume Euclidean 2D length (it is compared against an initial `rdist = 64`) **[K]**. The function returns a *location* (the nearest feature point), not a distance.

### 10.4 `sulfur(Pos:loc, Seed:num, TerrainSeed:num)` — one-column terrain + hazard generator

```
sulfur(Pos, Seed, TerrainSeed):
    vPos    = Voronoi(Pos, freq=2, Seed)                  # nearest feature point; cell = 8 blocks
    terrain = round(simplex2D(vPos, 0.5,1,1,1, TerrainSeed) * 5)
    tPos    = Pos shifted Y by (terrain - 0.5)
    fill stone from Pos to tPos                            # ground column; whole Voronoi cell shares height since vPos is used
    noise   = distance2D(vPos, Pos)                        # distance to the feature point
    detail  = simplex2D(Pos, 20,1,1,1, Seed)
    noise   = (noise / 7.5)^2
    fill air   from tPos to tPos+20                        # clear headroom
    p2      = tPos shifted Y by noise*10
    fill sulfur from tPos to p2                            # column height 10*noise: tall away from the feature point
    if noise < 0.2:                                        # i.e. within ~3.35 blocks (7.5*sqrt(0.2)) of the point
        p2 += 1 ; wPos = tPos shifted +2
        fill water_bucket(from p2 to wPos); fill air at wPos; set block water at wPos      # a pool
        if detail < 0.13: p2 -= 1; fill potent_sulfur at p2; p2 -= 1; set block magma_block at p2
    else:
        p2 += 1
        if detail < 0.2:   place sulfur_spike at p2
        else if detail < 0.25: place horn_coral at p2 with "waterlogged=false"
```
Implementation notes: `else` is followed by a nested `if` — that's how "else-if" is encoded (wire-format.md §1.5). Several block ids (`sulfur`, `potent_sulfur`, `sulfur_spike`) are newer Minecraft content than I can vouch for; they are namespaced IDs passed through as given **[K]**. The water placement (fill water, then air, then water) looks redundant; I'm documenting what the template does, not endorsing it.

### 10.5 `call.barrier()` — a 20-second "keep-out" bubble (no parameters)

Runs in the context of the **default target** (the casting player): `gv(Location@Default)` is the caster's position.

```
call.barrier():
    play beacon-activate sound at caster location, for AllPlayers ; burst 500 'ominous_spawning' particles there
    loc = caster.location ; loc.pitch = 90                       # look straight down
    loc = raycast(loc, 100, entities:ignore, blocks:all)         # ground point beneath the caster
    loc.y += 0.5 ; loc.pitch = 0
    # snapshot who is inside at cast time
    select players  within 5 of loc (circle)   -> allowedPlayers  = UUIDs ; reset selection
    select entities within 5 of loc (circle)   -> allowedEntities = UUIDs ; reset selection
    repeat 400:                                                  # 400 ticks = 20 s if Wait defaults to 1 tick
        wait 1 tick
        particle circle at loc (n = 10)                          # visual ring
        select players within 5 of loc (circle)
        give selection: Resistance(2t, amp 4), Weakness(2t, amp 127)   # tiny durations, refreshed every pass
        set selection invulnerable for 2 ticks
        filter selection: keep those whose UUID is NOT in allowedPlayers   # i.e. intruders only
        launch selection away from loc (power -10, additive velocity)
        reset
        select entities within 5 of loc (sphere); filter NOT in allowedEntities
        entity_action launch away (power -10) ; reset
    play beacon-deactivate sound + 500-particle burst at loc
```
What it teaches:
- **Snapshot-then-filter** is the idiom for "everyone except those present at start": store UUID lists, then `FilterCondition` with `NOT ListContains` against `UUID@Selection`.
- Protective potions + invulnerability are applied to *everyone* in the radius **before** the filter; only the push is applied to the filtered subset. (If the intent was to protect only the allowed group, this ordering is a likely bug — I'm describing the template, not vouching for it.)
- Players and entities use *different* block types (`player_action` vs `entity_action`) and different select blocks (`PlayersCond` vs `EntitiesCond`); the entity pass also switches the shape to Sphere.
- Always `Reset` after a selection; otherwise later targeted blocks keep acting on the stale selection.

---

### 10.6 `PlayerDmgPlayer` event line (sample 6) — the smallest complete line

```
event 'PlayerDmgPlayer'                 # header: event name in `action`, args.items = []
    player_action 'PlaySound'           # NO `target` key -> current selection if any, else the default target; an event line starts with no selection, so: the player the event is about
        [0] snd(minecraft:block.chain.hit, pitch 1.0, vol 2.0)  (item version 1)
        <Sound Source = Master>         # tag slot 26
```
It is the reference for how an event header serializes. In the builder: `t.header("event","PlayerDmgPlayer"); t.block("player_action","PlaySound",[snd("block.chain.hit")])`, which reproduces the sample byte-for-byte (the builder also reproduces samples 1 and 3 exactly).

---

## 11. Idioms seen in these samples (reuse them)

| Need | Pattern |
|---|---|
| Don't exceed the tick budget in a long loop | Call a `cpu_limit`-style function each iteration: if `Microseconds Since Startup >= deadline` (a **local** var) then `Wait` and recompute the deadline (§10.1). |
| Return a value from a function | Declare a `var`-typed parameter (conventionally first, named `Output`); assign to that line variable; callers pass a variable in that position (§10.2, §10.3). |
| Deterministic randomness | `xorshift` with `& 0xFFFFFFFF` after every left shift; seed via a variable (§10.2). |
| Cellular/feature-point terrain | Hash 3×3 neighbouring cells into offsets, keep the nearest (§10.3). The native `Noise` action has Voronoi/Worley return types but returns a cell id or a distance, not a location. |
| "Everyone except those present at start" | Snapshot `Selection Target UUIDs` into lists, later `FilterCondition` + `ListContains` with `NOT` against `UUID@Selection` (§10.5). |
| Call a function chosen at runtime ("softcoding") | Put a `%` code in the `call_func` name, e.g. a prefix plus a variable (§12). Give the callable functions a shared prefix and mark them hidden. |
| Else-if | `else { if … { } }` — there is no elseif block (§10.4). |
| Safe formulas in text | One `%math` per operator class, nested (text-codes.md §4). |
| Per-player variables | Put `%default` in the variable *name* (`Score %default`). |

---

## 12. Dynamic dispatch ("softcoding"): calling functions by name from data

The user's explanation: `%` codes work in `call_func` / `start_process` names, not just values, and *calling a function from a string* is one of DiamondFire's core capabilities (it is awkward in most compiled languages). The usual pattern is **softcoding**: store the function name in data (for example a tag on an item, or a field of a game object) and call it when the object is used or otherwise interacted with. Because arbitrary strings can then reach `call_func`, people **prefix the `%` code** (e.g. `bot.` + a variable) so that functions meant to be reached this way are easy to tell apart from ordinarily-called ones.

```
dispatch(kind):                       # kind = a string held in data, e.g. a block type under a bot
    call_func( "bot." + kind )        # written in the template as a name containing a % code, e.g. bot.%var(kind)
handler "bot.white_concrete":         # one tiny hidden function per kind
    ...
```

Seen in samples 8–10 (a subset of the user's bots-following-paths game; the bots walk over blocks and the block under them decides the behaviour):

| Function (all `Is Hidden = True`) | Does |
|---|---|
| `bot.white_concrete` (s8) | `ShiftInDirection` on local `loc`, Direction = Forward (default distance 1, per the dump): step forward |
| `bot.red_concrete` (s10) | `ShiftRotation` of local `loc` by -90 on the Yaw axis (turn left), then step forward |
| `bot.gray_concrete` (s9) | calls helper functions `botParticle(flame ×50, loc)` and `botSound(entity.item.break, pitch 2, vol 1)`, then sets local `killBot = 1` (the bot is removed by the caller) |

What it teaches:
- **Shared state through `local` variables.** `loc` and `killBot` are `local`, so a handler can change the caller's data without parameters; the dispatcher reads them after the call. (`local` = thread-wide; see language-semantics.md §2.)
- **A handler per case instead of a big if-chain**: adding a block type means adding one function, with no change to the dispatcher.
- **Hidden functions** (`Is Hidden = True`): hidden from the menu used to pick a function/process for *Call Function* / *Start Process* [U], which keeps those menus tidy; they are still callable by name.
- **Wire form of a dynamic call is confirmed (sample 11):** the `%` code is written directly inside the call's `data` string, `{"block":"call_func","data":"call.%var(call)"}`, with no special markers. The linter reports such calls as INFO because the target can't be checked statically.

### 12.1 The "default value of every item type" template (sample 7)

A `func` named `example` whose chest holds one default instance of each value item (text, component, number, location, vector, sound, particle, potion, variable, bucket variable, game value) followed by a `pn_el` parameter of type `any`. It is the reference for the item shapes in wire-format.md §3 and is reproduced exactly by the builder (`scripts/selftest.py`).

### 12.2 Item-ability dispatcher (sample 11): "right click runs the function named by an item tag"

```
abilityCall(item, calltag = "rccall", cdtag = "cd"):      # optional params with defaults
    if player has NO item cooldown on `item`:             # if_player NoItemCooldown, no target -> selection if any, else default
        cd   = GetItemTag(item, cdtag)                    # the cooldown length is stored on the item itself
        SetItemCooldown(item, cd)
        call = GetItemTag(item, calltag)                  # the function NAME is stored on the item
        call_func "call." + call                          # name built at runtime: data = "call.%var(call)"
```
- **Softcoded items:** to add a new item power, give the item a tag `rccall = "fireball"` and define a function `call.fireball`. Nothing else changes (the user's point: easy creation of new items with new functionality).
- **Prefix convention:** `call.` marks functions that exist to be reached from item tags, so they can't be confused with normally-called functions (the bot handlers in §12 use `bot.`).
- The dynamic call passes **no arguments**; the handler gets its context from the event's targets and `local` variables.
- `%var(call)` here reads the **line** variable `call` (`%var` reads any scope [T]; precedence if one name exists in several scopes is untested).
- **A call to a function that doesn't exist does nothing**, exactly like calling an existing but empty function [U]. An item tag naming a missing function is therefore harmless, which makes softcoded dispatch safe to leave unguarded.

### 12.3 Shapes confirmed by the small "example" templates (samples 12–14)

| Sample | Shows |
|---|---|
| 12 | `if_player 'SlotEquals'` with `target: Default` and NOT, `if_entity 'IsOxidized'` with `target: Victim`, NOT and its tag, `if_game 'EventBlockEquals'` (no target) |
| 13 | `repeat 'While'` with `subAction: "="`, NOT, args `[i, 10]`, repeat-typed brackets; body `+=` with only the variable (default increment) |
| 14 | every `target` name for `player_action` / `entity_action`; `SendMessage` has three tags (Inherit Styles, Text Value Merging, Alignment Mode) |

---

## 13. Header and parameter reference samples (15–20)

| Sample | Shows |
|---|---|
| 15 | `entity_event` header (`EntityDmgEntity`), `args.items` empty |
| 16 | `process` header named `process` (tag `Is Hidden`, **no hint item**) and a `start_process` of it with tags `Local Variables = Don't copy` (slot 25), `Target Mode = With current targets` (slot 26) |
| 17 | `game_event` header (`PlotShutdown`) |
| 18 | a function with one parameter of each value type: `txt comp num loc vec snd part pot item` |
| 19 | parameter kinds `dialog any var list dict`, plus `plural`, `optional`, and `optional` with `default_value`; `var` params can't be plural/optional |
| 20 | `if_player " StandingOn "` filled with a block item and with a location (both at slot 0); padded action name |

---

## 14. A real program: path-following bots (samples 21-27)

Seven templates from the user's bots game (bots follow paths made of particular block types). They show how a non-trivial DiamondFire program is organised. **Not included** in the samples (called but undefined here): `byteLoad`, `byteStore`, `botfly`, `checkIfMoveValid`, `botSound`, `botParticle`; the per-block handlers `bot.<blockid>` are in samples 8-10. The descriptions below are read from the code; intent is marked [K] unless the user has confirmed it [U].

| Sample | Template | Role |
|---|---|---|
| 21 | process `gBotLoop` | the scheduler (runs forever) |
| 22 | func `updateBots` | runs every bot of one group that is due |
| 23 | func `getBotData` | loads bot #i from the global lists into working `local` variables |
| 24 | func `botTick` | one step of one bot (the logic) |
| 25 | func `removeBot` | deletes bot #i from every list |
| 26 | func `setBotData` | writes the working variables back |
| 27 | func `cpu_limit` | variant of sample 1 that also sets `local tickHasElapsed = 1` when it waits |

### 14.1 Data layout: struct of arrays, selected by a group name

One bot = the same index in seven parallel **game** lists, whose names end in `_%var(Group)` (so "norm" and "max" groups have separate lists): `gBotLocs_`, `gBotVars_` (packed with `byteStore`/`byteLoad`), `gBotFlyHeights_`, `gBotActionTimes_`, `gBotLayerOwners_`, `gBotLayers_`, `gBotRunWhen_`. A bot's working record lives in **`local`** variables (`loc`, `flyHeight`, `moveTime`, `layerOwner`, `BotLayer`, `BotVars`, `killBot`); every function in the thread sees it, so no parameters are passed. Per-owner settings are game variables whose *name* starts with the owner's name: `%var(layerOwner) botSkin` / `botSkinStates` / `botSkinInvertdir` (a name may contain a space, a code and more text).

### 14.2 Scheduler (sample 21)

```
gBotLoop():
    deadline = now + budget(players, 90)             # same formula as cpu_limit with max_cpu pre-multiplied: 6.25*90 = 562.5
    forever:
        tickHasElapsed = 0
        updateBots(count, "norm")                    # fixed-rate group: once per tick
        silenced = 1
        do:
            updateBots(count, "max")                 # "max" group: runs repeatedly, soaking up the rest of the tick
            if count == 0:                           # nothing in the max group: just wait the tick out
                wait 1 tick ; tickHasElapsed = 1 ; deadline = now + budget
        while tickHasElapsed != 1                    # cpu_limit sets it to 1 when it has to wait
        silenced = 0
```
**Confirmed by the user [U]:** `silenced` mutes the sounds and particles of the max-rate group (`botSound`/`botParticle` aren't in the samples), and the `max` group runs as often as the CPU allows while the `norm` group runs once per tick.

### 14.3 One group update (sample 22)

```
updateBots(OUT botCount, Group):                     # botCount is a var-typed parameter used as an output
    gameTime_<Group> += 1                            # a per-group clock (game variable)
    botCount = length(gBotLocs_<Group>)
    botIndex = 0
    repeat botCount times:                           # count fixed when the loop starts
        botIndex += 1
        if gBotRunWhen_<Group>[botIndex] == gameTime_<Group>:        # %index on a GAME list, name built from a code
            getBotData(botIndex, Group)
            oldBotVars = BotVars
            botTick()
            if killBot == 1:
                removeBot(botIndex, Group) ; killBot = 0 ; botIndex -= 1      # later entries shifted down; this keeps the next iteration on the right bot
            else:
                gBotRunWhen_<Group>[botIndex] = gameTime_<Group> + moveTime  # schedule the next step
                setBotData(botIndex, Group)
        cpu_limit(90)                                # cooperative yield after every bot
```
**Why removal is safe (corrected by the user, and checked by simulation).** The repeat count is fixed when the loop starts, and a bot can only be removed during *its own* iteration (removal happens inside the "due" branch). The `botIndex -= 1` after a removal exactly offsets the shift of the later entries, so each iteration handles exactly one **original** bot, and the index never passes the end of the list. Worked example, 5 bots with bots 0 and 2 killed (iteration, bot handled, action, list length after): `(1,0,remove,4) (2,1,keep,4) (3,2,remove,3) (4,3,keep,3) (5,4,keep,3)`. (A 120,000-run randomised simulation never indexed past the end.) The number of iterations equals the number of bots at the start; the bot count may change afterwards, and the next call recounts.

Two further notes:
- A bot runs only when its scheduled time **equals** the clock, so `moveTime` must be at least 1; with 0 it would be scheduled for the tick that just ran and never fire again [inferred from the code].
- `cpu_limit(90)` can suspend the thread mid-loop, so another thread could in principle add or remove bots while this one waits; out-of-range `%index` returning 0 would then act as a safety net [K, speculation; the loop itself does not depend on it].

`setBotData` writes `BotVars` only when it changed (`oldBotVars != BotVars`, then `byteStore`), skipping the expensive packing step; the other six fields are always written back.

### 14.4 One bot step (sample 24): the world is the program

```
botTick():
    if not HasPlayer(layerOwner): killBot = 1 ; return
    if BlockAt(loc) != owner.botSkin: killBot = 1 ; return           # the bot's block was destroyed or replaced
    blockstates = owner.botSkinStates ; invert = owner.botSkinInvertdir
    cmdLoc = loc shifted -1 on Y                                      # the block UNDER the bot
    set air at loc                                                    # erase the old position
    cmdChain = [] ; chainLength = 0
    if flyHeight == 0:
        action = block id at cmdLoc                                   # e.g. "white_concrete"
        call function "bot." + action                                 # dynamic dispatch; handler moves `loc`, may set killBot
        botSound(...)
    else: botfly()
    if killBot == 1: return
    checkIfMoveValid()
    if killBot == 1: return
    dir = direction(loc) ; if invert: dir = -dir ; name = directionName(dir)   # e.g. "north"
    state = replace(blockstates, "<dir>", name)                       # a block-state template
    set owner.botSkin block at loc with `state`                       # redraw the bot at its (possibly new) position
```
So a path is built out of ground blocks; the block under a bot **selects the handler** (`bot.white_concrete` steps forward, `bot.red_concrete` turns -90 degrees then steps, `bot.gray_concrete` emits flame particles and a sound and sets `killBot`). New behaviours need only a new `bot.<blockid>` function. This is softcoding (section 12) driven by world data instead of an item tag.

### 14.5 What to reuse

| Idiom | Where |
|---|---|
| Separate lists per group via `_%var(Group)` in variable names | all |
| Load record into `local` vars, mutate, write back only what changed | 23, 26 |
| Output parameter (`var`-typed) for returning a count | 22 |
| Scheduling by `runWhen == clock`; delete during iteration with `index -= 1` so the fixed loop count still covers every original item | 22 |
| Cooperative CPU limiter called per item | 22, 27 |
| Fixed-rate loop + "use the leftover tick" loop | 21 |
| Dispatch on a world value (`bot.%var(blockId)`) | 24 |

New actions seen (all lint clean): `ListLength`, `GetListValue`, `SetListValue`, `RemoveListIndex`, `CreateList`, `GetBlockType` (tag `Return Value Type`), `GetDirection`, `MultiplyVector`, `DirectionName`, `ReplaceString` (tags `Replacement Type`, `Regular Expressions`), `-=`; `control Return`; `if_game HasPlayer` / `BlockEquals` (with `NOT`); `repeat Forever` and `DoWhile` (sub-condition `!=`).

