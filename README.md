# Ethereum EVM

This is an implementation in Rust of the ethereum EVM. This library provides a bytecode interpreter that can be used to execute smart contracts and calculate the state modification. Designed to eventually be used in a full ethereum rust node.

## Tests

This repo contains both some local tests as well as using the official Ethereum Test library. The official tests are included as a sub repo to make sure to clone sub repos in addition to the main repo before running tests. Currently about a third of the official arithmetic tests are passing. The JSON passing of tests is currently very inefficient as tests are repassed many times, this will be fixed soon.

## Project State

This project is very much a work in progress. Most of the Opcodes are now functional, however, some core functionality is still being implemented such as better transaction reverting etc. The aim is to get all of the basic official Ethereum VMTests working as quickly as possible, (of which about 60% are currently passing). Then teh code will be refactored significantly to clean up all the mess created getting things to work.

After this work will be done on optimisations of the interpreter, such as analysis to combine gas additions, as well as adding new instructions to combine blocks of code.
