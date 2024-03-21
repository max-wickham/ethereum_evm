mod mocks;
use std::{collections::HashMap, str::FromStr};
use ethereum_evm::{assembler::assemble, evm::{EVMContext, Message, Transaction}, runtime::Runtime, state::memory::Memory, util};
use ethnum::U256;
use mocks::mock_runtime::{Contract, MockRuntime};
use test_gen::generate_tests;

// Basic tests using only a single contract, (no gas checks)
generate_tests!("./tests/test_definitions/basic_tests.json");
