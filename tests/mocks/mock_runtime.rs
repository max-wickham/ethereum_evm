use ethereum_evm::runtime::Runtime;
use ethereum_evm::evm_logic::util::{h256_to_u256, keccak256, u256_to_h256};
use primitive_types::{H160, H256, U256};
use rlp::{Encodable, RlpStream};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::mem;

/*
!!!
This implementation is extremely inefficient. It is only for running test cases. Every context copies the EVM state.
!!!
*/

#[derive(Clone)]
pub struct Contract {
    pub balance: U256,
    pub code_size: U256,
    pub code_hash: H256,
    pub code: Vec<u8>,
    pub nonce: U256,
    // Should really be moved into a box for copying
    pub storage: BTreeMap<H256, H256>,
    pub is_deleted: bool,
    pub is_cold: bool,
    pub hot_keys: HashSet<U256>,
}

pub struct Context {
    pub prev_context: Option<Box<Context>>,
    pub contracts: BTreeMap<U256, Contract>,
}
pub struct MockRuntime {
    pub block_hashes: BTreeMap<U256, H256>,
    pub block_number: U256,
    pub block_coinbase: U256,
    pub block_timestamp: U256,
    pub block_difficulty: U256,
    pub block_randomness: U256,
    pub block_gas_limit: U256,
    pub block_base_fee_per_gas: U256,
    pub chain_id: U256,
    pub contracts: BTreeMap<U256, Contract>,
    pub current_context: Option<Box<Context>>,
}

#[derive(Debug)]
struct RLPContract {
    storage_root_hash: H256,
    code_hash: H256,
    nonce: U256,
    balance: U256,
    code_version: U256,
}

impl Encodable for RLPContract {
    fn rlp_append(&self, s: &mut RlpStream) {
        let use_short_version = self.code_version.eq(&U256::zero());
        match use_short_version {
            true => {
                s.begin_list(4);
            }
            false => {
                s.begin_list(5);
            }
        }
        println!("nonce: {:x?}", self.nonce);
        println!("balance: {:x?}", self.balance);
        println!("storage_root_hash: {:x?}", self.storage_root_hash);
        println!("code_hash: {:x?}", self.code_hash);
        s.append(&self.nonce)
            .append(&self.balance)
            .append(&self.storage_root_hash)
            .append(&self.code_hash);
        if !use_short_version {
            s.append(&self.code_version);
        }
    }
}

impl MockRuntime {
    pub fn state_root_hash(&self) -> H256 {
        let tree: Vec<(_, _)> = self
            .contracts
            .iter()
            .map(|(address, contract)| {
                (H160::from(u256_to_h256(*address)), {
                    println!("");
                    println!("address: {:x}", address);
                    println!("storage: {:?}", contract.storage);
                    let encoded_contract = rlp::encode(&RLPContract {
                        storage_root_hash: ethereum::util::sec_trie_root(
                            contract
                                .storage
                                .iter()
                                .map(|(key, value)| (key, rlp::encode(&h256_to_u256(*value)))),
                        ),
                        code_hash: keccak256(&contract.code),
                        nonce: contract.nonce,
                        balance: contract.balance,
                        code_version: U256::zero(),
                    });
                    // println!("encoded_contract: {:x?}", encode(keccak256(&encoded_contract.to_vec())));
                    encoded_contract
                })
            })
            .collect::<Vec<_>>();
        // println!("tree: {:x?}", &tree);
        let x = ethereum::util::sec_trie_root(tree);
        // println!("state_root_hash: {:x?}", x);
        x
    }
}

