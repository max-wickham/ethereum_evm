// use ethereum_evm::{
//     evm_logic::{evm::EVMContext, util::{keccak256, u256_to_h256}}, evm_logic::result::ExecutionResult, runtime::Runtime
// };
use ethereum_evm::{
    execute_transaction,
    result::ExecutionResult,
    runtime::Runtime,
    util::{ keccak256, u256_to_h256 },
};
use primitive_types::U256;
use serde_json::json;
use serde_json::value::Value;
use std::{ collections::{ BTreeMap, HashSet }, fs::File, io::BufReader };
use test_gen::generate_official_tests;

use crate::mocks::mock_runtime::{ Contract, MockRuntime };

use official_test_types::types::{ TestState, TestStateMulti };

// generate_official_tests!("./tests/official_tests/tests/GeneralStateTests/VMTests");

// generate_official_tests!("./tests/official_tests/tests/GeneralStateTests/stShift");
// generate_official_tests!("./tests/official_tests/tests/GeneralStateTests/stSLoadTest");
// generate_official_tests!("./tests/official_tests/tests/GeneralStateTests/stSStoreTest");
// generate_official_tests!("./tests/official_tests/tests/GeneralStateTests/stStackTests");
generate_official_tests!("./tests/official_tests/tests/GeneralStateTests/stPreCompiledContracts2");
// generate_official_tests!("./tests/official_tests/tests/GeneralStateTests/stStaticCall");

// generate_official_tests!("./tests/official_tests/tests/GeneralStateTests/VMTests");
// generate_official_tests!(
//     "./tests/official_tests/tests/GeneralStateTests/VMTests/vmIOandFlowOperations/jumpToPush.json"
// );

// generate_official_tests!(
// "./tests/official_tests/tests/GeneralStateTests/VMTests/vmTests/suicide.json"
// );

// generate_official_tests!(
//     "./tests/official_tests/tests/ABITests"
// );
// generate_official_tests!("./tests/official_tests/tests/GeneralStateTests/stZeroKnowledge2");
// generate_official_tests!("./tests/official_tests/tests/GeneralStateTests/VMTests/vmTests/suicide.json");

pub fn run_test_file(filename: String, debug: bool, index: usize) {
    let tests: BTreeMap<String, TestStateMulti> = serde_json
        ::from_reader(BufReader::new(File::open(filename).unwrap()))
        .unwrap();
    // println!("Debug: {:?}", debug);
    run_test(&tests.into_iter().nth(0).unwrap().1.tests().into_iter().nth(index).unwrap(), debug);
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
                contracts.insert(*address, Contract {
                    balance: contract.balance(),
                    code_size: U256::from(contract.code.0.len() as u64),
                    code_hash: keccak256(&contract.code.0),
                    code: contract.code.0.clone(),
                    nonce: contract.nonce(),
                    storage: contract.storage().clone(),
                    is_deleted: false,
                    is_cold: true,
                    hot_keys: HashSet::new(),
                });
                if debug {
                    println!("Storage: {:?}", contract.storage().clone());
                }
            }
            contracts.insert(test.env.current_coinbase, Contract {
                balance: U256::zero(),
                code_size: U256::zero(),
                code_hash: u256_to_h256(U256::zero()),
                code: vec![],
                nonce: U256::zero(),
                storage: BTreeMap::new(),
                is_deleted: false,
                is_cold: true,
                hot_keys: HashSet::new(),
            });
            contracts
        },
        current_context: None,
    };
    runtime.add_context();
    if debug {
        println!("Message data size : {}", test.transaction.data.len());
    }
    // Execute the transaction
    let (result, gas_usage) = execute_transaction(
        &mut runtime,
        test.transaction.to,
        test.transaction.sender,
        test.transaction.gas_limit.as_u64(),
        test.transaction.gas_price.unwrap_or_default(),
        test.transaction.value,
        &test.transaction.data,
        debug
    );

    // Calculate the gas usage
    let eth_usage = gas_usage * test.transaction.gas_price.unwrap_or_default().as_usize();
    if debug {
        println!("Gas Usage: {}", gas_usage);
        println!("Eth Usage: {}", eth_usage);
        println!("Value: {}", test.transaction.value);
    }
    // send value the wallet
    // runtime.increase_nonce(test.transaction.sender);
    // match result {
    //     ExecutionResult::Success(_) => {
    //         runtime.deposit(test.transaction.to, test.transaction.value);
    //         // withdraw the value from the sender
    //         runtime.withdrawal(test.transaction.sender, test.transaction.value);
    //     }
    //     _ => {}
    // }
    // withdraw the gas usage from the sender
    // runtime.withdrawal(test.transaction.sender, U256::from(eth_usage as u64));
    // runtime.deposit(test.env.current_coinbase, U256::from(eth_usage as u64));
    // runtime.merge_context();
    if debug {
        println!("Context {:?}", match runtime.current_context {
            Some(_) => "Exists",
            _ => "Doesn't Exist",
        });
        // for (address, contract) in &runtime.contracts {
        //     println!("Address: {:x}", address);
        //     println!("Storage: {:?}", contract.storage);
        // }
    }
    // Debug the balances
    assert_eq!(runtime.state_root_hash(), test.post.hash);
}

// generate_official_tests_from_folder!(
//     "./tests/official_tests/tests/GeneralStateTests/stMemoryTest"
// );

// generate_official_tests_from_folder!(
//     "./tests/official_tests/tests/GeneralStateTests/stMemoryTest"
// );

// generate_official_tests_from_folder!(
//     "./tests/official_tests/tests/GeneralStateTests/VMTests/vmPerformance"
// );

// generate_official_tests_from_file!(
//     "./tests/official_tests/tests/GeneralStateTests/stMemoryTest/mem32kb-33.json"
// );
// generate_official_tests_from_file!(
//     "./tests/official_tests/tests/GeneralStateTests/VMTests/vmArithmeticTest/fib.json"
// );
// generate_official_tests_from_folder!(
//     "./tests/official_tests/tests/GeneralStateTests/VMTests/vmArithmeticTest"
// );
// generate_official_tests_from_file!(
//     "./tests/official_tests/tests/GeneralStateTests/stMemoryTest/buffer.json"
// );
// generate_official_tests_from_folder!(
//     "./tests/official_tests/tests/GeneralStateTests/VMTests/vmBitwiseLogicOperation"
// );
