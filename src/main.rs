use std::{collections::BTreeMap, hash::Hash, str::FromStr};

use assembler::assemble;
use ethnum::U256;
use evm::{EVMContext, Message, Transaction};
use runtime::Runtime;
use state::memory::Memory;

pub mod assembler;
pub mod bytecode_spec;
pub mod evm;
pub mod runtime;
pub mod state;
pub mod util;

struct ContractModifications {}

struct Contract {
    balance: U256,
    code_size: U256,
    code_hash: U256,
    code: Vec<u8>,
    nonce: U256,
    storage: BTreeMap<U256, U256>,
    is_deleted: bool,
    is_cold: bool,
}

struct MockRuntime {
    block_hashes: BTreeMap<U256, U256>,
    block_number: U256,
    block_coinbase: U256,
    block_timestamp: U256,
    block_difficulty: U256,
    block_randomness: U256,
    block_gas_limit: U256,
    block_base_fee_per_gas: U256,
    chain_id: U256,

    contracts: BTreeMap<U256, Contract>,
}

impl Runtime for MockRuntime {
    // Block information
    fn block_hash(&self, block_number: U256) -> U256 {
        self.block_hashes[&block_number]
    }
    fn block_number(&self) -> U256 {
        self.block_number
    }
    fn block_coinbase(&self) -> U256 {
        self.block_coinbase
    }
    fn block_timestamp(&self) -> U256 {
        self.block_timestamp
    }
    fn block_difficulty(&self) -> U256 {
        self.block_difficulty
    }
    fn block_randomness(&self) -> U256 {
        self.block_randomness
    }
    fn block_gas_limit(&self) -> U256 {
        self.block_gas_limit
    }
    fn block_base_fee_per_gas(&self) -> U256 {
        self.block_base_fee_per_gas
    }
    fn chain_id(&self) -> U256 {
        self.chain_id
    }

    fn balance(&self, address: U256) -> U256 {
        self.contracts[&address].balance
    }
    fn code_size(&self, address: U256) -> U256 {
        self.contracts[&address].code_size
    }
    fn code_hash(&self, address: U256) -> U256 {
        self.contracts[&address].code_hash
    }
    fn nonce(&self, address: U256) -> U256 {
        self.contracts[&address].nonce
    }
    fn code(&self, address: U256) -> Vec<u8> {
        self.contracts[&address].code.clone()
    }
    fn exists(&self, address: U256) -> bool {
        self.contracts.contains_key(&address)
    }
    fn storage(&mut self, address: U256) -> & BTreeMap<U256, U256> {
        &mut self.contracts.get_mut(&address).unwrap().storage
    }

    // Modify Contract State
    fn is_deleted(&self, address: U256) -> bool {
        self.contracts[&address].is_deleted
    }
    fn is_cold(&self, address: U256) -> bool {
        self.contracts[&address].is_cold
    }
    fn is_hot(&self, address: U256) -> bool {
        !self.is_cold(address)
    }
    fn mark_hot(&mut self, address: U256) {
        self.contracts.get_mut(&address).unwrap().is_cold = false;
    }
    fn set_storage(&mut self, address: U256, index: U256, value: U256) {
        self.contracts
            .get_mut(&address)
            .unwrap()
            .storage
            .insert(index, value);
    }
    fn mark_delete(&mut self, address: U256) {
        self.contracts.get_mut(&address).unwrap().is_deleted = true;
    }
    fn reset_storage(&mut self, address: U256) {
        self.contracts.get_mut(&address).unwrap().storage = BTreeMap::new();
    }
    fn set_code(&mut self, address: U256, code: Vec<u8>) {
        self.contracts.get_mut(&address).unwrap().code = code;
    }
    fn reset_balance(&mut self, address: U256) {
        self.contracts.get_mut(&address).unwrap().balance = U256::from(0 as u64);
    }
    fn deposit(&mut self, target: U256, value: U256) {
        // TODO
        self.contracts.get_mut(&target).unwrap().balance += value;
    }
    fn withdrawal(&mut self, source: U256, value: U256) {
        // TODO
        self.contracts.get_mut(&source).unwrap().balance -= value;
    }
    fn increase_nonce(&mut self, address: U256) {
        self.contracts.get_mut(&address).unwrap().nonce += 1;
    }
}

fn main() {
    let x  = U256::from_str("10");
    let code1 = assemble(String::from("push_32 5 push_32 10 add push_32 10 sstore stop"));
    println!("Code: {:?}", code1);
    let mut contract1 = Contract {
        balance: U256::from(10 as u64),
        code_size: U256::from(code1.len() as u64),
        code_hash: util::keccak256(&code1),
        code: code1,
        nonce: U256::from(0 as u64),
        storage: BTreeMap::new(),
        is_deleted: false,
        is_cold: false,
    };
    let mut contracts: BTreeMap<U256, Contract> = BTreeMap::new();
    contracts.insert(U256::from(1 as u64), contract1);
    let mut mock_runtime = MockRuntime {
        block_hashes: BTreeMap::new(),
        block_number: U256::from(0 as u64),
        block_coinbase: U256::from(0 as u64),
        block_timestamp: U256::from(0 as u64),
        block_difficulty: U256::from(0 as u64),
        block_randomness: U256::from(0 as u64),
        block_gas_limit: U256::from(100000 as u64),
        block_base_fee_per_gas: U256::from(1 as u64),
        chain_id: U256::from(0 as u64),
        contracts: contracts,
    };

    let mut context = EVMContext::create_sub_context(
        U256::from(1 as u64),
        Message {
            caller: U256::from(0 as u64),
            value: U256::from(10 as u64),
            data: Memory::new(),
        },
        1000,
        mock_runtime.contracts[&U256::from(1 as u64)].code.clone(),
        Transaction {
            origin: U256::from(0 as u64),
            gas_price: U256::from(1 as u64),
        },
        U256::from(1 as u64),
    );
    let result = context.execute(&mut mock_runtime);
    assert_eq!(result, true);
    assert_eq!(*mock_runtime.storage(U256::from(1 as u64)).get(&U256::from(10 as u64)).unwrap(), U256::from(15 as u64));

}
