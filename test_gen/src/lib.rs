use std::{collections::HashMap, path::Path};

use proc_macro::TokenStream;
use quote::quote;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::fs;
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
    let folder_name_lit = parse_macro_input!(input as LitStr);
    let folder_name = folder_name_lit.value();
    let mut tests = Vec::new();
    for entry in fs::read_dir(folder_name).unwrap() {
        let entry = entry.unwrap();
        let file_name = String::from(entry.path().to_str().unwrap());
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
    }
    // Combine all generated tests into a single TokenStream
    let expanded = quote! {
        #(#tests)*
    };

    TokenStream::from(expanded)
}


#[proc_macro]
pub fn generate_official_tests_from_folder(input: TokenStream) -> TokenStream {
    let folder_name_lit = parse_macro_input!(input as LitStr);
    let folder_name = folder_name_lit.value();
    let mut tests = Vec::new();
    for entry in fs::read_dir(folder_name).unwrap() {
        let entry = entry.unwrap().path();
        let file_name = String::from(entry.to_str().unwrap());
        let test_name = entry.file_stem().unwrap().to_str().unwrap();

        // println!("File: {}", file_name);
        // Read the content of the specified JSON file
        let input_str = match std::fs::read_to_string(&file_name) {
            Ok(content) => content,
            Err(err) => {
                // Handle file reading error here
                panic!("Failed to read file {}: {:?}", file_name, err);
            }
        };

        let json_data: HashMap<String, Value> = serde_json::from_str(&input_str).unwrap();
        let mut num_tests: usize = 0;
        match json_data[test_name].clone() {
            Value::Object(obj) => match obj["post"].clone() {
                Value::Object(post) => match post["Berlin"].clone() {
                    Value::Object(berlin) => {
                        println!("Berlin: {:?}", berlin);
                    }

                    Value::Array(arr) => {
                        num_tests = arr.len();
                    }
                    _ => {
                        panic!("Expected a JSON object at the root");
                    }
                },
                _ => {
                    panic!("Expected a JSON object at the root");
                }
            },
            _ => {
                panic!("Expected a JSON object at the root");
            }
        }

        for i in 0..num_tests {
            let test_name = Ident::new(
                format!("run_test_{}_{}",test_name, i).as_str(),
                proc_macro2::Span::call_site(),
            );
            tests.push(quote! {
                #[test]
                fn #test_name() {
                    let filename = #file_name;
                    run_test_file(filename.to_string(), false, #i);
                }
            });
        }
    }
    // Combine all generated tests into a single TokenStream
    let expanded = quote! {
        #(#tests)*
    };

    TokenStream::from(expanded)
    // TokenStream::from(quote!{})
}


#[proc_macro]
pub fn generate_official_tests_from_file(input: TokenStream) -> TokenStream {
    let file_name_lit = parse_macro_input!(input as LitStr);
    let file_name = file_name_lit.value();
    let mut tests = Vec::new();
        // let entry = entry.unwrap().path();
        // let test_name = entry.file_stem().unwrap().to_str().unwrap();

        // println!("File: {}", file_name);
        // Read the content of the specified JSON file
        let input_str = match std::fs::read_to_string(&file_name) {
            Ok(content) => content,
            Err(err) => {
                // Handle file reading error here
                panic!("Failed to read file {}: {:?}", file_name, err);
            }
        };

        let json_data: HashMap<String, Value> = serde_json::from_str(&input_str).unwrap();
        let mut num_tests: usize = 0;
        let test_name = Path::new(&file_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("invalid_file_name");
        match json_data[test_name].clone() {
            Value::Object(obj) => match obj["post"].clone() {
                Value::Object(post) => match post["Berlin"].clone() {
                    Value::Object(berlin) => {
                        println!("Berlin: {:?}", berlin);
                    }

                    Value::Array(arr) => {
                        num_tests = arr.len();
                    }
                    _ => {
                        panic!("Expected a JSON object at the root");
                    }
                },
                _ => {
                    panic!("Expected a JSON object at the root");
                }
            },
            _ => {
                panic!("Expected a JSON object at the root");
            }
        }

        for i in 0..num_tests {
            let test_name = Ident::new(
                format!("run_test_{}", i).as_str(),
                proc_macro2::Span::call_site(),
            );
            tests.push(quote! {
                #[test]
                fn #test_name() {
                    let filename = #file_name;
                    run_test_file(filename.to_string(), true, #i);
                }
            });
        }

    // Combine all generated tests into a single TokenStream
    let expanded = quote! {
        #(#tests)*
    };

    TokenStream::from(expanded)
    // TokenStream::from(quote!{})
}
