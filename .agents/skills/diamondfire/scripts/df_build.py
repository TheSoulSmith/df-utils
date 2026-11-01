#!/usr/bin/env python3
"""Build DiamondFire templates in Python without hand-writing JSON.

Why this exists: the wire format has several easy-to-miss rules (padded action names, tags that must
sit in fixed slots with owner fields, alias-as-subAction, bracket types). This builder takes them from
the actiondump so generated templates are right by construction; df_lint.py then double-checks.

It only builds item shapes that appear in real templates (num, txt, comp, var, bucket_var, g_val, item,
loc, vec, snd, pot, part, pn_el, hint). `r_loc` has not been seen yet; add it only from a real export.

Example
-------
    from df_build import Template, num, var, gval, snd, param
    t = Template()                                   # loads ../assets/actiondump.json
    t.header("func", "xorshift", params=[param("Output", "var"), param("Input", "num")])
    t.block("set_var", "Bitwise", [var("Input"), var("Input"), num("4294967295")],
            tags={"Operator": "&"})                  # other tags default from the dump, in the right slots
    with t.cond("if_var", "<", [var("n"), num(10)]):
        t.block("player_action", "PlaySound", [snd("block.chain.hit")], target="AllPlayers")
    with t.else_():
        t.block("set_var", "=", [var("n"), num(0)])
    print(t.encode())                                # base64+gzip template code
    for f in t.lint(): print(f)

Arguments are positional: a list index is the chest slot. Use None to leave a slot empty (needed when
adjacent optional arguments have the same type, e.g. SetAllCoords X/Y/Z).
`target=` is only written when given (values seen: Default, Selection, Killer, Damager, Victim, Shooter, Projectile, AllPlayers, AllEntities, AllMobs, LastEntity). An ABSENT target key does not simply mean "default target": per the user, a block with no\ntarget acts on the current selection if there is one (an empty selection still counts), otherwise on the default target.
"""
import base64, contextlib, gzip, json, os, re, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import df_lint  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
DEFAULT_DUMP = os.path.join(HERE, "..", "assets", "actiondump.json")
_DUMP_CACHE = {}


# ----------------------------------------------------------------------------- items (no slot yet)
def _item(iid, data, version=0):
    return {"item": {"version": version, "id": iid, "data": data}}


def num(v):
    """Number. Stored as a STRING; may contain % codes, e.g. num('%math(%var(a)+1)')."""
    return _item("num", {"name": str(v)})


def txt(s):
    return _item("txt", {"name": str(s)})


def var(name, scope="line"):
    """scope: line | local | unsaved (= game) | saved"""
    if scope == "game":
        scope = "unsaved"
    if scope not in ("line", "local", "unsaved", "saved"):
        raise ValueError(f"bad scope {scope!r}")
    return _item("var", {"name": name, "scope": scope})


def gval(type_, target="Default"):
    """Game value; `type_` is the display name with colour codes removed, e.g. 'Player Count'."""
    return _item("g_val", {"type": type_, "target": target})


def mc_item(snbt):
    """Minecraft item as an SNBT string, e.g. '{DF_NBT:4903,count:1,id:"minecraft:stone"}'."""
    return _item("item", {"item": snbt})


def _mc(name):
    return name if ":" in name else "minecraft:" + name


def snd(sound, pitch=1.0, vol=2.0):
    return _item("snd", {"mappingVersion": 0, "sound": _mc(sound), "pitch": float(pitch), "vol": float(vol)}, version=1)


def pot(potion, dur, amp):
    """dur in ticks; amp = level - 1 (observed: 4 for level V)."""
    return _item("pot", {"mappingVersion": 0, "pot": _mc(potion), "dur": int(dur), "amp": int(amp)}, version=1)


def part(particle, amount=1, horizontal=0.0, vertical=0.0, x=1.0, y=0.0, z=0.0, motion_variation=100):
    return _item("part", {"mappingVersion": 0, "particle": _mc(particle),
                          "cluster": {"amount": int(amount), "horizontal": float(horizontal), "vertical": float(vertical)},
                          "data": {"x": float(x), "y": float(y), "z": float(z), "motionVariation": motion_variation}}, version=1)


