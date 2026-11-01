#!/usr/bin/env python3
"""Convert a CodeClient/DiamondFire actiondump.json into Markdown reference files.

Usage: python3 df_dump_to_md.py actiondump.json OUT_DIR   (skill: OUT_DIR = ../references)

Produces (in OUT_DIR):
  action-index.md              overview, wire-format rules, per-block name lists
  actions/set_var_1.md / _2.md  Set Variable actions (split in two for size)
  actions/player_action.md Player Action actions
  actions/entity_action.md Entity Action actions
  actions/game_action.md   Game Action actions
  actions/events.md        Player / Entity / Game events
  actions/conditions.md    If Player / If Entity / If Game / If Variable
  actions/flow.md          Control, Repeat, Select Object, Else, Function, Process, calls
  game-values.md           Game values by category
  particles-potions.md     Particles (with fields) and potions
  sounds.md                Sound ids grouped by prefix
Regenerate whenever the dump changes; nothing here is hand-edited.
"""
import collections, json, os, re, sys

CC = re.compile(r"\u00a7.")          # Minecraft colour/format codes (section sign + char)


def clean(s):
    return CC.sub("", s) if isinstance(s, str) else s


def para(lines):
    if isinstance(lines, str):
        return clean(lines).strip()
    return " ".join(clean(x).strip() for x in lines if x is not None).strip()


def paras(list_of_lists):
    """additionalInfo / notes are lists of paragraphs, each a list of wrapped lines."""
    out = []
    for p in list_of_lists or []:
        out.append(para(p) if isinstance(p, list) else para([p]))
    return [x for x in out if x]


def ident(name):
    """Show an action name exactly; quote padded names so the spaces are visible."""
    return f'"{name}"' if name != name.strip() else name


