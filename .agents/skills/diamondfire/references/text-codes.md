# DiamondFire text (`%`) codes

Consolidated reference. The actiondump has no `%`-code documentation, so everything here comes from the user's in-game tests, the user's quotations from the official reference book, DiamondFire's samples, and community sources.

Confidence tags: **[T]** tested in-game by the user (ground truth; wins over everything else) · **[U]** stated by the user (incl. quotes from the official reference book) · **[S]** used in a real sample template · **[A]** actiondump · **[C]** community source, possibly dated · **[P]** proposal · **[?]** unverified.

**General rule [T]: a code that is unsupported or cannot resolve is left in the output as literal text.** (`%empty`, `%{1+2}`, and target codes in a no-targets process all came out as raw `%...` text.) So an unexpected raw `%...` in chat means "not supported here".

---

## 1. Where text codes are evaluated

| Value type | Works? | Notes |
|---|---|---|
| `num` (number) | yes: **this is where arithmetic happens** | the number item's `name` is a string, so it can hold `%math(...)` [S][T] |
| `txt` (string), `comp` (styled text) | yes, **but `%math` here only concatenates with `+`** (no numeric operations); every other code works normally [T] | use a **number** item for any arithmetic, then print the variable |
| variable **names** | **yes, resolved when the variable is set and when it is read** (`dfScore%default` becomes `dfScore<player name>`; one variable per player) | [T] round 3 N01 |
| **function / process names** (`call_func` / `start_process`) | yes; wire form: the code sits in the call's `data` string (sample 11: `call.%var(call)`) | enables dynamic dispatch, see patterns-and-samples.md §12 |
| `loc`, `vec`, `snd`, `pot`, `part` items | not applicable (structured data) | |

**Literal `%` [T, round 3]:** no escaping is needed. A `%` that does not start a recognised code prints exactly as written: `50% done`, `100%`, `a%b c`, `5%(x)`, `unknown %nosuchcode(1) here`, and a double `%%` stays `%%` (it is **not** collapsed to one). **There is no backslash escape**: `\%default` printed `<#interpolate:0>` (an internal placeholder), not the literal text. No way to print a *recognised* code name literally was found.

**Spaces and line breaks in text [T]:** `%space` and `%n` do not exist. Use **`<space>`** and **`<newline>`**; they work in both strings and styled text.

---

## 2. Target codes (resolve to a target's name; most have a UUID twin)

| Code | Meaning | Source |
|---|---|---|
| `%default` | name of whatever (player or entity) triggered the event; usually clear from the event's description | [U] official reference book |
| `%damager` | the damager, in damage-related events | [U] |
| `%killer` | the killer, in death-related events | [U] |
| `%shooter` | the player/entity that shot (or is credited with shooting) the projectile, in projectile events | [U] |
| `%victim` | the player/entity that took damage or was clicked | [U] |
| `%projectile` | the projectile, in projectile-related events | [U] |
| `%selected` | the selected entity. **With no selection it falls back to `%default`** | [U]; fallback [T] |
| `%<target>uuid` | every target has a UUID twin: `%defaultuuid`, `%damageruuid`, `%killeruuid`, `%shooteruuid`, `%victimuuid`, `%projectileuuid` | [U] |
| `%uuid` | UUID of the selected entity (**not** `%selecteduuid`). **With no selection it falls back to the default target's UUID** | [U]; fallback [T] |

Behaviour:
- **No-targets thread [T]:** in a process started with Target Mode `With no targets`, the target codes are **not resolved**; they appear as raw text.
- With a selection of yourself, `%default`/`%selected`/`%uuid` all resolve to you (test T19: received; exact text not reported, so treat as expected **[?]**).
- To get UUIDs as data, prefer the game values `UUID` and `Selection Target UUIDs` (`game-values.md`) [S][A].
- Events and targets added after the reference book was written may have more codes **[?]**.

---

## 3. Functional codes

