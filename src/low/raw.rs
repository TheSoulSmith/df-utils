use std::error::Error;
use std::fmt;
use std::io;

mod catalog;
mod typed;

pub use catalog::actions::*;
pub use catalog::{
    BLOCKS, BlockKind, BlockSpec, GAME_VALUES, GameValueSpec, PARTICLES, POTIONS, ParticleCluster,
    ParticleKindSpec, ParticleMotion, ParticleNumber, ParticleSpec, PotionSpec, SOUNDS, SoundSpec,
};
pub use typed::{
    ActionCall, Block, BlockLength as TypedBlockLength, Chunk, ConditionParts, FunctionParameter,
    Header, Item, ItemKind, ParameterType, Script, ScriptError, Target, VariableScope, WireAction,
};

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
