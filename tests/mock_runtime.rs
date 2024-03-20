use std::{collections::HashMap, hash::Hash};

use ethereum_evm::assembler::assemble;
use ethereum_evm::evm::{EVMContext, Message, Transaction};
use ethereum_evm::runtime::Runtime;
use ethereum_evm::state::memory::Memory;
use ethereum_evm::util;
use ethnum::U256;

pub struct ContractModifications {}

pub struct Contract {
    pub balance: U256,
    pub code_size: U256,
    pub code_hash: U256,
    pub code: Vec<u8>,
    pub nonce: U256,
    pub storage: HashMap<U256, U256>,
    pub is_deleted: bool,
    pub is_cold: bool,
}

pub struct MockRuntime {
    pub block_hashes: HashMap<U256, U256>,
    pub block_number: U256,
    pub block_coinbase: U256,
    pub block_timestamp: U256,
    pub block_difficulty: U256,
    pub block_randomness: U256,
    pub block_gas_limit: U256,
    pub block_base_fee_per_gas: U256,
    pub chain_id: U256,
    pub contracts: HashMap<U256, Contract>,
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
    fn storage(&mut self, address: U256) -> &mut HashMap<U256, U256> {
        &mut self.contracts.get_mut(&address).unwrap().storage
    }

    // Modify Contract State
    fn is_deleted(&self, address: U256) -> bool {
        self.contracts[&address].is_deleted
    }
    fn is_cold(&self, address: U256, index: Option<U256>) -> bool {
        self.contracts[&address].is_cold
    }
    fn is_hot(&self, address: U256, index: Option<U256>) -> bool {
        !self.is_cold(address, index)
    }
    fn mark_hot(&mut self, address: U256, index: Option<U256>) {
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
        self.contracts.get_mut(&address).unwrap().storage = HashMap::new();
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
