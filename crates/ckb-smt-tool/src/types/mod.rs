//! Provides the essential types.

#![allow(missing_docs)]

#[allow(warnings)]
#[allow(clippy::all)]
pub(crate) mod generated;

#[cfg(feature = "with-prover")]
pub(crate) mod prover;
pub(crate) mod verifier;

#[cfg(feature = "with-prover")]
pub use molecule::bytes::Bytes;
#[cfg(feature = "with-prover")]
pub use prover::{ProofGenerator, Smt};
pub use sparse_merkle_tree::H256;

pub use generated::{DataWithProof, DataWithProofReader, SmtUpdate, SmtUpdateReader};
