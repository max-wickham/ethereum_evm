use primitive_types::{H256, U256};
pub trait Runtime {
    /*
    Runtime that can be used by the EVM, requires the following methods to be implemented.
    The runtime provides the means with which the EVM can interact with the global state
    */

    // Block information
    fn block_hash(&self, block_number: U256) -> H256;
    fn block_number(&self) -> U256;
    fn block_coinbase(&self) -> U256;
    fn block_timestamp(&self) -> U256;
    fn block_difficulty(&self) -> U256;
    fn block_randomness(&self) -> U256;
    fn block_gas_limit(&self) -> U256;
    fn block_base_fee_per_gas(&self) -> U256;
    fn chain_id(&self) -> U256;

    // Contract information
    fn balance(&self, address: U256) -> U256;
    fn code_size(&self, address: U256) -> U256;
    fn code_hash(&self, address: U256) -> H256;
    fn code(&self, address: U256) -> Vec<u8>;
    fn exists(&self, address: U256) -> bool;
    fn nonce(&self, address: U256) -> U256;
    fn read_storage(&self, address: U256, index: U256) -> H256;
    fn read_original_storage(&self, address: U256, index: U256) -> H256;

    // Modify Contract State
    fn is_deleted(&self, address: U256) -> bool;
    fn is_cold(&self, address: U256) -> bool;
    fn is_cold_index(&self, address: U256, index: U256) -> bool;
    fn is_hot(&self, address: U256) -> bool {
        !self.is_cold(address)
    }
    fn is_hot_index(&self, address: U256, index: U256) -> bool {
        !self.is_cold_index(address, index)
    }
    fn mark_hot(&mut self, address: U256);
    fn mark_hot_index(&mut self, address: U256, index: U256);
    fn set_storage(&mut self, address: U256, index: U256, value: H256);
    fn mark_delete(&mut self, address: U256);
    fn reset_storage(&mut self, address: U256);
    fn set_code(&mut self, address: U256, code: Vec<u8>);
    fn reset_balance(&mut self, address: U256);
    fn deposit(&mut self, target: U256, value: U256);
    fn withdrawal(&mut self, source: U256, value: U256);
    fn increase_nonce(&mut self, address: U256);

    // Create Contract
    fn create_contract(
        &mut self,
        address: U256,
        code: Vec<u8>,
    );
    fn set_contract_code(
        &mut self,
        address: U256,
        code: Vec<u8>,
    );

    fn add_context(&mut self);
    fn merge_context(&mut self);
    fn revert_context(&mut self);
}
