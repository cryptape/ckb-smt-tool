//! Types for proof generation.

use alloc::vec::Vec;
use std::collections::HashMap;

use ckb_hash::blake2b_256;
use molecule::{bytes::Bytes, prelude::*, Number, NUMBER_SIZE};
use sparse_merkle_tree::{
    blake2b::Blake2bHasher, default_store::DefaultStore, traits::Value, MerkleProof,
    SparseMerkleTree, H256,
};

use crate::{
    error::GeneratorError,
    types::generated::{self as packed, DataWithProof, SmtUpdate, SmtUpdateReader},
};

pub type Smt = SparseMerkleTree<Blake2bHasher, BytesOpt, DefaultStore<BytesOpt>>;

/// A generator to create proofs.
#[derive(Default)]
pub struct ProofGenerator {
    smt: Smt,
    changes: HashMap<Bytes, Option<Bytes>>,
}

/// Wrap `Option<Bytes>` to implement the trait `Value`.
#[derive(Default, Clone)]
pub struct BytesOpt(Option<Bytes>);

impl Value for BytesOpt {
    fn to_h256(&self) -> H256 {
        if let Some(ref bytes) = self.0 {
            blake2b_256(bytes).into()
        } else {
            H256::zero()
        }
    }
    fn zero() -> Self {
        Self(None)
    }
}

impl BytesOpt {
    fn to_packed(&self) -> packed::BytesOpt {
        if let Some(ref value) = self.0 {
            let packed_value = slice_to_packed_bytes(value);
            packed::BytesOpt::new_builder()
                .set(Some(packed_value))
                .build()
        } else {
            packed::BytesOpt::default()
        }
    }
}

impl ProofGenerator {
    /// Creates a new instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Gets value of a leaf return zero value if leaf not exists.
    pub fn get(&self, key: &[u8]) -> Result<Option<Bytes>, GeneratorError> {
        let key_h256 = blake2b_256(key).into();
        let value = self.smt.get(&key_h256)?.0;
        Ok(value)
    }

    /// Update a leaf, return new merkle root set to zero value to delete a key.
    pub fn update(
        &mut self,
        key: &[u8],
        value_opt: Option<Bytes>,
    ) -> Result<&H256, GeneratorError> {
        let key_h256 = blake2b_256(key).into();
        let value = BytesOpt(value_opt);
        self.smt.update(key_h256, value).map_err(Into::into)
    }

    /// Returns current merkle root.
    pub fn root(&self) -> &H256 {
        self.smt.root()
    }

    /// Generates merkle proof.
    pub fn merkle_proof(&self, keys: Vec<Bytes>) -> Result<MerkleProof, GeneratorError> {
        let keys_h256: Vec<_> = keys
            .into_iter()
            .map(|key| blake2b_256(&key).into())
            .collect();
        let proof = self.smt.merkle_proof(keys_h256.clone())?;
        Ok(proof)
    }

    /// Returns the values and their proof.
    pub fn data_with_proof(&self, keys: Vec<Bytes>) -> Result<DataWithProof, GeneratorError> {
        let mut keys_h256 = Vec::new();
        let mut kvs_builder = packed::KeyValues::new_builder();
        for k in keys {
            let key_h256: H256 = blake2b_256(&k).into();
            let key = slice_to_packed_bytes(&k);
            let value = self.smt.get(&key_h256)?.to_packed();
            let kv = packed::KeyValue::new_builder()
                .key(key)
                .value(value)
                .build();
            kvs_builder = kvs_builder.push(kv);
            keys_h256.push(key_h256);
        }
        let data = kvs_builder.build();
        let proof = {
            let proof = self
                .smt
                .merkle_proof(keys_h256.clone())?
                .compile(keys_h256)?;
            slice_to_packed_bytes(&proof.0)
        };
        let data_with_proof = packed::DataWithProof::new_builder()
            .data(data)
            .proof(proof)
            .build();
        Ok(data_with_proof)
    }

    /// Appends a change but not commit; returns the previous pending change of
    /// the same key if there is any.
    pub fn append_change(&mut self, key: Bytes, new_value: Option<Bytes>) -> Option<Option<Bytes>> {
        self.changes.insert(key, new_value)
    }

    /// Commits all pending changes, and returns an update which contains the proof of changes.
    pub fn commit_changes(&mut self) -> Result<SmtUpdate, GeneratorError> {
        let mut keys_h256 = Vec::new();
        let mut smt_changes_builder = packed::SmtChanges::new_builder();
        for (key, value_opt) in self.changes.drain() {
            let key_h256 = blake2b_256(&key).into();
            let new_value = BytesOpt(value_opt);
            let old_value = self.smt.get(&key_h256)?;

            let packed_key = slice_to_packed_bytes(&key);
            let packed_old_value = old_value.to_packed();
            let packed_new_value = new_value.to_packed();
            let smt_change = packed::SmtChange::new_builder()
                .key(packed_key)
                .old_value(packed_old_value)
                .new_value(packed_new_value)
                .build();
            smt_changes_builder = smt_changes_builder.push(smt_change);

            self.smt.update(key_h256, new_value)?;
            keys_h256.push(key_h256);
        }
        let new_root = {
            let root = self.smt.root();
            let bytes = Bytes::copy_from_slice(root.as_slice());
            packed::Hash::new_unchecked(bytes)
        };
        let changes = smt_changes_builder.build();
        let proof = {
            let proof = self
                .smt
                .merkle_proof(keys_h256.clone())?
                .compile(keys_h256)?;
            slice_to_packed_bytes(&proof.0)
        };
        let update = SmtUpdate::new_builder()
            .new_root(new_root)
            .changes(changes)
            .proof(proof)
            .build();
        Ok(update)
    }

    /// Applies an update.
    pub fn apply_update(&mut self, smt_update: SmtUpdateReader<'_>) -> Result<(), GeneratorError> {
        for data in smt_update.changes().iter() {
            let key = data.key();
            let value_opt = data
                .new_value()
                .to_opt()
                .as_ref()
                .map(|bytes| Bytes::copy_from_slice(bytes.raw_data()));
            self.update(key.raw_data(), value_opt)?;
        }
        Ok(())
    }
}

fn slice_to_packed_bytes(slice: &[u8]) -> packed::Bytes {
    let len = slice.len();
    let mut vec: Vec<u8> = Vec::with_capacity(NUMBER_SIZE + len);
    vec.extend_from_slice(&(len as Number).to_le_bytes()[..]);
    vec.extend_from_slice(slice);
    packed::Bytes::new_unchecked(Bytes::from(vec))
}
