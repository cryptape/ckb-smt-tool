use ckb_hash::{new_blake2b, BLAKE2B_LEN};
use ckb_std::{ckb_constants::Source, ckb_types::prelude::*, high_level as hl};

use crate::error::{InternalError, Result};

pub(crate) fn create(index: usize) -> Result<()> {
    debug!("create kvstore at outputs[{index}]");

    // Load script args.
    let script = hl::load_script()?;
    let script_args = script.args();
    let script_args_slice = script_args.as_reader().raw_data();

    // Check the script args: args length.
    if script_args_slice.len() != 32 {
        return Err(InternalError::CreateInvalidArgsLength.into());
    }

    // Check the script args: 32 bytes, the unique ID.
    let unique_id = load_then_calculate_unique_id(index)?;
    if unique_id != &script_args_slice[..32] {
        return Err(InternalError::CreateIncorrectUniqueId.into());
    }

    debug!("load the data from outputs[{index}]");
    let output_data = hl::load_cell_data(index, Source::Output)?;

    if output_data.len() != 32 {
        return Err(InternalError::CreateInitializedDataInvalidLength.into());
    }

    if output_data != &[0; 32] {
        return Err(InternalError::CreateInitializedDataNotEmpty.into());
    }

    Ok(())
}

// Load the first input and the index of the first output which uses current
// script, then calculate an unique ID with them.
pub(crate) fn load_then_calculate_unique_id(output_index: usize) -> Result<[u8; BLAKE2B_LEN]> {
    let input = hl::load_input(0, Source::Input)?;
    let mut blake2b = new_blake2b();
    blake2b.update(input.as_slice());
    blake2b.update(&(output_index as u64).to_le_bytes());
    let mut ret = [0; BLAKE2B_LEN];
    blake2b.finalize(&mut ret);
    Ok(ret)
}
