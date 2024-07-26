//! Errors.

#[cfg(feature = "with-prover")]
pub(crate) mod prover;
pub(crate) mod verifier;

#[cfg(feature = "with-prover")]
pub use prover::GeneratorError;
pub use verifier::{UpdateError, VerifyError};
