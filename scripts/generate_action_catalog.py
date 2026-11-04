#!/usr/bin/env python3
"""Generate Rust catalogs from DiamondFire's actiondump."""

import json
import re
import subprocess
import sys
from pathlib import Path


PARTICLE_FIELDS = {
    "Motion": ("motion", "ParticleMotion", ("x", "y", "z"), "motion"),
    "Motion Variation": ("motion_variation", "ParticleNumber", ("motionVariation",), "number"),
    "Water Blocks": ("water_blocks", "ParticleNumber", ("waterBlocks",), "number"),
    "Burst Impulse": ("burst_impulse", "ParticleNumber", ("burstImpulse",), "number"),
    "Color": ("color", "u32", ("rgb",), "color"),
    "Color Variation": ("color_variation", "ParticleNumber", ("colorVariation",), "number"),
    "Duration": ("duration", "ParticleNumber", ("time",), "number"),
    "Size": ("size", "ParticleNumber", ("size",), "number"),
    "Size Variation": ("size_variation", "ParticleNumber", ("sizeVariation",), "number"),
    "Material": ("material", "String", ("material",), "text"),
    "Fade Color": ("fade_color", "u32", ("rgb_fade",), "color"),
    "Power": ("power", "ParticleNumber", ("power",), "number"),
    "Opacity": ("opacity", "ParticleNumber", ("opacity",), "number"),
    "Roll": ("roll", "ParticleNumber", ("roll",), "number"),
}


