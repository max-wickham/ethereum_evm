# TODO

- [_] Refactor errors to not use execution results for success
- [x] Fix memory problems on the return (when size is 0)
- [x] Refactor create and create 2, also get create to work
- [x] Fix calling costs and split up function
- [x] Fix Calldata load, Code load etc.
- [_] Do general clean up
    - [x] Better error handling
    - [x] Refactor where macros and functions are
    - [x] Remove lambdas
    - [x] Move costs into a config
    - [x] Separate gas cost logic from main logic
- [x] Get Memory Buffer tests to work
    - Check CODECOPY
    - Check CALLDATACOPY
    - Check initial costs
- [x] Make gas refunds handle reverts
- [_] Change to H256 instead of U256 where needed (and H160)
- [_] Only pass JSON once in tests, (maybe pass in the proc macro and then directly insert in the code)
- [x] Replace macro with method in decoder?
- [x] Replace closure with macro
- [x] Move entire decode step into inline function
- [?] Restructure code into a folder system
- [x] More specific error handling
- [x] Create better gas tracking, (esplet success = 's: {
        // Try to send value two times.
        for _ in 0..2 {
            value = match send(value) {
                Ok(()) => break 's true,
                Err(SendError(value)) => value,
            }
        }
        false
    };

ecially for memory)
    - [x] Created gas tracker
    - [x] Apply gas tracker to memory operations
    - [x] Apply gas tracker to all other operations
- [?] Implement Reverts if failure (especially in calls)
- [_] Check the failure behavior of every instruction
- [x] Create a helper proc macro that creates a map from opcode to string value

- [_] Remove assembler? Maybe find an assembler crate
- [_] Fix test proc to auto detect if a file or folder and auto search sub directories
- [_] Allow specifying of a specific test in a file
- [?] Get all arithmetic tests to pass
- [?] Fix SSTORE and CALL costs (still not very clear)
- [?] Implement better system for reverts etc.
- [x] Check against official ethereum vm logs to debug
- [x] Fix memory access costs, need to consolidate all memory accesses to a single memory
- [x] Fix debug code to be prettier
- [x] Original Storage value, (modify gas usage to support this)
- [x] Apply gas usage
- [x] Get the root hash to work on a test, require making sure gas usage is applied and that the hash is calculated correctly
- [x] Restructure the way in which a transaction is called on the EVM
- [x] Allow reverting of storage (can be done inefficiently for now)
- [x] Implement hash of state for mock runtime

- [x] Finish implementing call, (without gas calculations)
- [N] Define tests for instructions not using runtime
    - [x] Create Proc macro to define tests in json
- [x] Add variable gas calculations
- [x] Create test implementing of Runtime
- [x] Revert only current context, (not also if a call fails)
- [x] Handle not enough values on stack
- [N] Define test for instructions using runtime
- [_] Implement delegate call and create 2
- [_] Implement Create
- [_] Define tests for mem and stack

## External

- [_] Submit fix for py eth vm, check that the address is hot
- [_] Submit fix for num256 conversion between signed and unsigned
