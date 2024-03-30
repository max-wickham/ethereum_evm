// use ethereum_evm::util::{h256_to_u256, u256_to_h256};
use hex::FromHex;
use primitive_types::{H256, U256};
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::BTreeMap, fmt};
use crate::util::{Hex, u256_to_h256};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
struct WrappedU256(U256);

impl<'de> Deserialize<'de> for WrappedU256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct H256Visitor;

        impl<'de> serde::de::Visitor<'de> for H256Visitor {
            type Value = WrappedU256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a hex-encoded H256 value")
            }

            fn visit_str<E>(self, value: &str) -> Result<WrappedU256, E>
            where
                E: serde::de::Error,
            {
                let fixed_length_str: String;
                let mut value_without_prefix = if value.starts_with("0x") {
                    &value[2..] // Skip the first two characters (0x)
                } else {
                    value
                };
                if value_without_prefix.len() % 2 == 1{
                    fixed_length_str = "0".to_string() + value_without_prefix;
                    value_without_prefix = fixed_length_str.as_str();

                }
                let hash_bytes: Vec<u8> = match Vec::<u8>::from_hex(value_without_prefix) {
                    Ok(bytes) => bytes,
                    Err(_) => return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(value), &self)),
                };

                let mut hash = [0u8; 32];
                let num_bytes_to_copy = hash_bytes.len().min(32);
                // println!("num_bytes_to_copy: {}", num_bytes_to_copy);
                // println!("hash_bytes: {:x?}", hash_bytes);
                hash[32 - num_bytes_to_copy..32].copy_from_slice(&hash_bytes);
                // println!("hash: {:x?}", hash);
                Ok(WrappedU256(U256::from(hash)))
            }
        }

        deserializer.deserialize_str(H256Visitor)
    }
}


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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPost {
    pub hash: H256,
    pub indexes: TestPostIndexes,
    pub logs: U256,
    #[serde(rename = "txbytes")]
    pub tx_bytes: Hex,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPostIndexes {
    pub data: usize,
    pub gas: usize,
    pub value: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestContract {
    balance: WrappedU256,
    pub code: Hex,
    nonce: WrappedU256,
    storage: BTreeMap<WrappedU256, WrappedU256>,
}

impl TestContract{
    pub fn storage(&self) -> BTreeMap<H256, H256> {
        self.storage.iter().map(|(k, v)| (u256_to_h256(k.0), u256_to_h256(v.0))).collect()
    }

    pub fn nonce(&self) -> U256 {
        self.nonce.0
    }

    pub fn balance(&self) -> U256 {
        self.balance.0
    }
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
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
