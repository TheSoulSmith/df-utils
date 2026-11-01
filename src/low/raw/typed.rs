//! Strongly typed DiamondFire lines backed by the generated action catalog.
//! Wire JSON is used only at import/export boundaries; script bodies are Rust
//! enums and nested control-flow nodes. Unsupported actiondump OR signatures
//! can be imported and preserved, but cannot be newly constructed.

use std::error::Error;
use std::fmt;

use serde_json::{Map, Value, json};

use super::catalog::{self, Action, ActionSpec, ArgKind, BlockKind, TagSpec};

#[derive(Debug)]
pub enum ScriptError {
    Invalid(String),
    Json(serde_json::Error),
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(message) => write!(f, "invalid DiamondFire script: {message}"),
            Self::Json(error) => write!(f, "invalid template JSON: {error}"),
        }
    }
}

impl Error for ScriptError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Invalid(_) => None,
        }
    }
}

impl From<serde_json::Error> for ScriptError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VariableScope {
    Line,
    Local,
    Unsaved,
    Saved,
}

impl VariableScope {
    fn as_wire(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Local => "local",
            Self::Unsaved => "unsaved",
            Self::Saved => "saved",
        }
    }

    fn from_wire(value: &str) -> Option<Self> {
        match value {
            "line" => Some(Self::Line),
            "local" => Some(Self::Local),
            "unsaved" => Some(Self::Unsaved),
            "saved" => Some(Self::Saved),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    Number(String),
    Text(String),
    Component(String),
    Variable {
        name: String,
        scope: VariableScope,
    },
    GameValue {
        name: String,
        target: String,
    },
    BucketVariable {
        name: String,
        bucket: String,
        namespace_type: String,
        namespace_alias: String,
    },
    Location {
        x: f64,
        y: f64,
        z: f64,
        pitch: f64,
        yaw: f64,
        block_aligned: bool,
    },
    Vector {
        x: f64,
        y: f64,
        z: f64,
    },
    MinecraftItem(String),
    Sound {
        id: String,
        pitch: f64,
        volume: f64,
        mapping_version: u32,
    },
    Particle {
        id: String,
        amount: u32,
        horizontal: f64,
        vertical: f64,
        x: f64,
        y: f64,
        z: f64,
        motion_variation: u32,
        mapping_version: u32,
    },
    Potion {
        id: String,
        duration_ticks: u32,
        amplifier: u8,
        mapping_version: u32,
    },
}

impl Item {
    fn wire_id(&self) -> &'static str {
        match self {
            Self::Number(_) => "num",
            Self::Text(_) => "txt",
            Self::Component(_) => "comp",
            Self::Variable { .. } => "var",
            Self::GameValue { .. } => "g_val",
            Self::BucketVariable { .. } => "bucket_var",
            Self::Location { .. } => "loc",
            Self::Vector { .. } => "vec",
            Self::MinecraftItem(_) => "item",
            Self::Sound { .. } => "snd",
            Self::Particle { .. } => "part",
            Self::Potion { .. } => "pot",
        }
    }

    fn version(&self) -> u8 {
        if matches!(
            self,
            Self::Sound { .. } | Self::Particle { .. } | Self::Potion { .. }
        ) {
            1
        } else {
            0
        }
    }

    fn matches(&self, expected: ArgKind) -> bool {
        use ArgKind::*;
        if expected == Variable {
            return matches!(self, Self::Variable { .. });
        }
        if matches!(expected, List | Dict) {
            return matches!(self, Self::Variable { .. });
        }
        if matches!(self, Self::Variable { .. } | Self::GameValue { .. }) {
            return true;
        }
        match expected {
            AnyType => true,
            Number | Byte => matches!(self, Self::Number(_)),
            Text | BlockTag => matches!(self, Self::Text(_)),
            EntityType => matches!(self, Self::Text(_) | Self::MinecraftItem(_)),
            Block => matches!(self, Self::Text(_) | Self::MinecraftItem(_)),
            Component => matches!(self, Self::Component(_)),
            Dialog => false,
            Variable | List | Dict => matches!(self, Self::Variable { .. }),
            Location => matches!(self, Self::Location { .. }),
            Vector => matches!(self, Self::Vector { .. }),
            Item | Projectile | SpawnEgg | Vehicle => {
                matches!(self, Self::MinecraftItem(_))
            }
            Sound => matches!(self, Self::Sound { .. }),
            Particle => matches!(self, Self::Particle { .. }),
            Potion => matches!(self, Self::Potion { .. }),
        }
    }

    fn to_value(&self) -> Value {
        let data = match self {
            Self::Number(name) | Self::Text(name) | Self::Component(name) => json!({"name": name}),
            Self::Variable { name, scope } => {
                json!({"name": name, "scope": scope.as_wire()})
            }
            Self::GameValue { name, target } => json!({"type": name, "target": target}),
            Self::BucketVariable {
                name,
                bucket,
                namespace_type,
                namespace_alias,
            } => json!({
                "name": name,
                "key": bucket,
                "namespace_type": namespace_type,
                "namespace_alias": namespace_alias
            }),
            Self::Location {
                x,
                y,
                z,
                pitch,
                yaw,
                block_aligned,
            } => json!({
                "isBlock": block_aligned,
                "loc": {"x": x, "y": y, "z": z, "pitch": pitch, "yaw": yaw}
            }),
            Self::Vector { x, y, z } => json!({"x": x, "y": y, "z": z}),
            Self::MinecraftItem(item) => json!({"item": item}),
            Self::Sound {
                id,
                pitch,
                volume,
                mapping_version,
            } => json!({
                "mappingVersion": mapping_version, "sound": id, "pitch": pitch, "vol": volume
            }),
            Self::Particle {
                id,
                amount,
                horizontal,
                vertical,
                x,
                y,
                z,
                motion_variation,
                mapping_version,
            } => json!({
                "mappingVersion": mapping_version,
                "particle": id,
                "cluster": {
                    "amount": amount,
                    "horizontal": horizontal,
                    "vertical": vertical
                },
                "data": {
                    "x": x,
                    "y": y,
                    "z": z,
                    "motionVariation": motion_variation
                }
            }),
            Self::Potion {
                id,
                duration_ticks,
                amplifier,
                mapping_version,
            } => json!({
                "mappingVersion": mapping_version, "pot": id, "dur": duration_ticks, "amp": amplifier
            }),
        };
        json!({"version": self.version(), "id": self.wire_id(), "data": data})
    }

    fn from_value(value: &Value) -> Result<Self, ScriptError> {
        let item = value
            .get("item")
            .ok_or_else(|| ScriptError::Invalid("slotted item has no `item` object".into()))?;
        Self::from_item_value(item)
    }

    fn from_item_value(item: &Value) -> Result<Self, ScriptError> {
        let id = string_field(item, "id")?;
        let data = item
            .get("data")
            .ok_or_else(|| ScriptError::Invalid(format!("{id} item has no data")))?;
        let read_num = |field: &str| -> Result<f64, ScriptError> {
            data.get(field).and_then(Value::as_f64).ok_or_else(|| {
                ScriptError::Invalid(format!("{id} item is missing numeric {field}"))
            })
        };
        let result = match id {
            "num" => Self::Number(string_field(data, "name")?.to_owned()),
            "txt" => Self::Text(string_field(data, "name")?.to_owned()),
            "comp" => Self::Component(string_field(data, "name")?.to_owned()),
            "var" => Self::Variable {
                name: string_field(data, "name")?.to_owned(),
                scope: VariableScope::from_wire(string_field(data, "scope")?)
                    .ok_or_else(|| ScriptError::Invalid("variable has an unknown scope".into()))?,
            },
            "g_val" => Self::GameValue {
                name: string_field(data, "type")?.to_owned(),
                target: string_field(data, "target")?.to_owned(),
            },
            "bucket_var" => Self::BucketVariable {
                name: string_field(data, "name")?.to_owned(),
                bucket: string_field(data, "key")?.to_owned(),
                namespace_type: string_field(data, "namespace_type")?.to_owned(),
                namespace_alias: string_field(data, "namespace_alias")?.to_owned(),
            },
            "loc" => {
                let loc = data
                    .get("loc")
                    .ok_or_else(|| ScriptError::Invalid("location has no `loc` object".into()))?;
                Self::Location {
                    x: number_field(loc, "x")?,
                    y: number_field(loc, "y")?,
                    z: number_field(loc, "z")?,
                    pitch: number_field(loc, "pitch")?,
                    yaw: number_field(loc, "yaw")?,
                    block_aligned: data
                        .get("isBlock")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                }
            }
            "vec" => Self::Vector {
                x: read_num("x")?,
                y: read_num("y")?,
                z: read_num("z")?,
            },
            "item" => Self::MinecraftItem(string_field(data, "item")?.to_owned()),
            "snd" => Self::Sound {
                id: string_field(data, "sound")?.to_owned(),
                pitch: read_num("pitch")?,
                volume: read_num("vol")?,
                mapping_version: mapping_version(data)?,
            },
            "part" => {
                let cluster = data.get("cluster").ok_or_else(|| {
                    ScriptError::Invalid("particle is missing its cluster data".into())
                })?;
                Self::Particle {
                    id: string_field(data, "particle")?.to_owned(),
                    amount: cluster
                        .get("amount")
                        .and_then(Value::as_u64)
                        .and_then(|n| u32::try_from(n).ok())
                        .ok_or_else(|| ScriptError::Invalid("invalid particle amount".into()))?,
                    horizontal: number_field(cluster, "horizontal")?,
                    vertical: number_field(cluster, "vertical")?,
                    x: number_field(
                        data.get("data").ok_or_else(|| {
                            ScriptError::Invalid("particle is missing data values".into())
                        })?,
                        "x",
                    )?,
                    y: number_field(data.get("data").unwrap(), "y")?,
                    z: number_field(data.get("data").unwrap(), "z")?,
                    motion_variation: data
                        .get("data")
                        .and_then(|values| values.get("motionVariation"))
                        .and_then(Value::as_u64)
                        .and_then(|n| u32::try_from(n).ok())
                        .ok_or_else(|| {
                            ScriptError::Invalid("invalid particle motion variation".into())
                        })?,
                    mapping_version: mapping_version(data)?,
                }
            }
            "pot" => Self::Potion {
                id: string_field(data, "pot")?.to_owned(),
                duration_ticks: data
                    .get("dur")
                    .and_then(Value::as_u64)
                    .and_then(|n| u32::try_from(n).ok())
                    .ok_or_else(|| ScriptError::Invalid("invalid potion duration".into()))?,
                amplifier: data
                    .get("amp")
                    .and_then(Value::as_u64)
                    .and_then(|n| u8::try_from(n).ok())
                    .ok_or_else(|| ScriptError::Invalid("invalid potion amplifier".into()))?,
                mapping_version: mapping_version(data)?,
            },
            other => {
                return Err(ScriptError::Invalid(format!(
                    "unsupported wire item type {other:?}"
                )));
            }
        };
        let expected_version = result.version();
        if item.get("version").and_then(Value::as_u64) != Some(u64::from(expected_version)) {
            return Err(ScriptError::Invalid(format!(
                "wire item {id:?} has an unsupported version"
            )));
        }
        Ok(result)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tag {
    name: String,
    option: String,
    owner_block: BlockKind,
    owner_action: String,
}

impl Tag {
    pub fn new(action: Action, name: &str, option: &str) -> Result<Self, ScriptError> {
        let spec = action.spec();
        let tag_spec = find_tag(spec, name).ok_or_else(|| {
            ScriptError::Invalid(format!("{} has no tag named {name:?}", spec.name))
        })?;
        if !tag_spec.options.contains(&option) {
            return Err(ScriptError::Invalid(format!(
                "{option:?} is not a valid option for {}",
                tag_spec.name
            )));
        }
        let owner_action = if matches!(
            spec.block,
            BlockKind::IfPlayer | BlockKind::IfEntity | BlockKind::IfVar | BlockKind::IfGame
        ) {
            spec.aliases.first().copied().unwrap_or(spec.name)
        } else {
            spec.name
        };
        Ok(Self {
            name: name.to_owned(),
            option: option.to_owned(),
            owner_block: spec.block,
            owner_action: owner_action.to_owned(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Selection,
    Default,
    Killer,
    Damager,
    Shooter,
    Victim,
    AllPlayers,
    Projectile,
    AllEntities,
    AllMobs,
    LastEntity,
}

impl Target {
    fn as_wire(self) -> &'static str {
        match self {
            Self::Selection => "Selection",
            Self::Default => "Default",
            Self::Killer => "Killer",
            Self::Damager => "Damager",
            Self::Shooter => "Shooter",
            Self::Victim => "Victim",
            Self::AllPlayers => "AllPlayers",
            Self::Projectile => "Projectile",
            Self::AllEntities => "AllEntities",
            Self::AllMobs => "AllMobs",
            Self::LastEntity => "LastEntity",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActionCall {
    action: Action,
    dynamic_name: Option<String>,
    sub_action: Option<Action>,
    imported: bool,
    arguments: Vec<(usize, Item)>,
    tags: Vec<Tag>,
    target: Option<Target>,
    negated: bool,
}

impl ActionCall {
    pub fn new(action: Action, arguments: Vec<Item>) -> Result<Self, ScriptError> {
        if matches!(
            action.spec().block,
            BlockKind::CallFunc | BlockKind::StartProcess
        ) {
            return Err(ScriptError::Invalid(
                "use ActionCall::dynamic for a function or process call".into(),
            ));
        }
        if let Some(replacement) = action.spec().legacy_replacement {
            return Err(ScriptError::Invalid(format!(
                "{} is deprecated; use {replacement}",
                action.spec().name
            )));
        }
        if action.spec().sub_action_blocks.is_empty() {
            validate_arguments(action.spec(), &arguments)?;
        }
        if arguments.len() > 27 {
            return Err(ScriptError::Invalid(
                "a DiamondFire block chest has 27 slots".into(),
            ));
        }
        Ok(Self {
            action,
            dynamic_name: None,
            sub_action: None,
            imported: false,
            arguments: arguments.into_iter().enumerate().collect(),
            tags: Vec::new(),
            target: None,
            negated: false,
        })
    }

    pub fn dynamic(
        action: Action,
        name: impl Into<String>,
        arguments: Vec<Item>,
    ) -> Result<Self, ScriptError> {
        if !matches!(
            action.spec().block,
            BlockKind::CallFunc | BlockKind::StartProcess
        ) {
            return Err(ScriptError::Invalid(
                "dynamic calls are only valid for call_func and start_process".into(),
            ));
        }
        if arguments.len() > 25 {
            return Err(ScriptError::Invalid(
                "dynamic calls have at most 25 argument slots".into(),
            ));
        }
        let name = name.into();
        if name.is_empty() {
            return Err(ScriptError::Invalid(
                "function or process call name cannot be empty".into(),
            ));
        }
        Ok(Self {
            action,
            dynamic_name: Some(name),
            sub_action: None,
            imported: false,
            arguments: arguments.into_iter().enumerate().collect(),
            tags: Vec::new(),
            target: None,
            negated: false,
        })
    }

    pub fn with_sub_action(mut self, sub_action: Action) -> Result<Self, ScriptError> {
        if !self
            .action
            .spec()
            .sub_action_blocks
            .contains(&sub_action.spec().block)
        {
            return Err(ScriptError::Invalid(format!(
                "{} does not accept a {} sub-action",
                self.action.spec().name,
                sub_action.spec().block.as_wire()
            )));
        }
        validate_arguments(
            sub_action.spec(),
            &self
                .arguments
                .iter()
                .map(|(_, item)| item.clone())
                .collect::<Vec<_>>(),
        )?;
        self.sub_action = Some(sub_action);
        Ok(self)
    }

    pub fn with_tag(mut self, name: &str, option: &str) -> Result<Self, ScriptError> {
        let owner = self.tag_owner();
        let tag = Tag::new(owner, name, option)?;
        if self.tags.iter().any(|existing| existing.name == tag.name) {
            return Err(ScriptError::Invalid(format!(
                "tag {:?} was set more than once",
                tag.name
            )));
        }
        self.tags.push(tag);
        Ok(self)
    }

    pub fn with_target(mut self, target: Target) -> Result<Self, ScriptError> {
        let kind = self.action.spec().block;
        if !matches!(
            kind,
            BlockKind::PlayerAction
                | BlockKind::EntityAction
                | BlockKind::IfPlayer
                | BlockKind::IfEntity
        ) {
            return Err(ScriptError::Invalid(format!(
                "{} does not accept a target",
                kind.as_wire()
            )));
        }
        if matches!(kind, BlockKind::PlayerAction | BlockKind::IfPlayer)
            && matches!(
                target,
                Target::Projectile | Target::AllEntities | Target::AllMobs | Target::LastEntity
            )
        {
            return Err(ScriptError::Invalid(
                "target is not supported by this player block".into(),
            ));
        }
        self.target = Some(target);
        Ok(self)
    }

    pub fn negated(mut self) -> Result<Self, ScriptError> {
        if !matches!(
            self.action.spec().block,
            BlockKind::IfPlayer
                | BlockKind::IfEntity
                | BlockKind::IfVar
                | BlockKind::IfGame
                | BlockKind::SelectObj
                | BlockKind::Repeat
        ) {
            return Err(ScriptError::Invalid(
                "NOT is only supported by conditional, repeat, and selection blocks".into(),
            ));
        }
        self.negated = true;
        Ok(self)
    }

    pub fn action(&self) -> Action {
        self.action
    }

    pub fn arguments(&self) -> impl Iterator<Item = &Item> {
        self.arguments.iter().map(|(_, item)| item)
    }

    pub fn with_arguments(mut self, arguments: Vec<Item>) -> Result<Self, ScriptError> {
        if arguments.len() > 25
            && matches!(
                self.action.spec().block,
                BlockKind::CallFunc | BlockKind::StartProcess
            )
        {
            return Err(ScriptError::Invalid(
                "dynamic calls have at most 25 argument slots".into(),
            ));
        }
        let signature = self.sub_action.unwrap_or(self.action).spec();
        if !matches!(
            self.action.spec().block,
            BlockKind::CallFunc | BlockKind::StartProcess
        ) {
            if self.imported {
                validate_imported_arguments(signature, &arguments)?;
            } else {
                validate_arguments(signature, &arguments)?;
            }
        }
        self.arguments = arguments.into_iter().enumerate().collect();
        self.imported = false;
        self.validate_complete()?;
        Ok(self)
    }

    pub fn without_tag(mut self, name: &str) -> Self {
        self.tags.retain(|tag| tag.name != name);
        self
    }

    fn validate_complete(&self) -> Result<(), ScriptError> {
        let spec = self.action.spec();
        if let Some(replacement) = spec.legacy_replacement {
            return Err(ScriptError::Invalid(format!(
                "{} is deprecated; use {replacement}",
                spec.name
            )));
        }
        if let Some(sub_action) = self.sub_action
            && let Some(replacement) = sub_action.spec().legacy_replacement
        {
            return Err(ScriptError::Invalid(format!(
                "{} is deprecated; use {replacement}",
                sub_action.spec().name
            )));
        }
        let signature = self.sub_action.unwrap_or(self.action).spec();
        let arguments = self
            .arguments
            .iter()
            .map(|(_, item)| item.clone())
            .collect::<Vec<_>>();
        if !matches!(spec.block, BlockKind::CallFunc | BlockKind::StartProcess) {
            if self.imported {
                validate_imported_arguments(signature, &arguments)?;
            } else {
                validate_arguments(signature, &arguments)?;
            }
        }
        if !spec.sub_action_blocks.is_empty() && self.sub_action.is_none() {
            return Err(ScriptError::Invalid(format!(
                "{} requires a condition sub-action",
                spec.name
            )));
        }
        if self.negated
            && !matches!(
                spec.block,
                BlockKind::IfPlayer
                    | BlockKind::IfEntity
                    | BlockKind::IfVar
                    | BlockKind::IfGame
                    | BlockKind::SelectObj
                    | BlockKind::Repeat
            )
        {
            return Err(ScriptError::Invalid(
                "NOT is not supported on this block".into(),
            ));
        }
        self.validate_tags()
    }

    fn tag_owner(&self) -> Action {
        self.sub_action.unwrap_or(self.action)
    }

    fn to_value(&self) -> Value {
        let spec = self.action.spec();
        let mut fields = serde_json::Map::new();
        fields.insert("id".into(), json!("block"));
        fields.insert("block".into(), json!(spec.block.as_wire()));
        if let Some(name) = &self.dynamic_name {
            fields.insert("data".into(), json!(name));
        } else {
            fields.insert("action".into(), json!(spec.name));
        }
        if let Some(target) = self.target {
            fields.insert("target".into(), json!(target.as_wire()));
        }
        if self.negated {
            fields.insert("attribute".into(), json!("NOT"));
        }
        if let Some(sub_action) = self.sub_action {
            fields.insert("subAction".into(), json!(sub_action_wire_name(sub_action)));
        }
        let mut items: Vec<Value> = self
            .arguments
            .iter()
            .map(|(slot, argument)| json!({"item": argument.to_value(), "slot": slot}))
            .collect();
        for tag in &self.tags {
            let Some(tag_spec) = find_tag(self.tag_owner().spec(), &tag.name) else {
                continue;
            };
            items.push(json!({
                "item": {
                    "version": 0,
                    "id": "bl_tag",
                    "data": {
                        "tag": tag.name,
                        "option": tag.option,
                        "action": tag.owner_action,
                        "block": tag.owner_block.as_wire()
                    }
                },
                "slot": tag_spec.slot
            }));
        }
        items.sort_by_key(|item| item.get("slot").and_then(Value::as_u64).unwrap_or(0));
        fields.insert("args".into(), json!({"items": items}));
        if !spec.sub_action_blocks.is_empty() && self.sub_action.is_none() {
            fields.insert("subAction".into(), json!(spec.sub_action));
        }
        Value::Object(fields)
    }

    fn from_value(value: &Value) -> Result<Self, ScriptError> {
        let block = BlockKind::from_wire(string_field(value, "block")?)
            .ok_or_else(|| ScriptError::Invalid("unknown block type".into()))?;
        let dynamic_name = if matches!(block, BlockKind::CallFunc | BlockKind::StartProcess) {
            Some(string_field(value, "data")?.to_owned())
        } else {
            None
        };
        let action_name = if dynamic_name.is_some() {
            "dynamic"
        } else {
            string_field(value, "action")?
        };
        let base_action = Action::from_wire(block, action_name).ok_or_else(|| {
            ScriptError::Invalid(format!(
                "action {action_name:?} is not in the bundled actiondump for {}",
                block.as_wire()
            ))
        })?;
        let mut arguments = Vec::new();
        let mut tags = Vec::new();
        if let Some(items) = value
            .get("args")
            .and_then(|args| args.get("items"))
            .and_then(Value::as_array)
        {
            for entry in items {
                let slot = entry
                    .get("slot")
                    .and_then(Value::as_u64)
                    .and_then(|slot| usize::try_from(slot).ok())
                    .ok_or_else(|| ScriptError::Invalid("invalid item slot".into()))?;
                let id = entry
                    .get("item")
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str);
                if id == Some("bl_tag") {
                    let data = entry
                        .get("item")
                        .and_then(|item| item.get("data"))
                        .ok_or_else(|| ScriptError::Invalid("tag has no data".into()))?;
                    let name = string_field(data, "tag")?;
                    let option = string_field(data, "option")?;
                    let owner_name = string_field(data, "action")?;
                    let owner_block = BlockKind::from_wire(string_field(data, "block")?)
                        .ok_or_else(|| ScriptError::Invalid("unknown tag owner block".into()))?;
                    let owner = find_action_by_owner(owner_block, owner_name).ok_or_else(|| {
                        ScriptError::Invalid(format!("unknown tag owner {owner_name:?}"))
                    })?;
                    let mut tag = Tag::new(owner, name, option)?;
                    if find_tag(owner.spec(), name).map(|spec| spec.slot) != Some(slot as u8) {
                        return Err(ScriptError::Invalid(format!(
                            "tag {name:?} is in the wrong chest slot"
                        )));
                    }
                    tag.owner_block = owner_block;
                    tag.owner_action = owner_name.to_owned();
                    tags.push((slot, tag));
                } else {
                    arguments.push((slot, Item::from_value(entry)?));
                }
            }
        }
        arguments.sort_by_key(|(slot, _)| *slot);
        tags.sort_by_key(|(slot, _)| *slot);
        let ordered_args: Vec<Item> = arguments.iter().map(|(_, item)| item.clone()).collect();
        let action = if base_action.spec().sub_action_blocks.is_empty()
            && !matches!(block, BlockKind::CallFunc | BlockKind::StartProcess)
        {
            resolve_action(block, action_name, &ordered_args).unwrap_or(base_action)
        } else {
            base_action
        };
        let sub_action = value
            .get("subAction")
            .and_then(Value::as_str)
            .and_then(|sub| find_sub_action(action, sub, &ordered_args));
        if !action.spec().sub_action_blocks.is_empty() && sub_action.is_none() {
            return Err(ScriptError::Invalid(format!(
                "{} requires a recognized condition sub-action",
                action.spec().name
            )));
        }
        if let Some(sub_action) = sub_action {
            validate_imported_arguments(sub_action.spec(), &ordered_args)?;
        } else if action.spec().sub_action_blocks.is_empty()
            && !matches!(block, BlockKind::CallFunc | BlockKind::StartProcess)
        {
            validate_imported_arguments(action.spec(), &ordered_args)?;
        }
        let mut call = Self {
            action,
            dynamic_name,
            sub_action,
            imported: true,
            arguments,
            tags: tags.into_iter().map(|(_, tag)| tag).collect(),
            target: match value.get("target") {
                None => None,
                Some(target) => Some(
                    target
                        .as_str()
                        .and_then(target_from_wire)
                        .ok_or_else(|| ScriptError::Invalid("unknown target value".into()))?,
                ),
            },
            negated: match value.get("attribute").and_then(Value::as_str) {
                None => false,
                Some("NOT") => true,
                Some(_) => return Err(ScriptError::Invalid("unknown block attribute".into())),
            },
        };
        call.validate_complete()?;
        if let Some(target) = call.target {
            call = call.with_target(target)?;
        }
        Ok(call)
    }

    fn validate_tags(&self) -> Result<(), ScriptError> {
        let mut used_slots = Vec::new();
        for (slot, _) in &self.arguments {
            if *slot > 26 || used_slots.contains(slot) {
                return Err(ScriptError::Invalid(
                    "invalid or duplicate argument chest slot".into(),
                ));
            }
            used_slots.push(*slot);
        }
        for tag in &self.tags {
            let spec = find_tag(self.tag_owner().spec(), &tag.name)
                .ok_or_else(|| ScriptError::Invalid(format!("unknown tag {:?}", tag.name)))?;
            if tag.owner_block != self.tag_owner().spec().block {
                return Err(ScriptError::Invalid(format!(
                    "tag {:?} belongs to a different block",
                    tag.name
                )));
            }
            if !spec.options.contains(&tag.option.as_str()) {
                return Err(ScriptError::Invalid(format!(
                    "invalid option {:?} for tag {:?}",
                    tag.option, tag.name
                )));
            }
            if used_slots.contains(&(spec.slot as usize)) {
                return Err(ScriptError::Invalid(
                    "argument and tag items overlap a chest slot".into(),
                ));
            }
            let owner = self.tag_owner();
            let expected_action = if matches!(
                owner.spec().block,
                BlockKind::IfPlayer | BlockKind::IfEntity | BlockKind::IfVar | BlockKind::IfGame
            ) {
                owner
                    .spec()
                    .aliases
                    .first()
                    .copied()
                    .unwrap_or(owner.spec().name)
            } else {
                owner.spec().name
            };
            if tag.owner_block != owner.spec().block || tag.owner_action != expected_action {
                return Err(ScriptError::Invalid(format!(
                    "tag {:?} has an owner inconsistent with its block",
                    tag.name
                )));
            }
            used_slots.push(spec.slot as usize);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Header(HeaderKind);

#[derive(Clone, Debug, PartialEq)]
enum HeaderKind {
    Event(ActionCall),
    Function {
        name: String,
        hidden: bool,
        hint: bool,
        parameters: Vec<(usize, FunctionParameter)>,
        extra_items: Vec<(usize, Item)>,
    },
    Process {
        name: String,
        hidden: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParameterType {
    Text,
    Component,
    Number,
    Location,
    Vector,
    Sound,
    Particle,
    Potion,
    Item,
    Dialog,
    Any,
    Variable,
    List,
    Dictionary,
}

impl ParameterType {
    fn as_wire(self) -> &'static str {
        match self {
            Self::Text => "txt",
            Self::Component => "comp",
            Self::Number => "num",
            Self::Location => "loc",
            Self::Vector => "vec",
            Self::Sound => "snd",
            Self::Particle => "part",
            Self::Potion => "pot",
            Self::Item => "item",
            Self::Dialog => "dialog",
            Self::Any => "any",
            Self::Variable => "var",
            Self::List => "list",
            Self::Dictionary => "dict",
        }
    }

    fn from_wire(value: &str) -> Option<Self> {
        Some(match value {
            "txt" => Self::Text,
            "comp" => Self::Component,
            "num" => Self::Number,
            "loc" => Self::Location,
            "vec" => Self::Vector,
            "snd" => Self::Sound,
            "part" => Self::Particle,
            "pot" => Self::Potion,
            "item" => Self::Item,
            "dialog" => Self::Dialog,
            "any" => Self::Any,
            "var" => Self::Variable,
            "list" => Self::List,
            "dict" => Self::Dictionary,
            _ => return None,
        })
    }

    fn accepts(self, item: &Item) -> bool {
        if self == Self::Dialog {
            return false;
        }
        item.matches(match self {
            Self::Text => ArgKind::Text,
            Self::Component => ArgKind::Component,
            Self::Number => ArgKind::Number,
            Self::Location => ArgKind::Location,
            Self::Vector => ArgKind::Vector,
            Self::Sound => ArgKind::Sound,
            Self::Particle => ArgKind::Particle,
            Self::Potion => ArgKind::Potion,
            Self::Item => ArgKind::Item,
            Self::Any => ArgKind::AnyType,
            Self::Variable => ArgKind::Variable,
            Self::List => ArgKind::List,
            Self::Dictionary => ArgKind::Dict,
            Self::Dialog => return false,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionParameter {
    pub name: String,
    pub parameter_type: ParameterType,
    pub plural: bool,
    pub optional: bool,
    pub description: Option<String>,
    pub note: Option<String>,
    pub default_value: Option<Item>,
}

type SlottedParameters = Vec<(usize, FunctionParameter)>;
type SlottedItems = Vec<(usize, Item)>;
type ParsedFunctionHeader = (SlottedParameters, SlottedItems);

impl FunctionParameter {
    pub fn new(name: impl Into<String>, parameter_type: ParameterType) -> Self {
        Self {
            name: name.into(),
            parameter_type,
            plural: false,
            optional: false,
            description: None,
            note: None,
            default_value: None,
        }
    }

    pub fn optional(mut self, default_value: Option<Item>) -> Result<Self, ScriptError> {
        if let Some(value) = &default_value
            && !self.parameter_type.accepts(value)
        {
            return Err(ScriptError::Invalid(
                "function parameter default does not match its declared type".into(),
            ));
        }
        self.optional = true;
        self.default_value = default_value;
        Ok(self)
    }

    pub fn plural(mut self) -> Self {
        self.plural = true;
        self
    }
}

impl Header {
    pub fn event(action: Action, arguments: Vec<Item>) -> Result<Self, ScriptError> {
        if !matches!(
            action.spec().block,
            BlockKind::Event | BlockKind::EntityEvent | BlockKind::GameEvent
        ) {
            return Err(ScriptError::Invalid(
                "event header requires a player, entity, or game event action".into(),
            ));
        }
        Ok(Self(HeaderKind::Event(ActionCall::new(action, arguments)?)))
    }

    pub fn function(name: impl Into<String>, hidden: bool) -> Result<Self, ScriptError> {
        let name = name.into();
        if name.is_empty() {
            return Err(ScriptError::Invalid(
                "function names cannot be empty".into(),
            ));
        }
        Ok(Self(HeaderKind::Function {
            name,
            hidden,
            hint: true,
            parameters: Vec::new(),
            extra_items: Vec::new(),
        }))
    }

    pub fn function_with_parameters(
        name: impl Into<String>,
        hidden: bool,
        parameters: Vec<FunctionParameter>,
    ) -> Result<Self, ScriptError> {
        let parameters = parameters.into_iter().enumerate().collect();
        Self::function_from_parts(name.into(), hidden, true, parameters, Vec::new())
    }

    pub fn function_with_items(
        name: impl Into<String>,
        hidden: bool,
        parameters: Vec<FunctionParameter>,
        extra_items: Vec<(usize, Item)>,
    ) -> Result<Self, ScriptError> {
        let mut occupied = std::collections::HashSet::new();
        for (slot, _) in &extra_items {
            if *slot >= 25 || !occupied.insert(*slot) {
                return Err(ScriptError::Invalid(
                    "function metadata item slots must be unique and below 25".into(),
                ));
            }
        }
        let mut slotted = Vec::new();
        let mut next_slot = 0;
        for parameter in parameters {
            while occupied.contains(&next_slot) {
                next_slot += 1;
            }
            if next_slot >= 25 {
                return Err(ScriptError::Invalid(
                    "function parameters and metadata exceed the 25 available slots".into(),
                ));
            }
            occupied.insert(next_slot);
            slotted.push((next_slot, parameter));
            next_slot += 1;
        }
        Self::function_from_parts(name.into(), hidden, true, slotted, extra_items)
    }

    fn function_from_parts(
        name: String,
        hidden: bool,
        hint: bool,
        parameters: SlottedParameters,
        extra_items: SlottedItems,
    ) -> Result<Self, ScriptError> {
        if parameters.len() + extra_items.len() > 25 {
            return Err(ScriptError::Invalid(
                "function headers have 25 chest slots before their hint and tag".into(),
            ));
        }
        let mut names = std::collections::HashSet::new();
        let mut occupied = std::collections::HashSet::new();
        for (slot, parameter) in &parameters {
            if *slot >= 25 || !occupied.insert(*slot) {
                return Err(ScriptError::Invalid(
                    "function parameter slots must be unique and below 25".into(),
                ));
            }
            if parameter.name.is_empty() || !names.insert(&parameter.name) {
                return Err(ScriptError::Invalid(
                    "function parameter names must be non-empty and unique".into(),
                ));
            }
            if parameter.parameter_type == ParameterType::Variable
                && (parameter.plural || parameter.optional)
            {
                return Err(ScriptError::Invalid(
                    "variable parameters cannot be plural or optional".into(),
                ));
            }
            if !parameter.optional && parameter.default_value.is_some() {
                return Err(ScriptError::Invalid(
                    "only optional parameters can have a default value".into(),
                ));
            }
            if parameter
                .default_value
                .as_ref()
                .is_some_and(|item| !parameter.parameter_type.accepts(item))
            {
                return Err(ScriptError::Invalid(
                    "function parameter default does not match its declared type".into(),
                ));
            }
        }
        for (slot, _) in &extra_items {
            if *slot >= 25 || !occupied.insert(*slot) {
                return Err(ScriptError::Invalid(
                    "function metadata overlaps a parameter or reserved header slot".into(),
                ));
            }
        }
        if name.is_empty() {
            return Err(ScriptError::Invalid(
                "function names cannot be empty".into(),
            ));
        }
        Ok(Self(HeaderKind::Function {
            name,
            hidden,
            hint,
            parameters,
            extra_items,
        }))
    }

    pub fn process(name: impl Into<String>, hidden: bool) -> Result<Self, ScriptError> {
        let name = name.into();
        if name.is_empty() {
            return Err(ScriptError::Invalid("process names cannot be empty".into()));
        }
        Ok(Self(HeaderKind::Process { name, hidden }))
    }

    fn to_value(&self) -> Value {
        match &self.0 {
            HeaderKind::Event(call) => call.to_value(),
            HeaderKind::Function {
                name,
                hidden,
                hint,
                parameters,
                extra_items,
            } => dynamic_header("func", name, *hidden, *hint, parameters, extra_items),
            HeaderKind::Process { name, hidden } => {
                dynamic_header("process", name, *hidden, false, &[], &[])
            }
        }
    }

    fn from_value(value: &Value) -> Result<Self, ScriptError> {
        match value.get("block").and_then(Value::as_str) {
            Some("event" | "entity_event" | "game_event") => {
                Ok(Self(HeaderKind::Event(ActionCall::from_value(value)?)))
            }
            Some("func") => {
                let (parameters, extra_items) = header_parameters(value)?;
                Self::function_from_parts(
                    string_field(value, "data")?.to_owned(),
                    header_hidden(value)?,
                    header_has_hint(value),
                    parameters,
                    extra_items,
                )
            }
            Some("process") => Self::process(string_field(value, "data")?, header_hidden(value)?),
            _ => Err(ScriptError::Invalid(
                "template must begin with an event, function, or process header".into(),
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block(BlockKindNode);

#[derive(Clone, Debug, PartialEq)]
enum BlockKindNode {
    Action(ActionCall),
    If {
        condition: ActionCall,
        then_body: Vec<Block>,
        else_body: Option<Vec<Block>>,
    },
    Repeat {
        repeat: ActionCall,
        body: Vec<Block>,
    },
}

pub type ConditionParts<'a> = (&'a ActionCall, &'a [Block], Option<&'a [Block]>);

impl Block {
    pub fn action(action: Action, arguments: Vec<Item>) -> Result<Self, ScriptError> {
        let call = ActionCall::new(action, arguments)?;
        call.validate_complete()?;
        if is_structural(call.action.spec().block) {
            return Err(ScriptError::Invalid(
                "headers, conditions, and repeats must use their structured constructors".into(),
            ));
        }
        Ok(Self(BlockKindNode::Action(call)))
    }

    pub fn call(call: ActionCall) -> Result<Self, ScriptError> {
        call.validate_complete()?;
        if is_structural(call.action.spec().block) {
            return Err(ScriptError::Invalid(
                "headers, conditions, and repeats must use their structured constructors".into(),
            ));
        }
        Ok(Self(BlockKindNode::Action(call)))
    }

    pub fn conditional(
        condition: ActionCall,
        then_body: Vec<Block>,
        else_body: Option<Vec<Block>>,
    ) -> Result<Self, ScriptError> {
        condition.validate_complete()?;
        if !is_condition(condition.action.spec().block) {
            return Err(ScriptError::Invalid(
                "conditional requires an if-player/entity/game/variable action".into(),
            ));
        }
        Ok(Self(BlockKindNode::If {
            condition,
            then_body,
            else_body,
        }))
    }

    pub fn repeat(repeat: ActionCall, body: Vec<Block>) -> Result<Self, ScriptError> {
        repeat.validate_complete()?;
        if repeat.action.spec().block != BlockKind::Repeat {
            return Err(ScriptError::Invalid(
                "repeat requires a repeat action".into(),
            ));
        }
        Ok(Self(BlockKindNode::Repeat { repeat, body }))
    }

    pub fn action_call(&self) -> Option<&ActionCall> {
        match &self.0 {
            BlockKindNode::Action(call) => Some(call),
            BlockKindNode::If { .. } | BlockKindNode::Repeat { .. } => None,
        }
    }

    pub fn condition(&self) -> Option<ConditionParts<'_>> {
        match &self.0 {
            BlockKindNode::If {
                condition,
                then_body,
                else_body,
            } => Some((condition, then_body, else_body.as_deref())),
            _ => None,
        }
    }

    pub fn repeat_parts(&self) -> Option<(&ActionCall, &[Block])> {
        match &self.0 {
            BlockKindNode::Repeat { repeat, body } => Some((repeat, body)),
            _ => None,
        }
    }
}

/// A structurally valid DiamondFire line: it always has one header, and
/// conditions/repeats own their nested bodies instead of exposing brackets.
#[derive(Clone, Debug, PartialEq)]
pub struct Script {
    header: Header,
    body: Vec<Block>,
    metadata: Map<String, Value>,
}

impl Script {
    pub fn new(header: Header) -> Self {
        Self {
            header,
            body: Vec::new(),
            metadata: Map::new(),
        }
    }

    pub fn body(&self) -> &[Block] {
        &self.body
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn set_header(&mut self, header: Header) {
        self.header = header;
    }

    pub fn push(&mut self, block: Block) {
        self.body.push(block);
    }

    pub fn insert(&mut self, index: usize, block: Block) -> Result<(), ScriptError> {
        if index > self.body.len() {
            return Err(ScriptError::Invalid(format!(
                "block index {index} exceeds body length {}",
                self.body.len()
            )));
        }
        self.body.insert(index, block);
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<Block, ScriptError> {
        if index >= self.body.len() {
            return Err(ScriptError::Invalid(format!(
                "block index {index} exceeds body length {}",
                self.body.len()
            )));
        }
        Ok(self.body.remove(index))
    }

    pub fn replace(&mut self, index: usize, block: Block) -> Result<Block, ScriptError> {
        let len = self.body.len();
        let current = self.body.get_mut(index).ok_or_else(|| {
            ScriptError::Invalid(format!("block index {index} exceeds body length {len}"))
        })?;
        Ok(std::mem::replace(current, block))
    }

    pub fn block_count(&self) -> usize {
        1 + count_blocks(&self.body)
    }

    pub fn measure(&self, limit: usize) -> BlockLength {
        BlockLength {
            blocks: self.block_count(),
            limit,
        }
    }

    pub fn to_value(&self) -> Value {
        let mut blocks = vec![self.header.to_value()];
        encode_blocks(&self.body, &mut blocks);
        let mut root = self.metadata.clone();
        root.insert("blocks".into(), json!(blocks));
        Value::Object(root)
    }

    pub fn to_json(&self) -> Result<String, ScriptError> {
        Ok(serde_json::to_string(&self.to_value())?)
    }

    pub fn to_json_pretty(&self) -> Result<String, ScriptError> {
        Ok(serde_json::to_string_pretty(&self.to_value())?)
    }

    pub fn from_json(json_text: &str) -> Result<Self, ScriptError> {
        Self::from_value(serde_json::from_str(json_text)?)
    }

    pub fn from_value(value: Value) -> Result<Self, ScriptError> {
        let mut metadata = value
            .as_object()
            .cloned()
            .ok_or_else(|| ScriptError::Invalid("template root must be an object".into()))?;
        let blocks = metadata
            .remove("blocks")
            .and_then(|value| value.as_array().cloned())
            .ok_or_else(|| ScriptError::Invalid("missing `blocks` array".into()))?;
        let header_value = blocks
            .first()
            .ok_or_else(|| ScriptError::Invalid("code line is empty".into()))?;
        let header = Header::from_value(header_value)?;
        let mut cursor = 1;
        let body = parse_sequence(&blocks, &mut cursor, None)?;
        if cursor != blocks.len() {
            return Err(ScriptError::Invalid("unexpected trailing bracket".into()));
        }
        Ok(Self {
            header,
            body,
            metadata,
        })
    }

    pub fn compile(&self) -> Result<String, ScriptError> {
        if let HeaderKind::Event(call) = &self.header.0
            && let Some(replacement) = call.action.spec().legacy_replacement
        {
            return Err(ScriptError::Invalid(format!(
                "cannot compile deprecated event {}; use {replacement}",
                call.action.spec().name
            )));
        }
        self.validate_compilable(&self.body)?;
        let json = serde_json::to_vec(&self.to_value())?;
        let mut encoder = flate2::GzBuilder::new()
            .mtime(0)
            .operating_system(255)
            .write(Vec::new(), flate2::Compression::default());
        std::io::Write::write_all(&mut encoder, &json)
            .map_err(|error| ScriptError::Invalid(error.to_string()))?;
        use base64::Engine;
        Ok(base64::engine::general_purpose::STANDARD.encode(
            encoder
                .finish()
                .map_err(|error| ScriptError::Invalid(error.to_string()))?,
        ))
    }

    pub fn decompile(code: &str) -> Result<Self, ScriptError> {
        use base64::Engine;
        use std::io::Read;
        let compact: String = code
            .chars()
            .filter(|ch| !ch.is_ascii_whitespace())
            .collect();
        let mut compact = compact;
        while compact.ends_with('=') {
            compact.pop();
        }
        match compact.len() % 4 {
            0 => {}
            2 => compact.push_str("=="),
            3 => compact.push('='),
            _ => {
                return Err(ScriptError::Invalid(
                    "template code has an invalid Base64 length".into(),
                ));
            }
        }
        let compressed = base64::engine::general_purpose::STANDARD
            .decode(compact)
            .map_err(|error| ScriptError::Invalid(error.to_string()))?;
        let mut decoder = flate2::read::GzDecoder::new(compressed.as_slice());
        let mut json = Vec::new();
        decoder
            .read_to_end(&mut json)
            .map_err(|error| ScriptError::Invalid(error.to_string()))?;
        Self::from_json(
            std::str::from_utf8(&json).map_err(|error| ScriptError::Invalid(error.to_string()))?,
        )
    }

    fn validate_compilable(&self, blocks: &[Block]) -> Result<(), ScriptError> {
        for block in blocks {
            let calls = match &block.0 {
                BlockKindNode::Action(call) => vec![call],
                BlockKindNode::If {
                    condition,
                    then_body,
                    else_body,
                } => {
                    self.validate_compilable(then_body)?;
                    if let Some(body) = else_body {
                        self.validate_compilable(body)?;
                    }
                    vec![condition]
                }
                BlockKindNode::Repeat { repeat, body } => {
                    self.validate_compilable(body)?;
                    vec![repeat]
                }
            };
            for call in calls {
                call.validate_complete()?;
                if let Some(replacement) = call.action.spec().legacy_replacement {
                    return Err(ScriptError::Invalid(format!(
                        "cannot compile deprecated action {}; use {replacement}",
                        call.action.spec().name
                    )));
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockLength {
    pub blocks: usize,
    pub limit: usize,
}

impl BlockLength {
    pub fn within_limit(self) -> bool {
        self.blocks <= self.limit
    }

    pub fn excess(self) -> usize {
        self.blocks.saturating_sub(self.limit)
    }
}

fn validate_arguments(spec: &ActionSpec, args: &[Item]) -> Result<(), ScriptError> {
    if matches_arguments(spec.arguments, args, 0, 0) {
        Ok(())
    } else {
        Err(ScriptError::Invalid(format!(
            "arguments [{}] do not match {} {} signature [{}]",
            args.iter()
                .map(Item::wire_id)
                .collect::<Vec<_>>()
                .join(", "),
            spec.block.as_wire(),
            spec.name,
            spec.arguments
                .iter()
                .map(|arg| format!(
                    "{:?}{}{}",
                    arg.kind,
                    if arg.optional { "?" } else { "" },
                    if arg.plural { "*" } else { "" }
                ))
                .collect::<Vec<_>>()
                .join(", ")
        )))
    }
}

fn validate_imported_arguments(spec: &ActionSpec, args: &[Item]) -> Result<(), ScriptError> {
    if matches_arguments_relaxed(spec.arguments, args, 0, 0) {
        Ok(())
    } else {
        validate_arguments(spec, args)
    }
}

fn resolve_action(block: BlockKind, name: &str, args: &[Item]) -> Option<Action> {
    let mut candidates = Action::ALL.iter().copied().filter(|action| {
        let spec = action.spec();
        spec.block == block && spec.name == name
    });
    candidates
        .clone()
        .find(|action| matches_arguments(action.spec().arguments, args, 0, 0))
        .or_else(|| {
            candidates.find(|action| matches_arguments_relaxed(action.spec().arguments, args, 0, 0))
        })
}

fn matches_arguments_relaxed(
    specs: &[catalog::ArgumentSpec],
    args: &[Item],
    spec_index: usize,
    arg_index: usize,
) -> bool {
    if spec_index == specs.len() {
        return arg_index == args.len();
    }
    let spec = specs[spec_index];
    let max_count = if spec.plural {
        args.len().saturating_sub(arg_index)
    } else {
        1
    };
    for count in 0..=max_count {
        let end = arg_index + count;
        if end > args.len()
            || !args[arg_index..end]
                .iter()
                .all(|arg| arg.matches(spec.kind))
        {
            continue;
        }
        if matches_arguments_relaxed(specs, args, spec_index + 1, end) {
            return true;
        }
    }
    false
}

fn matches_arguments(
    specs: &[catalog::ArgumentSpec],
    args: &[Item],
    spec_index: usize,
    arg_index: usize,
) -> bool {
    if spec_index == specs.len() {
        return arg_index == args.len();
    }
    let spec = specs[spec_index];
    let min_count = usize::from(!spec.optional);
    let max_count = if spec.plural {
        args.len().saturating_sub(arg_index)
    } else {
        1
    };
    for count in min_count..=max_count {
        if count == 0 && !spec.optional {
            continue;
        }
        let end = arg_index + count;
        if end > args.len()
            || !args[arg_index..end]
                .iter()
                .all(|arg| arg.matches(spec.kind))
        {
            continue;
        }
        if matches_arguments(specs, args, spec_index + 1, end) {
            return true;
        }
    }
    false
}

fn find_tag<'a>(spec: &'a ActionSpec, name: &str) -> Option<&'a TagSpec> {
    spec.tags.iter().find(|tag| tag.name == name)
}

fn sub_action_wire_name(action: Action) -> &'static str {
    let spec = action.spec();
    if !spec.sub_action.is_empty() {
        spec.sub_action
    } else {
        spec.aliases.first().copied().unwrap_or(spec.name)
    }
}

fn find_sub_action(parent: Action, wire_name: &str, args: &[Item]) -> Option<Action> {
    let mut candidates = Action::ALL.iter().copied().filter(|candidate| {
        let spec = candidate.spec();
        parent.spec().sub_action_blocks.contains(&spec.block)
            && (spec.name == wire_name
                || spec.sub_action == wire_name
                || spec.aliases.contains(&wire_name))
    });
    candidates
        .clone()
        .find(|action| matches_arguments(action.spec().arguments, args, 0, 0))
        .or_else(|| {
            candidates.find(|action| matches_arguments_relaxed(action.spec().arguments, args, 0, 0))
        })
}

fn find_action_by_owner(block: BlockKind, wire_name: &str) -> Option<Action> {
    Action::ALL.iter().copied().find(|action| {
        action.spec().block == block
            && (action.spec().name == wire_name
                || action.spec().aliases.contains(&wire_name)
                || action.spec().sub_action == wire_name)
    })
}

fn dynamic_header(
    block: &str,
    name: &str,
    hidden: bool,
    hint: bool,
    parameters: &[(usize, FunctionParameter)],
    extra_items: &[(usize, Item)],
) -> Value {
    let is_function = block == "func";
    let owner = if is_function { "func" } else { "process" };
    let mut items = Vec::new();
    if is_function {
        for (slot, parameter) in parameters {
            let mut data = serde_json::Map::new();
            data.insert("name".into(), json!(parameter.name));
            data.insert("type".into(), json!(parameter.parameter_type.as_wire()));
            data.insert("plural".into(), json!(parameter.plural));
            data.insert("optional".into(), json!(parameter.optional));
            if let Some(description) = &parameter.description {
                data.insert("description".into(), json!(description));
            }
            if let Some(note) = &parameter.note {
                data.insert("note".into(), json!(note));
            }
            if let Some(default_value) = &parameter.default_value {
                data.insert("default_value".into(), default_value.to_value());
            }
            items.push(json!({
                "item": {"version": 0, "id": "pn_el", "data": data},
                "slot": slot
            }));
        }
        items.extend(
            extra_items
                .iter()
                .map(|(slot, item)| json!({"item": item.to_value(), "slot": slot})),
        );
        if hint {
            items.push(json!({
                "item": {"version": 0, "id": "hint", "data": {"id": "function"}},
                "slot": 25
            }));
        }
    }
    items.push(json!({
        "item": {
            "version": 0,
            "id": "bl_tag",
            "data": {
                "tag": "Is Hidden",
                "option": if hidden { "True" } else { "False" },
                "action": "dynamic",
                "block": owner
            }
        },
        "slot": 26
    }));
    items.sort_by_key(|item| item.get("slot").and_then(Value::as_u64).unwrap_or(0));
    json!({
        "id": "block",
        "block": block,
        "data": name,
        "args": {"items": items}
    })
}

fn header_hidden(value: &Value) -> Result<bool, ScriptError> {
    for item in value
        .get("args")
        .and_then(|args| args.get("items"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let data = item.get("item").and_then(|item| item.get("data"));
        if data
            .and_then(|data| data.get("tag"))
            .and_then(Value::as_str)
            == Some("Is Hidden")
        {
            return match data
                .and_then(|data| data.get("option"))
                .and_then(Value::as_str)
            {
                Some("True") => Ok(true),
                Some("False") => Ok(false),
                other => Err(ScriptError::Invalid(format!(
                    "invalid Is Hidden option {other:?}"
                ))),
            };
        }
    }
    Ok(false)
}

fn header_has_hint(value: &Value) -> bool {
    value
        .get("args")
        .and_then(|args| args.get("items"))
        .and_then(Value::as_array)
        .is_some_and(|items| {
            items.iter().any(|entry| {
                entry
                    .get("item")
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str)
                    == Some("hint")
            })
        })
}

fn header_parameters(value: &Value) -> Result<ParsedFunctionHeader, ScriptError> {
    let mut parameters = Vec::new();
    let mut extra_items = Vec::new();
    for item in value
        .get("args")
        .and_then(|args| args.get("items"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let Some(wire_item) = item.get("item") else {
            continue;
        };
        let slot = item
            .get("slot")
            .and_then(Value::as_u64)
            .and_then(|slot| usize::try_from(slot).ok())
            .ok_or_else(|| ScriptError::Invalid("invalid function chest slot".into()))?;
        match wire_item.get("id").and_then(Value::as_str) {
            Some("pn_el") => {}
            Some("hint" | "bl_tag") => continue,
            _ => {
                extra_items.push((slot, Item::from_value(item)?));
                continue;
            }
        }
        let data = wire_item
            .get("data")
            .ok_or_else(|| ScriptError::Invalid("parameter item has no data".into()))?;
        let parameter_type = ParameterType::from_wire(string_field(data, "type")?)
            .ok_or_else(|| ScriptError::Invalid("unknown function parameter type".into()))?;
        let mut parameter = FunctionParameter::new(string_field(data, "name")?, parameter_type);
        parameter.plural = data.get("plural").and_then(Value::as_bool).unwrap_or(false);
        parameter.optional = data
            .get("optional")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        parameter.description = data
            .get("description")
            .and_then(Value::as_str)
            .map(str::to_owned);
        parameter.note = data.get("note").and_then(Value::as_str).map(str::to_owned);
        if let Some(default_value) = data.get("default_value") {
            parameter.default_value = Some(Item::from_item_value(default_value)?);
        }
        parameters.push((slot, parameter));
    }
    Ok((parameters, extra_items))
}

fn encode_blocks(blocks: &[Block], output: &mut Vec<Value>) {
    for block in blocks {
        match &block.0 {
            BlockKindNode::Action(call) => output.push(call.to_value()),
            BlockKindNode::If {
                condition,
                then_body,
                else_body,
            } => {
                output.push(condition.to_value());
                output.push(bracket("open", "norm"));
                encode_blocks(then_body, output);
                output.push(bracket("close", "norm"));
                if let Some(else_body) = else_body {
                    output.push(json!({"id": "block", "block": "else"}));
                    output.push(bracket("open", "norm"));
                    encode_blocks(else_body, output);
                    output.push(bracket("close", "norm"));
                }
            }
            BlockKindNode::Repeat { repeat, body } => {
                output.push(repeat.to_value());
                output.push(bracket("open", "repeat"));
                encode_blocks(body, output);
                output.push(bracket("close", "repeat"));
            }
        }
    }
}

fn count_blocks(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .map(|block| match &block.0 {
            BlockKindNode::Action(_) => 1,
            BlockKindNode::If {
                then_body,
                else_body,
                ..
            } => {
                3 + count_blocks(then_body)
                    + else_body
                        .as_ref()
                        .map(|body| 3 + count_blocks(body))
                        .unwrap_or(0)
            }
            BlockKindNode::Repeat { body, .. } => 3 + count_blocks(body),
        })
        .sum()
}

fn parse_sequence(
    blocks: &[Value],
    cursor: &mut usize,
    expected_close: Option<&str>,
) -> Result<Vec<Block>, ScriptError> {
    let mut output = Vec::new();
    while *cursor < blocks.len() {
        let value = &blocks[*cursor];
        if value.get("id").and_then(Value::as_str) == Some("bracket") {
            let direct = string_field(value, "direct")?;
            let bracket_type = string_field(value, "type")?;
            if direct == "close" {
                if expected_close == Some(bracket_type) {
                    *cursor += 1;
                    return Ok(output);
                }
                return Err(ScriptError::Invalid(
                    "unmatched or mistyped closing bracket".into(),
                ));
            }
            return Err(ScriptError::Invalid("unexpected opening bracket".into()));
        }
        if value.get("block").and_then(Value::as_str) == Some("else") {
            return Err(ScriptError::Invalid(
                "else must follow a conditional block".into(),
            ));
        }
        let call = ActionCall::from_value(value)?;
        match call.action.spec().block {
            kind if is_condition(kind) => {
                *cursor += 1;
                expect_open(blocks, cursor, "norm")?;
                let then_body = parse_sequence(blocks, cursor, Some("norm"))?;
                let else_body = if blocks
                    .get(*cursor)
                    .and_then(|next| next.get("block"))
                    .and_then(Value::as_str)
                    == Some("else")
                {
                    *cursor += 1;
                    expect_open(blocks, cursor, "norm")?;
                    Some(parse_sequence(blocks, cursor, Some("norm"))?)
                } else {
                    None
                };
                output.push(Block(BlockKindNode::If {
                    condition: call,
                    then_body,
                    else_body,
                }));
            }
            BlockKind::Repeat => {
                *cursor += 1;
                expect_open(blocks, cursor, "repeat")?;
                let body = parse_sequence(blocks, cursor, Some("repeat"))?;
                output.push(Block(BlockKindNode::Repeat { repeat: call, body }));
            }
            _ => {
                if is_structural(call.action.spec().block) {
                    return Err(ScriptError::Invalid("unexpected structural block".into()));
                }
                *cursor += 1;
                output.push(Block(BlockKindNode::Action(call)));
            }
        }
    }
    if expected_close.is_some() {
        Err(ScriptError::Invalid("unclosed block bracket".into()))
    } else {
        Ok(output)
    }
}

fn expect_open(blocks: &[Value], cursor: &mut usize, expected: &str) -> Result<(), ScriptError> {
    let bracket = blocks
        .get(*cursor)
        .ok_or_else(|| ScriptError::Invalid("missing opening bracket".into()))?;
    if bracket.get("id").and_then(Value::as_str) != Some("bracket")
        || bracket.get("direct").and_then(Value::as_str) != Some("open")
        || bracket.get("type").and_then(Value::as_str) != Some(expected)
    {
        return Err(ScriptError::Invalid(format!(
            "expected opening {expected} bracket after control block"
        )));
    }
    *cursor += 1;
    Ok(())
}

fn bracket(direct: &str, kind: &str) -> Value {
    json!({"id": "bracket", "direct": direct, "type": kind})
}

fn is_condition(kind: BlockKind) -> bool {
    matches!(
        kind,
        BlockKind::IfPlayer | BlockKind::IfEntity | BlockKind::IfVar | BlockKind::IfGame
    )
}

fn is_structural(kind: BlockKind) -> bool {
    is_condition(kind)
        || matches!(
            kind,
            BlockKind::Repeat
                | BlockKind::Event
                | BlockKind::EntityEvent
                | BlockKind::GameEvent
                | BlockKind::Func
                | BlockKind::Process
                | BlockKind::Else
        )
}

fn target_from_wire(value: &str) -> Option<Target> {
    Some(match value {
        "Selection" => Target::Selection,
        "Default" => Target::Default,
        "Killer" => Target::Killer,
        "Damager" => Target::Damager,
        "Shooter" => Target::Shooter,
        "Victim" => Target::Victim,
        "AllPlayers" => Target::AllPlayers,
        "Projectile" => Target::Projectile,
        "AllEntities" => Target::AllEntities,
        "AllMobs" => Target::AllMobs,
        "LastEntity" => Target::LastEntity,
        _ => return None,
    })
}

fn string_field<'a>(value: &'a Value, field: &str) -> Result<&'a str, ScriptError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| ScriptError::Invalid(format!("missing string field {field:?}")))
}

fn number_field(value: &Value, field: &str) -> Result<f64, ScriptError> {
    value
        .get(field)
        .and_then(Value::as_f64)
        .ok_or_else(|| ScriptError::Invalid(format!("missing numeric field {field:?}")))
}

fn mapping_version(value: &Value) -> Result<u32, ScriptError> {
    value
        .get("mappingVersion")
        .and_then(Value::as_u64)
        .and_then(|version| u32::try_from(version).ok())
        .ok_or_else(|| ScriptError::Invalid("missing or invalid mappingVersion".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action(block: BlockKind, name: &str) -> Action {
        Action::from_wire(block, name).expect("action exists in generated catalog")
    }

    #[test]
    fn action_signature_and_tags_are_checked_at_construction() {
        let send_message = action(BlockKind::PlayerAction, "SendMessage");
        assert!(ActionCall::new(send_message, vec![Item::Component("hello".into())]).is_ok());
        assert!(ActionCall::new(send_message, vec![Item::Number("1".into())]).is_err());

        let wait = action(BlockKind::Control, "Wait");
        assert!(ActionCall::new(wait, vec![Item::Number("2".into())]).is_ok());
        let legacy = action(BlockKind::PlayerAction, "NoNatRegen");
        assert!(ActionCall::new(legacy, vec![]).is_err());
        let alternative = action(BlockKind::IfPlayer, "HasRoomForItem");
        assert!(ActionCall::new(alternative, vec![]).is_err());
        assert!(ActionCall::new(Action::IfPlayerHasroomforitemAlternative2, vec![]).is_ok());

        let wait = ActionCall::new(wait, vec![])
            .unwrap()
            .with_tag("Time Unit", "Ticks")
            .unwrap();
        assert!(Block::call(wait).is_ok());
        let invalid_tag = ActionCall::new(action(BlockKind::Control, "Wait"), vec![])
            .unwrap()
            .with_tag("Time Unit", "Days");
        assert!(invalid_tag.is_err());
    }

    #[test]
    fn action_alternatives_validate_and_parse_by_argument_form() {
        let player_heal: Vec<_> = Action::variants(BlockKind::PlayerAction, "Heal").collect();
        assert_eq!(
            player_heal,
            vec![
                Action::PlayerActionHeal,
                Action::PlayerActionHealAlternative2
            ]
        );
        assert_eq!(player_heal[0].spec().alternative_index, 1);
        assert_eq!(player_heal[1].spec().alternative_index, 2);

        for (action, arguments) in [
            (Action::PlayerActionHeal, vec![Item::Number("2".into())]),
            (Action::PlayerActionHealAlternative2, vec![]),
            (Action::EntityActionHeal, vec![Item::Number("2".into())]),
            (Action::EntityActionHealAlternative2, vec![]),
        ] {
            let call = ActionCall::new(action, arguments).unwrap();
            let parsed = ActionCall::from_value(&call.to_value()).unwrap();
            assert_eq!(parsed.action(), action);
            assert_eq!(parsed.to_value(), call.to_value());
        }
    }

    #[test]
    fn structured_blocks_round_trip_and_count_physical_brackets() {
        let event = action(BlockKind::Event, "PlayerDmgPlayer");
        let mut script = Script::new(Header::event(event, vec![]).unwrap());
        let condition = action(BlockKind::IfVar, "=");
        let condition = ActionCall::new(
            condition,
            vec![Item::Number("1".into()), Item::Number("1".into())],
        )
        .unwrap();
        let message = action(BlockKind::PlayerAction, "SendMessage");
        let body = vec![Block::action(message, vec![Item::Component("hello".into())]).unwrap()];
        script.push(Block::conditional(condition, body.clone(), Some(body)).unwrap());
        assert_eq!(script.block_count(), 9);

        let decoded = Script::from_value(script.to_value()).unwrap();
        assert_eq!(decoded.to_value(), script.to_value());
        let compiled = script.compile().unwrap();
        assert_eq!(
            Script::decompile(&compiled).unwrap().to_value(),
            script.to_value()
        );
    }

    #[test]
    fn bundled_templates_parse_into_typed_scripts() {
        let mut failures = Vec::new();
        let mut round_trip_changes = Vec::new();
        for sample in 1..=27 {
            let path = format!(
                "{}/.agents/skills/diamondfire/assets/samples/sample{sample}.txt",
                env!("CARGO_MANIFEST_DIR")
            );
            let code = std::fs::read_to_string(path).unwrap();
            match Script::decompile(&code) {
                Err(error) => failures.push(format!("sample{sample}: {error}")),
                Ok(script) => {
                    use base64::Engine;
                    use std::io::Read;
                    let compact = code.trim();
                    let mut normalized = compact.trim_end_matches('=').to_owned();
                    match normalized.len() % 4 {
                        0 => {}
                        2 => normalized.push_str("=="),
                        3 => normalized.push('='),
                        _ => continue,
                    }
                    let compressed = base64::engine::general_purpose::STANDARD
                        .decode(normalized)
                        .unwrap();
                    let mut decoder = flate2::read::GzDecoder::new(compressed.as_slice());
                    let mut bytes = Vec::new();
                    decoder.read_to_end(&mut bytes).unwrap();
                    let original: Value = serde_json::from_slice(&bytes).unwrap();
                    if script.to_value() != original {
                        let before = original["blocks"].as_array().unwrap();
                        let after = script.to_value()["blocks"].as_array().unwrap().clone();
                        let differences = before
                            .iter()
                            .zip(&after)
                            .enumerate()
                            .filter_map(|(index, (left, right))| {
                                let left = left.as_object()?;
                                let right = right.as_object()?;
                                let keys: Vec<_> = left
                                    .keys()
                                    .chain(right.keys())
                                    .filter(|key| left.get(*key) != right.get(*key))
                                    .cloned()
                                    .collect();
                                (!keys.is_empty()).then(|| format!("{index}:{keys:?}"))
                            })
                            .collect::<Vec<_>>()
                            .join(" ");
                        round_trip_changes.push(format!(
                            "sample{sample}: {differences}; before={:?}; after={:?}",
                            before
                                .iter()
                                .flat_map(|block| block["args"]["items"]
                                    .as_array()
                                    .into_iter()
                                    .flatten())
                                .map(item_summary)
                                .collect::<Vec<_>>(),
                            after
                                .iter()
                                .flat_map(|block| block["args"]["items"]
                                    .as_array()
                                    .into_iter()
                                    .flatten())
                                .map(item_summary)
                                .collect::<Vec<_>>()
                        ));
                    }

                    fn item_summary(item: &Value) -> String {
                        let payload = &item["item"];
                        let data = &payload["data"];
                        format!(
                            "{}:{}:{}",
                            item["slot"],
                            payload["id"].as_str().unwrap_or("?"),
                            data.get("tag")
                                .and_then(Value::as_str)
                                .or_else(|| data.get("name").and_then(Value::as_str))
                                .unwrap_or("")
                        )
                    }
                }
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
        assert!(
            round_trip_changes.is_empty(),
            "round-trip changed samples: {}",
            round_trip_changes.join(", ")
        );
    }

    #[test]
    fn functions_and_dynamic_calls_use_typed_headers_and_blocks() {
        assert_eq!(Action::ALL.len(), 1323);
        let parameter = FunctionParameter::new("count", ParameterType::Number);
        let header =
            Header::function_with_parameters("count_players", true, vec![parameter]).unwrap();
        let mut script = Script::new(header);
        let call_action = action(BlockKind::CallFunc, "dynamic");
        let call =
            ActionCall::dynamic(call_action, "count_players", vec![Item::Number("1".into())])
                .unwrap();
        script.push(Block::call(call).unwrap());
        let output = script.compile().unwrap();
        assert_eq!(
            Script::decompile(&output).unwrap().to_value(),
            script.to_value()
        );
    }
}
