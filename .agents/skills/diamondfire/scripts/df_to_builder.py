#!/usr/bin/env python3
"""Convert an existing DiamondFire template into editable Python that uses scripts/df_build.py.

Why: to MODIFY a template, decompile it to builder code, edit the code, run it, lint it, encode it. Round-tripping is exact for
every real template tried (see selftest.py), so unchanged parts stay byte-identical.

Usage:  python3 df_to_builder.py CODE_OR_FILE|-            prints Python source (ends with `print(t.encode())`)
        python3 df_to_builder.py --check CODE_OR_FILE|-    also executes the source and verifies it rebuilds the template exactly

Library:  from df_to_builder import to_source, roundtrip
Only tags that differ from the dump's default are written (the builder fills in the rest). Item shapes the builder can't express
raise ValueError instead of being silently dropped.
"""
import json, os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import df_lint, df_tools

DEFAULT_DUMP = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "assets", "actiondump.json")
BRACKETED = {"if_player", "if_entity", "if_game", "if_var", "else", "repeat"}
R = repr


def _strip_mc(s):
    return s[len("minecraft:"):] if isinstance(s, str) and s.startswith("minecraft:") else s


def item_expr(it):
    x = it["item"]
    i, d = x["id"], x["data"]
    if i == "num":
        return f"num({R(d['name'])})"
    if i == "txt":
        return f"txt({R(d['name'])})"
    if i == "comp":
        return f"comp({R(d['name'])})"
    if i == "var":
        return f"var({R(d['name'])})" if d["scope"] == "line" else f"var({R(d['name'])}, {R(d['scope'])})"
    if i == "bucket_var":
        return f"bucket_var({R(d['name'])}, {R(d['key'])}, {R(d['namespace_type'])}, {R(d['namespace_alias'])})"
    if i == "g_val":
        return f"gval({R(d['type'])})" if d["target"] == "Default" else f"gval({R(d['type'])}, {R(d['target'])})"
    if i == "item":
        return f"mc_item({R(d['item'])})"
    if i == "loc":
        l = d["loc"]
        return f"loc({l['x']!r}, {l['y']!r}, {l['z']!r}, {l['pitch']!r}, {l['yaw']!r}, {d['isBlock']!r})"
    if i == "vec":
        return f"vec({d['x']!r}, {d['y']!r}, {d['z']!r})"
    if i == "snd":
        if d.get("mappingVersion") != 0 or x["version"] != 1:
            raise ValueError("snd item with unexpected version/mappingVersion")
        return f"snd({R(_strip_mc(d['sound']))}, {d['pitch']!r}, {d['vol']!r})"
    if i == "pot":
        return f"pot({R(_strip_mc(d['pot']))}, {d['dur']!r}, {d['amp']!r})"
    if i == "part":
        c, dd = d["cluster"], d["data"]
        return (f"part({R(_strip_mc(d['particle']))}, {c['amount']!r}, {c['horizontal']!r}, {c['vertical']!r}, "
                f"{dd['x']!r}, {dd['y']!r}, {dd['z']!r}, {dd['motionVariation']!r})")
    if i == "pn_el":
        a = [R(d["name"]), R(d["type"])]
        kw = []
        if d.get("plural"):
            kw.append("plural=True")
        if d.get("optional") and "default_value" not in d:
            kw.append("optional=True")
        if "description" in d:
            kw.append(f"description={R(d['description'])}")
        if "note" in d:
            kw.append(f"note={R(d['note'])}")
        if "default_value" in d:
            kw.append(f"default={item_expr({'item': d['default_value']})}")
        extra = set(d) - {"name", "type", "plural", "optional", "description", "note", "default_value"}
        if extra:
            raise ValueError(f"pn_el has unsupported keys {extra}")
        return f"param({', '.join(a + kw)})"
    raise ValueError(f"no builder constructor for item id {i!r}")


def _args(items):
    """slot-indexed list literal with None for gaps (tags/hints excluded)."""
    real = [it for it in items if it["item"]["id"] not in ("bl_tag", "hint")]
    if not real:
        return "[]"
    n = max(it["slot"] for it in real) + 1
    slots = [None] * n
    for it in real:
        slots[it["slot"]] = item_expr(it)
    return "[" + ", ".join("None" if s is None else s for s in slots) + "]"


def _tag_overrides(D, items):
    out = {}
    for it in items:
        if it["item"]["id"] != "bl_tag":
            continue
        d = it["item"]["data"]
        act, _ = df_lint.find(D, d["block"], d["action"])
        if act is None:
            raise ValueError(f"tag owner {d['block']}/{d['action']} not in dump")
        tg = [t for t in act["tags"] if t["name"] == d["tag"]]
        if not tg:
            raise ValueError(f"tag {d['tag']!r} not on {d['block']}/{d['action']}")
        if d["option"] != tg[0]["defaultOption"]:
            out[d["tag"]] = d["option"]
    return out