def _w(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(text)


def md_escape(s):
    return s.replace("|", "\\|")


def render_args(icon):
    args = icon.get("arguments") or []
    if not args:
        return []
    lines, n = [], 0
    for a in args:
        if "text" in a:                      # separator between alternative argument sets
            lines.append(f"   - **{clean(a['text']).strip() or 'OR'}**")
            continue
        flags = []
        if a.get("optional"):
            flags.append("optional")
        if a.get("plural"):
            flags.append("plural")
        desc = para(a.get("description", []))
        note = " ".join(paras(a.get("notes")))
        flag_txt = f" _({', '.join(flags)})_" if flags else ""
        lines.append(f"   - `[{n}]` `{a['type']}`{flag_txt} — {desc}" + (f" — _{note}_" if note else ""))
        n += 1
    return lines


def render_tags(tags):
    lines = []
    for t in tags or []:
        opts = []
        for o in t["options"]:
            label = f"`{o['name']}`"
            if o["name"] == t.get("defaultOption"):
                label += " (default)"
            opts.append(label)
        lines.append(f"   - `{t['name']}` (slot {t['slot']}): " + ", ".join(opts))
        for o in t["options"]:
            d = para(o["icon"].get("description", []))
            if d:
                lines.append(f"       - `{o['name']}`: {d}")
    return lines


def flags_of(icon):
    f = []
    if icon.get("requiredRank"):
        f.append(f"rank: {icon['requiredRank']}")
    if icon.get("requireTokens"):
        f.append("requires tokens")
    if icon.get("requireSparks"):
        f.append("requires sparks")
    if icon.get("requireRankAndTokens"):
        f.append("requires rank and tokens")
    if icon.get("advanced"):
        f.append("advanced")
    if icon.get("worldExclusive"):
        f.append("world-plot only")
    return f


def render_action(a, cbid):
    icon = a["icon"]
    name = a["name"]
    out = [f"### `{ident(name)}`"]
    disp = clean(icon.get("name", "")).strip()
    meta = []
    if disp:
        meta.append(f"**{disp}**")
    if a.get("aliases"):
        meta.append("aliases: " + ", ".join(f"`{ident(x)}`" for x in a["aliases"]))
    if a.get("subAction") == "" and cbid.startswith("if_"):
        meta.append("**wire `subAction` = alias** (name collides across if-blocks)")
    if meta:
        out.append(" · ".join(meta))
    if "legacyReplacement" in a:
        lr = a["legacyReplacement"]
        txt = para(lr.get("text", []))
        out.append(f"> ⚠ **LEGACY** → use `{ident(lr.get('action', ''))}`" + (f" ({txt})" if txt else ""))
    if icon.get("deprecatedNote"):
        out.append(f"> ⚠ **DEPRECATED:** {para(icon['deprecatedNote'])}")
    d = para(icon.get("description", []))
    if d:
        out.append(d)
    ex = [clean(x).strip() for x in icon.get("example", [])]
    if ex:
        out.append("- _Examples:_ " + " ".join(ex))
    ww = icon.get("worksWith") or []
    if ww:
        out.append("- _Works with:_ " + "; ".join(clean(x) if isinstance(x, str) else para(x) for x in ww))
    for p in paras(icon.get("additionalInfo")):
        out.append(f"- _Info:_ {p}")
    if "cancellable" in icon:
        out.append(f"- Cancellable: {icon['cancellable']}; auto-cancelled: {icon.get('cancelledAutomatically')}")
    if a.get("subActionBlocks"):
        out.append("- Condition sub-blocks allowed: " + ", ".join(f"`{x}`" for x in a["subActionBlocks"]))
    al = render_args(icon)
    if al:
        out.append("- **Args** (`[n]` = documented order; see action-index.md §3 for slot rules):")
        out.extend(al)
    tl = render_tags(a.get("tags"))
    if tl:
        out.append("- **Tags:**")
        out.extend(tl)
    rv = icon.get("returnValues") or []
    if rv:
        out.append("- **Returns:** " + "; ".join(f"`{r.get('type')}` {para(r.get('description', []))}" for r in rv))
    fl = flags_of(icon)
    if fl:
        out.append("- _Flags:_ " + ", ".join(fl))
    out.append("")
    return "\n".join(out)


def main(dump_path, out_dir):
    D = json.load(open(dump_path, encoding="utf-8"))
    os.makedirs(out_dir, exist_ok=True)
    cb_by_name = {c["name"]: c for c in D["codeblocks"]}
    ver = D.get("versions", {})
    patch = ver.get("patch", "?")

    by_block = collections.OrderedDict()
    for c in D["codeblocks"]:
        by_block[c["identifier"]] = []
    for a in D["actions"]:
        by_block[cb_by_name[a["codeblockName"]]["identifier"]].append(a)

    files = {
        "actions/set_var_1.md": ("Set Variable (`set_var`), part 1", [("set_var", 0, 179)]),
        "actions/set_var_2.md": ("Set Variable (`set_var`), part 2", [("set_var", 179, None)]),
        "actions/player_action.md": ("Player Action (`player_action`)", ["player_action"]),
        "actions/entity_action.md": ("Entity Action (`entity_action`)", ["entity_action"]),
        "actions/game_action.md": ("Game Action (`game_action`)", ["game_action"]),
        "actions/events.md": ("Events (`event`, `entity_event`, `game_event`)", ["event", "entity_event", "game_event"]),
        "actions/conditions.md": ("Conditions (`if_player`, `if_entity`, `if_game`, `if_var`)", ["if_player", "if_entity", "if_game", "if_var"]),
        "actions/flow.md": ("Flow, selection and structure (`control`, `repeat`, `select_obj`, `else`, `func`, `process`, `call_func`, `start_process`)",
                               ["control", "repeat", "select_obj", "else", "func", "process", "call_func", "start_process"]),
    }
    written = {}
    for fname, (title, ids) in files.items():
        out = [f"# DiamondFire actions — {title}", "",
               f"_Generated from actiondump (patch {patch}). Action names are exact wire strings; "
               "names in double quotes include **leading/trailing spaces** that must be kept._", ""]
        total = 0
        for spec in ids:
            bid, lo, hi = spec if isinstance(spec, tuple) else (spec, 0, None)
            acts = by_block[bid][lo:hi]
            cbi = next(c for c in D["codeblocks"] if c["identifier"] == bid)
            out += [f"## Block `{bid}` — {clean(cbi['item']['name'])} ({len(acts)} actions in this file)", ""]
            bd = para(cbi["item"].get("description", []))
            if bd:
                out += [bd, ""]
            for p in paras(cbi["item"].get("additionalInfo")):
                out += [f"- {p}"]
            if cbi["item"].get("additionalInfo"):
                out.append("")
            for a in acts:
                out.append(render_action(a, bid))
            total += len(acts)
        _w(os.path.join(out_dir, fname), "\n".join(out))
        written[fname] = total

    # ---------------- game values ----------------
    gv = ["# DiamondFire game values", "",
          f"_Generated from actiondump (patch {patch}). In a template a game value is "
          '`{"id":"g_val","data":{"type":"<exact name below>","target":"Default"}}`. '
          "The `type` string is the display name with colour codes removed (verified on 6 names from sample templates)._", ""]
    cats = collections.OrderedDict((c["identifier"], []) for c in D["gameValueCategories"])
    for g in D["gameValues"]:
        cats.setdefault(g["category"], []).append(g)
    for cat, items in cats.items():
        gv += [f"## {cat} ({len(items)})", ""]
        for g in items:
            ic = g["icon"]
            line = f"- **{clean(ic['name'])}** (`{ic.get('returnType')}`) — {para(ic.get('description', []))}"
            rd = para(ic.get("returnDescription", []))
            if rd:
                line += f" _Returns:_ {rd}."
            ai = paras(ic.get("additionalInfo"))
            if ai:
                line += " _Info:_ " + " ".join(ai)
            if g.get("aliases"):
                line += " _Aliases:_ " + ", ".join(clean(x) for x in g["aliases"]) + "."
            fl = flags_of(ic)
            if fl:
                line += " _(" + ", ".join(fl) + ")_"
            gv.append(line)
        gv.append("")
    _w(os.path.join(out_dir, "game-values.md"), "\n".join(gv))

    # ---------------- particles / potions ----------------
    pp = ["# DiamondFire particles and potions", "",
          f"_Generated from actiondump (patch {patch}). Version-1 `part`/`pot` items use the Minecraft id: "
          "`\"minecraft:\" + <id below>` (confirmed by a sample using `minecraft:ominous_spawning` and `minecraft:resistance`)._", "",
          "## Particles", "",
          "`fields` = extra per-particle settings the chest accepts (colour, size, …); `Motion`/`Motion Variation` are the common ones.", ""]
    pcat = collections.OrderedDict((c["identifier"], []) for c in D["particleCategories"])
    for p in D["particles"]:
        pcat.setdefault(p["category"], []).append(p)
    for cat, items in pcat.items():
        pp += [f"### {cat} ({len(items)})", "", "| id | name | fields |", "|---|---|---|"]
        for p in items:
            pp.append(f"| `{p['particleId']}` | {clean(p['icon']['name'])} | {md_escape(', '.join(p['fields']) or '—')} |")
        pp.append("")
    pp += ["## Potions", "", "| id | name | effect |", "|---|---|---|"]
    for p in D["potions"]:
        pp.append(f"| `{p['potionId']}` | {clean(p['icon']['name'])} | {md_escape(para(p['icon'].get('description', [])))} |")
    pp.append("")
    _w(os.path.join(out_dir, "particles-potions.md"), "\n".join(pp))

    # ---------------- sounds ----------------
    snd = ["# DiamondFire sounds", "",
           f"_Generated from actiondump (patch {patch}). {len(D['sounds'])} sounds. "
           "Version-1 `snd` items use `\"minecraft:\" + <id>`. `(n)` = number of variants the dump lists for that sound "
           "(variants carry an id and a seed; the sample templates do not set a variant)._", ""]
    groups = collections.OrderedDict()
    for s in D["sounds"]:
        groups.setdefault(s["soundId"].split(".")[0], []).append(s)
    for g, items in sorted(groups.items()):
        snd += [f"## {g} ({len(items)})", ""]
        row = []
        for s in items:
            nv = len(s.get("variants") or [])
            row.append(f"`{s['soundId']}`" + (f" ({nv})" if nv else ""))
        snd += ["; ".join(row), ""]
    _w(os.path.join(out_dir, "sounds.md"), "\n".join(snd))

    # ---------------- index ----------------
    legacy_n = sum(1 for a in D["actions"] if "legacyReplacement" in a)
    padded_n = sum(1 for a in D["actions"] if a["name"] != a["name"].strip())
    idx = [f"# DiamondFire action dump — INDEX (patch {patch})", "",
           "Source: user-supplied CodeClient `actiondump.json` **[A]**. Everything below is generated by `df_dump_to_md.py`; "
           "interpretive notes are marked and cite which sample templates they were checked against.", "",
           "## 1. What is in the dump", "",
           "| Collection | Count | Documented in |", "|---|---|---|",
           f"| codeblocks | {len(D['codeblocks'])} | this file §4 |",
           f"| actions | {len(D['actions'])} | `actions/*.md` |",
           f"| game values | {len(D['gameValues'])} ({len(D['gameValueCategories'])} categories) | `game-values.md` |",
           f"| particles | {len(D['particles'])} ({len(D['particleCategories'])} categories) | `particles-potions.md` |",
           f"| potions | {len(D['potions'])} | `particles-potions.md` |",
           f"| sounds | {len(D['sounds'])} | `sounds.md` |",
           f"| cosmetics / shops | {len(D['cosmetics'])} / {len(D['shops'])} | _not coding-relevant; omitted_ |",
           "", "Item format versions in this dump: " + ", ".join(f"`{k}`={v}" for k, v in ver.items() if k != "patch") + ".", "",
           "## 2. Naming rules that affect generated templates", "",
           "1. **Exact strings.** The wire `action` of a block is the dump `name` byte-for-byte. "
           f"{padded_n} names have leading/trailing spaces (e.g. `\" RoundNumber \"`, `\" SetBlock \"`, `\" Range \"`). "
           "Never trim them. Generated docs show such names in double quotes.",
           f"2. **Padded twin = current, plain twin = legacy.** {legacy_n} actions carry `legacyReplacement`. "
           "In 29 of the 35 same-block padded/plain pairs the padded name is current and the plain one is legacy, "
           "and the legacy entry's `legacyReplacement.action` is the padded name. Always emit the replacement, never the legacy action. "
           "(4 pairs have two current twins; 2 pairs are odd — each doc entry carries its own LEGACY flag, trust that.)",
           "3. **Aliases.** `aliases` are alternative search names. **Exception that matters for the wire format:** for the 19 `if_*` actions whose "
           "name collides across if-blocks (`IsNear`, `NameEquals`, `StandingOn`, …) the dump's `subAction` is empty and the **alias is the identifier** "
           "used in `subAction` of `select_obj`/`repeat` blocks and in tag `action` fields. Verified: sample uses `PIsNear`, `EIsNear` (aliases of `IsNear` in `if_player`/`if_entity`); "
           "`ListContains` is unique so it needs no alias. Alias pattern: `P`/`E`/`V`/`G` + name for player/entity/var/game, with exceptions "
           "(e.g. `HasPotion` in `if_player` has alias `HasEffect`; `IsHitboxNear` → `PHitboxIsNear`). Use the alias listed on each entry.",
           "4. **Game value / particle / potion / sound identifiers.** Game values: display name with `\\u00a7` colour codes removed. "
           "Particles, potions, sounds (item version 1): `minecraft:` + id from the dump.", "",
           "## 3. Argument ↔ slot rules (checked against 5 sample templates)", "",
           "```",
           "canonical layout:  chest slot n  <-  the n-th typed argument in the entry's Args list (plural args absorb following slots)",
           "tags:              right-aligned at the end of the chest; slot numbers are stated on each tag (26 first, then 25, 24, ...)",
           "```",
           "Test result: of 219 sample items (excluding variables/game values, which can sit in any typed slot), **214 sit exactly at slot == argument index**. "
           "The other 5 are all the same phenomenon: an *optional* argument was omitted and the editor **compacted** the items "
           "(`ShiftOnAxis` number in slot 1 instead of 2; `repeat Multiple` amount in slot 0 instead of 1). "
           "So DF matches chest items to arguments in order **and by type**; compaction is accepted when the remaining items "
           "are unambiguous by type. **When adjacent optional arguments have the same type (e.g. `SetAllCoords` X/Y/Z/Pitch/Yaw), "
           "leave real gaps** — position is the only way to say which one you mean.",
           "`OR` rows in an Args list separate alternative argument sets. **Verified** (sample 20, `if_player StandingOn` = `BLOCK` OR `LOCATION`): when the `OR` group starts the argument list, **each alternative is filled starting at slot 0**. Unverified: groups where an argument precedes the `OR` (e.g. a leading `VARIABLE`).", "",
           "Argument type ↔ item id (from samples + dump types):", "",
           "| Dump arg type | Item id in template | Evidence |", "|---|---|---|",
           "| `NUMBER` | `num` (`name` is a string; may hold `%` codes) | sample |",
           "| `TEXT` | `txt` | sample |",
           "| `COMPONENT` | `comp` | dump type list; shape unseen |",
           "| `LOCATION` | `loc` | dump; shape unseen in samples |",
           "| `VECTOR` | `vec` | dump; shape unseen |",
           "| `ITEM` | `item` | sample |",
           "| `BLOCK` | `item` (block item) **or** `txt` (block id, e.g. `air`) | sample 2 |",
           "| `BLOCK_TAG` | `txt` (e.g. `waterlogged=false`) | sample 2 |",
           "| `SOUND` / `POTION` / `PARTICLE` | `snd` / `pot` / `part` (version 1) | sample 5 |",
           "| `LIST` / `DICT` | a `var` (or `g_val` returning a list) — there is no list/dict literal item id | dump item-type list |",
           "| `VARIABLE` | `var` | sample |",
           "| `ANY_TYPE` | any of the above | dump |",
           "| any | `var` or `g_val` can stand in for any typed argument | samples |", "",
           "## 4. Code blocks", "",
           "| Identifier | Name | Actions | File |", "|---|---|---|---|"]
    fmap = collections.defaultdict(list)
    for fname, (t, ids) in files.items():
        for spec in ids:
            fmap[spec[0] if isinstance(spec, tuple) else spec].append(fname)
    for c in D["codeblocks"]:
        i = c["identifier"]
        idx.append(f"| `{i}` | {clean(c['item']['name'])} | {len(by_block[i])} | " + ", ".join(f"`{f}`" for f in fmap[i]) + " |")
    idx += ["", "Previously unconfirmed block ids (`event`, `entity_event`, `process`, `start_process`, `if_game`) are now confirmed by this dump, "
            "and **`game_event`** (Game Event, 35 actions) exists, which the sample templates never used.", "",
            "## 5. Flags that appear on actions", "",
            "- `rank:` Noble / Mythic / Overlord / Emperor / Dev — minimum plot-owner rank for the action. "
            "`requires tokens`, `advanced`, `world-plot only` are shown when set. A template using a gated action may fail to place for plots that lack it.",
            "- `cancellable` / `auto-cancelled` (events): whether `Control: CancelEvent` applies / the event is cancelled by default.",
            "- `DEPRECATED` notes are shown verbatim; `LEGACY` means a replacement action exists.", "",
            "## 6. Action names at a glance (exact strings; padded names in quotes; ⚠ = legacy)", ""]
    for bid, acts in by_block.items():
        fl = fmap[bid]
        if len(fl) == 1:
            idx.append(f"**`{bid}`** ({len(acts)}) → `{fl[0]}`: " +
                       ", ".join(("⚠" if "legacyReplacement" in a else "") + "`" + ident(a["name"]) + "`" for a in acts))
            idx.append("")
        else:
            half = 179
            for k, (f, part) in enumerate(zip(fl, (acts[:half], acts[half:]))):
                idx.append(f"**`{bid}`** part {k+1} ({len(part)}) → `{f}`: " +
                           ", ".join(("⚠" if "legacyReplacement" in a else "") + "`" + ident(a["name"]) + "`" for a in part))
                idx.append("")
    _w(os.path.join(out_dir, "action-index.md"), "\n".join(idx))
    return written


if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    w = main(sys.argv[1], sys.argv[2])
    print("actions written per file:", w, "total", sum(w.values()))
