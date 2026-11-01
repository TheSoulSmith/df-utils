# DiamondFire language semantics

How programs execute: threads, selection, functions, variables, limits.

Confidence tags used throughout: **[T]** tested in-game by the user (ground truth, wins over everything) · **[A]** actiondump patch 8.13 (authoritative for names/tags/args) · **[S]** observed in the user's real sample templates (authoritative for wire format) · **[D]** official DiamondFire docs · **[C]** community source (forum/wiki/third-party tooling; may be dated) · **[U]** stated by the user · **[P]** proposal, not known to be implemented · **[K]** my prior knowledge, unverified — say so to the user when a template depends on one.

## 1. Mental model

DF is a Minecraft Java server where programs are built from physical *code blocks* placed in a dev space. A program is a set of **code lines**. Each line = one **header block** (event, function, or process) followed by a body of blocks, with **brackets** around nested bodies. [C: Blackstone/pyre docs list the block kinds; "all code must be inside an Event, Function, or Process"]

Execution model (simplified algorithm):

```
trigger fires (event | call_func | start_process):
    thread.selection = NONE                       # NONE (no selection) is different from EMPTY (a selection with 0 targets)
    pc = first block after the header
    while pc < end_of_line:
        blk = blocks[pc]
        if blk needs a target (player/entity action, if_player, if_entity):
            for each t in resolve_targets(blk): run blk on t        # an empty target list -> the block runs 0 times
        else (set_var, if_var, game_action, repeat, control, call_func...):
            run blk exactly once                                    # even with an empty selection [C]
        if blk is a conditional and result is false:
            pc = index after matching '}' ; then if next block is 'else', run its body instead
        if blk is suspending (e.g. control Wait, Load Bucket): yield thread, resume later
        pc += 1

resolve_targets(blk):                              # [U] for the absent-target rule
    if blk.target is absent:                       # e.g. sample 6's PlaySound
        return thread.selection if thread.selection is not NONE else {default target}
        #      ^ NOT simply "the default target": an active selection wins, and an EMPTY selection still counts as a selection
    if blk.target == "Selection":                                  # [T] with NO selection the message still reached the player
        return thread.selection if thread.selection is not NONE else {default target}
    else: the named target (e.g. "AllPlayers")                     # other names unverified

select_obj Reset:  thread.selection = NONE        # [A] "Deactivates the selection. Code that follows will run normally with event targets."
```

