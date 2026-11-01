use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use serde_json::{Map, Value};

mod catalog;
mod typed;

pub use catalog::{Action, ActionSpec, ArgKind, BlockKind, TagSpec};
pub use typed::{
    ActionCall, Block, BlockLength as TypedBlockLength, ConditionParts, FunctionParameter, Header,
    Item, ParameterType, Script, ScriptError, Tag, Target, VariableScope,
};

/// A raw DiamondFire code line.
///
/// Compatibility wrapper for lossless JSON transport. Prefer [`Script`] for
/// new code: it stores blocks as typed Rust values and validates them against
/// the bundled action catalog.
#[derive(Clone, Debug, PartialEq)]
pub struct RawScript {
    metadata: Map<String, Value>,
    blocks: Vec<Value>,
}

/// A measured code-line length and the caller-provided maximum.
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

#[derive(Debug)]
pub enum RawError {
    InvalidTemplate(String),
    InvalidBlockIndex { index: usize, len: usize },
    Base64(base64::DecodeError),
    Json(serde_json::Error),
    Io(io::Error),
    Typed(ScriptError),
}

impl fmt::Display for RawError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTemplate(message) => write!(f, "invalid DiamondFire template: {message}"),
            Self::InvalidBlockIndex { index, len } => {
                write!(f, "block index {index} is out of bounds for {len} blocks")
            }
            Self::Base64(error) => write!(f, "invalid template-code Base64: {error}"),
            Self::Json(error) => write!(f, "invalid template JSON: {error}"),
            Self::Io(error) => write!(f, "I/O or gzip error: {error}"),
            Self::Typed(error) => write!(f, "{error}"),
        }
    }
}

impl Error for RawError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Base64(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::Typed(error) => Some(error),
            Self::InvalidTemplate(_) | Self::InvalidBlockIndex { .. } => None,
        }
    }
}

impl From<base64::DecodeError> for RawError {
    fn from(error: base64::DecodeError) -> Self {
        Self::Base64(error)
    }
}

impl From<serde_json::Error> for RawError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<io::Error> for RawError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<ScriptError> for RawError {
    fn from(error: ScriptError) -> Self {
        Self::Typed(error)
    }
}

impl RawScript {
    /// Create an empty, editable script. Add a header block before compiling it.
    pub fn new() -> Self {
        Self {
            metadata: Map::new(),
            blocks: Vec::new(),
        }
    }

    /// Parse the raw JSON form (`{"blocks":[...]}`).
    pub fn from_json(json: &str) -> Result<Self, RawError> {
        Self::from_value(serde_json::from_str(json)?)
    }

    /// Parse a template code (Base64-encoded gzip JSON).
    pub fn decompile(code: &str) -> Result<Self, RawError> {
        Self::from_value(Script::decompile(code)?.to_value())
    }

    /// Construct a script from its JSON value, retaining non-block root fields.
    pub fn from_value(value: Value) -> Result<Self, RawError> {
        let mut object = value
            .as_object()
            .cloned()
            .ok_or_else(|| RawError::InvalidTemplate("root must be a JSON object".to_owned()))?;
        let blocks = object
            .remove("blocks")
            .ok_or_else(|| RawError::InvalidTemplate("missing `blocks` array".to_owned()))?
            .as_array()
            .cloned()
            .ok_or_else(|| RawError::InvalidTemplate("`blocks` must be an array".to_owned()))?;

        Ok(Self {
            metadata: object,
            blocks,
        })
    }

    /// Return all blocks in their code-line order.
    pub fn blocks(&self) -> &[Value] {
        &self.blocks
    }

    /// Return mutable access to the raw ordered blocks.
    pub fn blocks_mut(&mut self) -> &mut Vec<Value> {
        &mut self.blocks
    }

    /// Append a raw block to the code line.
    pub fn push_block(&mut self, block: Value) {
        self.blocks.push(block);
    }

    /// Insert a block before `index`; `index == block_count()` appends.
    pub fn insert_block(&mut self, index: usize, block: Value) -> Result<(), RawError> {
        if index > self.blocks.len() {
            return Err(RawError::InvalidBlockIndex {
                index,
                len: self.blocks.len(),
            });
        }
        self.blocks.insert(index, block);
        Ok(())
    }

