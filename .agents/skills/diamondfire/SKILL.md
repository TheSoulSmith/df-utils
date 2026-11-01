---
name: "diamondfire"
description: Read, explain, write, build, lint and debug code templates for the MCDiamondFire (DiamondFire, "DF") Minecraft server's block-based programming language. Covers template codes (base64+gzip strings that start with "H4sI"), code blocks (Player/Entity/Game Event, Function, Process, Set Variable, Select Object, Repeat, If ...), actions, tags, game values, particles/potions/sounds, variable scopes, selections, and the "%" text codes (%var, %index, %entry, %math, %random, dynamic function calls by name). Includes the full CodeClient actiondump (1,201 actions), a template linter, and a Python builder. Use this whenever the user mentions DiamondFire, DF code, a codeblock/plot/template, CodeClient, actiondump.json, pastes a string beginning "H4sI", asks how to do something in DiamondFire, or wants a DiamondFire script generated, translated from another language, turned into pseudocode, or checked for errors, even if they never say "skill" or "template".
---

# DiamondFire templates

DiamondFire programs are chains of code blocks. They travel as **template codes**: JSON, gzip-compressed, base64-encoded (always starts `H4sI`). This skill lets you read them, write them, and check them against the real action list so what you hand over actually imports.

All paths are relative to this skill's directory. Run scripts with `python3 scripts/<name>.py`.

## Decide what the user needs

| User wants | Do this |
|---|---|
| "What does this template do?" / pastes `H4sI…` | **Read**: decompile, lint, explain (below) |
| "Write / generate / port this to DiamondFire" | **Write**: look up → build → lint → deliver (below) |
| "Why doesn't this work?" | **Debug**: lint first (most import failures are wire-format errors), then logic |
| "Which action does X?" / "what tags does Y have?" | **Look up**: grep `references/actions/` (below) |
| Questions about `%var`, `%math`, `%index`… | `references/text-codes.md` |
| User supplies a newer `actiondump.json` | **Update** (last section) |

## Read a template

1. `python3 scripts/df_tools.py "<code>"` (or a file path, or `-` for stdin) prints an indented outline: block, action, slotted arguments, tags, brackets as `{ }`. `--json` prints the raw JSON.
2. `python3 scripts/df_lint.py "<code>"` reports structure/action/tag problems. A real, working template should lint with 0 errors; INFO lines (rank requirements, calls to functions defined elsewhere) are normal.
3. Look up unfamiliar actions in `references/actions/*.md` (see Look up).
4. Explain it as an algorithm: inputs (parameters), state (variables and their scopes), control flow (loops, branches, selection changes), outputs/side effects. Convert `%math(...)` nests back to ordinary formulas and state units (ticks, µs, blocks). Samples with worked examples: `references/patterns-and-samples.md`.
5. Say what you couldn't verify (calls to functions not in the template, rank-gated actions).

## Write a template

1. **Sketch the logic first** as pseudocode: header (event / function / process), variables and scopes, loops, branches, selection changes.
2. **Look up every action** you'll use and read its Args and Tags. Do not rely on memory for names: many are padded with spaces, some have legacy twins, and `x` is multiplication.
3. **Build it with `scripts/df_build.py`** instead of hand-writing JSON. It fills tags with the right slots, defaults and owner fields from the dump, and refuses legacy/unknown actions:

