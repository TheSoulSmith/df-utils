//! Strongly typed DiamondFire lines backed by the generated action catalog.
//! Wire JSON is used only at import/export boundaries; script bodies are Rust
//! enums and nested control-flow nodes. Each generated action enum owns its
//! parameters, tag options, and wire serialization.

use std::error::Error;
use std::fmt;

use serde_json::{Map, Value, json};

use super::{
    super::text_codes::StrTextCode,
    catalog::{self, BlockKind, ParticleCluster, ParticleSpec},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind {
    AnyType,
    Block,
    BlockTag,
    Byte,
    Component,
    Dialog,
    Dict,
    EntityType,
    Item,
    List,
    Location,
    Number,
    Particle,
    Potion,
    Projectile,
    Sound,
    SpawnEgg,
    Text,
    Variable,
    Vector,
    Vehicle,
}

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

pub trait WireAction {
    fn to_wire_value(&self) -> Value;
    fn sub_action_wire_name(&self) -> String;
}

impl WireAction for ActionCall {
    fn to_wire_value(&self) -> Value {
        self.to_value()
    }

    fn sub_action_wire_name(&self) -> String {
        self.0
            .get("subAction")
            .or_else(|| self.0.get("action"))
            .or_else(|| self.0.get("data"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned()
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
        particle: ParticleSpec,
        cluster: ParticleCluster,
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
    pub fn particle(particle: ParticleSpec) -> Self {
        Self::Particle {
            particle,
            cluster: ParticleCluster::default(),
            mapping_version: 0,
        }
    }

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

    pub(crate) fn matches(&self, expected: ItemKind) -> bool {
        use ItemKind::*;
        if expected == Variable {
            return matches!(self, Self::Variable { .. });
        }
        if matches!(self, Self::Variable { .. }) {
            return true;
        }
        if let Self::Sound { id, .. } = self
            && !catalog::SOUNDS
                .iter()
                .any(|sound| sound.id == catalog_item_id(id))
        {
            return false;
        }
        if let Self::Potion { id, .. } = self
            && !catalog::POTIONS
                .iter()
                .any(|potion| potion.id == catalog_item_id(id))
        {
            return false;
        }
        if let Self::GameValue { name, .. } = self {
            return catalog::find_game_value(name)
                .is_some_and(|value| expected == AnyType || value.return_type == expected);
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

    pub(crate) fn to_value(&self) -> Value {
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
                particle,
                cluster,
                mapping_version,
            } => json!({
                "mappingVersion": mapping_version,
                "particle": format!("minecraft:{}", particle.id()),
                "cluster": {
                    "amount": cluster.amount,
                    "horizontal": cluster.horizontal.to_value(),
                    "vertical": cluster.vertical.to_value()
                },
                "data": particle.to_data()
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

/// A wire block stored in a script; prefer generated action enums for creation.
#[derive(Clone, Debug, PartialEq)]
pub struct ActionCall(Value);

impl ActionCall {
    pub fn new(value: Value) -> Result<Self, ScriptError> {
        Self::from_value(&value)
    }

    pub fn from_action(value: impl WireAction) -> Result<Self, ScriptError> {
        Self::new(value.to_wire_value())
    }

    pub fn with_target(mut self, target: Target) -> Result<Self, ScriptError> {
        let block = self.block_kind();
        if !matches!(
            block,
            BlockKind::PlayerAction
                | BlockKind::EntityAction
                | BlockKind::IfPlayer
                | BlockKind::IfEntity
        ) {
            return Err(ScriptError::Invalid(format!(
                "{} does not accept a target",
                block.as_wire()
            )));
        }
        if matches!(block, BlockKind::PlayerAction | BlockKind::IfPlayer)
            && matches!(
                target,
                Target::Projectile | Target::AllEntities | Target::AllMobs | Target::LastEntity
            )
        {
            return Err(ScriptError::Invalid(
                "target is not supported by this player block".into(),
            ));
        }
        self.0["target"] = json!(target.as_wire());
        Ok(self)
    }

    pub fn negated(mut self) -> Result<Self, ScriptError> {
        if !matches!(
            self.block_kind(),
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
        self.0["attribute"] = json!("NOT");
        Ok(self)
    }

    fn block_kind(&self) -> BlockKind {
        BlockKind::from_wire(self.0["block"].as_str().expect("validated block"))
            .expect("validated block")
    }

    fn to_value(&self) -> Value {
        self.0.clone()
    }

    fn from_value(value: &Value) -> Result<Self, ScriptError> {
        let block = value
            .get("block")
            .and_then(Value::as_str)
            .ok_or_else(|| ScriptError::Invalid("block is missing its block type".into()))?;
        if BlockKind::from_wire(block).is_none() {
            return Err(ScriptError::Invalid(format!(
                "unknown block type {block:?}"
            )));
        }
        if value.get("id").and_then(Value::as_str) != Some("block") {
            return Err(ScriptError::Invalid("expected a block value".into()));
        }
        Ok(Self(value.clone()))
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

    fn accepts(self, item: &Item) -> bool {
        if self == Self::Dialog {
            return false;
        }
        item.matches(match self {
            Self::Text => ItemKind::Text,
            Self::Component => ItemKind::Component,
            Self::Number => ItemKind::Number,
            Self::Location => ItemKind::Location,
            Self::Vector => ItemKind::Vector,
            Self::Sound => ItemKind::Sound,
            Self::Particle => ItemKind::Particle,
            Self::Potion => ItemKind::Potion,
            Self::Item => ItemKind::Item,
            Self::Any => ItemKind::AnyType,
            Self::Variable => ItemKind::Variable,
            Self::List => ItemKind::List,
            Self::Dictionary => ItemKind::Dict,
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
    pub fn event(action: impl WireAction) -> Result<Self, ScriptError> {
        let action = ActionCall::from_action(action)?;
        if !matches!(
            action.block_kind(),
            BlockKind::Event | BlockKind::EntityEvent | BlockKind::GameEvent
        ) {
            return Err(ScriptError::Invalid(
                "event header requires a player, entity, or game event action".into(),
            ));
        }
        Ok(Self(HeaderKind::Event(action)))
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

/// A group of instructions that can be measured independently before being
/// inlined into a code line.
#[derive(Clone, Debug)]
pub struct Chunk {
    instructions: Vec<Block>,
    io: Vec<(StrTextCode, VariableScope, bool)>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            io: Vec::new(),
        }
    }

    pub fn instructions(&self) -> &[Block] {
        &self.instructions
    }

    pub fn push(&mut self, instruction: Block) {
        self.instructions.push(instruction);
    }

    pub fn block_count(&self) -> usize {
        count_blocks(&self.instructions)
    }

    pub fn measure(&self, limit: usize) -> BlockLength {
        BlockLength {
            blocks: self.block_count(),
            limit,
        }
    }
}

impl Block {
    pub fn action(action: impl WireAction) -> Result<Self, ScriptError> {
        let action = ActionCall::from_action(action)?;
        if is_structural(action.block_kind()) {
            return Err(ScriptError::Invalid(
                "headers, conditions, and repeats must use their structured constructors".into(),
            ));
        }
        Ok(Self(BlockKindNode::Action(action)))
    }

    pub fn call(call: ActionCall) -> Result<Self, ScriptError> {
        if is_structural(call.block_kind()) {
            return Err(ScriptError::Invalid(
                "headers, conditions, and repeats must use their structured constructors".into(),
            ));
        }
        Ok(Self(BlockKindNode::Action(call)))
    }

    pub fn conditional(
        condition: impl WireAction,
        then_body: Vec<Block>,
        else_body: Option<Vec<Block>>,
    ) -> Result<Self, ScriptError> {
        let condition = ActionCall::from_action(condition)?;
        if !is_condition(condition.block_kind()) {
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

    pub fn repeat(repeat: impl WireAction, body: Vec<Block>) -> Result<Self, ScriptError> {
        let repeat = ActionCall::from_action(repeat)?;
        if repeat.block_kind() != BlockKind::Repeat {
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
#[derive(Clone, Debug)]
pub struct Script {
    header: Header,
    chunks: Vec<Chunk>,
    metadata: Map<String, Value>,
}

impl Script {
    pub fn new(header: Header) -> Self {
        Self {
            header,
            chunks: Vec::new(),
            metadata: Map::new(),
        }
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn push_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn set_header(&mut self, header: Header) {
        self.header = header;
    }

    pub fn push(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn insert(&mut self, index: usize, chunk: Chunk) -> Result<(), ScriptError> {
        if self.chunks.len() < index {
            Err(ScriptError::Invalid(format!(
                "index {} exceeds length {}",
                index,
                self.chunks.len()
            )))
        } else {
            self.chunks.insert(index, chunk);
            Ok(())
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<Chunk, ScriptError> {
        if index >= self.chunks.len() {
            Err(ScriptError::Invalid(format!(
                "index {} exceeds maximum {}",
                index,
                self.chunks.len() - 1
            )))
        } else {
            Ok(self.chunks.remove(index))
        }
    }

    pub fn replace(&mut self, index: usize, chunk: Chunk) -> Result<Chunk, ScriptError> {
        if index >= self.chunks.len() {
            Err(ScriptError::Invalid(format!(
                "index {} exceeds maximum {}",
                index,
                self.chunks.len() - 1
            )))
        } else {
            Ok(std::mem::replace(&mut self.chunks[index], chunk))
        }
    }

    pub fn block_count(&self) -> usize {
        1 + self.chunks.iter().map(Chunk::block_count).sum::<usize>()
    }

    pub fn measure(&self, limit: usize) -> BlockLength {
        BlockLength {
            blocks: self.block_count(),
            limit,
        }
    }

    pub fn to_value(&self) -> Value {
        let mut blocks = vec![self.header.to_value()];
        for chunk in &self.chunks {
            encode_blocks(&chunk.instructions, &mut blocks);
        }
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

    pub fn compile(&self) -> Result<String, ScriptError> {
        for chunk in &self.chunks {
            self.validate_compilable(&chunk.instructions)?;
        }
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
                ActionCall::from_value(&call.to_value())?;
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

fn catalog_item_id(id: &str) -> &str {
    id.strip_prefix("minecraft:").unwrap_or(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_action_values_serialize_named_parameters_and_enum_tags() {
        use super::catalog::actions::{PlayerAction, TagOptionsCenteredRegular};

        let action = PlayerAction::SendMessage {
            params: (Some(vec![Item::Component("hello".into())]),),
            tags: (Some(TagOptionsCenteredRegular::Centered), None, None),
        };
        let wire = action.to_wire();

        assert_eq!(wire["block"], "player_action");
        assert_eq!(wire["action"], "SendMessage");
        assert_eq!(wire["subAction"], "SendMessage");
        let items = wire["args"]["items"].as_array().unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0]["slot"], 0);
        assert_eq!(items[0]["item"]["id"], "comp");
        assert_eq!(items[1]["slot"], 26);
        assert_eq!(items[1]["item"]["data"]["tag"], "Alignment Mode");
        assert_eq!(items[1]["item"]["data"]["option"], "Centered");
    }

    #[test]
    fn generated_hotbar_action_preserves_empty_slot_positions() {
        use super::catalog::actions::PlayerAction;

        let action = PlayerAction::SetHotbar {
            params: (
                Some(Item::MinecraftItem("minecraft:stone".into())),
                None,
                Some(Item::MinecraftItem("minecraft:dirt".into())),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            tags: (),
        };
        let wire = action.to_wire();
        let items = wire["args"]["items"].as_array().unwrap();

        assert_eq!(wire["subAction"], "SetHotbar");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0]["slot"], 0);
        assert_eq!(items[1]["slot"], 2);
    }

    #[test]
    fn generated_action_values_serialize_condition_sub_actions() {
        use super::catalog::actions::{IfVar, SelectObj, SelectObjPlayerscondSubAction};

        let condition = IfVar::Empty {
            params: (Item::Number("1".into()), Item::Number("2".into())),
            tags: (),
        };
        let action = SelectObj::PlayersCond {
            params: (),
            tags: (),
            sub_action: SelectObjPlayerscondSubAction::IfVar(condition),
        };
        let wire = action.to_wire();

        assert_eq!(wire["block"], "select_obj");
        assert_eq!(wire["action"], "PlayersCond");
        assert_eq!(wire["subAction"], "<=");
        assert_eq!(wire["args"]["items"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn generated_dynamic_action_values_encode_names_and_arguments() {
        use super::catalog::actions::{
            CallFunc, StartProcess, TagOptionsForEachInSelectionWithCurrentSelection,
        };

        let action = CallFunc::Dynamic {
            name: "worker.run".into(),
            params: vec![Item::Number("7".into())],
            tags: (),
        };
        let wire = action.to_wire();

        assert_eq!(wire["block"], "call_func");
        assert_eq!(wire["data"], "worker.run");
        assert_eq!(wire["args"]["items"][0]["slot"], 0);
        assert_eq!(wire["args"]["items"][0]["item"]["id"], "num");

        let process = StartProcess::Dynamic {
            name: "worker.loop".into(),
            params: Vec::new(),
            tags: (
                Some(TagOptionsForEachInSelectionWithCurrentSelection::WithNoTargets),
                None,
            ),
        };
        let wire = process.to_wire();
        assert_eq!(wire["block"], "start_process");
        assert_eq!(wire["data"], "worker.loop");
        assert_eq!(wire["args"]["items"][0]["slot"], 26);
        assert_eq!(
            wire["args"]["items"][0]["item"]["data"]["option"],
            "With no targets"
        );
    }

    #[test]
    fn game_values_match_only_their_catalogued_return_type() {
        let current_health = Item::GameValue {
            name: "Current Health".into(),
            target: "Default".into(),
        };
        assert!(current_health.matches(ItemKind::Number));
        assert!(!current_health.matches(ItemKind::Text));

        let hotbar_items = Item::GameValue {
            name: "Hotbar Items".into(),
            target: "Default".into(),
        };
        assert!(hotbar_items.matches(ItemKind::List));
        assert!(!hotbar_items.matches(ItemKind::Item));

        let unknown_value = Item::GameValue {
            name: "Not a game value".into(),
            target: "Default".into(),
        };
        assert!(!unknown_value.matches(ItemKind::AnyType));

        assert!(
            Item::Sound {
                id: "minecraft:block.chain.hit".into(),
                pitch: 1.0,
                volume: 1.0,
                mapping_version: 1,
            }
            .matches(ItemKind::Sound)
        );
        assert!(
            !Item::Sound {
                id: "minecraft:not_a_sound".into(),
                pitch: 1.0,
                volume: 1.0,
                mapping_version: 1,
            }
            .matches(ItemKind::Sound)
        );
        assert!(
            !Item::Sound {
                id: "minecraft:not_a_sound".into(),
                pitch: 1.0,
                volume: 1.0,
                mapping_version: 1,
            }
            .matches(ItemKind::AnyType)
        );
    }

    #[test]
    fn generated_catalogs_cover_the_dumped_domains() {
        assert_eq!(catalog::BLOCKS.len(), 19);
        assert_eq!(catalog::GAME_VALUES.len(), 143);
        assert_eq!(catalog::SOUNDS.len(), 1962);
        assert_eq!(catalog::PARTICLES.len(), 123);
        assert!(catalog::PARTICLES.iter().any(|particle| {
            particle.id == "geyser_base"
                && particle.fields
                    == [
                        "Motion",
                        "Motion Variation",
                        "Water Blocks",
                        "Burst Impulse",
                    ]
        }));
        assert_eq!(catalog::POTIONS.len(), 40);
    }
}