| Code | Behaviour | Source |
|---|---|---|
| `%var(name)` | value of variable `name`. **Reads every scope.** If the same name exists in several scopes, **the `line` scope wins**. An undefined variable gives **0**. The name can contain other codes: `%var(dfScore%default)` reads the variable named for the current player. **Names are not trimmed**: `%var(dfScore %default)` is a different name and gave 0. | [T] |
| `%index(varname,i)` | element `i` of a multi-valued variable. Works on lists **and locations (order: x, y, z, pitch, yaw)**. **1-based.** **Any out-of-range index returns `0`** (0, 4, a negative index, the 6th element of a location). Takes the variable's *name* (which may itself contain codes). Also works on a **game-scope** list: `%index(gBotRunWhen_%var(Group),%var(botIndex))` in the real bots program (sample 22). | [T] (`[10] [30] [0] [0]` for 1,3,0,4 on a 3-element list; location `[10] [50] [10] [0] [0] [0]`) |
| `%entry(dictname,key)` | value under `key` in the dictionary variable. **Spaces are NOT trimmed**: `%entry(d, k)` looks up the key `" k"`. A **nonexistent key returns `0`**. | [T] |
| `%math(expr)` | arithmetic **only inside number items**; in strings it only concatenates with `+` (§4) | [T] |
| `%random(min,max)` | random integer, **both bounds inclusive**; spaces around the bounds are trimmed. **Reversed bounds work** (`%random(5,1)` returned 3, i.e. a value in 1..5). | [T] |
| `%round(number)` | **rounds toward zero**: floor for positive numbers, ceiling for negative (`%round(-2.5)` → -2; `%round(3)` = 3, `%round(-3)` = -3). **This contradicts the official reference book, which says floor.** For other modes use the `" RoundNumber "` action (`Floor`/`Nearest`/`Ceiling`). Whether `%round(-0.5)` prints `0` or `-0` was not distinguished. | [T] |
| `%bvar(n,b,var)` | value of a bucket variable: namespace `n` (optional; plot default if omitted), bucket `b`, variable `var`. With the namespace omitted the user expects `%bvar(b,var)`. Untested. | [U], [?] |
| `%empty` `%space` `%n` | **do not exist** (stay raw text). Use `<space>` / `<newline>`. | [T] |
| `%{...}` | **does not exist** (stays raw text); it was only a proposal. | [T] |

Codes **nest** (e.g. `%math(%var(a)+%math(%var(b)*2))`) [C][S].

### Spaces inside `%` codes [T]

The pattern the user found: **spaces are trimmed from numeric arguments but not from string arguments.**
- Trimmed: `%random` bounds, `%index`'s index (`%index(L, 1)` works).
- **Not** trimmed: `%var` names, `%entry` keys and dictionary names, and (by the same pattern) other name/key arguments, so `%index( L,1)` would look for a variable named `" L"` **[?]**.
- Rule for generated code: **don't put spaces inside `%` codes.** It is always safe and only matters for string arguments.

**1-based everywhere [U][T]:** the first element is index 1 (`%index`, list actions, substring start).

---

## 4. `%math`

**In a string or styled-text value [T]:** only `+` works, and it **concatenates**. Do not expect arithmetic there.

**In a number item [T, round 2]:** real arithmetic with these rules (results not individually reported matched the expected values):

| Rule | Evidence |
|---|---|
| **Strictly left to right, no operator precedence**: `%math(2+3*4)` = 20 | R01 |
| Operators: `+ - * /` and **`%` = modulo** (`7%3` = 1). **No exponent operator** (`^` is unsupported). | R03, R11 |
| Unary minus works (`-5+2` = -3); subtraction is left-associative (`8-2-3` = 3); `%var(...)` works inside | R07-R09 |
| Division: `10/4` = 2.5; **`1/0` = 0** (no error) | R02, R10 |
| **Three-decimal fixed point, and every intermediate result is truncated to 3 decimals**: `1/3` = 0.333; `0.1+0.2` = 0.3; **`10/3*3` = 9.999, not 10** | R04-R06 |