- **Selection** = the list of entities that targeted blocks iterate over; set by `select_obj`. **Default target** = the player/entity that started the event (or the one given when a process is started). [C][U]
- **Consequence for generated code:** a targeted block with no `target` key silently follows any selection that is still active. Always finish a selection with `select_obj Reset` before blocks meant for the event's own target, or give those blocks an explicit target. An *empty* selection (e.g. nobody was near) makes later untargeted blocks do nothing, not fall back to the default.
- With **no selection at all**, text codes like `%default` resolve normally; in a *selectionless* thread (`start_process` Target Mode `With no targets`) `%default` does not resolve as expected. [C]
- Function vs process **[A]**: `call_func` — "code will not continue past this block until the Function completes/returns" (`control Return` exits early; it also happens automatically at the end). `start_process` starts a **separate** thread: waits inside the process do not stop the starting line. The dump warns that processes are much more CPU-intensive than functions and are not a replacement for them.
- `start_process` has two tags **[A]**: *Target Mode* (slot 26: `With current targets` (default) · `With current selection` · `With no targets` · `For each in selection` — one process per selected target, which becomes each process's default target) and *Local Variables* (slot 25: `Don't copy` (default) · `Copy` · `Share`). This is how thread-local state moves between threads.

---

## 2. Variables

### 2.1 Scopes

| JSON `scope` | Name | Lifetime | Status |
|---|---|---|---|
| `unsaved` | Game | until plot restart, shared | [C] (pyre: "unsaved = game") |
| `saved` | Saved | persists, loaded with the plot | [C] |
| `local` | Local | thread-wide: visible across function calls within the same thread; copied/shared/not-copied into started processes via the Start Process *Local Variables* tag | **[S]** usage + **[A]** (tag) |
| `line` | Line | one code line/function invocation; function parameters live here | **[S]** + [C] |
| *(bucket var item)* | Bucket | persistent, dynamically loaded, can be cross-plot | [D] |

Evidence for the local/line distinction **[S]**: `cpu_limit` stores `limiter.deadline` as **local** because it must survive between separate calls and be shared by whatever function is running; its temporaries (`players`) are **line**. All parameters (`limiter.max_cpu`, `Pos`, `Seed`…) are read as **line** vars.

Variable and function names may contain dots (`call.barrier`, `limiter.deadline`) and are case-sensitive in practice (`limiter.deadline`, `Pos` vs `pos` are different) **[S: dotted names; case K]**.

Unset variables read as `0`/empty **[K]** — which is why `cpu_limit`'s first call (deadline unset) is expected to take the "over budget" branch.

**Name sharing [C, DFOnline wiki "Variable"]:** a GAME (`unsaved`) variable shares its name space with SAVED variables — setting a GAME variable while a SAVED variable of the same name exists modifies the SAVED one. Avoid reusing a name across those two scopes. Variable **names** accept `%` codes, which is how per-player variables are made (`Score %default` → one variable per player).

### 2.2 Numbers, lists, limits

- **Numbers** are fixed-point with **three decimal places**, stored as a 64-bit signed integer ×1000 (`Bitwise` tag text: "long / 1000"; forum: "up to the thousandths") **[A][C]**. Precision below 0.001 is lost; the representable magnitude is about ±9.2×10¹⁵.
- **Indexing starts at 1 everywhere**, including `%index`, list actions and `Starting index Default = 1` on `IndexOfSubstring` **[U][A]**.
- **Plot limits [C, DFOnline wiki, ~6 months old]:** string 10,000 chars · list 10,000 values · dictionary 5,000 · local/line variables 50,000 · game variables 500,000 · saved ≈ 4–7 MB.
- List/dictionary values are held in variables (there is no list or dict *item* type; see wire-format.md §3).


### 2.3 Bucket variables [D]

Persistent storage that is loaded/unloaded on demand, for more total data and cross-plot access.
- Hierarchy: **Namespace** (globally unique name; owner controls which plots may use it) → **Bucket** (group of vars loaded/saved together) → **Bucket Variable** (name + value).
- Each plot maps a namespace to an **alias** (aliases are per-plot, so dev and main plots can share code but not data).
- Set-Variable actions: **Load Bucket** (suspending), **Save Bucket**, **Save and Unload Bucket**, **Get Loaded Buckets**. Once loaded, bucket-var items behave like saved vars.
- Commands: `/plot namespace create <namespace> <alias>`, `/namespace whitelist add <namespace> [plot]`, `/bvar [alias] <bucket> <name> [amount]`, `/p namespace setdefault <alias>`, `/namespace remove <namespace>` (deletes all its data, irreversible).
- Typical per-player pattern: Load `data-%uuid` on join → Save periodically → Unload on quit. If players leave during a LagSlayer freeze, use Get Loaded Buckets to unload the stragglers.
- Limits: load/save/unload are rate-limited (~1 op/s average, bursts allowed, queued); 4 namespaces per player, 100 MB per namespace, 100k buckets per namespace, 64 KB per bucket (not raisable), bucket name 1–256 chars, namespace name 4–256, alias 1–128.

---

## 3. Functions

- **`Is Hidden`** (function/process tag): whether the function or process is hidden from the menu used to choose one for Call Function / Start Process; hidden ones remain callable [U].
- **Function names may be built at runtime** from `%` codes (dynamic dispatch, patterns-and-samples.md §12). Calling a function that doesn't exist is a silent no-op, the same as calling an empty one [U].
- **Optional parameters** can carry a default value (`default_value` in the parameter item); a caller may then omit the argument.

```
func <name>            data = "<name>"
    args: [pn_el params at slots 0..], optional [item icon at slot 0 — see below], [hint at 25], [tag Is Hidden at 26]
```
- Parameters: `pn_el` items in slot order = positional parameter order. **[S]**
- `cpu_limit` shows richer form: slot 0 holds an `item` (a repeating command block with custom name + lore) = the function's display icon, slot 1 the parameter (with `description` and `note` text). So icon/description are optional presentation. **[S]; purpose [K]**
- Functions without an icon use the `hint` item at slot 25 instead. **[S]** A **process** header carries no hint item at all, only the `Is Hidden` tag [S, sample 16].
- `call_func`: `data = <name>`; args are positional values/variables matched to the callee's parameters. **[S]**

---

### 1.1 Delivery order [T]

Messages and other actions run and arrive **in program order**, whatever their `target` form (round-2 series O01-O10: absent target with and without a selection, `Default`, `AllPlayers`, `Selection` with and without a selection, after `Reset`). An apparent delay seen in round 1 was caused by the test template's own block order, not by the engine; there is no ordering caveat for selection-targeted blocks.

Confirmed in-game [T] (rounds 1 and 3):

| Situation | Untargeted block | `target: Selection` | `target: Default` / `AllPlayers` |
|---|---|---|---|
| **no selection** (start of an event, or after `Reset`) | acts on the default target (fires) | still reaches the player (falls back to the default target) | fires |
| **empty selection** (a `select_obj` matched nobody) | **does nothing** | **does nothing** | **fires** (explicit targets ignore the selection) |
| **non-empty selection** | acts on the selection | acts on the selection | fires |

So *no selection* and *empty selection* are genuinely different states, and an empty selection silently disables every untargeted block until `select_obj Reset`.

