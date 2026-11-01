# DiamondFire template wire format

Read this before building or parsing template JSON by hand. For building, prefer `scripts/df_build.py`, which applies these rules automatically.

Confidence tags used throughout: **[T]** tested in-game by the user (ground truth, wins over everything) · **[A]** actiondump patch 8.13 (authoritative for names/tags/args) · **[S]** observed in the user's real sample templates (authoritative for wire format) · **[D]** official DiamondFire docs · **[C]** community source (forum/wiki/third-party tooling; may be dated) · **[U]** stated by the user · **[P]** proposal, not known to be implemented · **[K]** my prior knowledge, unverified — say so to the user when a template depends on one.

## 1. Wire format (what a "template code" actually is)

### 1.1 Encode / decode algorithm [S]

```
decode(code):  bytes = base64_decode(code) ; json_text = gunzip(bytes) ; return json_parse(json_text)
encode(tpl):   json_text = json_stringify(tpl, compact) ; return base64_encode(gzip(json_text))
```

- All five samples start with `H4sI`, which is base64 for the gzip magic bytes — a quick sanity check that a pasted string is a template.
- Round-trip decode→encode→decode was verified lossless on all five samples. Compression level/byte layout doesn't matter; the JSON content does.
- Sending to the game: community tools (dfpyre, CodeClient, recode) push templates directly into the client. [C]

### 1.2 Envelope [S]

```json
{ "blocks": [ <element>, <element>, ... ] }
```
`blocks` is the flat, ordered list for **one code line** (all twenty samples are single-line templates, each starting with a header). A template holds exactly one code line [U].

### 1.3 Element kinds [S]

**Code block**
```json
{ "id": "block", "block": "<block-type>", "args": { "items": [ <slotted item>, ... ] },
  "action": "<action name>"      // for most blocks
  // OR "data": "<name>"         // for func / call_func (the function's name)
}
```
- **Event header [S, sample 6]:** `{"id":"block","block":"event","args":{"items":[]},"action":"PlayerDmgPlayer"}` — the event name goes in `action` (names are in `actions/events.md`), `args.items` is empty unless the event has tags. `entity_event` and `game_event` have exactly the same shape [S, samples 15 and 17: `{"block":"entity_event","args":{"items":[]},"action":"EntityDmgEntity"}`]. `func`/`call_func` carry the name in `data` instead.
- `else` has no `args` and no `action`: `{"id":"block","block":"else"}`.
- `args` may be absent (e.g. `else`) or present-but-empty (`"args":{"items":[]}`, seen on `select_obj` / `Reset`).
- **Extra block-level keys [S, sample 5]** (all optional, placed beside `action`):
  - `"target"` — on targeted blocks. **Values seen [S, samples 12 and 14]:** `player_action`: `Selection`, `Default`, `Killer`, `Damager`, `Shooter`, `Victim`, `AllPlayers`; `entity_action`: those plus `Projectile`, `AllEntities`, `AllMobs`, `LastEntity`; `if_player`: `Default`; `if_entity`: `Victim`. `if_game` and `if_var` take no target. **Absent `target` is NOT simply "default target"** [U]: the block acts on the **current selection if one exists (an empty selection still counts), otherwise on the default target**. Sample 6's `PlaySound` has no `target` because an event line starts with no selection; sample 11's `if_player` has none either. (`g_val` items carry their own `target` inside `data`, with `"Default"` / `"Selection"` seen.)
  - `"subAction"` — on `select_obj` **and `repeat` `While`/`DoWhile`**: names the *condition* applied. For `repeat While` the block's own args are the **condition's** args, e.g. `{"block":"repeat","action":"While","subAction":"=","attribute":"NOT"}` with args `[i, 10]` means `while NOT (i = 10)`; its brackets are type `repeat` [S, sample 13]. Colliding condition names use the alias (`PIsNear`, `EIsNear`).
  - `"attribute": "NOT"` — inverts the condition. **Confirmed on `if_player`, `if_entity`, `select_obj` and `repeat While`** [S, samples 5, 12, 13]. `if_game` and `if_var` use the same `attribute: "NOT"` [U].

**Bracket**
```json
{ "id": "bracket", "direct": "open" | "close", "type": "norm" | "repeat" }
```
`type` is `"repeat"` for the brackets of a `repeat` block and `"norm"` for everything else (if_*, else). [S]

