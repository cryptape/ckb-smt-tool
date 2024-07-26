# Demo Contract: On-Chain Key-Value Store

A demo contract to show how to use the [CKB SMT tool] to store data on
chain.

It should be used as a type script.

## Description

This contract is used to record an SMT, which could determine some key-value
pairs, into the chain.

Users could get the certain value of any key from the chain data, and
generate a proof of the value off-chain.

Then, the proof could be verified on-chain.

[CKB SMT tool]: ../../crates/ckb-smt-tool