Consequences for writing formulas:
- **Multiply before you divide** (`a*b/c`, not `a/c*b`), otherwise the truncated quotient loses precision.
- Powers need another route: repeated multiplication, or the `Exponent` Set Variable action (a real action in the dump).
- Guard divisors yourself if a zero divisor would matter, since `x/0` silently gives 0.
- Because there is no precedence, group explicitly. Algorithm (what the real samples do):

```
emit(expr):
    if expr is a literal or variable:  return literal  /  "%var(name)"
    group consecutive operators of the SAME precedence class (all * and /, or all + and -) into one %math, left to right
    wrap any sub-expression of a different class in its own %math(...)
```

Worked example from sample 1 (`cpu_limit`):

```
budget = (40 + (players - 1) * 6) * 6.25 * max_cpu
%math( %math( 40 + %math( %math(%var(players)-1) * 6 ) ) * 6.25 * %var(limiter.max_cpu) )
          A          B     C: (players-1)         D: *6          outer: A * 6.25 * max_cpu (all * -> one group)
```

Number model **[A][T]**: fixed-point, three decimal places, 64-bit signed integers ×1000 (`Bitwise` tag text: "long / 1000").

---

## 5. In-game tests

The user runs templates in-game and pastes back the chat output, so **ask rather than guess**. `python3 scripts/make_test_templates.py` generates the templates (`assets/tests/`): rounds 1-3 are done (`--round 1|2|3`). Record every result below.

### Round 1 results (user-reported)

| ID | Test | Result | Conclusion |
|---|---|---|---|
| T01-T04 | `%math` arithmetic | not valid | ran in styled text, where `%math` only concatenates; redone in number items as R01-R11 |
| T05/T06 | `%round(-2.5)`, `%round(2.7)` | toward zero | floor for positive, ceiling for negative |
| T07/T08 | `%random` | both 1 and 2 appeared | inclusive bounds |
| T09 | `%var` per scope | all scopes resolve | `%var` reads every scope |
| T10 | `%index(L,1)`, `(L,3)`, `(L,0)`, `(L,4)` | `[10] [30] [0] [0]` | 1-based; out of range returns 0 |
| T11 | `%index(L, 1)` | trimmed (works) | spaces trimmed in numeric args |
| T12 | `%index(loc,1..6)` | `[10] [50] [10] [0] [0] [0]` | x, y, z, pitch, yaw; 6th = 0 |
| T13 | `%entry(d,k)` vs `%entry(d, k)` | spaces kept in the key | not trimmed (string arg) |
| T14 | `%random(1, 1)` | trimmed | numeric args trimmed |
| T15 | `%selected`, `%uuid` with no selection | fall back to the default target | confirmed |
| T16 | target `Selection` with no selection | message **received** | `Selection` with no selection still reaches the player (falls back to the default target) |
| T17 | untargeted block after an empty selection | not received (**confirmed by round-3 E02**) | an empty selection still counts as a selection |
| T18 | untargeted block after `Reset` | received | Reset restores default targeting |
| T19 | untargeted block with yourself selected | received | |
| T20 | `%empty %space %n` | raw text | do not exist; use `<space>` / `<newline>` (both work in strings and styled text) |
| T21 | `%{1+2}` | raw text | proposal never shipped |
| T23 | target codes in a no-targets process | raw text | target codes don't resolve without targets |

**Correction (found after round 2):** round 1 appeared to show T16/T18/T19 arriving late. That was an artifact of the test template, which emitted them *after* T20/T21 and the loop (program order). Round 2's ordering series (O01-O10, every `target` form) arrived in order, so **message delivery order is normal**; no ordering caveat applies.

### Round 2 results (user-reported; only unexpected results were reported, the rest matched the expected column)

