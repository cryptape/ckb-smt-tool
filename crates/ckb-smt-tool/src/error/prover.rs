//! Errors for proof generation.

use alloc::string::{String, ToString};

use thiserror::Error;

use sparse_merkle_tree::error::Error as SmtLibError;

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("smt lib error: {0}")]
    SmtLib(#[from] SmtLibError),

    #[error("{0}")]
    Other(String),
}

impl GeneratorError {
    pub fn other<S: ToString>(arg: S) -> Self {
        Self::Other(arg.to_string())
    }
}
