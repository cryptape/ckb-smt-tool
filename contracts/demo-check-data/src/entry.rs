use alloc::vec::Vec;

use ckb_smt_tool::types::{DataWithProofReader, H256};
use ckb_std::{
    ckb_constants::Source, ckb_types::prelude::*, debug, high_level as hl, syscalls::SysError,
};

use crate::error::{InternalError, Result};

pub fn main() -> Result<()> {
    debug!("{} Starting ...", module_path!());

    let script_hash = hl::load_script_hash()?;
    debug!("script hash = {:#x}", script_hash.pack());

    let script = hl::load_script()?;
    let script_args = script.args();
    let script_args_slice = script_args.as_reader().raw_data();

    // Check the script args: args length.
    if script_args_slice.len() != 32 {
        return Err(InternalError::InvalidArgsLength.into());
    }
    let cell_dep_type_script_hash = &script_args_slice[..32];

    debug!("find then load the SMT root from the cell dep");
    // First 32 bytes is the type script hash for the KV-store cell.
    let root = find_then_load_smt_root_from_cell_dep(cell_dep_type_script_hash)?;

    debug!("the SMT root is [{root:?}]");
    check_witnesses(&script_hash, root)?;

    debug!("{} DONE.", module_path!());

    Ok(())
}

fn find_then_load_smt_root_from_cell_dep(script_hash: &[u8]) -> Result<H256> {
    let mut indexes = Vec::new();
    for (index, type_hash_opt) in
        hl::QueryIter::new(hl::load_cell_type_hash, Source::CellDep).enumerate()
    {
        if let Some(type_hash) = type_hash_opt {
            if type_hash == script_hash {
                if indexes.is_empty() {
                    indexes.push(index);
                } else {
                    return Err(InternalError::CellDepMoreThanOne.into());
                }
            }
        }
    }
    if indexes.is_empty() {
        return Err(InternalError::CellDepNotFound.into());
    }

    let cell_dep_index = indexes[0];
    debug!("the SMT root is in the cell-deps[{cell_dep_index}]");

    let cell_dep_data = hl::load_cell_data(cell_dep_index, Source::CellDep)?;

    if cell_dep_data.len() != 32 {
        return Err(InternalError::CellDepInvalidCellData.into());
    }

    Ok(checked_slice_to_h256(&cell_dep_data))
}

fn check_witnesses(script_hash: &[u8], root: H256) -> Result<()> {
    for (index, lock_hash) in hl::QueryIter::new(hl::load_cell_lock_hash, Source::Input).enumerate()
    {
        debug!("{index}-th lock hash of inputs: {:#x}", lock_hash.pack());
        if lock_hash == script_hash {
            debug!("found cell: inputs[{index}]");
            let witness_args = hl::load_witness_args(index, Source::Input)?;
            debug!("check the witness args for index [{index}]");
            if let Some(args) = witness_args.lock().to_opt() {
                let data_with_proof_slice = &args.raw_data();
                let data_with_proof = DataWithProofReader::from_slice(data_with_proof_slice)
                    .map_err(|_| SysError::Encoding)?;
                data_with_proof.verify_smt(&root)?;
            } else {
                return Err(InternalError::WitnessIsNotExisted.into());
            }
        }
    }
    Ok(())
}

fn checked_slice_to_h256(slice: &[u8]) -> H256 {
    let mut v = [0u8; 32];
    v.copy_from_slice(slice);
    v.into()
}
