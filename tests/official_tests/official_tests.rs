use ethereum_evm::{
    evm::{EVMContext, Message, Transaction},
    runtime::Runtime,
    state::memory::Memory,
    util::keccak256,
};
use ethnum::U256;
use std::{collections::{BTreeMap, HashSet}, fs::File, hash::Hash, io::BufReader};

use crate::mocks::mock_runtime::{Contract, MockRuntime};

use super::{
    error::{self, Error},
    types::TestStateMulti,
};

pub fn run_test_file(filename: String, debug: bool) {
    let tests: BTreeMap<String, TestStateMulti> =
        serde_json::from_reader(BufReader::new(File::open(filename).unwrap())).unwrap();
    for (name, test) in &tests {
        run_test(test, debug);
    }
}

pub fn run_test(test: &TestStateMulti, debug: bool) {
    for test in test.tests() {
        let mut runtime = MockRuntime {
            block_hashes: BTreeMap::new(),
            block_number: test.env.current_number,
            block_coinbase: test.env.current_coinbase,
            block_timestamp: test.env.current_timestamp,
            block_difficulty: test.env.current_difficulty,
            block_randomness: test.env.current_random,
            block_gas_limit: test.env.current_gas_limit,
            block_base_fee_per_gas: test.env.current_base_fee,
            chain_id: U256::ZERO,
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
                contracts
            },
        };
        // TODO
        let mut evm_context = EVMContext::create_sub_context(
            test.transaction.to,
            Message {
                caller: test.transaction.sender,
                value: test.transaction.value,
                data: Memory::from(test.transaction.data),
            },
            test.transaction.gas_limit.as_u64(),
            runtime.code(test.transaction.to),
            Transaction {
                origin: test.transaction.sender,
                gas_price: test.transaction.gas_price.unwrap_or_default(),
            },
            test.transaction.gas_price.unwrap_or_default(),
        );
        evm_context.execute(&mut runtime);
        let eth_usage = (21000 + evm_context.gas_usage) * evm_context.gas_price.as_u64();
        println!("Gas Usage: {}", evm_context.gas_usage + 21000);
        println!("Eth Usage: {}", eth_usage);
        println!("Value: {}", test.transaction.value);
        // send value the wallet
        runtime.deposit(evm_context.contract_address, test.transaction.value);
        // withdraw the value from the sender
        runtime.withdrawal(test.transaction.sender, test.transaction.value);
        // withdraw the gas usage from the sender
        runtime.withdrawal(test.transaction.sender, U256::from(eth_usage));


        println!("{:?}", runtime.storage(U256::from_str_hex("0x100").unwrap())[&U256::from(0x00 as u64)]);
        for (key, contract) in test.pre {
            println!("{:?},", runtime.balance(key));
        }
        assert_eq!(runtime.state_root_hash(), test.post.hash);
        return;
        // generate EVM context,
        // send transaction to EVM context
        // check the the post data is correct, (assert the the root hash is the same as the hash of the post data)
    }

    // For each transaction create an EVM context, send the transaction and then apply the changes
}


#[test]
fn run_basic_add() {
    let filename = "tests/official_tests/tests/GeneralStateTests/VMTests/vmArithmeticTest/add.json";
    run_test_file(filename.to_string(), true);
}