def param(name, type_, plural=False, optional=False, description=None, note=None, default=None):
    """Function parameter. `default` = an item (e.g. txt("cd")) used when the caller omits the argument;
    giving one makes the parameter optional and is serialized as `default_value` (seen in a real export)."""
    d = {"name": name, "type": type_}
    if default is not None:
        d["default_value"] = json.loads(json.dumps(default["item"]))
        optional = True
    d.update({"plural": plural, "optional": optional})
    if description:
        d["description"] = description
    if note:
        d["note"] = note
    return _item("pn_el", d)


def loc(x=0.0, y=0.0, z=0.0, pitch=0.0, yaw=0.0, is_block=False):
    """Location item. `is_block` is the raw `isBlock` flag (default False; meaning of True is unverified)."""
    return _item("loc", {"isBlock": bool(is_block), "loc": {"x": float(x), "y": float(y), "z": float(z), "pitch": float(pitch), "yaw": float(yaw)}})


def vec(x=0.0, y=0.0, z=0.0):
    return _item("vec", {"x": float(x), "y": float(y), "z": float(z)})


def comp(text):
    """Styled-text component; `text` may contain MiniMessage-style markup and % codes."""
    return _item("comp", {"name": str(text)})


def bucket_var(name, key, namespace_type="DEFAULT", namespace_alias=""):
    """Bucket variable `name` inside bucket `key`. namespace_type DEFAULT uses the plot's default namespace."""
    return _item("bucket_var", {"name": name, "key": key, "namespace_type": namespace_type, "namespace_alias": namespace_alias})