```python
import sys; sys.path.insert(0, "scripts")
from df_build import Template, num, txt, var, gval, snd, pot, part, mc_item, param

t = Template()                                        # bundled dump (assets/actiondump.json)
t.header("event", "PlayerDmgPlayer")                  # or entity_event / game_event / ("func", "name", params=[param("x","num")]) / ("process", "name")
t.block("player_action", "PlaySound", [snd("block.chain.hit")])            # no target = selection if any, else default
t.block("player_action", "GivePotion", [pot("speed", 100, 1)], target="Selection", tags={"Show Icon": "False"})
with t.cond("if_var", "<", [var("n"), num(10)]):                           # if { ... }
    t.block("set_var", "+=", [var("n"), num(1)])
with t.else_():                                                            # else { ... }
    t.block("set_var", "=", [var("n"), num(0)])
with t.repeat("Multiple", [num(5)]):                                       # loop var omitted: count in slot 0 (as exported)
    t.block("control", "Wait")
t.block("select_obj", "PlayersCond", [gval("Location"), num(5)], sub="PIsNear", tags={"Shape": "Circle"})
t.block("select_obj", "Reset")
print(t.encode())                                     # the template code
for f in t.lint(): print(f)                           # also run automatically by df_lint.py
```

   Builder facts: arguments are positional (list index = chest slot, `None` leaves a gap); `target=` is written only when given; `negate=True` writes `attribute: NOT`; `sub=` takes a condition name or alias for `select_obj`. Item constructors: `num txt comp var bucket_var gval mc_item loc vec snd pot part param`, all verified against real exports (`param(..., default=txt("x"))` makes an optional parameter with a default; parameter types: `txt comp num loc vec snd part pot item dialog any var list dict`; `plural=True`/`optional=True`; `t.block("start_process", name)` for starting processes). There is no `r_loc` constructor (not obtainable by players). `t.repeat("While", args, sub="=", negate=True)` and `t.cond(..., negate=True, target=...)` cover loops and negated conditions.
4. **Lint**: `t.lint()` or `python3 scripts/df_lint.py "<code>"`. Fix every ERROR; read every WARN.
5. **Deliver**: the template code in a fenced block, then a short outline of what it does, then a plain list of assumptions: anything the lint reported as INFO, rank-gated actions, functions it calls that must exist, any `[K]` item from `references/known-gaps-and-sources.md` the template depends on, and that it has not been run in-game.

If the user needs something not covered by a real sample (see `references/known-gaps-and-sources.md`: e.g. an `OR` group with an argument before the `OR`, an optional **and** plural parameter), say that wire shape hasn't been verified and either ask for an exported example or, if it is a behaviour question, offer an in-game test (below). A template holds exactly one code line.

## Modify an existing template

1. `python3 scripts/df_to_builder.py "<code>" > edit.py` turns any template into builder code (readable `with t.cond(...)` structure; only non-default tags are written). `--check` verifies the conversion rebuilds the original exactly; it does for all 27 real samples.
2. Edit `edit.py` (change a value, insert/remove blocks, rename a function), keeping edits minimal so unchanged parts stay identical.
3. Run it (`python3 edit.py` prints the new template code), then lint (`python3 scripts/df_lint.py "<code>"`).
4. Deliver the code plus a short list of what changed. If you removed or renamed something other templates call, say so.

For large programs (several functions per template line), explain the architecture first: entry points, shared `local` state, data layout, scheduling. `references/patterns-and-samples.md` §14 is a worked example of a real multi-function program.

## Look up an action, tag, game value

- Names of every action by block: `references/action-index.md` §6 (⚠ = legacy).
- One action with args/tags/notes: `grep -n '^### `ActionName`' references/actions/*.md`, then read that entry (padded names appear in quotes, e.g. `" RoundNumber "`; grep for the stripped name).
- By purpose: `grep -n -i 'location' references/actions/set_var_*.md | head` to search descriptions.
- Game values: `references/game-values.md`; sounds/particles/potions: `references/sounds.md`, `references/particles-potions.md`.
- Actions are listed in the dump's own order, not alphabetically.

## Rules that decide whether a template imports or works

