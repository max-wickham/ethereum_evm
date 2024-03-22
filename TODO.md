# TODO

- [_] Check against official ethereum vm logs to debug
- [_] Submit fix for py eth vm, check that the address is hot
- [_] Fix memory access costs, need to consolidate all memory accesses to a single memory
- [x] Fix debug code to be prettier
- [_] Original Storage value, (modify gas usage to support this)
- [_] Apply gas usage
- [_] Get the root hash to work on a test, require making sure gas usage is applied and that the hash is calculated correctly
- [_] Restructure the way in which a transaction is called on the EVM
- [_] Allow reverting of storage (can be done inefficiently for now)
- [_] Implement hash of state for mock runtime

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
