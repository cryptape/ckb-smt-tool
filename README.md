# CKB SMT Tool

[![License]](#license)
[![GitHub Actions]](https://github.com/cryptape/ckb-smt-tool/actions)

A tool to integrate SMT into CKB contracts easier.

[License]: https://img.shields.io/badge/License-MIT-blue.svg
[GitHub Actions]: https://github.com/cryptape/ckb-smt-tool/workflows/CI/badge.svg

## Description

This tool is used to store an SMT root into chain, also provide methods to
update the SMT and generate proof to verify whether some data is on the SMT
or not.

## Crates

- [CKB SMT Tool]

  A tool to integrate SMT into CKB contracts easier.

## Examples

There is two demo contracts:

- [On-Chain Key-Value Store]

  To store SMT on chain.

- [Check Data with On-Chain Key-Value Store]

  To verify if some data was in the on-chain store (equivalent to on the SMT).

## License

Licensed under [MIT License].

[CKB]: https://github.com/nervosnetwork/ckb

[MIT License]: LICENSE
[CKB SMT Tool]: crates/ckb-smt-tool
[On-Chain Key-Value Store]: contracts/demo-onchain-kvstore
[Check Data with On-Chain Key-Value Store]: contracts/demo-onchain-kvstore