1. **Action names are exact strings**, spaces included (`" RoundNumber "`, `" SetBlock "`, `" Range "`). Never trim or "fix" them. Multiplication is `x`; power is `Exponent`; bitwise operators are a *tag* on `Bitwise`.
2. **Never use a legacy action.** Each entry shows `⚠ LEGACY → use …`; the padded twin is usually the current one.
3. **Alias-as-subAction.** For colliding condition names, `subAction` and the tag's `action` field use the alias (`PIsNear`, `EIsNear`). The builder enforces this.
4. **Tags belong in fixed slots** (26, then 25, 24…), with `action`/`block` fields naming their owner. For a block with a `subAction`, the owner is the sub-action's own block (e.g. `PIsNear` / `if_player`). The builder does this.
5. **Slot = argument position.** Plural arguments absorb following slots. Keep real gaps between same-typed optional arguments (`SetAllCoords` X/Y/Z). When an optional argument is omitted, the editor may compact later items down; both forms appear in real exports.
6. **Numbers are strings** (`"name": "8"`) and can hold `%` codes. Only `snd`/`part`/`pot` items are `version: 1` (Minecraft ids, `minecraft:` prefix); everything else is `version: 0`.
7. **Brackets:** `repeat` uses bracket type `repeat`, everything else `norm`; `else` must directly follow a closing `norm` bracket; else-if is `else { if … }`. Every line begins with a header block.
8. **Absent `target` is not "default target"**: a targeted block with no `target` acts on the **current selection if one exists** (even an empty one), otherwise on the default target. Explicit values (all seen): player actions `Selection Default Killer Damager Shooter Victim AllPlayers`; entity actions add `Projectile AllEntities AllMobs LastEntity`; `if_player`/`if_entity` also take a target. Write one when you mean it. After `select_obj`, always finish with `select_obj Reset` or later untargeted blocks keep following the old selection.
   **Verified table:** with *no* selection an untargeted block uses the default target and `Selection` falls back to it; after a selection that matched **nobody** (empty), untargeted blocks and `Selection` do **nothing** while `Default`/`AllPlayers` still fire; `Reset` restores the no-selection state.
9. **Variable scopes:** `line` (function-local; parameters live here), `local` (thread-wide, shared across function calls in a thread), `unsaved` (= game), `saved`. A function "returns" a value through a `var`-typed parameter. Details: `references/language-semantics.md`.
10. **Numbers** are fixed-point with 3 decimals (64-bit ×1000). Indexing starts at **1** everywhere.

## `%` text codes (short form; full table and test results in `references/text-codes.md`)

- **Arithmetic only works in number items.** In strings/styled text `%math` just concatenates with `+`. Store `%math(...)` in a `num` value, then print or use the variable. `%math` is **strictly left to right with no precedence** (`2+3*4` = 20), has `+ - * /` and `%` (modulo), **no power operator**, and `x/0` = 0. Numbers are 3-decimal fixed point and **every intermediate is truncated** (`10/3*3` = 9.999), so **multiply before dividing**. Nest one operator class per `%math`: `(a+1)*2` is `%math(%math(%var(a)+1)*2)`.
- `%var(name)` reads **any** scope (the `line` scope wins on a name clash; undefined = 0). `%index(var,i)`: **1-based**, any out-of-range index returns **0**, works on lists and locations (x,y,z,pitch,yaw). `%entry(dict,key)` (missing key = 0). `%random(min,max)` is inclusive and accepts reversed bounds. **`%round` rounds toward zero** (floor if positive, ceiling if negative). `%bvar([namespace,]bucket,var)` (untested).
- Target codes: `%default %selected %damager %victim %killer %shooter %projectile`, each with a UUID twin (`%defaultuuid`, …) except the selected entity's, which is `%uuid`. `%selected`/`%uuid` fall back to the default target when nothing is selected. In a **no-targets process** they don't resolve and print raw.
- **Spaces:** trimmed in numeric arguments (`%random` bounds, `%index` index), **kept** in string arguments (`%var` names, `%entry` keys). Never put spaces inside `%` codes.
- A literal `%` needs no escape (`50%`, `100%`, `a%b` print as written; `%%` stays `%%`); a backslash does **not** escape a code (`\%default` printed `<#interpolate:0>`). Codes in **variable names** resolve when setting and reading (`Score%default` = per-player variable; names are not trimmed).
- Space and line break are **`<space>` and `<newline>`** (strings and styled text); `%space`, `%n`, `%empty` and `%{...}` do **not** exist. Any unsupported code is output as raw text.
- `%` codes also work in **function and process names** (dynamic dispatch / softcoding): write the code directly in the call's name, e.g. `t.call("call.%var(call)")`. Convention: give such functions a shared prefix (`call.`, `bot.`) and mark them hidden. A call to a missing function does nothing. See `references/patterns-and-samples.md` §12.

