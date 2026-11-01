#!/usr/bin/env python3
"""Regression check for the skill: run after replacing assets/actiondump.json or editing the scripts.

1. every sample in assets/samples lints with 0 errors            (format rules still match real templates)
2. builder reproduces samples 1, 3, 6, 7, 9-20 exactly as JSON   (builder still encodes the format correctly)
3. every sample converts to builder code and rebuilds exactly       (df_to_builder.py is faithful)
4. the linter still catches known-bad edits                      (linter is not silently passing everything)
Exit code 0 = all good.
"""
import copy, glob, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import df_lint as L, df_tools as T
from df_build import Template, num, txt, comp, loc, vec, var, bucket_var, gval, snd, pot, part, mc_item, param

D = L.load_dump(os.path.join(HERE, "..", "assets", "actiondump.json"))
fails = []
def check(ok, label):
    print(("PASS " if ok else "FAIL ") + label)
    if not ok:
        fails.append(label)

samples = {}
for p in sorted(glob.glob(os.path.join(HERE, "..", "assets", "samples", "sample*.txt"))):
    tpl = T.decode(open(p).read()); name = os.path.basename(p)[:-4]; samples[name] = tpl
    errs = [f for f in L.lint(tpl, D) if f[0] == "ERROR"]
    check(not errs, f"{name} lints clean" + (f" ({errs[0][2][:70]})" if errs else ""))

# builder reproductions
t = Template(); t.header("event", "PlayerDmgPlayer"); t.block("player_action", "PlaySound", [snd("block.chain.hit")])
check(t.to_json() == samples["sample6"], "builder reproduces sample6 (event + PlaySound)")

M = "4294967295"; t = Template()
t.header("func", "xorshift", params=[param("Output", "var"), param("Input", "num")])
bw = lambda d, a, b, op: t.block("set_var", "Bitwise", [var(d), var(a), b], tags={"Operator": op})
bw("Input", "Input", num(M), "&"); t.block("set_var", "=", [var("Output"), var("Input")])
for src_, sh, op in (("Input", 13, "<<"), ("Output", 17, ">>"), ("Output", 5, "<<")):
    if src_ == "Input": bw("Input", "Input", num(sh), op)
    else: bw("Input", "Output", num(sh), op)
    bw("Input", "Input", num(M), "&"); bw("Output", "Output", var("Input"), "^")
check(t.to_json() == samples["sample3"], "builder reproduces sample3 (xorshift)")

orig = samples["sample1"]; icon = orig["blocks"][0]["args"]["items"][0]["item"]["data"]["item"]; pn = orig["blocks"][0]["args"]["items"][1]["item"]["data"]
t = Template(); t.header("func", "cpu_limit", icon=icon, params=[param(pn["name"], pn["type"], description=pn["description"], note=pn["note"])])
with t.cond("if_var", ">=", [gval("Microseconds Since Startup"), var("limiter.deadline", "local")]):
    t.block("control", "Wait"); t.block("set_var", "MinNumber", [var("players"), gval("Player Count"), num(8)])
    t.block("set_var", "+", [var("limiter.deadline", "local"), gval("Microseconds Since Startup"),
            num("%math(%math(%math(40+%math(%math(%var(players)-1)*6))*6.25*%var(limiter.max_cpu)))")])
check(t.to_json() == orig, "builder reproduces sample1 (function icon, nested if, Wait)")


# builder reproduces the user's "default value for every item type" template (loc/vec/comp/bucket_var shapes)
t = Template()
t.header("func", "example", params=[txt("string"), comp("text"), num(0), loc(), vec(), snd("block.note_block.pling"), part("cloud"),
                                    pot("speed", 1000000, 0), var("\u00a7eVariable", "unsaved"), bucket_var("variable", "bucket"),
                                    gval("Location"), param("name", "any")])
check(t.to_json() == samples["sample7"], "builder reproduces sample7 (default value of every item type)")

t = Template(); t.header("func", "bot.red_concrete", hidden=True)
t.block("set_var", "ShiftRotation", [var("loc", "local"), num(-90)], tags={"Rotation Axis": "Yaw"})
t.block("set_var", "ShiftInDirection", [var("loc", "local")])
check(t.to_json() == samples["sample10"], "builder reproduces sample10 (hidden function, ShiftRotation + ShiftInDirection)")

t = Template(); t.header("func", "bot.gray_concrete", hidden=True)
t.call("botParticle", [part("flame", 50, horizontal=0.25, vertical=0.25, x=0.5), var("loc", "local")])
t.call("botSound", [snd("entity.item.break", pitch=2.0, vol=1.0)]); t.block("set_var", "=", [var("killBot", "local"), num(1)])
check(t.to_json() == samples["sample9"], "builder reproduces sample9 (call_func with particle/sound args)")

# dynamic dispatch (call_func with a % code in its name), optional params with defaults
t = Template()
t.header("func", "abilityCall", params=[param("item", "item"), param("calltag", "txt", default=txt("rccall")), param("cdtag", "txt", default=txt("cd"))])
with t.cond("if_player", "NoItemCooldown", [var("item")]):
    t.block("set_var", "GetItemTag", [var("cd"), var("item"), var("cdtag")])
    t.block("player_action", "SetItemCooldown", [var("item"), var("cd")])
    t.block("set_var", "GetItemTag", [var("call"), var("item"), var("calltag")])
    t.call("call.%var(call)")
