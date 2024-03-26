use ethereum_evm::{
    evm_logic::evm::EVMContext,
    runtime::Runtime,
    util::{keccak256, u256_to_h256},
};
use primitive_types::U256;
use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::BufReader,
};
use test_gen::{generate_official_tests_from_file, generate_official_tests_from_folder};

use crate::mocks::mock_runtime::{Context, Contract, MockRuntime};

use super::types::{TestState, TestStateMulti};

pub fn run_test_file(filename: String, debug: bool, index: usize) {
    let tests: BTreeMap<String, TestStateMulti> =
        serde_json::from_reader(BufReader::new(File::open(filename).unwrap())).unwrap();
    println!("Debug: {:?}", debug);
    run_test(
        &tests
            .into_iter()
            .nth(0)
            .unwrap()
            .1
            .tests()
            .into_iter()
            .nth(index)
            .unwrap(),
        debug,
    );
}

pub fn run_test(test: &TestState, debug: bool) {
    let test = test.clone();
    let mut runtime = MockRuntime {
        block_hashes: BTreeMap::new(),
        block_number: test.env.current_number,
        block_coinbase: test.env.current_coinbase,
        block_timestamp: test.env.current_timestamp,
        block_difficulty: test.env.current_difficulty,
        block_randomness: test.env.current_random,
        block_gas_limit: test.env.current_gas_limit,
        block_base_fee_per_gas: test.env.current_base_fee,
        chain_id: U256::zero(),
        contracts: {
            let mut contracts = BTreeMap::new();
            for (address, contract) in &test.pre {
                contracts.insert(
                    *address,
                    Contract {
                        balance: contract.balance,
                        code_size: U256::from(contract.code.0.len() as u64),
                        code_hash: keccak256(&contract.code.0),
                        code: contract.code.0.clone(),
                        nonce: contract.nonce,
                        storage: contract.storage.clone(),
                        is_deleted: false,
                        is_cold: true,
                        hot_keys: HashSet::new(),
                    },
                );
            }
            contracts.insert(
                test.env.current_coinbase,
                Contract {
                    balance: U256::zero(),
                    code_size: U256::zero(),
                    code_hash: u256_to_h256(U256::zero()),
                    code: vec![],
                    nonce: U256::zero(),
                    storage: BTreeMap::new(),
                    is_deleted: false,
                    is_cold: true,
                    hot_keys: HashSet::new(),
                },
            );
            contracts
        },
        current_context: None,
    };
    runtime.add_context();

    // Execute the transaction
    let gas_usage = EVMContext::execute_transaction(
        &mut runtime,
        test.transaction.to,
        test.transaction.sender,
        test.transaction.gas_limit.as_u64(),
        test.transaction.gas_price.unwrap_or_default(),
        test.transaction.value,
        test.transaction.data,
        debug,
    );

    // Calculate the gas usage
    let eth_usage = (gas_usage) * test.transaction.gas_price.unwrap_or_default().as_usize();
    println!("Debug: {:?}", debug);
    if debug {
        println!("Gas Usage: {}", gas_usage);
        println!("Eth Usage: {}", eth_usage);
        println!("Value: {}", test.transaction.value);
    }
    // send value the wallet
    runtime.increase_nonce(test.transaction.sender);
    runtime.deposit(test.transaction.to, test.transaction.value);
    // withdraw the value from the sender
    runtime.withdrawal(test.transaction.sender, test.transaction.value);
    // withdraw the gas usage from the sender
    runtime.withdrawal(test.transaction.sender, U256::from(eth_usage as u64));
    runtime.deposit(test.env.current_coinbase, U256::from(eth_usage as u64));
    runtime.merge_context();
    println!("Context {:?}", match runtime.current_context {
        Some(_) => "Exists",
        _ => "Doesn't Exist",
    });
    for (address, contract) in &runtime.contracts {
        println!("Address: {:x}", address);
        println!("Storage: {:?}", contract.storage);
    }
    // Debug the balances
    assert_eq!(runtime.state_root_hash(), test.post.hash);
}

generate_official_tests_from_folder!(
    "./tests/official_tests/tests/GeneralStateTests/VMTests/vmArithmeticTest"
);

// generate_official_tests_from_folder!(
//     "./tests/official_tests/tests/GeneralStateTests/stRandom"
// );

// generate_official_tests_from_file!(
//     "./tests/official_tests/tests/GeneralStateTests/VMTests/vmArithmeticTest/mul.json"
// );
