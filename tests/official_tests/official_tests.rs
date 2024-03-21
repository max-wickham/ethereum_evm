use std::{collections::{BTreeMap, BTreeMap}, fs::File, hash::Hash, io::BufReader};
use ethereum_evm::{evm::EVMContext, util::keccak256};
use ethnum::U256;

use crate::mocks::mock_runtime::{Contract, MockRuntime};

use super::{error::{self, Error}, types::TestStateMulti};

pub fn run_test_file(filename: String, debug: bool) {
    let tests: BTreeMap<String, TestStateMulti> =
        serde_json::from_reader(BufReader::new(File::open(filename)?))?;

    for (name, test) in &tests {
        run_test(test);
    }

}

pub fn run_test(test: &TestStateMulti) {
    let runtime = MockRuntime {
        block_hashes: BTreeMap::new(),
        block_number:test.env.current_number,
        block_coinbase:test.env.current_coinbase,
        block_timestamp:test.env.current_timestamp,
        block_difficulty:test.env.current_difficulty,
        block_randomness:test.env.current_random,
        block_gas_limit:test.env.current_gas_limit,
        block_base_fee_per_gas:test.env.current_base_fee,
        chain_id:U256::ZERO,
        contracts: {
            let mut contracts = BTreeMap::new();
            for (address, contract) in &test.pre {
                contracts.insert(*address, Contract {
                    balance: contract.balance,
                    code_size: U256::from(contract.code.0.len() as u64) ,
                    code_hash: keccak256(&contract.code.0),
                    code: contract.code.0,
                    nonce: contract.nonce,
                    storage: contract.storage,
                    is_deleted: false,
                    is_cold: false,
                });
            }
            contracts
        },
    };

    for test in test.tests() {
        // generate EVM context,
        // send transaction to EVM context
        // check the the post data is correct, (assert the the root hash is the same as the hash of the post data)
    }



    // For each transaction create an EVM context, send the transaction and then apply the changes
}