check(t.to_json() == samples["sample11"], "builder reproduces sample11 (dynamic call_func, optional params with defaults)")

# negated conditions with explicit targets
t = Template(); t.header("func", "example")
with t.cond("if_player", "SlotEquals", [], target="Default", negate=True): pass
with t.cond("if_entity", "IsOxidized", [], target="Victim", negate=True): pass
with t.cond("if_game", "EventBlockEquals", []): pass
check(t.to_json() == samples["sample12"], "builder reproduces sample12 (NOT + target on if_player/if_entity)")

# repeat While with a negated sub-condition
t = Template(); t.header("func", "example"); t.block("set_var", "=", [var("i"), num(0)])
with t.repeat("While", [var("i"), num(10)], sub="=", negate=True): t.block("set_var", "+=", [var("i")])
check(t.to_json() == samples["sample13"], "builder reproduces sample13 (repeat While, sub-condition, NOT)")

# every target name
t = Template(); t.header("func", "example")
for tg in ("Selection", "Default", "Killer", "Damager", "Shooter", "Victim", "AllPlayers"): t.block("player_action", "SendMessage", [], target=tg)
for tg in ("Selection", "Default", "Killer", "Damager", "Victim", "Shooter", "Projectile", "AllEntities", "AllMobs", "LastEntity"): t.block("entity_action", "Heal", [], target=tg)
check(t.to_json() == samples["sample14"], "builder reproduces sample14 (all target names)")

# headers: entity_event, game_event, process (no hint item) + start_process tags
check(Template().header("entity_event", "EntityDmgEntity").to_json() == samples["sample15"], "builder reproduces sample15 (entity_event header)")
t = Template().header("process", "process"); t.block("start_process", "process")
check(t.to_json() == samples["sample16"], "builder reproduces sample16 (process header + start_process tags)")
check(Template().header("game_event", "PlotShutdown").to_json() == samples["sample17"], "builder reproduces sample17 (game_event header)")

# parameter kinds
check(Template().header("func", "example", params=[param("String", "txt"), param("Styled Text", "comp"), param("Number", "num"), param("Location", "loc"),
      param("Vector", "vec"), param("Sound", "snd"), param("Particle Effect", "part"), param("Potion Effect", "pot"), param("Item", "item")]).to_json() == samples["sample18"],
      "builder reproduces sample18 (txt/comp/num/loc/vec/snd/part/pot/item parameters)")
pn = {i["item"]["data"]["name"]: i["item"]["data"] for i in samples["sample19"]["blocks"][0]["args"]["items"] if i["item"]["id"] == "pn_el"}
t = Template().header("func", "example", params=[param("Dialogue", "dialog"), param("Any Value", "any"),
      param("Variable", "var", description=pn["Variable"]["description"], note=pn["Variable"]["note"]), param("List", "list"), param("Dictionary", "dict"),
      param("Plural", "any", plural=True, note=pn["Plural"]["note"]), param("Optional", "any", optional=True, note=pn["Optional"]["note"]),
      param("Optional With Default", "any", default=num(0))])
check(t.to_json() == samples["sample19"], "builder reproduces sample19 (dialog/any/var/list/dict, plural, optional, default)")

# OR-group action filled both ways (each alternative starts at slot 0)
snbt = samples["sample20"]["blocks"][1]["args"]["items"][0]["item"]["data"]["item"]
t = Template().header("func", "example")
with t.cond("if_player", " StandingOn ", [mc_item(snbt)]): pass
with t.cond("if_player", " StandingOn ", [loc(27.5, 49.5, 26.5, is_block=True)]): pass
check(t.to_json() == samples["sample20"], "builder reproduces sample20 (OR group: block alternative / location alternative)")

# every real sample converts to builder code and rebuilds exactly (df_to_builder.py)
import df_to_builder as _B
for _name, _tpl in sorted(samples.items(), key=lambda kv: int(kv[0][6:])):
    try:
        _ok, _src = _B.roundtrip(_tpl, D)
    except Exception as _e:               # a conversion error is a failure too
        _ok = False; print("   ", _name, type(_e).__name__, str(_e)[:100])
    if not _ok:
        fails.append(f"{_name} round trip"); print("FAIL " + f"{_name} converts to builder code and rebuilds exactly")
print(f"PASS all {len(samples)} samples convert to builder code and rebuild exactly" if not [f for f in fails if f.endswith("round trip")] else "")

# mutation checks
def mutated(tpl, fn):
    c = copy.deepcopy(tpl); fn(c); return [f for f in L.lint(c, D) if f[0] in ("ERROR", "WARN")]
def pad(c):
    for b in c["blocks"]:
        if b.get("action") == " RoundNumber ": b["action"] = "RoundNumber"
def slot(c):
    for b in c["blocks"]:
        for i in b.get("args", {}).get("items", []):
            if i["item"]["id"] == "bl_tag": i["slot"] = 3; return
check(bool(mutated(samples["sample2"], pad)), "linter flags trimmed padded action name")
check(bool(mutated(samples["sample2"], slot)), "linter flags tag in wrong slot")
check(samples["sample2"]["blocks"][-1].get("id") == "bracket" and bool(mutated(samples["sample2"], lambda c: c["blocks"].pop())), "linter flags unbalanced brackets")
print("\nALL GOOD" if not fails else f"\n{len(fails)} FAILURE(S)")
sys.exit(1 if fails else 0)
