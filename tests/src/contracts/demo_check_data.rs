use ckb_smt_tool::types::ProofGenerator;
use ckb_testtool::{
    builtin::ALWAYS_SUCCESS,
    ckb_types::{
        bytes::Bytes,
        core::{DepType, TransactionBuilder},
        packed::*,
        prelude::*,
    },
    context::Context,
};

use crate::{prelude::*, utilities, Loader};

fn build_generator() -> ProofGenerator {
    let mut generator = ProofGenerator::new();
    for i in 0..20u8 {
        let k = [i; 4];
        let v = Some(Bytes::copy_from_slice(&[i; 10]));
        generator.update(&k, v).expect("smt update");
    }
    generator
}

#[test]
fn success() {
    utilities::setup();

    let mut context = Context::default();

    // Build the data.
    let generator = build_generator();
    let root = generator.root().to_owned();
    let data_with_proof = {
        let keys = (0..5u8)
            .map(|x| Bytes::copy_from_slice(&[x * 5; 4]))
            .collect();
        generator
            .data_with_proof(keys)
            .expect("generate data with proof")
    };

    let success_lock_script = {
        let out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
        context
            .build_script(&out_point, Default::default())
            .expect("success lock script")
    };

    let demo_type_script = {
        let contract_bin: Bytes = Loader::default().load_binary("demo-onchain-kvstore");
        let out_point = context.deploy_cell(contract_bin);
        let unique_id = [0u8; 32];
        context
            .build_script(&out_point, Bytes::from(unique_id.to_vec()))
            .expect("demo type script")
    };

    let demo_lock_script = {
        let contract_bin: Bytes = Loader::default().load_binary("demo-check-data");
        let out_point = context.deploy_cell(contract_bin);
        let smt_script_hash = demo_type_script.calc_script_hash();
        context
            .build_script(
                &out_point,
                Bytes::copy_from_slice(&smt_script_hash.raw_data()),
            )
            .expect("demo lock script")
    };

    let smt_cell_dep = {
        let type_script_opt = ScriptOpt::new_builder().set(Some(demo_type_script)).build();
        let out_point = context.create_cell(
            CellOutput::new_builder()
                .capacity(1000u64.pack())
                .lock(success_lock_script)
                .type_(type_script_opt.clone())
                .build(),
            Bytes::copy_from_slice(root.as_slice()),
        );
        CellDep::new_builder()
            .out_point(out_point)
            .dep_type(DepType::Code.into())
            .build()
    };

    let input = {
        let out_point = context.create_cell(
            CellOutput::new_builder()
                .capacity(1000u64.pack())
                .lock(demo_lock_script.clone())
                .build(),
            Bytes::new(),
        );
        CellInput::new_builder().previous_output(out_point).build()
    };

    let output = CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(demo_lock_script.clone())
        .build();

    let witness = {
        let lock_args = BytesOpt::new_builder()
            .set(Some(data_with_proof.as_slice().pack()))
            .build();
        let witness_args = WitnessArgs::new_builder().lock(lock_args).build();
        witness_args.as_bytes()
    };

    let tx = TransactionBuilder::default()
        .cell_dep(smt_cell_dep)
        .input(input)
        .output(output)
        .output_data(Default::default())
        .witness(witness.pack())
        .build();
    let tx = context.complete_tx(tx);

    let _ = context.should_be_passed_without_limit(&tx);
}
