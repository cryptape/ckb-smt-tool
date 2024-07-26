use ckb_testtool::{
    builtin::ALWAYS_SUCCESS,
    ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*},
    context::Context,
};

use crate::{prelude::*, utilities, Loader};

#[test]
fn success() {
    utilities::setup();

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
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    // prepare outputs
    let outputs = vec![CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(lock_script.clone())
        .build()];
    let outputs_data = vec![Bytes::new(); outputs.len()];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let _ = context.should_be_passed_without_limit(&tx);
}