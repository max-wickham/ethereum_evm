use proc_macro::TokenStream;
use quote::quote;
use serde::{Deserialize, Deserializer, Serialize};
use syn::{parse_macro_input, Ident, LitStr};

#[derive(Serialize, Deserialize, Debug)]
struct BasicTestObject {
    name: String,
    code: String,
    result_address: String,
    result_value: String,
}

#[proc_macro]
pub fn generate_tests(input: TokenStream) -> TokenStream {
    let file_name_lit = parse_macro_input!(input as LitStr);
    let file_name = file_name_lit.value();

    // Read the content of the specified JSON file
    let input_str = match std::fs::read_to_string(&file_name) {
        Ok(content) => content,
        Err(err) => {
            // Handle file reading error here
            panic!("Failed to read file {}: {:?}", file_name, err);
        }
    };

    let input: Vec<BasicTestObject> = match serde_json::from_str(&input_str) {
        Ok(obj) => obj,
        Err(err) => {
            // Handle parsing error here
            panic!("Failed to parse input from file {}: {:?}", file_name, err);
        }
    };

    let mut tests = Vec::new();
    for (_, test) in input.into_iter().enumerate() {
        let test_name = Ident::new(test.name.as_str(), proc_macro2::Span::call_site());
        let code = test.code.as_str();
        let result_address = test.result_address;
        let result_value = test.result_value;
        tests.push(quote! {
                #[test]
                fn #test_name() {

                    let code = assemble(String::from(#code));
                    let contract = Contract {
                        balance: U256::from(10 as u64),
                        code_size: U256::from(code.len() as u64),
                        code_hash: util::keccak256(&code),
                        code: code,
                        nonce: U256::from(0 as u64),
                        storage: BTreeMap::new(),
                        is_deleted: false,
                        is_cold: false,
                        hot_keys: HashSet::new(),
                    };
                    let mut contracts: BTreeMap<U256, Contract> = BTreeMap::new();
                    contracts.insert(U256::from(1 as u64), contract);
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
                    assert_eq!(*mock_runtime.storage(U256::from(1 as u64)).get(&(U256::from_str(#result_address).unwrap())).unwrap(), U256::from_str(#result_value).unwrap());
                }
            });
    }

    // Combine all generated tests into a single TokenStream
    let expanded = quote! {
        #(#tests)*
    };

    TokenStream::from(expanded)
}
