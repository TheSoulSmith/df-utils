#!/usr/bin/env python3
"""Lint a DiamondFire template against a CodeClient actiondump.

Usage:  python3 df_lint.py [actiondump.json] TEMPLATE_CODE_OR_FILE [more ...]   (default dump: ../assets/actiondump.json)
        (TEMPLATE_CODE is the base64+gzip string; a path to a file containing it also works)

Library: from df_lint import load_dump, lint ; findings = lint(template_dict, dump)
Each finding is (severity, where, message); severity is ERROR | WARN | INFO.

What it checks (all derived from the dump, validated on 5 real templates with 0 findings):
  structure   header first, brackets balanced and typed, `else` placement, bracket-owning blocks
  actions     exact wire name (padding!), legacy twins, alias use, subAction rules, rank flags
  tags        name/option/slot per the dump, owner (action/block) fields, missing tags
  items       slot range, duplicates, item id vs argument type, item versions, game value names
  functions   calls to functions not defined in the same template (INFO)
"""
import base64, collections, gzip, json, re, sys

CC = re.compile(r"\u00a7.")
HEADERS = {"event", "entity_event", "game_event", "func", "process"}
BRACKETED = {"if_player", "if_entity", "if_game", "if_var", "else", "repeat"}
LITERAL_OK = {  # item id -> dump arg types it can satisfy
    "num": {"NUMBER"}, "txt": {"TEXT", "BLOCK", "BLOCK_TAG", "DIALOG", "ENTITY_TYPE", "PROJECTILE", "VEHICLE", "SPAWN_EGG", "BYTE"},
    "item": {"ITEM", "BLOCK", "SPAWN_EGG", "PROJECTILE", "VEHICLE"}, "loc": {"LOCATION"}, "vec": {"VECTOR"},
    "snd": {"SOUND"}, "pot": {"POTION"}, "part": {"PARTICLE"}, "comp": {"COMPONENT", "TEXT"},
}
ITEM_VERSIONS = {"snd": 1, "part": 1, "pot": 1}      # everything else is 0 in patch 8.13


def decode(code):
    return json.loads(gzip.decompress(base64.b64decode(code.strip())))


def load_dump(path):
    D = json.load(open(path, encoding="utf-8"))
    cb = {c["name"]: c["identifier"] for c in D["codeblocks"]}
    idx = collections.defaultdict(list)
    for a in D["actions"]:
        idx[cb[a["codeblockName"]]].append(a)
    D["_idx"] = idx
    D["_gv"] = {CC.sub("", g["icon"]["name"]) for g in D["gameValues"]} | \
               {CC.sub("", x) for g in D["gameValues"] for x in g["aliases"]}
    D["_ver"] = D.get("versions", {})
    return D


def find(D, block, name, allow_alias=True):
    idx = D["_idx"].get(block, [])
    for a in idx:
        if a["name"] == name:
            return a, "name"
    if allow_alias:
        for a in idx:
            if name in a["aliases"]:
                return a, "alias"
    return None, None


