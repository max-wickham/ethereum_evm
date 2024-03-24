# Ethereum EVM

This is an implementation in Rust of the ethereum EVM. This library provides a bytecode interpreter that can be used to execute smart contracts and calculate the state modification. Designed to eventually be used in a full ethereum rust node.

## Tests

This repo contains both some local tests as well as using the official Ethereum Test library. The official tests are included as a sub repo to make sure to clone sub repos in addition to the main repo before running tests. Currently about a third of the official arithmetic tests are passing.
