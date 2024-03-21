use ethnum::U256;
use serde::{
	de::{Error, Visitor},
	Deserialize, Deserializer,
};
use std::collections::BTreeMap;
use std::fmt;
use hex::FromHex;

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
			let mut new_tests: Vec<TestState> = post_states.iter().enumerate().map(|(index, post_state)| {
				TestState {
					info: self.info.clone(),
					env: self.env.clone(),
					fork: fork.clone(),
					post: post_state.clone(),
					pre: self.pre.clone(),
					transaction: self.transaction.clone()
				}
			}).collect();
			tests.append(&mut new_tests);
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
	pub transaction: TestTransactionMulti,
}



#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestInfo {
	pub comment: String,
	#[serde(rename = "filling-rpc-server")]
	pub filling_rpc_server: String,
	#[serde(rename = "filling-tool-version")]
	pub filling_tool_version: String,
	pub labels: BTreeMap<String,String>,
	pub generated_test_hash: String,
	#[serde(rename = "llcversion")]
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
	pub hash: U256,
	pub indexes: TestPostIndexes,
	pub logs: U256,
	pub tx_bytes: HexBytes
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
	pub code: HexBytes,
	pub nonce: U256,
	pub storage: BTreeMap<U256,U256>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestTransactionMulti {
	pub data: Vec<HexBytes>,
	pub gas_limit: Vec<U256>,
	pub gas_price: Option<U256>,
	pub max_fee_per_gas: Option<U256>,
	pub max_priority_fee_per_gas: Option<U256>,
	pub nonce: U256,
	pub secret_key: U256,
	pub to: U256,
	pub value: Vec<U256>
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestTransaction {
	pub data: U256,
	pub gas_limit: U256,
	pub gas_price: Option<U256>,
	pub max_fee_per_gas: Option<U256>,
	pub max_priority_fee_per_gas: Option<U256>,
	pub nonce: U256,
	pub secret_key: U256,
	pub to: U256,
	pub value: U256
}


// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct BlockState {
// 	header: BlockHeader,
// 	rlp: String,
// 	transactions: Vec<TransactionState>
// }

// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct BlockHeader {
// 	bloom: String,
// 	coinbase: U256,
// 	difficulty: U256,
// 	extra_data: HexBytes,
// 	gas_limit: U256,
// 	gas_used: U256,
// 	hash: U256,
// 	mix_hash: U256,
// 	nonce: U256,
// 	number: U256,
// 	parent_hash: U256,
// 	receipt_trie: U256,
// 	state_root: U256,
// 	timestamp: U256,
// 	transaction_trie: U256,
// 	uncle_hash: U256,
// }


// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TransactionState {
// 	data: HexBytes,
// 	gas_limit: U256,
// 	gas_price: U256,
// 	nonce: U256,
// 	r: U256,
// 	s: U256,
// 	sender: U256,
// 	to: U256,
// 	v: u8,
// 	value: U256,
// }

// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Account {
// 	balance: U256,
// 	code: HexBytes,
// 	nonce: U256,
// 	storage: BTreeMap<U256,U256>,
// }


// const FORK: &str = "Berlin";

// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Tests {
// 	#[serde(rename = "_info")]
// 	pub info: TestInfo,
// 	pub env: TestEnv,
// 	pub post: BTreeMap<Fork, Vec<TestPost>>,
// 	pub pre: BTreeMap<U256, TestPre>,
// 	pub transaction: TestsTransaction,
// }

// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TestsTransaction {
// 	pub data: Vec<HexBytes>,
// 	pub gas_limit: Vec<U256>,
// 	pub gas_price: Option<U256>,
// 	pub max_fee_per_gas: Option<U256>,
// 	pub max_priority_fee_per_gas: Option<U256>,
// 	pub nonce: U256,
// 	pub secret_key: U256,
// 	pub sender: U256,
// 	pub to: U256,
// 	pub value: Vec<U256>,
// 	pub access_lists: Option<Vec<Vec<TestAccessListItem>>>,
// }

// // #[derive(Clone, Debug, Eq, PartialEq)]
// // pub struct TestData {
// // 	pub info: TestInfo,
// // 	pub env: TestEnv,
// // 	pub fork: Fork,
// // 	pub index: usize,
// // 	pub post: TestPost,
// // 	pub pre: BTreeMap<U256, TestPre>,
// // 	pub transaction: TestTransaction,
// // }

// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TestInfo {
// 	pub comment: String,
// 	#[serde(rename = "filling-rpc-server")]
// 	pub filling_rpc_server: String,
// 	#[serde(rename = "filling-tool-version")]
// 	pub filling_tool_version: String,
// 	pub generated_test_hash: String,
// 	pub lllcversion: String,
// 	pub solidity: String,
// 	pub source: String,
// 	pub source_hash: String,
// }

// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TestEnv {
// 	pub current_base_fee: U256,
// 	// pub current_beacon_root: U256,
// 	pub current_coinbase: U256,
// 	pub current_difficulty: U256,
// 	pub current_gas_limit: U256,
// 	pub current_number: U256,
// 	pub current_random: U256,
// 	pub current_timestamp: U256,
// 	// pub current_withdrawals_root: U256,
// 	// pub previous_hash: U256,
// }

// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// pub struct TestPost {
// 	pub hash: U256,
// 	pub indexes: TestPostStateIndexes,
// 	pub logs: U256,
// 	pub txbytes: HexBytes,
// 	pub expect_exception: Option<TestExpectException>,
// }

// #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Deserialize)]
// pub enum Fork {
// 	Berlin,
// 	Cancun,
// 	London,
// 	Merge,
// 	Shanghai,
// 	Byzantium,
// 	Constantinople,
// 	ConstantinopleFix,
// 	EIP150,
// 	EIP158,
// 	Frontier,
// 	Homestead,
// 	Istanbul,
// }

// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// pub enum TestExpectException {
// 	TR_TypeNotSupported,
// 	TR_IntrinsicGas,
// }

// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// pub struct TestPostStateIndexes {
// 	pub data: usize,
// 	pub gas: usize,
// 	pub value: usize,
// }

// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// pub struct TestPre {
// 	pub balance: U256,
// 	pub code: HexBytes,
// 	pub nonce: U256,
// 	pub storage: BTreeMap<U256, U256>,
// }

// #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TestAccessListItem {
// 	pub address: U256,
// 	pub storage_keys: Vec<U256>,
// }

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct TestTransaction {
// 	pub data: Vec<u8>,
// 	pub gas_limit: U256,
// 	pub gas_price: U256,
// 	pub nonce: U256,
// 	pub secret_key: U256,
// 	pub sender: U256,
// 	pub to: U256,
// 	pub value: U256,
// 	pub access_list: Vec<TestAccessListItem>,
// }

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct HexBytes(#[serde(deserialize_with = "deserialize_hex_bytes")] pub Vec<u8>);

fn deserialize_hex_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
	D: Deserializer<'de>,
{
	struct HexStrVisitor;

	impl<'de> Visitor<'de> for HexStrVisitor {
		type Value = Vec<u8>;

		fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "a hex encoded string")
		}

		fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
		where
			E: Error,
		{
			if &data[0..2] != "0x" {
				return Err(Error::custom("should start with 0x"));
			}

			FromHex::from_hex(&data[2..]).map_err(Error::custom)
		}

		fn visit_borrowed_str<E>(self, data: &'de str) -> Result<Self::Value, E>
		where
			E: Error,
		{
			if &data[0..2] != "0x" {
				return Err(Error::custom("should start with 0x"));
			}

			FromHex::from_hex(&data[2..]).map_err(Error::custom)
		}
	}

	deserializer.deserialize_str(HexStrVisitor)
}