    /// Replace the block at `index`, returning the previous raw block.
    pub fn replace_block(&mut self, index: usize, block: Value) -> Result<Value, RawError> {
        let len = self.blocks.len();
        let slot = self
            .blocks
            .get_mut(index)
            .ok_or(RawError::InvalidBlockIndex { index, len })?;
        Ok(std::mem::replace(slot, block))
    }

    /// Remove and return the block at `index`.
    pub fn remove_block(&mut self, index: usize) -> Result<Value, RawError> {
        if index >= self.blocks.len() {
            return Err(RawError::InvalidBlockIndex {
                index,
                len: self.blocks.len(),
            });
        }
        Ok(self.blocks.remove(index))
    }

    /// Number of physical entries in the code line, including its header and brackets.
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Compare the code-line length with a caller-supplied DiamondFire limit.
    pub fn measure(&self, limit: usize) -> BlockLength {
        BlockLength {
            blocks: self.block_count(),
            limit,
        }
    }

    /// Serialize the script as compact raw JSON.
    pub fn to_json(&self) -> Result<String, RawError> {
        Ok(serde_json::to_string(&self.to_value())?)
    }

    /// Serialize the script as pretty-printed raw JSON.
    pub fn to_json_pretty(&self) -> Result<String, RawError> {
        Ok(serde_json::to_string_pretty(&self.to_value())?)
    }

    /// Rebuild the template JSON value, retaining non-block root fields.
    pub fn to_value(&self) -> Value {
        let mut object = self.metadata.clone();
        object.insert("blocks".to_owned(), Value::Array(self.blocks.clone()));
        Value::Object(object)
    }

    /// Convert this compatibility representation into the catalog-validated,
    /// structured script model.
    pub fn to_typed(&self) -> Result<Script, ScriptError> {
        Script::from_value(self.to_value())
    }

    /// Compile the raw JSON template to DiamondFire template code.
    pub fn compile(&self) -> Result<String, RawError> {
        Ok(self.to_typed()?.compile()?)
    }

    /// Load a raw JSON script from disk.
    pub fn load_json(path: impl AsRef<Path>) -> Result<Self, RawError> {
        Self::from_json(&fs::read_to_string(path)?)
    }

    /// Save this script as raw JSON.
    pub fn save_json(&self, path: impl AsRef<Path>) -> Result<(), RawError> {
        fs::write(path, self.to_json_pretty()?)?;
        Ok(())
    }

    /// Load compiled template code from disk.
    pub fn load_code(path: impl AsRef<Path>) -> Result<Self, RawError> {
        Self::decompile(&fs::read_to_string(path)?)
    }

    /// Save compiled template code to disk.
    pub fn save_code(&self, path: impl AsRef<Path>) -> Result<(), RawError> {
        fs::write(path, self.compile()?)?;
        Ok(())
    }
}

impl Default for RawScript {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn header() -> Value {
        json!({
            "id": "block",
            "block": "event",
            "args": { "items": [] },
            "action": "PlayerDmgPlayer"
        })
    }

    #[test]
    fn template_code_round_trip_preserves_blocks_and_metadata() {
        let script = RawScript::from_value(json!({
            "blocks": [header()],
            "custom": "kept"
        }))
        .unwrap();

        let compiled = script.compile().unwrap();
        assert!(compiled.starts_with("H4sI"));
        let decoded = RawScript::decompile(&compiled).unwrap();
        assert_eq!(decoded, script);
    }

    #[test]
    fn block_edits_and_measurement_track_code_line_length() {
        let mut script = RawScript::new();
        script.push_block(header());
        script.push_block(json!({"id": "bracket", "direct": "open", "type": "norm"}));
        assert!(script.measure(2).within_limit());
        assert_eq!(script.measure(1).excess(), 1);

        script
            .insert_block(1, json!({"id": "block", "block": "player_action"}))
            .unwrap();
        assert_eq!(script.block_count(), 3);
        script.remove_block(1).unwrap();
        assert_eq!(script.block_count(), 2);
    }

    #[test]
    fn compiling_requires_a_header() {
        assert!(matches!(
            RawScript::new().compile(),
            Err(RawError::Typed(_))
        ));
    }
}
