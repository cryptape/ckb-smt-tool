//! Utilities for tests only.

use ckb_testtool::{
    ckb_hash::{new_blake2b, BLAKE2B_LEN},
    ckb_types::{packed, prelude::*},
};
use env_logger::{Builder, Target};
use log::LevelFilter;

pub(crate) fn setup() {
    let _ = Builder::new()
        .filter_module("tests", LevelFilter::Trace)
        .filter_module("ckb_smt_tool", LevelFilter::Trace)
        .target(Target::Stdout)
        .is_test(true)
        .try_init();
    println!();
}

pub(crate) fn calculate_unique_id(
    input: packed::CellInput,
    output_index: usize,
) -> [u8; BLAKE2B_LEN] {
    let mut blake2b = new_blake2b();
    blake2b.update(input.as_slice());
    blake2b.update(&(output_index as u64).to_le_bytes());
    let mut ret = [0; BLAKE2B_LEN];
    blake2b.finalize(&mut ret);
    ret
}
