# CKB SMT Tool

A tool to integrate SMT into CKB contracts easier.

## Description

This tool is used to store an SMT root into chain, also provide methods to
update the SMT and generate proof to verify whether some data is on the SMT
or not.

## Usages

This library includes 2 parts, and each part has on-chain operations and
off-chain operations.

### Part 1. Maintain an SMT on chain

- Off-chain operations:

  The core struct `ProofGenerator` has two important methods:

  - `fn append_change(&mut self, key: Bytes, new_value: Option<Bytes>)`

    This method is used to append a change but not commit.

  - `fn commit_changes(&mut self) -> Result<SmtUpdate, GeneratorError>`

    This method is used to commit all pending changes, then return a copy of
    these changes with a proof of them and the new SMT root.

  Then, users could submit the result of the previous step into witness, to
  update the on-chain SMT.

- On-chain operations:

  Users should check the result of the previous step in their contracts on
  chain, with the following method:

  - `SmtUpdateReader::verify_smt(&self, old_root: &H256) -> Result<(), UpdateError>`

  If the check is passed, then the on-chain SMT root could be updated to the
  new root.

### Part 2. Verify a Proof on chain

- Off-chain operations:

  In the Part 1, all updates of the SMT are stored in the witnesses, so
  users should find all these updates from witnesses at first.


  Then, with the following method, the `ProofGenerator` could be restored:

  - `fn apply_update(&mut self, smt_update: SmtUpdateReader<'_>) -> Result<(), GeneratorError>`


  At last, call the following method to create a proof for any data:

  - `fn data_with_proof(&self, keys: Vec<Bytes>) -> Result<DataWithProof, GeneratorError>`

- On-chain operations:

  Use the following method to check the proof:

  - `DataWithProofReader::verify_smt(&self, root: &H256) -> Result<(), VerifyError>`

  If the check is passed, then the data which is included in the proof could
  be trusted.
