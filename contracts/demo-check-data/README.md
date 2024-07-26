# Demo Contract: Check Data with On-Chain Key-Value Store

A demo contract to show how to use the [CKB SMT tool] to verify on-chain data.

It should be used as a lock script.

## Description

Store the type hash of an [on-chain KV store] instance in the `args` field of
the lock script.

Put some data of key-value pairs in the witness, and provide the proof of
them.

Return success when the verification of the proof is passed.

[CKB SMT tool]: ../../crates/ckb-smt-tool
[on-chain KV store]: ../demo-onchain-kvstore
