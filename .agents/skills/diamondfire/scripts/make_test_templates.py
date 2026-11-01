#!/usr/bin/env python3
"""Generate in-game test templates for the open DiamondFire questions, with a table of expected results.

Usage: python3 make_test_templates.py [--round 1|2|3] [OUT_DIR]      (default: round 3, ../assets/tests)

ROUND 1 (done; results are recorded in references/text-codes.md section 5):
  text-code-tests.md, test-main.txt (event line `Join`), test-process.txt (process `dfTestProc`, needed for T23)
ROUND 2 (done; numeric %math in number items, %var precedence, edge cases, message ordering): round2-tests.md, round2-main.txt
ROUND 3 (done: empty-selection targeting, literal % characters, % codes in variable names): round3-tests.md, round3-main.txt

LESSON: when a test prints results, emit the messages in the SAME order as their IDs. Round 1 emitted T16-T19 after T20/T21, which
looked like a delivery delay but was just program order; round 2's ordering series proved delivery order is normal.

How to use (give the user these steps): import both templates, place them, rejoin the plot (the main test runs on the Join
event) and paste the chat output back. Compare each line with the 'Expected' column; any difference is a finding to record in
references/text-codes.md. Add new tests by appending to TESTS below and re-running.

Message text never contains a literal % except inside the code being tested, so unsupported codes show up as raw text.
"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from df_build import Template, num, txt, comp, loc, var, gval

# (id, label, message-body-with-codes, expected-under-current-assumptions, what-it-settles)
TESTS = [
    ("T01", "math 2+3*4",            "[%math(2+3*4)]",            "20 (left to right); 14 would mean precedence", "%math precedence"),
    ("T02", "math 10/4",             "[%math(10/4)]",             "2.5", "division"),
    ("T03", "math 7%3",              "[%math(7%3)]",              "1 (modulo). Raw text or 0 = no % operator", "% as modulo inside %math"),
    ("T04", "math 1/3",              "[%math(1/3)]",              "0.333 (3-decimal fixed point)", "numeric precision"),
    ("T05", "round -2.5",            "[%round(-2.5)]",            "-3 (floor)", "%round on negatives"),
    ("T06", "round 2.7",             "[%round(2.7)]",             "2 (floor)", "%round floors"),
    ("T07", "random 1,1",            "[%random(1,1)]",            "1", "%random basic"),
    ("T09", "var scopes",            "line=[%var(dfTestLine)] local=[%var(dfTestLocal)] game=[%var(dfTestGame)] saved=[%var(dfTestSaved)]",
                                     "line=1 and the others EMPTY/0/raw if %var is line-only; numbers 2,3,4 would mean it reads that scope", "%var scope"),
    ("T10", "index list 1,3,0,4",    "[%index(dfTestList,1)] [%index(dfTestList,3)] [%index(dfTestList,0)] [%index(dfTestList,4)]",
                                     "10 30 then something for 0 and 4 (empty/0/raw?) - the last two are the finding", "%index bounds (1-based)"),
    ("T11", "index comma-space",     "[%index(dfTestList, 1)]",   "10 if spaces are trimmed; empty/0 if the space breaks it", "comma-space in %index"),
    ("T12", "index location 1..6",   "[%index(dfTestLoc,1)] [%index(dfTestLoc,2)] [%index(dfTestLoc,3)] [%index(dfTestLoc,4)] [%index(dfTestLoc,5)] [%index(dfTestLoc,6)]",
                                     "x y z pitch yaw then 6 = empty/0/raw", "%index on locations; 6th element"),
    ("T13", "entry key vs space",    "[%entry(dfTestDict,k)] [%entry(dfTestDict, k)]",
                                     "7 then EMPTY/0 (space becomes part of the key; the dict only has key 'k')", "%entry comma-space"),
    ("T14", "random comma-space",    "[%random(1, 1)]",           "1 if trimmed; raw/0 if the space breaks it", "comma-space in %random"),
    ("T15", "targets, no selection", "default=[%default] selected=[%selected] uuid=[%uuid] defaultuuid=[%defaultuuid]",
                                     "default=your name; selected=? (falls back to you?); uuid=?; defaultuuid=your UUID", "%selected/%uuid with no selection"),
    ("T20", "misc codes",            "a[%empty]b[%space]c[%n]d",  "if they exist: ab c<newline>d. Raw text means not supported", "%empty %space %n"),
    ("T21", "brace proposal",        "[%{1+2}]",                  "raw text (proposal expected NOT to exist)", "%{...}"),
    ("T22", "uuid of selected after select, see T19", "", "", ""),   # placeholder to keep ids stable; not emitted
]


def build_main():
    t = Template().header("event", "Join")
    # ---- setup
    t.block("set_var", "CreateList", [var("dfTestList"), num(10), num(20), num(30)])
    t.block("set_var", "CreateDict", [var("dfTestDict")])
    t.block("set_var", "SetDictValue", [var("dfTestDict"), txt("k"), num(7)])
    t.block("set_var", "=", [var("dfTestLoc"), gval("Location")])
    t.block("set_var", "=", [var("dfTestLine"), num(1)])
    t.block("set_var", "=", [var("dfTestLocal", "local"), num(2)])
    t.block("set_var", "=", [var("dfTestGame", "unsaved"), num(3)])
    t.block("set_var", "=", [var("dfTestSaved", "saved"), num(4)])
    say = lambda s, target=None: t.block("player_action", "SendMessage", [comp(s)], target=target)
    say("=== DF test run: compare each line with the Expected column ===")
    for tid, label, body, *_ in TESTS:
        if not body:
            continue
        if tid == "T09":      # scopes: also try reading by the same name from a different scope later if needed
            pass
        say(f"{tid} {label} => {body}")
    # ---- T08: random inclusivity
    say("T08 random(1,2) x20 (do BOTH 1 and 2 appear?):")
    with t.repeat("Multiple", [num(20)]):
        say("T08 [%random(1,2)]")
    # ---- T16: target Selection with no selection
    say("T16 sent with target=Selection while NO selection exists (expected: you probably do not see it; note either way)", target="Selection")
    # ---- T17/T18: empty selection then untargeted block
    t.block("select_obj", "PlayersCond", [loc(100000, 0, 100000), num(1)], sub="PIsNear")
    say("T17 untargeted block after an EMPTY selection (expected: you do NOT see this line)")
    t.block("select_obj", "Reset")
    say("T18 untargeted block after Reset (expected: you DO see this line)")
    # ---- T19: selection containing the player
    t.block("select_obj", "PlayersCond", [gval("Location"), num(5)], sub="PIsNear")
    say("T19 with yourself selected: default=[%default] selected=[%selected] uuid=[%uuid] (expected: all you / your UUID)")
    t.block("select_obj", "Reset")
    # ---- T23: process started with no targets (separate template)
    t.block("start_process", "dfTestProc", tags={"Target Mode": "With no targets"})
    return t


def build_process():
    t = Template().header("process", "dfTestProc")
    t.block("player_action", "SendMessage",
            [comp("T23 process with NO targets: default=[%default] selected=[%selected] uuid=[%uuid] (expected: %default does NOT resolve to you)")],
            target="AllPlayers")
    return t


def main(out):
    os.makedirs(out, exist_ok=True)
    main_t, proc_t = build_main(), build_process()
    for name, t in (("main", main_t), ("process", proc_t)):
        errs = [f for f in t.lint() if f[0] == "ERROR"]
        assert not errs, errs
    codes = {"test-main.txt": main_t.encode(), "test-process.txt": proc_t.encode()}
    for fn, code in codes.items():
        open(os.path.join(out, fn), "w").write(code + "\n")
    rows = [(i, l, b, e, s) for i, l, b, e, s in TESTS if b]
    extra = [("T08", "random(1,2) x20", "%random(1,2)", "a mix of 1 and 2 (both bounds possible = inclusive); only 1 = exclusive max", "%random inclusivity"),
             ("T16", "target Selection, no selection", "(Selection target)", "likely nothing shown; note what happens", "Selection with no selection"),
             ("T17", "untargeted after empty selection", "(untargeted block)", "NOT shown", "absent-target rule: an empty selection still counts"),
             ("T18", "untargeted after Reset", "(untargeted block)", "shown", "Reset restores default targeting"),
             ("T19", "targets with yourself selected", "%default %selected %uuid", "all resolve to you", "target codes with a selection"),
             ("T23", "process, no targets", "%default %selected %uuid", "%default does not resolve to you", "selectionless thread")]
    md = ["# In-game test templates for open DiamondFire questions", "",
          "Generated by `scripts/make_test_templates.py`. Both templates lint clean against the actiondump.", "",
          "## How to run", "",
          "1. Import **test-process** first (a process named `dfTestProc`), then **test-main** (an event line on `Join`).",
          "2. Rejoin the plot. The main template sets some variables (`dfTest*`; one is a **saved** variable), then prints every result to chat.",
          "3. Paste the chat output back. Differences from *Expected* are the findings.", "",
          "Notes: the saved variable `dfTestSaved` persists on the plot; delete it afterwards if you care. Tests T08/T16-T19/T23 involve several lines each.", "",
          "## Template codes", "", "**test-process**", "", "```txt", codes["test-process.txt"], "```", "", "**test-main**", "", "```txt", codes["test-main.txt"], "```", "",
          "## Tests and expectations (under current assumptions)", "", "| ID | Test | Codes | Expected | Settles |", "|---|---|---|---|---|"]
    for i, l, b, e, s in sorted(rows + extra, key=lambda r: r[0]):
        md.append(f"| {i} | {l} | `{b}` | {e} | {s} |")
    open(os.path.join(out, "text-code-tests.md"), "w").write("\n".join(md) + "\n")
    print("wrote", out, "| blocks:", len(main_t.blocks), "+", len(proc_t.blocks), "| lint errors: 0")




# =====================================================================================================================
# ROUND 2
# (id, label, expected, settles)  -- numeric tests store the %math result in a NUMBER item, then print the variable
NUM_TESTS = [
    ("R01", "2+3*4",            "%math(2+3*4)",            "20 if left-to-right; 14 if precedence", "%math precedence in a number item"),
    ("R02", "10/4",             "%math(10/4)",             "2.5", "division"),
    ("R03", "7%3",              "%math(7%3)",              "1 if % is modulo; 0 or garbage otherwise", "modulo operator inside %math"),
    ("R04", "1/3",              "%math(1/3)",              "0.333 (3-decimal fixed point)", "precision"),
    ("R05", "10/3*3",           "%math(10/3*3)",           "9.999 if intermediates are truncated to 3 decimals; 10 if not", "intermediate precision"),
    ("R06", "0.1+0.2",          "%math(0.1+0.2)",          "0.3", "decimal addition"),
    ("R07", "-5+2",             "%math(-5+2)",             "-3 (unary minus works)", "negative literals"),
    ("R08", "8-2-3",            "%math(8-2-3)",            "3 (left-associative); 9 would mean right-to-left", "associativity"),
    ("R09", "var+var",          "%math(%var(dfA)+%var(dfA))", "10 (dfA = 5)", "%var inside %math"),
    ("R10", "1/0",              "%math(1/0)",              "record whatever happens (0? error? raw?)", "division by zero"),
    ("R11", "2^3",              "%math(2^3)",              "likely unsupported (raw/0); 8 if a power operator exists", "exponent operator"),
]
TEXT_TESTS2 = [
    ("R12", "var scope precedence",  "[%var(dfDup)]",  "one of 1 (line) / 2 (local) / 3 (game) / 4 (saved): which scope wins when all four hold the same name", "%var precedence"),
    ("R13", "undefined variable",    "[%var(dfNoSuchVar)]", "0, empty, or raw text", "%var on a missing variable"),
    ("R14", "index negative/valid",  "[%index(dfTestList,-1)] [%index(dfTestList,2)]", "?, then 20", "%index with a negative index (0? last element?)"),
    ("R15", "entry missing key",     "[%entry(dfTestDict,nokey)]", "0, empty, or raw text", "%entry on a missing key"),
    ("R16", "random reversed bounds", "[%random(5,1)]", "record (1..5? 0? 5?)", "%random with min > max"),
    ("R17", "round edges",           "[%round(3)] [%round(-0.5)] [%round(-3)]", "3, then 0 or -0, then -3 (rounds toward zero)", "%round edge cases"),
    ("R18", "space/newline tags",    "a<space>b<newline>c", "a b / c on two lines", "re-confirm <space> <newline> in a comp"),
]


def build_round2():
    t = Template().header("event", "Join")
    say = lambda *items, target=None: t.block("player_action", "SendMessage", list(items), target=target)
    # ---- setup
    t.block("set_var", "CreateList", [var("dfTestList"), num(10), num(20), num(30)])
    t.block("set_var", "CreateDict", [var("dfTestDict")])
    t.block("set_var", "SetDictValue", [var("dfTestDict"), txt("k"), num(7)])
    t.block("set_var", "=", [var("dfA"), num(5)])
    for scope, val in (("line", 1), ("local", 2), ("unsaved", 3), ("saved", 4)):      # the same NAME in every scope
        t.block("set_var", "=", [var("dfDup", scope), num(val)])
    say(comp("=== DF test run 2: compare each line with the Expected column ==="))
    # ---- numeric %math inside NUMBER items
    for tid, label, code, *_ in NUM_TESTS:
        t.block("set_var", "=", [var("df" + tid), num(code)])
        say(comp(f"{tid} {label} => "), var("df" + tid))
    # ---- text-value tests
    for tid, label, body, *_ in TEXT_TESTS2:
        say(comp(f"{tid} {label} => {body}"))
    # ---- ordering isolation: the same message sent with different target forms; read the ARRIVAL ORDER against the O-number
    say(comp("--- ordering test: lines should arrive O01..O10; report any that arrive out of order ---"))
    say(comp("O01 absent target (no selection has been used yet)"))
    say(comp("O02 target=Default"), target="Default")
    say(comp("O03 target=AllPlayers"), target="AllPlayers")
    t.block("select_obj", "PlayersCond", [gval("Location"), num(5)], sub="PIsNear")
    say(comp("O04 absent target, selection ACTIVE (contains you)"))
    say(comp("O05 target=Selection, selection active"), target="Selection")
    t.block("select_obj", "Reset")
    say(comp("O06 absent target after Reset"))
    say(comp("O07 target=Selection with NO selection"), target="Selection")
    say(comp("O08 absent target again"))
    say(comp("O09 target=Default again"), target="Default")
    say(comp("O10 last line of the template"))
    return t


def main_round2(out):
    os.makedirs(out, exist_ok=True)
    t = build_round2()
    assert not [f for f in t.lint() if f[0] == "ERROR"], t.lint()
    code = t.encode()
    open(os.path.join(out, "round2-main.txt"), "w").write(code + "\n")
    rows = [(i, l, f"`{c}`", e, s) for i, l, c, e, s in NUM_TESTS] + [(i, l, f"`{b}`", e, s) for i, l, b, e, s in TEXT_TESTS2]
    rows += [("O01-O10", "message ordering", "same message, different `target` forms", "arrive in order O01..O10. Report any that arrive out of order",
              "whether any target form is delivered out of order")]
    md = ["# In-game tests, round 2", "",
          "Generated by `scripts/make_test_templates.py`. Round 1 results are recorded in `references/text-codes.md` §5; this round follows up what round 1 could not settle.", "",
          "## What changed from round 1", "",
          "- Round 1 ran `%math` in *styled text*, where only `+` (concatenation) works. Here every arithmetic test stores `%math(...)` in a **number** value, then prints the variable.",
          "- `%var` is known to read every scope; this round asks which scope **wins** when the same name exists in all four.",
          "- A new O-series sends the same message with different `target` forms so the arrival order shows which forms are deferred.", "",
          "## How to run", "",
          "1. Import **round2-main** (an event line on `Join`) and rejoin the plot. No process template is needed this time.",
          "2. Paste the chat output back. Differences from *Expected* are the findings.", "",
          "Side effects: variables named `dfTest*`, `dfA`, `dfR*`, `dfDup` are created; `dfDup` also exists as a **saved** variable (value 4) and persists on the plot, so delete it afterwards if you like.", "",
          "## Template code", "", "```txt", code, "```", "",
          "## Tests and expectations", "", "| ID | Test | Code | Expected | Settles |", "|---|---|---|---|---|"]
    md += [f"| {i} | {l} | {c} | {e} | {s} |" for i, l, c, e, s in rows]
    open(os.path.join(out, "round2-tests.md"), "w").write("\n".join(md) + "\n")
    print("wrote", out, "| round 2 blocks:", len(t.blocks), "| lint errors: 0")


# =====================================================================================================================
# ROUND 3
def build_round3():
    t = Template().header("event", "Join")
    say = lambda text, target=None: t.block("player_action", "SendMessage", [comp(text)], target=target)
    # setup for the variable-name test (the name contains a target code)
    t.block("set_var", "=", [var("dfScore%default"), num(5)])
    say("=== DF test run 3 ===", target="Default")
    say("--- E series: empty selection. Lines are in ID order; tell me which of E01-E06 you actually SEE ---", target="Default")
    say("E01 start marker, target=Default", target="Default")
    t.block("select_obj", "PlayersCond", [loc(100000, 0, 100000), num(1)], sub="PIsNear")      # nobody is there: EMPTY selection
    say("E02 absent target, EMPTY selection (expected: NOT seen)")
    say("E03 target=Selection, EMPTY selection (expected: NOT seen)", target="Selection")
    say("E04 target=Default, EMPTY selection (expected: seen)", target="Default")
    say("E05 target=AllPlayers, EMPTY selection (expected: seen)", target="AllPlayers")
    t.block("select_obj", "Reset")
    say("E06 absent target after Reset (expected: seen)")
    say("--- P series: literal percent characters. Report exactly what is printed ---", target="Default")
    for tid, body in (("P01", "50% done"), ("P02", "100%"), ("P03", "double %% sign"), ("P04", "a%b c"), ("P05", "unknown %nosuchcode(1) here"),
                      ("P06", "backslash \\%default"), ("P07", "percent then paren 5%(x)")):
        say(f"{tid} [{body}]", target="Default")
    say("--- N series: % code inside a VARIABLE NAME ---", target="Default")
    say("N01 var named dfScore%default was set to 5; reading it back: [%var(dfScore%default)] (expected 5)", target="Default")
    say("N02 same name typed with a space: [%var(dfScore %default)] (expected 0: a different name)", target="Default")
    return t


def main_round3(out):
    os.makedirs(out, exist_ok=True)
    t = build_round3()
    assert not [f for f in t.lint() if f[0] == "ERROR"], t.lint()
    code = t.encode()
    open(os.path.join(out, "round3-main.txt"), "w").write(code + "\n")
    md = ["# In-game tests, round 3", "",
          "Generated by `scripts/make_test_templates.py --round 3`. Rounds 1 and 2 are recorded in `references/text-codes.md` §5.", "",
          "## Why this round", "",
          "- **E series:** the earlier T17 result was only *inferred* (an untargeted block after an empty selection did not appear in the round-1 list). This round tests all target forms after an EMPTY selection and marks every line so there is nothing to infer.",
          "- **P series:** what a literal `%` prints (`50%`, `100%`, `%%`, `a%b`, an unknown code, a backslash escape). Needed when generating messages that contain percentages.",
          "- **N series:** a `%` code inside a **variable name** (per-player variables).", "",
          "## How to run", "",
          "Import **round3-main** (an event line on `Join`), rejoin, and paste the chat output. For the E series, just tell me which of E01-E06 appeared. The messages are emitted in ID order, so any out-of-order arrival would be real.", "",
          "Side effect: a line variable `dfScore%default` (nothing persists).", "",
          "## Template code", "", "```txt", code, "```", "",
          "## Expectations", "",
          "| ID | Test | Expected | Settles |", "|---|---|---|---|",
          "| E01-E06 | all `target` forms after an empty selection | E02 and E03 NOT seen; E01, E04, E05, E06 seen | the empty-selection rule, incl. explicit `Selection` |",
          "| P01-P07 | literal `%` | probably printed as written (unsupported/unresolvable codes stay raw); P06 unknown | how to write `%` in messages |",
          "| N01 | `%var` with a code in the name | 5 | codes in variable names |",
          "| N02 | same with a space | 0 | names are not trimmed |"]
    open(os.path.join(out, "round3-tests.md"), "w").write("\n".join(md) + "\n")
    print("wrote", out, "| round 3 blocks:", len(t.blocks), "| lint errors: 0")


if __name__ == "__main__":
    args = sys.argv[1:]
    rnd = "3"
    if args[:1] == ["--round"]:
        rnd, args = args[1], args[2:]
    out = args[0] if args else os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "assets", "tests")
    {"1": main, "2": main_round2, "3": main_round3}[rnd](out)
