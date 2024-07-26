use ckb_smt_tool::types::{SmtUpdateReader, H256};
use ckb_std::{ckb_constants::Source, ckb_types::prelude::*, error::SysError, high_level as hl};

use crate::error::{InternalError, Result};

pub(crate) fn update(input_index: usize, output_index: usize) -> Result<()> {
    debug!("update kvstore from input[{input_index}] to outputs[{output_index}]");

    debug!("load the data from inputs[{input_index}]");
    let input_data = hl::load_cell_data(input_index, Source::Input)?;
    if input_data.len() != 32 {
        return Err(InternalError::UpdateInputDataInvalidLength.into());
    }

    debug!("load the data from outputs[{output_index}]");
    let output_data = hl::load_cell_data(output_index, Source::Output)?;
    if output_data.len() != 32 {
        return Err(InternalError::UpdateOutputDataInvalidLength.into());
    }

    debug!("load the update from witness");
    let witness_args = hl::load_witness_args(output_index, Source::Output)?;
    if let Some(args) = witness_args.output_type().to_opt() {
        debug!("verify the update");
        let update_slice = &args.raw_data();
        let update = SmtUpdateReader::from_slice(update_slice).map_err(|_| SysError::Encoding)?;
        if update.new_root().as_slice() != &output_data {
            return Err(InternalError::UpdateNewRootIsMismatch.into());
        }

        let old_root = checked_slice_to_h256(&input_data);
        update.verify_smt(&old_root)?;
    } else {
        return Err(InternalError::UpdateWitnessIsNotExisted.into());
    }

    Ok(())
}

fn checked_slice_to_h256(slice: &[u8]) -> H256 {
    let mut v = [0u8; 32];
    v.copy_from_slice(slice);
    v.into()
}
