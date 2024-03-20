mod mock_runtime;
use std::{collections::HashMap, str::FromStr};
use ethereum_evm::{assembler::assemble, evm::{EVMContext, Message, Transaction}, runtime::Runtime, state::memory::Memory, util};
use ethnum::U256;
use mock_runtime::{Contract, MockRuntime};
use test_gen::generate_tests;

// const a: U256 = U256::from_str("avav").unwrap();
generate_tests!("./tests/basic_test.json");

// #[test]
// fn basic_test() {
//     let code1 = assemble(String::from("push_32 5 push_32 10 add push_32 10 sstore stop"));
//     println!("Code: {:?}", code1);
//     let mut contract1 = Contract {
//         balance: U256::from(10 as u64),
//         code_size: U256::from(code1.len() as u64),
//         code_hash: util::keccak256(&code1),
//         code: code1,
//         nonce: U256::from(0 as u64),
//         storage: HashMap::new(),
//         is_deleted: false,
//         is_cold: false,
//     };
//     let mut contracts: HashMap<U256, Contract> = HashMap::new();
//     contracts.insert(U256::from(1 as u64), contract1);
//     let mut mock_runtime = MockRuntime {
//         block_hashes: HashMap::new(),
//         block_number: U256::from(0 as u64),
//         block_coinbase: U256::from(0 as u64),
//         block_timestamp: U256::from(0 as u64),
//         block_difficulty: U256::from(0 as u64),
//         block_randomness: U256::from(0 as u64),
//         block_gas_limit: U256::from(100000 as u64),
//         block_base_fee_per_gas: U256::from(1 as u64),
//         chain_id: U256::from(0 as u64),
//         contracts: contracts,
//     };

//     let mut context = EVMContext::create_sub_context(
//         U256::from(1 as u64),
//         Message {
//             caller: U256::from(0 as u64),
//             value: U256::from(10 as u64),
//             data: Memory::new(),
//         },
//         1000,
//         mock_runtime.contracts[&U256::from(1 as u64)].code.clone(),
//         Transaction {
//             origin: U256::from(0 as u64),
//             gas_price: U256::from(1 as u64),
//         },
//         U256::from(1 as u64),
//     );
//     let result = context.execute(&mut mock_runtime);
//     assert_eq!(result, true);
//     assert_eq!(*mock_runtime.storage(U256::from(1 as u64)).get(&U256::from(10 as u64)).unwrap(), U256::from(15 as u64));

// }