impl Runtime for MockRuntime {
    // Block State
    fn block_hash(&self, block_number: U256) -> H256 {
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

    // TODO add default values if address is not found
    // Context state
    fn balance(&self, address: U256) -> U256 {
        self.current_context.as_ref().unwrap().contracts[&address].balance
    }
    fn code_size(&self, address: U256) -> U256 {
        self.current_context.as_ref().unwrap().contracts[&address].code_size
    }
    fn code_hash(&self, address: U256) -> H256 {
        self.current_context.as_ref().unwrap().contracts[&address].code_hash
    }
    fn nonce(&self, address: U256) -> U256 {
        self.current_context.as_ref().unwrap().contracts[&address].nonce
    }
    fn code(&self, address: U256) -> Vec<u8> {
        self.current_context.as_ref().unwrap().contracts[&address]
            .code
            .clone()
    }
    fn exists(&self, address: U256) -> bool {
        self.current_context
            .as_ref()
            .unwrap()
            .contracts
            .contains_key(&address)
    }
    fn storage(&mut self, address: U256) -> &BTreeMap<H256, H256> {
        &mut self
            .current_context
            .as_mut()
            .unwrap()
            .contracts
            .get_mut(&address)
            .unwrap()
            .storage
    }
    fn original_storage(&mut self, address: U256) -> &BTreeMap<H256, H256> {
        &self.contracts[&address].storage
    }

    // TODO add logic if address is not found
    // Modify Contract State (Should always be valid addresses)
    fn is_deleted(&self, address: U256) -> bool {
        self.current_context.as_ref().unwrap().contracts[&address].is_deleted
    }
    fn is_cold(&self, address: U256) -> bool {
        self.current_context.as_ref().unwrap().contracts[&address].is_cold
    }
    fn is_cold_index(&self, address: U256, index: U256) -> bool {
        !self.current_context.as_ref().unwrap().contracts[&address]
            .hot_keys
            .contains(&index)
    }
    fn mark_hot(&mut self, address: U256) {
        self.current_context
            .as_mut()
            .unwrap()
            .contracts
            .get_mut(&address)
            .unwrap()
            .is_cold = false;
    }
    fn mark_hot_index(&mut self, address: U256, index: U256) {
        self.current_context
            .as_mut()
            .unwrap()
            .contracts
            .get_mut(&address)
            .unwrap()
            .hot_keys
            .insert(index);
    }
    fn set_storage(&mut self, address: U256, index: U256, value: H256) {
        self.current_context
            .as_mut()
            .unwrap()
            .contracts
            .get_mut(&address)
            .unwrap()
            .storage
            .insert(u256_to_h256(index), value);
        // for (address, contract) in &self.current_context.as_ref().unwrap().contracts {
        //     println!("Storage: {:?}", contract.storage);
        // }
    }
    fn mark_delete(&mut self, address: U256) {
        self.current_context
            .as_mut()
            .unwrap()
            .contracts
            .get_mut(&address)
            .unwrap()
            .is_deleted = true;
    }
    fn reset_storage(&mut self, address: U256) {
        self.current_context
            .as_mut()
            .unwrap()
            .contracts
            .get_mut(&address)
            .unwrap()
            .storage = BTreeMap::new();
    }
    fn set_code(&mut self, address: U256, code: Vec<u8>) {
        self.current_context
            .as_mut()
            .unwrap()
            .contracts
            .get_mut(&address)
            .unwrap()
            .code = code;
    }
    fn reset_balance(&mut self, address: U256) {
        self.current_context
            .as_mut()
            .unwrap()
            .contracts
            .get_mut(&address)
            .unwrap()
            .balance = U256::from(0 as u64);
    }
    fn deposit(&mut self, target: U256, value: U256) {
        self.current_context
            .as_mut()
            .unwrap()
            .contracts
            .get_mut(&target)
            .unwrap()
            .balance += value;
    }
    fn withdrawal(&mut self, source: U256, value: U256) {
        self.current_context
            .as_mut()
            .unwrap()
            .contracts
            .get_mut(&source)
            .unwrap()
            .balance -= value;
    }
    fn increase_nonce(&mut self, address: U256) {
        self.current_context
            .as_mut()
            .unwrap()
            .contracts
            .get_mut(&address)
            .unwrap()
            .nonce += U256::from(1);
    }

    // Modify context stack
    fn add_context(&mut self) {
        // Could be slightly faster with a swap perhaps
        match mem::take(&mut self.current_context) {
            Some(context) => {
                self.current_context = Some(Box::new(Context {
                    contracts: context.contracts.clone(),
                    prev_context: Some(context)
                }));
            }
            None => {
                self.current_context = Some(Box::new(Context {
                    contracts: self.contracts.clone(),
                    prev_context: None,
                }));
            }
        };
    }
    fn merge_context(&mut self) {
        match mem::take(&mut self.current_context) {
            Some(mut context) => {
                match &mut context.prev_context {
                    Some(prev_context) => {
                        prev_context.contracts = context.contracts;
                    }
                    None => {
                        println!("Setting Contracts");
                        self.contracts = context.contracts;
                    }
                }
                self.current_context = context.prev_context;
            }
            None => {
                self.current_context = None;
            }
        }
    }
    fn revert_context(&mut self) {
        match mem::take(&mut self.current_context) {
            Some(context) => {
                self.current_context = context.prev_context;
            }
            None => {
                self.current_context = None;
            }
        }
    }
}