**Slotted item**
```json
{ "item": { "version": 0, "id": "<item-type>", "data": { ... } }, "slot": <0..26> }
```

### 1.4 Slot semantics (algorithm) [S]

A block's argument chest has 27 slots (0–26).

```
place_args(block, positional_args, tags, hint=None):
    for i, item in enumerate(positional_args):       # slot = argument position
        if item is not EMPTY: emit {item, slot: i}   # gaps are legal and MEANINGFUL
    tags are right-aligned at the end of the chest:
        for j, tag in enumerate(tags): slot = 27 - len(tags) + j      # 1 tag -> 26 ; 2 -> 25,26 ; 3 -> 24,25,26
    func header: hint item (`function`) at slot 25, "Is Hidden" tag at slot 26   # a PROCESS header has NO hint item, only the tag at 26 [S sample 16]
    start_process: tags "Local Variables" (25) and "Target Mode" (26)
```
Three-tag blocks (`GivePotion`) occupy 24, 25, 26 [S, sample 5]. Evidence for gaps: `SetAllCoords` in Voronoi uses slots 0,1,2,4 and leaves 3 empty (x in 2, z in 4 → slot 3 is y, deliberately skipped).

**Slots are not always strict parameter positions [S].** `ShiftOnAxis` in sample 2 puts its number in slot **1** (`[T, 1]`), while sample 5 puts the same kind of number in slot **2** with slot 1 empty (`[T, _, 0.5]`). Both are valid exports of "shift T in place by n", so DF appears to resolve at least some arguments by *type* rather than exact slot (an omitted location means "use T itself"). Rule for generating: use the canonical form `[T, source_loc?, amount]` with real gaps, which is what the editor exports when someone fills the chest in order; don't depend on type-resolution **[K]**. Two-tag blocks (`Bitwise`, `Noise`) use 25 and 26; one-tag blocks use 26.

### 1.5 Bracket-matching rule (algorithm)

```
emit_conditional(cond_block, body, else_body=None):
    emit cond_block ; emit bracket(open, "norm") ; emit body ; emit bracket(close, "norm")
    if else_body: emit {"id":"block","block":"else"} ; emit bracket(open,"norm") ; emit else_body ; emit bracket(close,"norm")

emit_repeat(repeat_block, body):
    emit repeat_block ; emit bracket(open,"repeat") ; emit body ; emit bracket(close,"repeat")

validate(blocks):   # every template must satisfy this
    stack = []
    for e in blocks:
        if e is bracket(open):  stack.push(e.type)
        if e is bracket(close): assert stack.pop() == e.type
    assert stack is empty
    for each 'else' block: assert previous element is bracket(close,"norm") and next is bracket(open,"norm")
```
`else` chains are written as `else { if ... { } }` — there is no `elseif` block in the samples (see sulfur in patterns-and-samples.md §10.4). [S]

---

## 2. Block catalog

All 19 block identifiers are confirmed by the dump **[A]**; JSON usage marked **[S]** also appears in the samples. Every header and block family now has at least one real sample. Remaining families (other `select_obj`/`repeat` actions, `if_game`/`if_var` with `NOT`) follow the same rules in the dump [U]/[K].

