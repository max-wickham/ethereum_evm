mod mocks;
mod official_tests;
// use std::{collections::BTreeMap, str::FromStr, collections::HashSet};
// // use ethereum_evm::{assembler::assemble, evm::{EVMContext, Message, Transaction}, runtime::Runtime, state::memory::Memory, util};
// use mocks::mock_runtime::{Contract, MockRuntime};
// // use test_gen::generate_tests;
use test_gen::generate_official_tests;
// use official_tests::official_tests::run_test_file;
// generate_official_tests!("./tests/official_tests/VMTests/vmArithmeticTest.json");
// // Basic tests using only a single contract, (no gas checks)
// // generate_tests!("./tests/test_definitions/basic_tests.json");
