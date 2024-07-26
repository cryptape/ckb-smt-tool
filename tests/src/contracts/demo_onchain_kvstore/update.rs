use ckb_smt_tool::types::ProofGenerator;
use ckb_testtool::{
    builtin::ALWAYS_SUCCESS,
    ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*},
    context::Context,
};

use crate::{prelude::*, utilities, Loader};

#[test]
fn success() {
    utilities::setup();

    let mut generator = ProofGenerator::new();
    {
        for i in 0..20u8 {
            let k = [i; 4];
            let v = Some(Bytes::copy_from_slice(&[i; 10]));
            generator.update(&k, v).expect("smt update");
        }
    }
    let old_root = generator.root().to_owned();
    {
        // Remove
        generator.append_change(Bytes::copy_from_slice(&[10u8; 4]), None);
        // Update
        generator.append_change(
            Bytes::copy_from_slice(&[15u8; 4]),
            Some(Bytes::copy_from_slice(&[15; 20])),
        );
        // Append
        generator.append_change(
            Bytes::copy_from_slice(&[25u8; 4]),
            Some(Bytes::copy_from_slice(&[25; 20])),
        );
    }
    let smt_update = generator.commit_changes().expect("smt commit");
    let new_root = generator.root().to_owned();

    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("demo-onchain-kvstore");
    let type_out_point = context.deploy_cell(contract_bin);
    let lock_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());

    // prepare scripts
    let lock_script = context
        .build_script(&lock_out_point, Default::default())
        .expect("lock script");
    let type_script = context
        .build_script(&type_out_point, Bytes::from([0u8; 32].to_vec()))
        .expect("type script");
    let type_script_opt = ScriptOpt::new_builder().set(Some(type_script)).build();

    // prepare inputs
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .type_(type_script_opt.clone())
            .build(),
        Bytes::copy_from_slice(old_root.as_slice()),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    // prepare outputs
    let outputs = vec![CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(lock_script.clone())
        .type_(type_script_opt.clone())
        .build()];
    let outputs_data = vec![Bytes::copy_from_slice(new_root.as_slice()); outputs.len()];

    // prepare witnesses
    let witness = {
        let type_args = BytesOpt::new_builder()
            .set(Some(smt_update.as_slice().pack()))
            .build();
        let witness_args = WitnessArgs::new_builder().output_type(type_args).build();
        witness_args.as_bytes()
    };

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .witness(witness.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let _ = context.should_be_passed_without_limit(&tx);
}