| ID | Test | Result |
|---|---|---|
| R01 | `%math(2+3*4)` in a number | 20: left to right, no precedence |
| R03 | `%math(7%3)` | 1: `%` is modulo |
| R05 | `%math(10/3*3)` | 9.999: intermediates truncated to 3 decimals |
| R10 | `%math(1/0)` | 0 |
| R11 | `%math(2^3)` | unsupported (powers don't exist in `%math`) |
| R12 | same name in line/local/game/saved | **line** wins |
| R13 | undefined variable | 0 |
| R14/R15 | `%index` negative / `%entry` missing key | 0 (out-of-range and missing key return 0) |
| R16 | `%random(5,1)` | returned 3, so reversed bounds work |
| R02, R04, R06-R09, R17, R18 | other numeric and edge cases | matched expectations |
| O01-O10 | message order by target form | **correct order** |

### Round 3 results (from the Minecraft log's `[CHAT]` lines)

| ID | Test | Result | Conclusion |
|---|---|---|---|
| E01 | marker, `target: Default` | seen | |
| E02 | **untargeted** block after an **empty** selection | **not seen** | an empty selection still counts as a selection: nothing runs |
| E03 | `target: Selection` with an **empty** selection | **not seen** | explicit `Selection` also hits nobody |
| E04 | `target: Default` with an empty selection | seen | explicit `Default` ignores the selection |
| E05 | `target: AllPlayers` with an empty selection | seen | |
| E06 | untargeted block after `Reset` | seen | `Reset` restores default targeting |
| P01-P07 | literal `%` | `50% done`, `100%`, `double %% sign`, `a%b c`, `unknown %nosuchcode(1) here`, `5%(x)` printed as written; `\%default` printed **`<#interpolate:0>`** | no escape needed; `%%` is not collapsed; a backslash does not escape |
| N01 | `%var(dfScore%default)` after setting `dfScore%default` = 5 | 5 | codes in variable names resolve on both set and read. (My N01 label text itself contained `%default`, which printed as the player's name, a nice confirmation that `%default` resolves to the player name.) |
| N02 | `%var(dfScore %default)` | 0 | a space makes a different name; names are not trimmed |

Combined with round 1 (T16: `Selection` with **no** selection **did** reach the player): *no selection* and *empty selection* behave differently for `target: Selection` (falls back to default vs hits nobody).

**Still untested:** `%bvar` (needs a loaded bucket variable; ask the user to try `%bvar(bucket,var)` and `%bvar(namespace,bucket,var)`), `%call`, and printing a recognised code name literally.

---

## 6. Related limits (DFOnline wiki, plot page; snapshot ≈ 6 months old) **[C]**

String/text length 10,000 characters · list length 10,000 values · dictionary length 5,000 values · local/line variables 50,000 · game variables 500,000 · saved variables ≈ 4–7 MB · item tags 500. Text-code expansion happens inside those limits, so a long `%math` chain building a string can hit the 10,000-character cap.

---

## 7. Sources

- **In-game tests by the user** (rounds 1-3, tables above; round 3 reported as Minecraft log `[CHAT]` lines) **[T]**
- User-quoted official DiamondFire reference book (target codes + uuid variants, `%random`, `%round`, `%bvar`) **[U]**; where tested, the tests win (`%round`)
- Forum "Reference Book" (≈2020): https://mcdiamondfire.com/threads/reference-book.396/ **[C]**
- Forum "`%{}`" proposal: https://mcdiamondfire.com/threads/915/ **[P]** (never shipped, per tests)
- DFOnline wiki "Variable", "Selection", "Plots" (search snippets): https://wiki.dfonline.dev/wiki/Variable , /wiki/Selection , /wiki/Plots **[C]**
- Embyr post (numbers may hold text like `%math(2+3)`): https://mcdiamondfire.com/threads/embyr-a-text-representation-for-diamondfire-code-blocks-and-a-compiler-for-it.5715/ **[C]**
- Sample templates **[S]**; actiondump patch 8.13 **[A]**.