def lint(tpl, D):
    F = []
    add = lambda sev, where, msg: F.append((sev, where, msg))
    blocks = tpl.get("blocks")
    if not isinstance(blocks, list) or not blocks:
        return [("ERROR", "template", "missing or empty `blocks` list")]

    # ---------- structure ----------
    first = blocks[0]
    if first.get("id") != "block" or first.get("block") not in HEADERS:
        add("ERROR", "#0", f"template must start with a header block {sorted(HEADERS)}, got {first.get('block', first.get('id'))!r}")
    stack = []
    defined_funcs, called = set(), []
    for n, b in enumerate(blocks):
        w = f"#{n}"
        if b.get("id") == "bracket":
            if b.get("direct") == "open":
                prev = blocks[n - 1] if n else {}
                if prev.get("block") not in BRACKETED:
                    add("ERROR", w, f"`open` bracket must follow a bracket-owning block {sorted(BRACKETED)}; follows {prev.get('block')!r}")
                want = "repeat" if prev.get("block") == "repeat" else "norm"
                if b.get("type") != want:
                    add("ERROR", w, f"bracket type {b.get('type')!r} but the owning block `{prev.get('block')}` needs {want!r}")
                stack.append(b.get("type"))
            elif b.get("direct") == "close":
                if not stack:
                    add("ERROR", w, "closing bracket with nothing open")
                elif stack.pop() != b.get("type"):
                    add("ERROR", w, "closing bracket type does not match its opener")
            else:
                add("ERROR", w, f"bracket `direct` must be open/close, got {b.get('direct')!r}")
            continue
        if b.get("id") != "block":
            add("ERROR", w, f"unknown element id {b.get('id')!r}")
            continue
        blk = b.get("block")
        if blk in BRACKETED:
            nxt = blocks[n + 1] if n + 1 < len(blocks) else {}
            if not (nxt.get("id") == "bracket" and nxt.get("direct") == "open"):
                add("ERROR", w, f"`{blk}` must be followed by an opening bracket")
        if blk == "else":
            prv = blocks[n - 1] if n else {}
            if not (prv.get("id") == "bracket" and prv.get("direct") == "close" and prv.get("type") == "norm"):
                add("ERROR", w, "`else` must come directly after the closing bracket of an if-block")
            continue
        if blk in HEADERS and n != 0:
            add("ERROR", w, f"header block `{blk}` can only be first in a code line")
        if blk not in D["_idx"]:
            add("ERROR", w, f"unknown block id {blk!r}")
            continue

        # ---------- action resolution ----------
        if blk in ("func", "call_func", "process", "start_process"):
            act, name = find(D, blk, "dynamic")[0], b.get("data")
            if blk == "func":
                defined_funcs.add(name)
            if blk == "call_func":
                called.append((w, name))
            if not name:
                add("ERROR", w, f"`{blk}` needs a name in `data`")
        else:
            name = b.get("action")
            act, how = find(D, blk, name, allow_alias=False)
            if act is None:
                alt, how2 = find(D, blk, name)
                if alt is not None:
                    add("WARN", w, f"{blk} action {name!r} is an alias of {alt['name']!r}; use the real name unless it is a subAction")
                    act = alt
                else:
                    near = [a["name"] for a in D["_idx"][blk] if a["name"].strip() == (name or "").strip()]
                    hint = f" — did you mean {near[0]!r} (padding differs)?" if near else ""
                    add("ERROR", w, f"unknown {blk} action {name!r}{hint}")
                    continue
        if "legacyReplacement" in act:
            add("WARN", w, f"{blk} action {name!r} is LEGACY -> use {act['legacyReplacement'].get('action')!r}")
        ic = act["icon"]
        if ic.get("requiredRank"):
            add("INFO", w, f"{blk}/{name!r} requires plot rank {ic['requiredRank']}")
        if ic.get("advanced") or ic.get("worldExclusive"):
            add("INFO", w, f"{blk}/{name!r} is {'advanced' if ic.get('advanced') else ''}{' world-plot-only' if ic.get('worldExclusive') else ''}".replace("  ", " "))
        if ic.get("deprecatedNote"):
            add("WARN", w, f"{blk}/{name!r} deprecated: {' '.join(CC.sub('', x) for x in ic['deprecatedNote'])}")

        # ---------- subAction (select_obj etc.) ----------
        owner_for_tags = (blk, name, act)
        if act.get("subActionBlocks"):
            sub = b.get("subAction")
            if sub:
                found = None
                for c in act["subActionBlocks"]:
                    s_act, how = find(D, c, sub)
                    if s_act is not None:
                        found = (c, sub, s_act, how)
                        break
                if not found:
                    add("ERROR", w, f"subAction {sub!r} not found in any of {act['subActionBlocks']}")
                else:
                    c, sname, s_act, how = found
                    if s_act["subAction"] == "" and how == "name":
                        add("ERROR", w, f"subAction {sub!r} is ambiguous across if-blocks; use the alias {s_act['aliases']}")
                    owner_for_tags = (c, sub, s_act)
            else:
                add("ERROR", w, f"{blk}/{name!r} needs a `subAction` (one of {act['subActionBlocks']})")
        elif b.get("subAction"):
            add("WARN", w, f"unexpected subAction on {blk}/{name!r}")

        # ---------- items & tags ----------
        items = b.get("args", {}).get("items", []) if b.get("args") is not None else []
        slots = collections.Counter(i["slot"] for i in items)
        for s, k in slots.items():
            if k > 1:
                add("ERROR", w, f"slot {s} used {k} times")
        tags_seen = set()
        typed_args = [x for x in (owner_for_tags[2]["icon"].get("arguments") or []) if "type" in x]
        data_items = []
        for it in items:
            slot, x = it.get("slot"), it.get("item", {})
            iid, d = x.get("id"), x.get("data", {})
            if not isinstance(slot, int) or not 0 <= slot <= 26:
                add("ERROR", w, f"slot {slot!r} out of range 0..26")
            exp_ver = ITEM_VERSIONS.get(iid, 0)
            if iid in D["_ver"] or iid in ITEM_VERSIONS:
                exp_ver = D["_ver"].get(iid, exp_ver)
            if iid in ("snd", "part", "pot", "num", "txt", "var", "bucket_var", "loc", "vec", "comp", "item", "g_val", "bl_tag", "pn_el", "hint") and x.get("version") != exp_ver:
                add("WARN" if iid not in ITEM_VERSIONS else "ERROR", w, f"item `{iid}` has version {x.get('version')!r}, dump says {exp_ver}")
            if iid == "num" and not isinstance(d.get("name"), str):
                add("WARN", w, "num `name` should be a string (samples always use strings)")
            if iid == "g_val" and d.get("type") not in D["_gv"]:
                add("ERROR", w, f"unknown game value {d.get('type')!r}")
            if iid == "var" and d.get("scope") not in ("unsaved", "saved", "local", "line"):
                add("ERROR", w, f"variable scope {d.get('scope')!r} must be unsaved/saved/local/line")
            if iid == "bl_tag":
                tags_seen.add((d.get("tag")))
                t_owner, t_act = d.get("block"), d.get("action")
                if blk in ("func", "call_func", "process", "start_process"):
                    want = (blk, "dynamic")
                else:
                    want = (owner_for_tags[0], owner_for_tags[1])
                if (t_owner, t_act) != want:
                    add("ERROR", w, f"tag {d.get('tag')!r} owner fields {(t_owner, t_act)} should be {want}")
                o_act, _ = find(D, t_owner, t_act)
                if o_act is None:
                    continue
                tg = [t for t in o_act["tags"] if t["name"] == d.get("tag")]
                if not tg:
                    add("ERROR", w, f"tag {d.get('tag')!r} does not exist on {t_owner}/{t_act!r} (has {[t['name'] for t in o_act['tags']]})")
                    continue
                tg = tg[0]
                if d.get("option") not in [o["name"] for o in tg["options"]]:
                    add("ERROR", w, f"tag {d.get('tag')!r}: option {d.get('option')!r} invalid; valid {[o['name'] for o in tg['options']]}")
                if tg["slot"] != slot:
                    add("ERROR", w, f"tag {d.get('tag')!r} must be in slot {tg['slot']}, found {slot}")
            elif iid not in ("hint", "pn_el"):
                data_items.append((slot, iid))
                # type check against the canonical argument at this slot (compaction tolerated -> only WARN-free if any arg accepts it)
                if iid in LITERAL_OK and typed_args:
                    accepts = {a["type"] for a in typed_args}
                    if "ANY_TYPE" not in accepts and not (LITERAL_OK[iid] & accepts):
                        add("ERROR", w, f"item `{iid}` at slot {slot} matches no argument of {blk}/{name!r} (args: {sorted({a['type'] for a in typed_args})})")
        own_tags = {t["name"] for t in owner_for_tags[2]["tags"]}
        if blk in ("func",) and not own_tags:
            own_tags = set()
        for missing in sorted(own_tags - tags_seen):
            add("WARN", w, f"tag {missing!r} not listed (default will apply; real exports always list every tag)")
        req = [a for a in typed_args if not a.get("optional")]
        has_or = any("text" in a for a in (owner_for_tags[2]["icon"].get("arguments") or []))
        if blk not in ("func", "call_func", "process", "start_process") and not has_or and len(data_items) < len(req):
            add("WARN", w, f"{blk}/{name!r} lists {len(req)} required argument(s) but only {len(data_items)} item(s) present")
    if stack:
        add("ERROR", "end", f"{len(stack)} bracket(s) never closed")
    for w, nme in called:
        if nme and "%" in nme:
            add("INFO", w, f"dynamic call: function name {nme!r} is built at runtime from % codes; the target function cannot be checked statically")
        elif nme not in defined_funcs:
            add("INFO", w, f"calls function {nme!r} which is not defined in this template (fine if it lives elsewhere on the plot)")
    return F


DEFAULT_DUMP = __import__("os").path.join(__import__("os").path.dirname(__import__("os").path.abspath(__file__)), "..", "assets", "actiondump.json")


def main(argv):
    args = argv[1:]
    dump = DEFAULT_DUMP
    if args and args[0].endswith(".json"):
        dump, args = args[0], args[1:]
    if not args:
        sys.exit(__doc__)
    D = load_dump(dump)
    rc = 0
    for src in args:
        try:
            code = open(src).read()
        except (FileNotFoundError, OSError):
            code = src
        findings = lint(decode(code), D)
        errs = sum(1 for f in findings if f[0] == "ERROR")
        print(f"== {src[:40]}…  {errs} error(s), {sum(1 for f in findings if f[0]=='WARN')} warning(s)")
        for sev, where, msg in findings:
            print(f"  {sev:5} {where:5} {msg}")
        rc |= 1 if errs else 0
    return rc


if __name__ == "__main__":
    sys.exit(main(sys.argv))