# ----------------------------------------------------------------------------- template
class Template:
    def __init__(self, dump_path=None):
        p = os.path.abspath(dump_path or DEFAULT_DUMP)
        if p not in _DUMP_CACHE:
            _DUMP_CACHE[p] = df_lint.load_dump(p)
        self.D = _DUMP_CACHE[p]
        self.blocks = []
        self._open = []

    # -- lookup helpers
    def _action(self, blk, name):
        a, how = df_lint.find(self.D, blk, name, allow_alias=False)
        if a is None:
            near = [x["name"] for x in self.D["_idx"].get(blk, []) if x["name"].strip() == name.strip()]
            raise ValueError(f"unknown {blk} action {name!r}" + (f"; did you mean {near[0]!r}?" if near else ""))
        if "legacyReplacement" in a:
            raise ValueError(f"{blk} action {name!r} is LEGACY; use {a['legacyReplacement'].get('action')!r}")
        return a

    def _tags(self, owner_blk, owner_name, act, overrides):
        overrides = dict(overrides or {})
        out = []
        for t in act["tags"]:
            opts = [o["name"] for o in t["options"]]
            opt = overrides.pop(t["name"], t["defaultOption"])
            if opt not in opts:
                raise ValueError(f"tag {t['name']!r}: {opt!r} invalid; choose from {opts}")
            out.append({"item": {"version": 0, "id": "bl_tag", "data": {"option": opt, "tag": t["name"], "action": owner_name, "block": owner_blk}},
                        "slot": t["slot"]})
        if overrides:
            raise ValueError(f"{owner_blk}/{owner_name!r} has no tag(s) {sorted(overrides)}; tags are {[t['name'] for t in act['tags']]}")
        return out

    @staticmethod
    def _ordered(items):
        """The editor exports items in ascending slot order; match it."""
        return sorted(items, key=lambda i: i["slot"])

    @staticmethod
    def _slots(args):
        items = []
        for i, a in enumerate(args or []):
            if a is None:
                continue
            c = json.loads(json.dumps(a))
            c["slot"] = i
            items.append(c)
        return items

    # -- headers
    def header(self, kind, name, params=(), icon=None, hidden=False, tags=None):
        """kind: event | entity_event | game_event | func | process (all layouts verified against real exports).
        func: params are pn_el items (use param()); `icon` is an optional SNBT string placed in slot 0
        (sample: functions with an icon have no hint item)."""
        if self.blocks:
            raise ValueError("header must be the first block")
        if kind in ("func", "process"):
            items = []
            start = 0
            if icon:
                items.append({**mc_item(icon), "slot": 0})
                start = 1
            for i, p in enumerate(params):
                c = json.loads(json.dumps(p))
                c["slot"] = start + i
                items.append(c)
            if not icon and kind == "func":          # a func header carries a `function` hint; a process header has none [S]
                items.append({**_item("hint", {"id": "function"}), "slot": 25})
            items.append({"item": {"version": 0, "id": "bl_tag", "data": {"option": "True" if hidden else "False", "tag": "Is Hidden", "action": "dynamic", "block": kind}}, "slot": 26})
            self.blocks.append({"id": "block", "block": kind, "args": {"items": items}, "data": name})
        elif kind in ("event", "entity_event", "game_event"):
            act = self._action(kind, name)
            self.blocks.append({"id": "block", "block": kind, "args": {"items": self._tags(kind, name, act, tags)}, "action": name})
        else:
            raise ValueError(f"unknown header kind {kind!r}")
        return self

    # -- ordinary blocks
    def block(self, blk, action, args=(), tags=None, target=None, sub=None, negate=False):
        """Add one block. `sub` = condition name/alias for select_obj / repeat While etc.
        `negate=True` writes attribute NOT (seen on select_obj; expected on if_* blocks)."""
        if blk in ("call_func", "start_process"):
            b = {"id": "block", "block": blk, "args": {"items": self._ordered(self._slots(args) + self._tags(blk, "dynamic", self._action(blk, "dynamic"), tags))}, "data": action}
            self.blocks.append(b)
            return self
        act = self._action(blk, action)
        owner = (blk, action, act)
        if act.get("subActionBlocks"):
            if not sub:
                raise ValueError(f"{blk}/{action!r} needs sub= one of {act['subActionBlocks']}")
            found = None
            for c in act["subActionBlocks"]:
                s_act, how = df_lint.find(self.D, c, sub)
                if s_act is not None:
                    found = (c, sub, s_act, how)
                    break
            if not found:
                raise ValueError(f"sub {sub!r} not found in {act['subActionBlocks']}")
            c, sname, s_act, how = found
            if s_act["subAction"] == "" and how == "name":
                raise ValueError(f"sub {sub!r} is ambiguous across if-blocks; use the alias {s_act['aliases']}")
            owner = (c, sname, s_act)
        elif sub:
            raise ValueError(f"{blk}/{action!r} takes no sub")
        b = {"id": "block", "block": blk, "args": {"items": self._ordered(self._slots(args) + self._tags(owner[0], owner[1], owner[2], tags))}, "action": action}
        if target:
            b["target"] = target
        if sub:
            b["subAction"] = sub
        if negate:
            b["attribute"] = "NOT"
        self.blocks.append(b)
        return self

    def call(self, func_name, args=()):
        """Call a function by name. The name may contain % codes (e.g. 'bot.%var(blockName)') for dynamic
        dispatch; the wire form for that is presumed to be the code in `data` (confirm with a real export)."""
        self.blocks.append({"id": "block", "block": "call_func", "args": {"items": self._slots(args)}, "data": func_name})
        return self

    # -- bracketed structures
    @contextlib.contextmanager
    def _bracket(self, kind):
        self.blocks.append({"id": "bracket", "direct": "open", "type": kind})
        yield self
        self.blocks.append({"id": "bracket", "direct": "close", "type": kind})

    def cond(self, blk, action, args=(), tags=None, target=None, negate=False):
        """with t.cond('if_var', '<', [a, b]): ...  (blk in if_player/if_entity/if_game/if_var)"""
        if blk not in ("if_player", "if_entity", "if_game", "if_var"):
            raise ValueError("cond() is for if_* blocks")
        self.block(blk, action, args, tags, target, negate=negate)
        return self._bracket("norm")

    def else_(self):
        self.blocks.append({"id": "block", "block": "else"})
        return self._bracket("norm")

    def repeat(self, action, args=(), tags=None, sub=None, negate=False):
        """with t.repeat('Multiple', [None, num(5)]): ...   (loop var first, optional)
        While loops: t.repeat('While', [var('i'), num(10)], sub='=', negate=True)  # while NOT (i = 10)"""
        self.block("repeat", action, args, tags, sub=sub, negate=negate)
        return self._bracket("repeat")

    # -- output
    def to_json(self):
        return {"blocks": self.blocks}

    def encode(self):
        import df_tools
        return df_tools.encode(self.to_json())

    def lint(self):
        return df_lint.lint(self.to_json(), self.D)


if __name__ == "__main__":
    print(__doc__)
