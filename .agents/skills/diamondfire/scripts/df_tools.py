"""DiamondFire template tools: decode / encode / decompile to pseudo-code."""
import base64, gzip, json

def decode(code: str) -> dict:
    return json.loads(gzip.decompress(base64.b64decode(code.strip())))

def encode(template: dict) -> str:
    raw = json.dumps(template, separators=(",", ":")).encode()
    g = bytearray(gzip.compress(raw, mtime=0))
    g[8], g[9] = 0, 0xFF          # XFL=0, OS=unknown: the header real DiamondFire (Java) templates have ("H4sIAAAAAAAA...")
    return base64.b64encode(bytes(g)).decode()


def fmt_item(it):
    x, d = it["item"], it["item"]["data"]
    t = x["id"]
    if t == "var":  return f'{d["scope"]}:{d["name"]}'
    if t in ("num", "txt", "comp"): return f'{t}({d["name"]!r})'
    if t == "g_val": return f'gv({d["type"]}@{d["target"]})'
    if t == "pn_el": return f'param {d["name"]}:{d["type"]}'
    if t == "item":  return "mcitem(" + d["item"].split("id:")[-1].strip('}"\\') + ")"
    if t == "snd":   return f'snd({d.get("sound")} pitch={d.get("pitch")} vol={d.get("vol")} v{x["version"]})'
    if t == "pot":   return f'pot({d.get("pot")} dur={d.get("dur")} amp={d.get("amp")} v{x["version"]})'
    if t == "part":  return f'part({d.get("particle")} n={d.get("cluster",{}).get("amount")} v{x["version"]})'
    if t == "hint":  return f'hint({d["id"]})'
    return f"{t}({json.dumps(d)})"

def decompile(tpl: dict) -> str:
    out, depth = [], 0
    for b in tpl["blocks"]:
        if b["id"] == "bracket":
            if b["direct"] == "close": depth -= 1
            out.append("  " * depth + ("{" if b["direct"] == "open" else "}") +
                       ("" if b["type"] == "norm" else "  [repeat]"))
            if b["direct"] == "open": depth += 1
            continue
        name = b["block"]
        label = b.get("action", b.get("data", ""))
        args, tags = [], []
        for it in b.get("args", {}).get("items", []):
            if it["item"]["id"] == "bl_tag":
                tags.append(f'{it["item"]["data"]["tag"]}={it["item"]["data"]["option"]}')
            else:
                args.append(f'[{it["slot"]}]{fmt_item(it)}')
        line = f'{name}' + (f'@{b["target"]}' if "target" in b else "") + f' {label!r}'
        if "subAction" in b: line += f' sub={b["subAction"]!r}'
        if "attribute" in b: line += f' {b["attribute"]}'
        line = line.rstrip()
        if args: line += " (" + ", ".join(args) + ")"
        if tags: line += " <" + "; ".join(tags) + ">"
        out.append("  " * depth + line)
    return "\n".join(out)

if __name__ == "__main__":
    import os, sys
    # usage: df_tools.py CODE_OR_FILE|-   -> prints the decompiled outline
    #        df_tools.py --json CODE_OR_FILE|-   -> prints the raw JSON
    args = sys.argv[1:]
    as_json = args[:1] == ["--json"]
    if as_json:
        args = args[1:]
    if not args:
        sys.exit(__doc__ + "\nusage: df_tools.py [--json] CODE_OR_FILE|-")
    src = sys.stdin.read() if args[0] == "-" else (open(args[0]).read() if os.path.exists(args[0]) else args[0])
    tpl = decode(src)
    print(json.dumps(tpl, indent=1) if as_json else decompile(tpl))