| In-game block | JSON `block` id | Header? | Has brackets? | Status |
|---|---|---|---|---|
| Player Event | `event` (name in `action`) | yes | no | **[S]** sample 6 + [A] |
| Entity Event | `entity_event` | yes | no | **[S]** sample 15 + [A] |
| Game Event | `game_event` | yes | no | **[S]** sample 17 + [A] (35 actions) |
| Function | `func` (name in `data`) | yes | no | **[S]** |
| Process | `process` (name in `data`) | yes | no | **[S]** sample 16 + [A] — no hint item; tag `Is Hidden` (owner `dynamic`/`process`) |
| Call Function | `call_func` (name in `data`; the name may contain `%` codes, e.g. `call.%var(call)`) | no | no | **[S]** (samples 9, 11) |
| Start Process | `start_process` (process name in `data`, may hold `%` codes) | no | no | **[S]** sample 16 — tags `Local Variables` (slot 25) and `Target Mode` (26), owner `dynamic`/`start_process`; like any tag, non-default options come straight from the dump (`With no targets`, `For each in selection`, `Share`, …) [U] |
| Player Action | `player_action` | no | no | **[S]** |
| Entity Action | `entity_action` | no | no | **[S]** |
| Game Action | `game_action` | no | no | **[S]** |
| Set Variable | `set_var` | no | no | **[S]** |
| If Player / If Entity | `if_player` / `if_entity` | no | yes (norm) | **[A]** (ids) + [S] (via tags) |
| If Game | `if_game` | no | yes (norm) | **[A]** |
| If Variable | `if_var` | no | yes (norm) | **[S]** |
| Else | `else` | no | yes (norm) | **[S]** |
| Repeat | `repeat` | no | yes (**repeat**) | **[S]** |
| Control | `control` | no | no | **[S]** |
| Select Object | `select_obj` | no | **no** (modifies the thread's selection) | **[S]** |

Block-level fields: see §1 (Element kinds) for `target`, `subAction`, `attribute` (all confirmed by real samples).

Blocks that need no targets (`set_var`, `if_var`, `game_action`, `repeat`, `control`, `call_func`, `select_obj`) carry no `target` field in the samples. **[S]**

### 2.1 Select Object — selection mechanics (algorithm) [S + C]

`select_obj` is the only block that edits the thread's *selection* (see language-semantics.md §1). It has **no brackets**; it changes state for every block after it until another `select_obj`.

```
select_obj 'PlayersCond'  sub=<if_player action>  : selection = players for which the condition holds
select_obj 'EntitiesCond' sub=<if_entity action>  : selection = entities for which the condition holds
select_obj 'FilterCondition' sub=<if_var action> [NOT] : selection = { e in selection | cond(e) }   # per-element; g_val UUID@Selection = e's UUID
select_obj 'Reset'                                : selection = NONE (deactivated; following code runs with event targets) [A]   # args present but empty
blocks with target "Selection" then run once per element of the selection
```
Tags on these blocks belong to the **sub-action**, not to `select_obj` (see §4).

---

## 3. Code items (the things in a block's chest)

Official list of item type ids (from the action-dump `versions` object) **[D]**:
`bucket_var, var, num, txt, comp, loc, r_loc, vec, part, snd, pot, item, g_val, bl_tag, pn_el, hint`

Seen in samples **[S]**, with the exact `data` shape:

| id | Meaning | `data` shape (observed) |
|---|---|---|
| `num` | number | `{"name": "<string>"}` — value is a **string**, may contain text codes: `"8"`, `"%math(...)"`, `"4294967295"` |
| `txt` | text | `{"name": "air"}` |
| `var` | variable | `{"name": "<n>", "scope": "line" \| "local"}` (see language-semantics.md for the other scopes) |
| `g_val` | game value | `{"type": "Player Count", "target": "Default"}` |
| `item` | Minecraft item | `{"item": "<SNBT string>"}` e.g. `{DF_NBT:4903,count:1,id:"minecraft:stone"}` — note `DF_NBT` is an integer tag on serialized items [D]; its role is probably the MC data version [K]. Custom names/lore use 1.21 `components:{"minecraft:custom_name":…}` syntax. |
| `pn_el` | function parameter | `{"name","type","plural","optional"}` + optional `"description"`, `"note"`. **Optional parameter with a default:** `{"name":"cdtag","type":"txt","default_value":{"version":0,"id":"txt","data":{"name":"cd"}},"plural":false,"optional":true}` [S, sample 11] |
| `hint` | function-chest hint icon | `{"id": "function"}` (slot 25 on func headers) |
| `snd` (**version 1**) | sound | `{"mappingVersion":0,"sound":"minecraft:block.beacon.activate","pitch":1.0,"vol":2.0}` — **S, sample 5** |
| `part` (**version 1**) | particle | `{"mappingVersion":0,"particle":"minecraft:ominous_spawning","cluster":{"amount":500,"horizontal":2.0,"vertical":2.0},"data":{"x":1.0,"y":0.0,"z":0.0,"motionVariation":100}}` — **S, sample 5** |
| `pot` (**version 1**) | potion effect | `{"mappingVersion":0,"pot":"minecraft:resistance","dur":2,"amp":4}` — **S**. `dur` is in ticks; `amp` is the level minus 1 (4 = level V; 127 = level 128) **[K for the amp mapping]** |
| `bl_tag` | block tag (dropdown option) | `{"tag","option","action","block"}` — see §4 |

Item shapes confirmed by the user's "default value of every item type" template (sample 7) **[S]**:

| id | `data` shape | Notes |
|---|---|---|
| `comp` | `{"name": "text"}` | styled text; may hold markup and `%` codes |
| `loc` | `{"isBlock": false, "loc": {"x":0.0,"y":0.0,"z":0.0,"pitch":0.0,"yaw":0.0}}` | floats. `isBlock` only controls how the location is *drawn* in development mode (a whole block vs a smaller cube); it has no effect on behaviour **[U]** |
| `vec` | `{"x":0.0,"y":0.0,"z":0.0}` | floats |
| `bucket_var` | `{"name":"variable","key":"bucket","namespace_type":"DEFAULT","namespace_alias":""}` | `key` = the bucket; `DEFAULT` = the plot's default namespace (see language-semantics.md §2.3) |
| `var` | `{"name":"…","scope":"unsaved"}` | the default-named variable in that sample is `§eVariable` (a colour-coded display default; use real names) |
| `pn_el` | `{"name":"name","type":"any","plural":false,"optional":false}` | parameter type **`any`** exists |

Default values in that sample: `txt` "string", `comp` "text", `num` "0", `snd` `minecraft:block.note_block.pling` (pitch 1.0, vol 2.0), `part` `minecraft:cloud` (amount 1, spread 0/0, data x=1.0 y=0 z=0, motionVariation 100), `pot` `minecraft:speed` (dur 1000000, amp 0). Still unseen: `r_loc` (**[D]** for existence only).

Game values seen so far: `Player Count`, `Microseconds Since Startup`, `Location` (target `Default`), `Selection Target UUIDs` (target `Default`), `UUID` (target `Selection` — evaluated per selected entity). **[S]**

**Versioning [D]:** every item has a `version`. Samples are all `0`. Official docs describe sounds, potions, and particles moving to **version 1**, which store *Minecraft IDs* (e.g. `minecraft:entity.player.attack.crit`) plus a `mappingVersion`, replacing display names; the doc page presents this as an upcoming/rolling change, and the `versions` object it shows still lists `num/txt/var/...` at 0 and `part/snd/pot` at 1, with patch `8.13`. **Confirmed live [S, sample 5]:** the user's newest template uses `"version": 1` on `snd`, `part`, and `pot` with Minecraft IDs and `mappingVersion: 0`. → Emit v1 forms for these three item types; do **not** emit display-name (v0) forms. Everything else stays `version: 0`.

Parameter types seen on `pn_el` **[S, samples 18 and 19]**: `txt`, `comp`, `num`, `loc`, `vec`, `snd`, `part`, `pot`, `item`, `dialog`, `any`, `var`, `list`, `dict`. Flags: `plural: true` (the caller's values arrive in the function **as a list**), `optional: true`, and `default_value` (a full item of any type, used when optional). A `var` parameter cannot be plural or optional (the sample's note says so). A parameter can also be both optional and plural (stated by the user, no sample). Names are free text (the sample names them after their types).

### 3.1 Parameter semantics (algorithm) **[S + C]**

```
call_func(name, a0, a1, ..., an):
    for i in 0..n: callee.line_vars[ params[i].name ] = a_i       # positional, slot i -> param i   [S]; params become line variables [C forum]
    run callee body
    for each param p where p.type == "var":                       # "Output"-style param           [S pattern]
        caller_var_passed_at_p.value = callee.line_vars[p.name]   # written back on return
```
Evidence: `xorshift(Output:var, Input:num)` and `Voronoi(Output:var, Location, Frequency, Seed)` both assign to a line var literally named `Output`, and callers pass a variable as the first argument (`vPos`, `dist`). A forum design thread independently describes "first parameter is a variable that receives the return value". [C] I have **not** seen the in-game write-back rule stated in official docs, so confidence in the exact copy-back timing is medium.

---

## 4. Tags

A tag is a `bl_tag` item in the last slots of the chest. Its `action` and `block` fields repeat the owner's identity — **which owner depends on the block**: **[S pattern; error behaviour K]**

```
tag_owner(block):
    if block is a function header:      (action="dynamic", block="func")
    elif block has a subAction:         (action=block.subAction, block=<the if-block that subAction belongs to>)
                                         # e.g. PlayersCond/PIsNear -> ("PIsNear","if_player"); EntitiesCond/EIsNear -> ("EIsNear","if_entity");
                                         #      FilterCondition/ListContains -> ("ListContains","if_var")
    else:                                (action=block.action, block=block.block)
```

Observed tags (block / action → tag = option, slot):

| Block / action | Tag = option(s) seen | Slot(s) |
|---|---|---|
| `func` / `dynamic` | `Is Hidden = False` | 26 |
| `control` / `Wait` | `Time Unit = Ticks` | 26 |
| `set_var` / `Noise` | `Dimensions = 2D`, `Return Type = Simplex` | 25, 26 |
| `set_var` / `" RoundNumber "` | `Round Mode = Nearest` | 26 |
| `set_var` / `ShiftOnAxis` | `Coordinate = Y` | 26 |
| `set_var` / `Distance` | `Distance Type = Distance 2D (X/Z)` | 26 |
| `set_var` / `Bitwise` | `Bit Precision = Default`, `Operator = & \| ^ \| << \| >>` | 25, 26 |
| `set_var` / `/` | `Division Mode = Floor result` | 26 |
| `set_var` / `SetAllCoords` | `Coordinate Type = Plot coordinate` | 26 |
| `set_var` / `SetCoord` | `Coordinate Type = Plot coordinate`, `Coordinate = Pitch` | 25, 26 |
| `set_var` / `Raycast` | `Entity Collision = False`, `Block Collision = All blocks` | 25, 26 |
| `player_action` / `PlaySound` | `Sound Source = Master` | 26 |
| `player_action` / `GivePotion` | `Show Icon = False`, `Overwrite Effect = True`, `Effect Particles = None` | 24, 25, 26 |
| `player_action` / `LaunchToward` | `Add to Current Velocity = True`, `Ignore Distance = False` | 25, 26 |
| `entity_action` / `LaunchToward` | same two tags (owner block = `entity_action`) | 25, 26 |
| `select_obj` / `PlayersCond` (sub `PIsNear`) | `Shape = Circle` (tag owner `if_player`) | 26 |
| `select_obj` / `EntitiesCond` (sub `EIsNear`) | `Shape = Circle` or `Sphere` (owner `if_entity`) | 26 |
| `select_obj` / `FilterCondition` (sub `ListContains`) | `Check Mode = Has Any Value` (owner `if_var`) | 26 |

Actions with **no** tags in the samples: `Particle`, `ParticleCircle`, `SetInvulTicks`, `select_obj/Reset`, `set_var/=`, `repeat/Multiple`. **[S]**

The `Operator` of `Bitwise` is a **tag**, not a separate action: the action is always `"Bitwise"`. **[S]**

Omitting a tag: pyre states that unspecified tags fall back to defaults [C], but every sample explicitly lists its tags, so emit them explicitly.

---

## 5. Naming rules that decide whether a template imports (details: `action-index.md` §2)

1. **Exact strings.** A block's `action` is the dump `name` byte-for-byte. 38 names are padded with spaces (`" RoundNumber "`, `" SetBlock "`, `" Range "`); never trim them.
2. **Padded twin = current, plain twin = legacy** (29 of 35 pairs). Each entry in `actions/*.md` carries its own LEGACY flag and replacement; never emit a legacy action.
3. **Alias-as-subAction.** For the 19 `if_*` actions whose names collide across if-blocks (`IsNear`, `NameEquals`, `StandingOn`, …), `subAction` and the tag `action` field use the **alias** (`PIsNear`, `EIsNear`, …), because the dump leaves `subAction` empty for them.
4. **Identifiers for data items:** game value `type` = display name without colour codes; particle/potion/sound = `minecraft:` + id (item version 1).
5. **Slots:** slot n = n-th typed argument (plural args absorb following slots); tags right-aligned at the slot numbers the dump gives; the editor exports items in ascending slot order. If an optional argument is omitted the editor may compact later items down — fine when types disambiguate, but keep real gaps between same-typed optionals (e.g. `SetAllCoords` X/Y/Z).

### 3.2 `OR` argument groups [S, sample 20]

`if_player " StandingOn "` takes a block **or** a location (`BLOCK` OR `LOCATION` in the dump). Filled both ways, **each alternative sits at slot 0**: an `item` (oak_log) in one block, a `loc` in the other. So when an `OR` group starts the argument list, number every alternative from 0. A group with an argument *before* the `OR` is untested. In that sample the location has `isBlock: true` with half-block coordinates (27.5, 49.5, 26.5): a block-centre location. `isBlock` only changes how the location is drawn in development mode [U].

