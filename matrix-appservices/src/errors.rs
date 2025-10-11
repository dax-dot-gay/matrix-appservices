use std::sync::Arc;

/// Library-specific error type
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    /// Wrapper for unhandled errors
    #[error("An unknown error occurred: {0:?}")]
    Unknown(Arc<anyhow::Error>),

    /// A namespace kind could not be converted
    #[error("Unknown namespace kind: {0}")]
    UnknownNamespaceKind(String),

    /// YAML error
    #[error("Error in YAML encoding/decoding: {0:?}")]
    YamlError(Arc<serde_norway::Error>),
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Unknown(Arc::new(value))
    }
}

impl From<serde_norway::Error> for Error {
    fn from(value: serde_norway::Error) -> Self {
        Self::YamlError(Arc::new(value))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
