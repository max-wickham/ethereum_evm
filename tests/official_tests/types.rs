use primitive_types::{H256, U256};
use serde::Deserialize;
use std::collections::BTreeMap;

use super::util::Hex;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestStateMulti {
    #[serde(rename = "_info")]
    pub info: TestInfo,
    pub env: TestEnv,
    pub post: BTreeMap<String, Vec<TestPost>>,
    pub pre: BTreeMap<U256, TestContract>,
    pub transaction: TestTransactionMulti,
}

impl TestStateMulti {
    pub fn tests(&self) -> Vec<TestState> {
        let mut tests = Vec::new();
        for (fork, post_states) in &self.post {
            if fork == "Berlin" {
                let mut new_tests: Vec<TestState> = post_states
                    .iter()
                    .enumerate()
                    .map(|(index, post_state)| TestState {
                        info: self.info.clone(),
                        env: self.env.clone(),
                        fork: fork.clone(),
                        post: post_state.clone(),
                        pre: self.pre.clone(),
                        transaction: TestTransaction {
                            data: self.transaction.data[post_state.indexes.data].0.clone(),
                            gas_limit: self.transaction.gas_limit[post_state.indexes.gas],
                            gas_price: self.transaction.gas_price,
                            max_fee_per_gas: self.transaction.max_fee_per_gas,
                            max_priority_fee_per_gas: self.transaction.max_priority_fee_per_gas,
                            nonce: self.transaction.nonce,
                            secret_key: self.transaction.secret_key,
                            sender: self.transaction.sender,
                            to: self.transaction.to,
                            value: self.transaction.value[post_state.indexes.value],
                        },
                    })
                    .collect();
                tests.append(&mut new_tests);
            }
        }
        tests
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestState {
    #[serde(rename = "_info")]
    pub info: TestInfo,
    pub env: TestEnv,
    pub fork: String,
    pub post: TestPost,
    pub pre: BTreeMap<U256, TestContract>,
    pub transaction: TestTransaction,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestInfo {
    pub comment: String,
    #[serde(rename = "filling-rpc-server")]
    pub filling_rpc_server: String,
    #[serde(rename = "filling-tool-version")]
    pub filling_tool_version: String,
    pub labels: Option<BTreeMap<String, String>>,
    pub generated_test_hash: String,
    #[serde(rename = "lllcversion")]
    pub lllc_version: String,
    pub solidity: String,
    pub source: String,
    pub source_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestEnv {
    pub current_base_fee: U256,
    pub current_coinbase: U256,
    pub current_difficulty: U256,
    pub current_excess_blob_gas: U256,
    pub current_gas_limit: U256,
    pub current_number: U256,
    pub current_random: U256,
    pub current_timestamp: U256,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPost {
    pub hash: H256,
    pub indexes: TestPostIndexes,
    pub logs: U256,
    #[serde(rename = "txbytes")]
    pub tx_bytes: Hex,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPostIndexes {
    pub data: usize,
    pub gas: usize,
    pub value: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestContract {
    pub balance: U256,
    pub code: Hex,
    pub nonce: U256,
    pub storage: BTreeMap<H256, H256>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestTransactionMulti {
    pub data: Vec<Hex>,
    pub gas_limit: Vec<U256>,
    pub gas_price: Option<U256>,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
    pub nonce: U256,
    pub secret_key: U256,
    pub to: U256,
    pub sender: U256,
    pub value: Vec<U256>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestTransaction {
    pub data: Vec<u8>,
    pub gas_limit: U256,
    pub gas_price: Option<U256>,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
    pub nonce: U256,
    pub secret_key: U256,
    pub sender: U256,
    pub to: U256,
    pub value: U256,
}
