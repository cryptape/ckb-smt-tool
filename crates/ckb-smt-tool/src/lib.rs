//! A tool to integrate SMT into CKB contracts easier.

#![no_std]

extern crate alloc;

#[cfg(feature = "with-prover")]
extern crate std;

pub mod error;
pub mod types;
