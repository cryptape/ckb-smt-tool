//! Types for verification.

use alloc::{borrow::ToOwned as _, vec::Vec};
use core::result::Result;

use ckb_hash::blake2b_256;
use molecule::prelude::*;
use sparse_merkle_tree::{blake2b::Blake2bHasher, traits::Value, CompiledMerkleProof, H256};

use crate::{
    error::{UpdateError, VerifyError},
    types::generated::{
        BytesOptReader, DataWithProofReader, KeyValuesReader, SmtChangeReader, SmtChangesReader,
        SmtUpdateReader,
    },
};

struct LeafChange {
    key: H256,
    old_value: H256,
    new_value: H256,
}

impl SmtUpdateReader<'_> {
    /// Verifies self with the old SMT root.
    pub fn verify_smt(&self, old_root: &H256) -> Result<(), UpdateError> {
        let proof_data = self.proof().raw_data().to_vec();
        let proof = CompiledMerkleProof(proof_data);
        let leaves_changes = self.changes().leaves_changes();
        let old_leaves = leaves_changes
            .iter()
            .map(|lc| (lc.key.to_owned(), lc.old_value.to_owned()))
            .collect();
        let expected_old_root = if let Ok(root) = proof.compute_root::<Blake2bHasher>(old_leaves) {
            root
        } else {
            return Err(UpdateError::ComputeOldRoot);
        };
        if expected_old_root != *old_root {
            return Err(UpdateError::MismatchedOldRoot);
        }
        let new_leaves = leaves_changes
            .into_iter()
            .map(|lc| (lc.key, lc.new_value))
            .collect();
        let expected_new_root = if let Ok(root) = proof.compute_root::<Blake2bHasher>(new_leaves) {
            root
        } else {
            return Err(UpdateError::ComputeNewRoot);
        };
        if expected_new_root.as_slice() != self.new_root().raw_data() {
            return Err(UpdateError::MismatchedNewRoot);
        }
        Ok(())
    }
}

impl SmtChangeReader<'_> {
    fn key_to_h256(&self) -> H256 {
        blake2b_256(self.key().raw_data()).into()
    }
}

impl SmtChangesReader<'_> {
    fn leaves_changes(&self) -> Vec<LeafChange> {
        self.iter()
            .map(|change| {
                let key = change.key_to_h256();
                let old_value = change.old_value().to_h256();
                let new_value = change.new_value().to_h256();
                LeafChange {
                    key,
                    old_value,
                    new_value,
                }
            })
            .collect()
    }
}

impl Value for BytesOptReader<'_> {
    fn to_h256(&self) -> H256 {
        if let Some(bytes) = self.to_opt() {
            blake2b_256(bytes.raw_data()).into()
        } else {
            H256::zero()
        }
    }
    fn zero() -> Self {
        // `BytesOpt::DEFAULT_VALUE`
        Self::new_unchecked(&[])
    }
}

impl DataWithProofReader<'_> {
    /// Verifies self with the SMT root.
    pub fn verify_smt(&self, root: &H256) -> Result<(), VerifyError> {
        let proof_data = self.proof().raw_data().to_vec();
        let proof = CompiledMerkleProof(proof_data);
        let leaves = self.data().as_leaves();
        let expected_root = if let Ok(root) = proof.compute_root::<Blake2bHasher>(leaves) {
            root
        } else {
            return Err(VerifyError::ComputeRoot);
        };
        if expected_root != *root {
            return Err(VerifyError::MismatchedRoot);
        }
        Ok(())
    }
}

impl KeyValuesReader<'_> {
    fn as_leaves(&self) -> Vec<(H256, H256)> {
        self.iter()
            .map(|kv| {
                let key = blake2b_256(kv.key().raw_data()).into();
                let value = kv.value().to_h256();
                (key, value)
            })
            .collect()
    }
}