def rust_string(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def ident(value: str) -> str:
    words = re.sub(r"[^A-Za-z0-9]+", "_", value).strip("_")
    if not words:
        words = "Empty"
    if words[0].isdigit():
        words = f"N_{words}"
    return "".join(part.title() for part in words.split("_"))


def clean_display_name(value: str) -> str:
    return re.sub(r"§[0-9A-FK-ORa-fk-or]", "", value)


def field_ident(value: str, used: set[str]) -> str:
    words = re.sub(r"[^A-Za-z0-9]+", "_", value).strip("_").lower()
    if not words:
        words = "argument"
    if words[0].isdigit():
        words = f"field_{words}"
    if words in {
        "as",
        "break",
        "const",
        "continue",
        "crate",
        "else",
        "enum",
        "extern",
        "false",
        "fn",
        "for",
        "if",
        "impl",
        "in",
        "let",
        "loop",
        "match",
        "mod",
        "move",
        "mut",
        "pub",
        "ref",
        "return",
        "self",
        "static",
        "struct",
        "super",
        "trait",
        "true",
        "type",
        "unsafe",
        "use",
        "where",
        "while",
        "async",
        "await",
        "dyn",
    }:
        words += "_"
    base = words
    suffix = 2
    while words in used:
        words = f"{base}_{suffix}"
        suffix += 1
    used.add(words)
    return words


def variant_ident(value: str, used: set[str]) -> str:
    segments = re.findall(r"[A-Z]+(?=[A-Z][a-z]|\b)|[A-Z]?[a-z]+|[0-9]+", value)
    base = "".join(segment[:1].upper() + segment[1:] for segment in segments)
    if not base:
        base = ident(value)
    if base[0].isdigit():
        base = f"N{base}"
    if base in {"Self", "Super"}:
        base += "Value"
    variant = base
    suffix = 2
    while variant in used:
        variant = f"{base}{suffix}"
        suffix += 1
    used.add(variant)
    return variant


def typed_action_values_source(records: list[tuple]) -> str:
    headers = {"event", "entity_event", "game_event", "func", "process"}
    records = [record for record in records if record[1] not in headers]
    grouped: dict[str, list[tuple[tuple, str]]] = {}
    names: dict[str, str] = {}
    parameters: dict[str, list[str]] = {}
    tags_by_action: dict[str, list[tuple[str, list[tuple[str, str]], dict]]] = {}
    tag_option_types: dict[tuple[str, ...], tuple[str, list[tuple[str, str]]]] = {}
    used_tag_type_names: set[str] = set()
    sub_actions: dict[str, tuple[str, list[str]]] = {}

    for record in records:
        action_id, block, action, arguments, *_ = record
        block_type = ident(block)
        used_names = {variant for _, variant in grouped.get(block_type, [])}
        base_name = action["name"]
        if record[4]:
            base_name += f"Alternative{record[5]}"
        variant_name = variant_ident(base_name, used_names)
        grouped.setdefault(block_type, []).append((record, variant_name))
        names[action_id] = variant_name

        if block in {"call_func", "start_process"}:
            parameters[action_id] = ["Vec<Item>"]
        elif action["name"] == "SetHotbar":
            parameters[action_id] = ["Option<Item>"] * 9
        else:
            parameters[action_id] = [
                "Option<Vec<Item>>"
                if arg["plural"] and arg["optional"]
                else "Vec<Item>"
                if arg["plural"]
                else "Option<Item>"
                if arg["optional"]
                else "Item"
                for arg in arguments
            ]

        action_tags = []
        for tag in action.get("tags", []):
            option_names = tuple(sorted(option["name"] for option in tag["options"]))
            if option_names not in tag_option_types:
                used_options: set[str] = set()
                options = [
                    (variant_ident(name, used_options), name)
                    for name in option_names
                ]
                name_hint = "".join(variant for variant, _ in options[:2])
                base_type = f"TagOptions{name_hint}"
                tag_type = base_type
                suffix = 2
                while tag_type in used_tag_type_names:
                    tag_type = f"{base_type}{suffix}"
                    suffix += 1
                used_tag_type_names.add(tag_type)
                tag_option_types[option_names] = (tag_type, options)
            tag_type, options = tag_option_types[option_names]
            action_tags.append((tag_type, options, tag))
        tags_by_action[action_id] = action_tags

        allowed_blocks = action.get("subActionBlocks", [])
        if allowed_blocks:
            sub_actions[action_id] = (
                f"{action_id}SubAction",
                [ident(allowed) for allowed in allowed_blocks],
            )

    lines = [
        "// Generated by scripts/generate_action_catalog.py; do not edit by hand.",
        "use super::super::typed::Item;",
        "use serde_json::{Value, json};",
        "",
    ]

    for tag_type, options in tag_option_types.values():
        lines += [
            "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
            f"pub enum {tag_type} {{",
            *(f"    {option}," for option, _ in options),
            "}",
            "",
        ]

    for block_type, members in grouped.items():
        lines += [f"pub enum {block_type} {{"]
        for record, variant_name in members:
            action_id, block, *_ = record
            lines.append(f"    {variant_name} {{")
            if block in {"call_func", "start_process"}:
                lines.append("        name: String,")
            if block == "call_func":
                lines.append("        params: Vec<Item>,")
            else:
                param_types = parameters[action_id]
                params_type = "Vec<Item>" if block == "start_process" else (
                    "()"
                    if not param_types
                    else f"({', '.join(param_types)},)"
                )
                lines.append(f"        params: {params_type},")
            tag_types = [tag_type for tag_type, _, _ in tags_by_action[action_id]]
            tags_type = (
                "()"
                if not tag_types
                else f"({', '.join(f'Option<{tag_type}>' for tag_type in tag_types)},)"
            )
            lines.append(f"        tags: {tags_type},")
            if action_id in sub_actions:
                lines.append(f"        sub_action: {sub_actions[action_id][0]},")
            lines.append("    },")
        lines += ["}", ""]

    for action_id, (sub_type, allowed_block_types) in sub_actions.items():
        lines += [
            f"pub enum {sub_type} {{",
            *(
                f"    {block_type}({block_type}),"
                for block_type in allowed_block_types
            ),
            "}",
            "",
            f"impl {sub_type} {{",
            "    fn to_wire(&self) -> Value {",
            "        match self {",
            *(
                f"            Self::{block_type}(action) => action.to_wire(),"
                for block_type in allowed_block_types
            ),
            "        }",
            "    }",
            "",
            "    fn sub_action_wire_name(&self) -> String {",
            "        match self {",
            *(
                f"            Self::{block_type}(action) => action.sub_action_wire_name(),"
                for block_type in allowed_block_types
            ),
            "        }",
            "    }",
            "}",
            "",
        ]

    for block_type, members in grouped.items():
        lines += [
            f"impl {block_type} {{",
            "    pub fn to_wire(&self) -> Value {",
            "        match self {",
        ]
        for record, variant_name in members:
            action_id, block, action, arguments, *_ = record
            pattern_fields = [
                "params" if block in {"call_func", "start_process"} or arguments else "params: _",
                "tags" if tags_by_action[action_id] else "tags: _",
            ]
            if block in {"call_func", "start_process"}:
                pattern_fields.insert(0, "name")
            if block == "call_func":
                pattern_fields = ["name", "params", "tags: _"]
            if action_id in sub_actions:
                pattern_fields.append("sub_action")
            lines.append(f"            Self::{variant_name} {{ {', '.join(pattern_fields)} }} => {{")

            if block == "call_func":
                lines += [
                    f"                encode_dynamic(\"{block}\", name, params, &[])",
                    "            },",
                ]
                continue

            if block == "start_process":
                lines.append("                let arguments = params;")
            elif action["name"] == "SetHotbar":
                lines.append("                let mut arguments = Vec::new();")
                lines.extend(
                    f"                arguments.push(params.{index}.clone());"
                    for index in range(9)
                )
            else:
                lines.append(
                    "                let mut arguments = Vec::new();"
                    if arguments
                    else "                let arguments = Vec::new();"
                )
                for index, argument in enumerate(arguments):
                    if argument["plural"] and argument["optional"]:
                        lines += [
                            f"                if let Some(values) = &params.{index} {{",
                            "                    arguments.extend(values.iter().cloned().map(Some));",
                            "                } else {",
                            "                    arguments.push(None);",
                            "                }",
                        ]
                    elif argument["plural"]:
                        lines.append(
                            f"                arguments.extend(params.{index}.iter().cloned().map(Some));"
                        )
                    elif argument["optional"]:
                        lines.append(f"                arguments.push(params.{index}.clone());")
                    else:
                        lines.append(f"                arguments.push(Some(params.{index}.clone()));")

            tag_values = []
            for index, (tag_type, options, tag) in enumerate(tags_by_action[action_id]):
                option_name = f"tag_option_{index}"
                lines.append(
                    f"                let {option_name} = tags.{index}.map(|value| match value {{"
                )
                lines.extend(
                    f"                    {tag_type}::{variant} => {rust_string(option)},"
                    for variant, option in options
                )
                lines += ["                });"]
                tag_values.append(
                    f"({tag['slot']}, {rust_string(tag['name'])}, {option_name}, "
                    f"{rust_string((action.get('aliases') or [action['name']])[0] if block in {'if_player', 'if_entity', 'if_var', 'if_game'} else action['name'])})"
                )

            if block == "start_process":
                lines.append(
                    f"                encode_dynamic(\"{block}\", name, &arguments, &[{', '.join(tag_values)}])"
                )
            else:
                child = (
                    "Some((sub_action.to_wire(), sub_action.sub_action_wire_name()))"
                    if action_id in sub_actions
                    else "None"
                )
                sub_action_name = action.get("subAction") or ""
                lines.append(
                    f"                encode_action(\"{block}\", {rust_string(action['name'])}, "
                    f"{rust_string(sub_action_name)}, arguments, &[{', '.join(tag_values)}], {child})"
                )
            lines.append("            },")
        lines += [
            "        }",
            "    }",
            "    pub fn sub_action_wire_name(&self) -> String {",
            "        match self {",
        ]
        for record, variant_name in members:
            _, _, action, *_ = record
            wire_name = action.get("subAction") or (action.get("aliases") or [action["name"]])[0]
            pattern = f"Self::{variant_name} {{ .. }}"
            lines.append(f"            {pattern} => {rust_string(wire_name)}.to_owned(),")
        lines += [
            "        }",
            "    }",
            "}",
            "",
            f"impl super::super::typed::WireAction for {block_type} {{",
            "    fn to_wire_value(&self) -> Value {",
            "        self.to_wire()",
            "    }",
            "    fn sub_action_wire_name(&self) -> String {",
            "        Self::sub_action_wire_name(self)",
            "    }",
            "}",
            "",
        ]

    lines += [
        "fn encode_action(",
        "    block: &str,",
        "    action: &str,",
        "    sub_action_name: &str,",
        "    arguments: Vec<Option<Item>>,",
        "    tags: &[(u8, &'static str, Option<&'static str>, &'static str)],",
        "    sub_action: Option<(Value, String)>,",
        ") -> Value {",
        "    let mut items: Vec<Value> = arguments.into_iter().enumerate()",
        '        .filter_map(|(slot, argument)| argument.map(|argument| json!({"item": argument.to_value(), "slot": slot})))',
        "        .collect();",
        "    for (slot, name, option, owner_action) in tags {",
        "        if let Some(option) = option {",
        '            items.push(json!({"item": {"version": 0, "id": "bl_tag", "data": {"tag": name, "option": option, "action": owner_action, "block": block}}, "slot": slot}));',
        "        }",
        "    }",
        '    items.sort_by_key(|item| item.get("slot").and_then(Value::as_u64).unwrap_or(0));',
        '    let mut value = json!({"id": "block", "block": block, "action": action, "args": {"items": items}});',
        '    if !sub_action_name.is_empty() { value["subAction"] = json!(sub_action_name); }',
        '    if let Some((child, child_sub_action)) = sub_action {',
        '        if let Some(child_items) = child.get("args").and_then(|args| args.get("items")).and_then(Value::as_array) { value["args"]["items"].as_array_mut().unwrap().extend(child_items.iter().cloned()); }',
        '        value["subAction"] = json!(child_sub_action);',
        '    }',
        "    value",
        "}",
        "",
        "fn encode_dynamic(block: &str, name: &str, arguments: &[Item], tags: &[(u8, &'static str, Option<&'static str>, &'static str)]) -> Value {",
        '    let mut items: Vec<Value> = arguments.iter().enumerate().map(|(slot, argument)| json!({"item": argument.to_value(), "slot": slot})).collect();',
        '    for (slot, tag, option, owner_action) in tags { if let Some(option) = option { items.push(json!({"item": {"version": 0, "id": "bl_tag", "data": {"tag": tag, "option": option, "action": owner_action, "block": block}}, "slot": slot})); } }',
        '    items.sort_by_key(|item| item.get("slot").and_then(Value::as_u64).unwrap_or(0));',
        '    json!({"id": "block", "block": block, "data": name, "args": {"items": items}})',
        "}",
    ]
    return "\n".join(lines)


def particle_catalog_source(particles: list[dict]) -> str:
    schemas = []
    for particle in particles:
        fields = []
        for field in particle["fields"]:
            try:
                fields.append(PARTICLE_FIELDS[field])
            except KeyError as error:
                raise SystemExit(
                    f"unsupported field {field!r} for particle {particle['particleId']!r}"
                ) from error
        schemas.append((particle, fields))

    lines = [
        "// Generated by scripts/generate_action_catalog.py; do not edit by hand.",
        "use serde_json::{Map, Number, Value, json};",
        "",
        "#[derive(Clone, Debug, PartialEq, Eq)]",
        "pub struct ParticleNumber(Number);",
        "",
        "impl ParticleNumber {",
        "    pub fn decimal(value: f64) -> Option<Self> {",
        "        Number::from_f64(value).map(Self)",
        "    }",
        "",
        "    pub fn from_value(value: &Value) -> Option<Self> {",
        "        match value {",
        "            Value::Number(number) => Some(Self(number.clone())),",
        "            _ => None,",
        "        }",
        "    }",
        "",
        "    pub fn to_value(&self) -> Value {",
        "        Value::Number(self.0.clone())",
        "    }",
        "}",
        "",
        "macro_rules! particle_number_from_integer {",
        "    ($($type:ty),* $(,)?) => {",
        "        $(impl From<$type> for ParticleNumber {",
        "            fn from(value: $type) -> Self { Self(Number::from(value)) }",
        "        })*",
        "    };",
        "}",
        "",
        "particle_number_from_integer!(i32, i64, u32, u64);",
        "",
        "#[derive(Clone, Debug, PartialEq, Eq)]",
        "pub struct ParticleMotion {",
        "    pub x: ParticleNumber,",
        "    pub y: ParticleNumber,",
        "    pub z: ParticleNumber,",
        "}",
        "",
        "#[derive(Clone, Debug, PartialEq, Eq)]",
        "pub struct ParticleCluster {",
        "    pub amount: u32,",
        "    pub horizontal: ParticleNumber,",
        "    pub vertical: ParticleNumber,",
        "}",
        "",
        "impl Default for ParticleCluster {",
        "    fn default() -> Self {",
        "        Self { amount: 1, horizontal: 0.into(), vertical: 0.into() }",
        "    }",
        "}",
        "",
        "#[derive(Clone, Debug, PartialEq, Eq)]",
        "pub enum ParticleSpec {",
    ]
    for particle, fields in schemas:
        variant = ident(particle["particle"])
        if fields:
            lines.append(f"    {variant} {{")
            lines.extend(f"        {field[0]}: {field[1]}," for field in fields)
            lines.append("    },")
        else:
            lines.append(f"    {variant},")
    lines += [
        "}",
        "",
        "impl ParticleSpec {",
        "    pub fn id(&self) -> &'static str {",
        "        match self {",
    ]
    for particle, _ in schemas:
        variant = ident(particle["particle"])
        pattern = f"Self::{variant} {{ .. }}" if particle["fields"] else f"Self::{variant}"
        lines.append(f"            {pattern} => {rust_string(particle['particleId'])},")
    lines += ["        }", "    }", ""]

    lines += [
        "    pub fn to_data(&self) -> Map<String, Value> {",
        "        let mut data = Map::new();",
        "        match self {",
    ]
    for particle, fields in schemas:
        variant = ident(particle["particle"])
        if fields:
            bindings = ", ".join(field[0] for field in fields)
            lines.append(f"            Self::{variant} {{ {bindings} }} => {{")
            for name, _, wire_fields, kind in fields:
                if kind == "motion":
                    for axis in wire_fields:
                        lines.append(f'                data.insert({rust_string(axis)}.into(), {name}.{axis}.to_value());')
                elif kind == "number":
                    lines.append(
                        f'                data.insert({rust_string(wire_fields[0])}.into(), {name}.to_value());'
                    )
                else:
                    lines.append(
                        f'                data.insert({rust_string(wire_fields[0])}.into(), json!({name}));'
                    )
            lines.append("            }")
        else:
            lines.append(f"            Self::{variant} => {{}}")
    lines += ["        }", "        data", "    }", ""]

    lines += [
        "    pub fn from_wire(id: &str, data: &Map<String, Value>) -> Result<Self, String> {",
        "        let expected_fields: &[&str] = match id {",
    ]
    for particle, fields in schemas:
        wire_fields = sorted({wire for field in fields for wire in field[2]})
        lines.append(
            f'            {rust_string(particle["particleId"])} => &[{", ".join(rust_string(field) for field in wire_fields)}],'
        )
    lines += [
        '            _ => return Err(format!("unknown particle id {id:?}")),',
        "        };",
        "        if let Some(field) = data.keys().find(|field| !expected_fields.contains(&field.as_str())) {",
        '            return Err(format!("unexpected field {field:?} for particle {id:?}"));',
        "        }",
        "        match id {",
    ]
    for particle, fields in schemas:
        values = []
        for name, rust_type, wire_fields, kind in fields:
            if kind == "motion":
                value = "ParticleMotion { " + ", ".join(
                    f"{axis}: number_field(data, {rust_string(axis)})?"
                    for axis in wire_fields
                ) + " }"
            elif kind == "number":
                value = f"number_field(data, {rust_string(wire_fields[0])})?"
            elif kind == "color":
                value = f"color_field(data, {rust_string(wire_fields[0])})?"
            else:
                value = f"text_field(data, {rust_string(wire_fields[0])})?"
            values.append(f"{name}: {value}")
        variant = ident(particle["particle"])
        value = f"Self::{variant} {{ {', '.join(values)} }}" if fields else f"Self::{variant}"
        lines.append(f'            {rust_string(particle["particleId"])} => Ok({value}),')
    lines += [
        '            _ => Err(format!("unknown particle id {id:?}")),',
        "        }",
        "    }",
        "}",
        "",
        "fn number_field(data: &Map<String, Value>, name: &str) -> Result<ParticleNumber, String> {",
        "    data.get(name)",
        "        .and_then(ParticleNumber::from_value)",
        '        .ok_or_else(|| format!("particle field {name:?} is missing or not numeric"))',
        "}",
        "",
        "fn color_field(data: &Map<String, Value>, name: &str) -> Result<u32, String> {",
        "    data.get(name)",
        "        .and_then(Value::as_u64)",
        "        .and_then(|value| u32::try_from(value).ok())",
        '        .ok_or_else(|| format!("particle field {name:?} is missing or not a 32-bit color"))',
        "}",
        "",
        "fn text_field(data: &Map<String, Value>, name: &str) -> Result<String, String> {",
        "    data.get(name)",
        "        .and_then(Value::as_str)",
        "        .map(str::to_owned)",
        '        .ok_or_else(|| format!("particle field {name:?} is missing or not text"))',
        "}",
        "",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
        "pub struct ParticleKindSpec {",
        "    pub name: &'static str,",
        "    pub id: &'static str,",
        "    pub fields: &'static [&'static str],",
        "}",
        "",
        "pub static PARTICLES: &[ParticleKindSpec] = &[",
    ]
    for particle, _ in schemas:
        fields = ", ".join(rust_string(field) for field in particle["fields"])
        lines += [
            "    ParticleKindSpec {",
            f"        name: {rust_string(particle['particle'])},",
            f"        id: {rust_string(particle['particleId'])},",
            f"        fields: &[{fields}],",
            "    },",
        ]
    lines += ["];", ""]
    return "\n".join(lines)


def main() -> None:
    if len(sys.argv) < 2:
        raise SystemExit(
            "usage: python3 scripts/generate_action_catalog.py "
            "<actiondump.json> [src/low/raw/catalog.rs]"
        )
    source = Path(sys.argv[1])
    output = Path(sys.argv[2]) if len(sys.argv) > 2 else Path(
        "src/low/raw/catalog.rs"
    )
    dump = json.loads(source.read_text())
    codeblocks = {entry["name"]: entry["identifier"] for entry in dump["codeblocks"]}
    records = []
    used_variants: set[str] = set()
    for action in dump["actions"]:
        block = codeblocks[action["codeblockName"]]
        base = f'{ident(block)}{ident(action["name"])}'
        alternatives = [[]]
        for argument in action["icon"].get("arguments", []):
            if "type" not in argument:
                alternatives.append([])
            elif argument["type"] != "NONE":
                alternatives[-1].append(argument)

        for alternative_index, arguments in enumerate(alternatives):
            preferred = (
                base
                if alternative_index == 0
                else f"{base}Alternative{alternative_index + 1}"
            )
            variant = preferred
            suffix = 1
            while variant in used_variants:
                variant = f"{preferred}Variant{suffix}"
                suffix += 1
            used_variants.add(variant)
            has_or_alternatives = len(alternatives) > 1
            records.append(
                (
                    variant,
                    block,
                    action,
                    arguments,
                    has_or_alternatives,
                    alternative_index + 1 if has_or_alternatives else 0,
                    len(alternatives),
                )
            )

    block_ids = sorted({entry["identifier"] for entry in dump["codeblocks"]})
    block_lines = [
        "// Generated by scripts/generate_action_catalog.py; do not edit by hand.",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]",
        "pub enum BlockKind {",
        *(f"    {ident(block)}," for block in block_ids),
        "}",
        "",
        "impl BlockKind {",
        "    pub const fn as_wire(self) -> &'static str {",
        "        match self {",
        *(f"            Self::{ident(block)} => {rust_string(block)}," for block in block_ids),
        "        }",
        "    }",
        "",
        "    pub fn from_wire(value: &str) -> Option<Self> {",
        "        match value {",
        *(f"            {rust_string(block)} => Some(Self::{ident(block)})," for block in block_ids),
        "            _ => None,",
        "        }",
        "    }",
        "}",
        "",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
        "pub struct BlockSpec {",
        "    pub kind: BlockKind,",
        "    pub name: &'static str,",
        "    pub material: &'static str,",
        "    pub required_rank: &'static str,",
        "}",
        "",
        "pub static BLOCKS: &[BlockSpec] = &[",
    ]
    for block in dump["codeblocks"]:
        item = block["item"]
        block_lines += [
            "    BlockSpec {",
            f"        kind: BlockKind::{ident(block['identifier'])},",
            f"        name: {rust_string(block['name'])},",
            f"        material: {rust_string(item['material'])},",
            f"        required_rank: {rust_string(item.get('requiredRank', ''))},",
            "    },",
        ]
    block_lines += ["];", ""]
    lines = typed_action_values_source(records)
    output.parent.mkdir(parents=True, exist_ok=True)
    generated_dir = output.parent / "catalog"
    generated_dir.mkdir(parents=True, exist_ok=True)
    output.write_text(
        "\n".join(
            [
                "// Generated by scripts/generate_action_catalog.py; do not edit by hand.",
                '#[path = "catalog/blocks.rs"]',
                "mod blocks;",
                '#[path = "catalog/actions.rs"]',
                "pub mod actions;",
                '#[path = "catalog/game_values.rs"]',
                "mod game_values;",
                '#[path = "catalog/sounds.rs"]',
                "mod sounds;",
                '#[path = "catalog/particles.rs"]',
                "mod particles;",
                '#[path = "catalog/potions.rs"]',
                "mod potions;",
                "",
                "pub use blocks::{BLOCKS, BlockKind, BlockSpec};",
                "pub use game_values::{GAME_VALUES, GameValueSpec, find_game_value};",
                "pub use particles::{",
                "    PARTICLES, ParticleCluster, ParticleKindSpec, ParticleMotion, ParticleNumber,",
                "    ParticleSpec,",
                "};",
                "pub use potions::{POTIONS, PotionSpec};",
                "pub use sounds::{SOUNDS, SoundSpec};",
            ]
        )
        + "\n"
    )
    (generated_dir / "blocks.rs").write_text("\n".join(block_lines) + "\n")
    (generated_dir / "actions.rs").write_text(lines + "\n")

    return_types = {
        "NUMBER": "Number",
        "TEXT": "Text",
        "LIST": "List",
        "ITEM": "Item",
        "LOCATION": "Location",
        "VECTOR": "Vector",
        "COMPONENT": "Component",
        "BLOCK_TAG": "BlockTag",
        "SPAWN_EGG": "SpawnEgg",
    }
    game_values = [
        "// Generated by scripts/generate_action_catalog.py; do not edit by hand.",
        "use super::super::typed::ItemKind;",
        "",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
        "pub struct GameValueSpec {",
        "    pub name: &'static str,",
        "    pub aliases: &'static [&'static str],",
        "    pub category: &'static str,",
        "    pub return_type: ItemKind,",
        "}",
        "",
        "pub static GAME_VALUES: &[GameValueSpec] = &[",
    ]
    for value in dump["gameValues"]:
        raw_type = value["icon"]["returnType"]
        try:
            return_type = return_types[raw_type]
        except KeyError as error:
            raise SystemExit(f"unsupported game value return type {raw_type!r}") from error
        name = clean_display_name(value["icon"]["name"])
        game_values += [
            "    GameValueSpec {",
            f"        name: {rust_string(name)},",
            "        aliases: &["
            + ", ".join(rust_string(alias) for alias in value.get("aliases", []))
            + "],",
            f"        category: {rust_string(value['category'])},",
            f"        return_type: ItemKind::{return_type},",
            "    },",
        ]
    game_values += [
        "];",
        "",
        "pub fn find_game_value(name: &str) -> Option<&'static GameValueSpec> {",
        "    GAME_VALUES.iter().find(|value| value.name == name)",
        "}",
    ]
    (generated_dir / "game_values.rs").write_text("\n".join(game_values) + "\n")

    catalogs = (
        ("sounds.rs", "SoundSpec", "SOUNDS", "sounds", "sound", "soundId"),
        ("particles.rs", "ParticleSpec", "PARTICLES", "particles", "particle", "particleId"),
        ("potions.rs", "PotionSpec", "POTIONS", "potions", "potion", "potionId"),
    )
    for filename, spec_name, constant, source_key, name_key, id_key in catalogs:
        if source_key == "particles":
            (generated_dir / filename).write_text(
                particle_catalog_source(dump[source_key]) + "\n"
            )
            continue
        entries = [
            "// Generated by scripts/generate_action_catalog.py; do not edit by hand.",
            "",
            "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
            f"pub struct {spec_name} {{",
            "    pub name: &'static str,",
            "    pub id: &'static str,",
            "}",
        ]
        entries += ["", f"pub static {constant}: &[{spec_name}] = &["]
        for entry in dump[source_key]:
            entries += [
                f"    {spec_name} {{",
                f"        name: {rust_string(entry[name_key])},",
                f"        id: {rust_string(entry[id_key])},",
                "    },",
            ]
        entries += ["];", ""]
        (generated_dir / filename).write_text("\n".join(entries))

    subprocess.run(
        [
            "rustfmt",
            "--edition",
            "2024",
            *(
                str(path)
                for path in [
                    output,
                    generated_dir / "blocks.rs",
                    generated_dir / "actions.rs",
                    generated_dir / "game_values.rs",
                    generated_dir / "sounds.rs",
                    generated_dir / "particles.rs",
                    generated_dir / "potions.rs",
                ]
            ),
        ],
        check=True,
    )


if __name__ == "__main__":
    main()