## Reference map

| File | Read when |
|---|---|
| `references/wire-format.md` | building/parsing JSON by hand, tag owner rules, item shapes, naming rules |
| `references/language-semantics.md` | threads, selection, functions/processes, variable scopes, limits |
| `references/text-codes.md` | anything with `%`; includes an in-game test plan |
| `references/patterns-and-samples.md` | worked examples (CPU limiter, xorshift PRNG, Voronoi, terrain, barrier, event line, bot handlers) and idioms incl. dynamic dispatch |
| `references/action-index.md` | every action name by block; slot/arg-type rules |
| `references/actions/*.md` | args, tags, returns, flags per action (`set_var_1/2`, `player_action`, `entity_action`, `game_action`, `events`, `conditions`, `flow`) |
| `references/game-values.md`, `particles-potions.md`, `sounds.md` | identifiers for those item types |
| `references/known-gaps-and-sources.md` | what is unverified, and where facts came from |

Confidence tags used in the references: **[T]** tested in-game by the user (ground truth), **[A]** actiondump, **[S]** real sample templates, **[D]** official docs, **[C]** community source, **[U]** user-confirmed, **[P]** proposal, **[K]** unverified prior knowledge. Treat [K] and [?] as hypotheses and say so.

## In-game testing (the user has offered)

The user can run templates in-game and paste back the chat output, so **when behaviour is uncertain, test instead of guessing**: build a minimal template that prints labelled results (`SendMessage` lines), lint it, give the code plus a table of expected results, and ask for the output. For the `%` codes and selection rules, `python3 scripts/make_test_templates.py` already generates two ready-made templates (`assets/tests/`; the second is a process because a template holds one code line). Record findings in `references/text-codes.md` §5. `make_test_templates.py` has three rounds (`--round 1|2|3`, default 3); add tests by appending to its lists with the expected result beside each. **Emit test messages in the same order as their IDs**: an out-of-order layout once looked like a delivery delay and produced a false finding. Keep messages free of stray `%` so unsupported codes show up as raw text, and mention any variable the test leaves behind (saved variables persist).

Facts the user has confirmed in play: a call to a missing function does nothing; `start_process` tags behave like any other tag; `NOT` is the same `attribute` on every `if_*` block; the whole `%`-code behaviour table (round 1) is in `references/text-codes.md` §5.

## Update from a new actiondump

1. Replace `assets/actiondump.json` (CodeClient export).
2. `python3 scripts/df_dump_to_md.py assets/actiondump.json references` regenerates `action-index.md`, `actions/`, `game-values.md`, `particles-potions.md`, `sounds.md`.
3. `python3 scripts/selftest.py`: all twenty real samples must still lint clean and the builder must still reproduce those listed in `selftest.py`. A failure means the format or an action changed; review `references/wire-format.md` before trusting generated templates.

## Things not to do

- Don't edit a template by hand-patching its JSON or base64; go through `df_to_builder.py` so tags, slots and brackets stay valid.
- Don't claim a template was tested in-game; the linter checks structure against the dump, not behaviour.
- Don't invent action names, tag options, or item shapes. If it isn't in the dump or a sample, say so.
- Don't emit actions flagged with a required rank without telling the user (`INFO` lines from the linter).
