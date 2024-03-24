# TODO

- [_] Create better gas tracking, especially for memory
- [_] Fix test proc to auto detect if a file or folder and auto search sub directories
- [_] Allow specifying of a specific test in a file
- [_] Get all arithmetic tests to pass
- [_] Fix SSTORE and CALL costs (still not very clear)
- [_] Implement better system for reverts etc.
- [x] Check against official ethereum vm logs to debug
- [x] Fix memory access costs, need to consolidate all memory accesses to a single memory
- [x] Fix debug code to be prettier
- [x] Original Storage value, (modify gas usage to support this)
- [x] Apply gas usage
- [x] Get the root hash to work on a test, require making sure gas usage is applied and that the hash is calculated correctly
- [x] Restructure the way in which a transaction is called on the EVM
- [_] Allow reverting of storage (can be done inefficiently for now)
- [x] Implement hash of state for mock runtime

- [x] Finish implementing call, (without gas calculations)
- [_] Define tests for instructions not using runtime
    - [x] Create Proc macro to define tests in json
    - [_] Create tests for all basic arithmetic instructions
- [x] Add variable gas calculations
- [x] Create test implementing of Runtime
- [_] Revert only current context, (not also if a call fails)
- [_] Handle not enough values on stack
- [_] Define test for instructions using runtime
- [_] Implement delegate call and create 2
- [_] Implement Create
- [_] Define tests for mem and stack

## External

- [_] Submit fix for py eth vm, check that the address is hot
- [_] Submit fix for num256 conversion