def to_source(tpl, D=None):
    D = D or df_lint.load_dump(DEFAULT_DUMP)
    blocks = tpl["blocks"]
    L = ["import sys", "sys.path.insert(0, 'scripts')   # adjust to this skill's scripts directory",
         "from df_build import Template, num, txt, comp, var, bucket_var, gval, mc_item, loc, vec, snd, pot, part, param", "", "t = Template()"]
    h = blocks[0]
    items = h.get("args", {}).get("items", [])
    kind = h["block"]
    if kind in ("func", "process"):
        hidden = any(i["item"]["id"] == "bl_tag" and i["item"]["data"]["option"] == "True" for i in items)
        body = sorted([i for i in items if i["item"]["id"] not in ("bl_tag", "hint")], key=lambda i: i["slot"])
        icon = None
        if kind == "func" and body and body[0]["slot"] == 0 and body[0]["item"]["id"] == "item":
            icon = item_expr(body.pop(0))[len("mc_item("):-1]
        params = ", ".join(item_expr(i) for i in body)
        kw = [f"params=[{params}]"] if params else []
        if icon:
            kw.append(f"icon={icon}")
        if hidden:
            kw.append("hidden=True")
        L.append(f"t.header({R(kind)}, {R(h.get('data'))}" + "".join(", " + k for k in kw) + ")")
    else:
        ov = _tag_overrides(D, items)
        L.append(f"t.header({R(kind)}, {R(h['action'])}" + (f", tags={ov!r}" if ov else "") + ")")

    indent = 0
    pending_open = []          # stack: did the current body get any statement yet?
    i = 1
    n = len(blocks)
    pad = lambda: "    " * indent
    while i < n:
        b = blocks[i]
        if b.get("id") == "bracket":
            if b["direct"] == "close":
                if not pending_open.pop():
                    L.append(pad() + "pass")
                indent -= 1
            i += 1
            continue
        blk = b["block"]
        if pending_open:
            pending_open[-1] = True
        its = b.get("args", {}).get("items", []) if b.get("args") is not None else []
        if blk == "else":
            L.append(pad() + "with t.else_():")
            indent += 1; pending_open.append(False); i += 2        # skip the open bracket
            continue
        args = _args(its)
        kw = []
        ov = _tag_overrides(D, its)
        if ov:
            kw.append(f"tags={ov!r}")
        if b.get("target"):
            kw.append(f"target={R(b['target'])}")
        if b.get("subAction"):
            kw.append(f"sub={R(b['subAction'])}")
        if b.get("attribute") == "NOT":
            kw.append("negate=True")
        elif b.get("attribute"):
            raise ValueError(f"unknown attribute {b['attribute']!r}")
        extra = set(b) - {"id", "block", "args", "action", "data", "target", "subAction", "attribute"}
        if extra:
            raise ValueError(f"block has unsupported keys {extra}")
        if blk in BRACKETED:
            if blk == "repeat":
                L.append(pad() + f"with t.repeat({R(b['action'])}, {args}" + "".join(", " + k for k in kw) + "):")
            else:
                L.append(pad() + f"with t.cond({R(blk)}, {R(b['action'])}, {args}" + "".join(", " + k for k in kw) + "):")
            indent += 1; pending_open.append(False); i += 2        # skip the open bracket
            continue
        if blk == "call_func" and not ov:
            L.append(pad() + f"t.call({R(b['data'])}, {args})")
        elif blk in ("call_func", "start_process"):
            L.append(pad() + f"t.block({R(blk)}, {R(b['data'])}, {args}" + "".join(", " + k for k in kw) + ")")
        else:
            L.append(pad() + f"t.block({R(blk)}, {R(b['action'])}, {args}" + "".join(", " + k for k in kw) + ")")
        i += 1
    L += ["", "print(t.encode())"]
    return "\n".join(L) + "\n"


def roundtrip(tpl, D=None):
    """Execute the generated source and return (identical?, source)."""
    src = to_source(tpl, D)
    ns = {"__name__": "gen"}
    import io, contextlib
    with contextlib.redirect_stdout(io.StringIO()):
        exec(compile(src.replace("sys.path.insert(0, 'scripts')", "pass"), "<gen>", "exec"), ns)
    return ns["t"].to_json() == tpl, src


if __name__ == "__main__":
    args = sys.argv[1:]
    check = args[:1] == ["--check"]
    if check:
        args = args[1:]
    if not args:
        sys.exit(__doc__)
    src = sys.stdin.read() if args[0] == "-" else (open(args[0]).read() if os.path.exists(args[0]) else args[0])
    tpl = df_tools.decode(src)
    if check:
        ok, code = roundtrip(tpl)
        print(code if "--quiet" not in sys.argv else "")
        print("# round trip exact:", ok)
        sys.exit(0 if ok else 1)
    print(to_source(tpl), end="")
