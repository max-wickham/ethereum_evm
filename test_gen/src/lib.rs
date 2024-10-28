use official_test_types::types::TestStateMulti;
use proc_macro::TokenStream;
use quote::quote;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::{collections::HashMap, path::Path};
use syn::{parse_macro_input, Ident, LitStr};



#[proc_macro]
pub fn generate_official_tests(input: TokenStream) -> TokenStream {
    println!("Generating");
    let path_name = parse_macro_input!(input as LitStr);
    let path_name = path_name.value();
    let tests = generate_tests_from_path(&path_name);
    let expanded = quote! {
        #(#tests)*
    };
    TokenStream::from(expanded)
}


fn generate_tests_from_path(path: &str) -> Vec<proc_macro2::TokenStream> {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_file() {

                return generate_tests_from_file(path);
            } else if metadata.is_dir() {
                let mut tests = Vec::new();
                for entry in fs::read_dir(path).unwrap() {
                    tests.append(&mut generate_tests_from_path(
                        entry.unwrap().path().to_str().unwrap(),
                    ));
                }
                return tests;
            }
        }
        Err(_) => println!("Failed to get metadata for {}", path),
    }
    return vec![];
}

fn generate_tests_from_file(file_path: &str) -> Vec<proc_macro2::TokenStream> {
    let mut test_functions = vec![];
    let parsed_tests: BTreeMap<String, TestStateMulti> =
        serde_json::from_reader(BufReader::new(File::open(file_path).unwrap())).unwrap();
    let clean_file_path = clean_path(file_path);
    for (index, test) in parsed_tests.iter().nth(0).unwrap().1.tests().iter().enumerate() {
        if index != 1 {
            continue;
        }
        let test_name = Ident::new(
            &(clean_file_path.clone() + &index.to_string()),
            proc_macro2::Span::call_site(),
        );
        let test_str = serde_json::to_string(test).unwrap();
        let test_str = LitStr::new(&test_str, proc_macro2::Span::call_site());
        test_functions.push(quote! {
            #[test]
            fn #test_name() {
                let test_string = #test_str;
                // let test_string: str = #test_str;
                let test: TestState = serde_json::from_str(test_string).unwrap();
                run_test(&test, true);
            }
        });
    }
    test_functions
}

fn clean_path(path: &str) -> String {
    path.replace("/", "_")
        .replace(".", "")
        .replace("-", "_sub_")
        .replace("+", "_add_")
}


// #[derive(Serialize, Deserialize, Debug)]
// struct BasicTestObject {
//     name: String,
//     code: String,
//     result_address: String,
//     result_value: String,
// }

// #[proc_macro]
// pub fn generate_tests(input: TokenStream) -> TokenStream {
//     let folder_name_lit = parse_macro_input!(input as LitStr);
//     let folder_name = folder_name_lit.value();
//     let mut tests = Vec::new();
//     for entry in fs::read_dir(folder_name).unwrap() {
//         let entry = entry.unwrap();
//         let file_name = String::from(entry.path().to_str().unwrap());
//         // Read the content of the specified JSON file
//         let input_str = match std::fs::read_to_string(&file_name) {
//             Ok(content) => content,
//             Err(err) => {
//                 // Handle file reading error here
//                 panic!("Failed to read file {}: {:?}", file_name, err);
//             }
//         };

//         let input: Vec<BasicTestObject> = match serde_json::from_str(&input_str) {
//             Ok(obj) => obj,
//             Err(err) => {
//                 // Handle parsing error here
//                 panic!("Failed to parse input from file {}: {:?}", file_name, err);
//             }
//         };

//         for (_, test) in input.into_iter().enumerate() {
//             let test_name = Ident::new(test.name.as_str(), proc_macro2::Span::call_site());
//             let code = test.code.as_str();
//             let result_address = test.result_address;
//             let result_value = test.result_value;
//             tests.push(quote! {
//                 #[test]
//                 fn #test_name() {

//                     let code = assemble(String::from(#code));
//                     let contract = Contract {
//                         balance: U256::from(10 as u64),
//                         code_size: U256::from(code.len() as u64),
//                         code_hash: util::keccak256(&code),
//                         code: code,
//                         nonce: U256::from(0 as u64),
//                         storage: BTreeMap::new(),
//                         is_deleted: false,
//                         is_cold: false,
//                         hot_keys: HashSet::new(),
//                     };
//                     let mut contracts: BTreeMap<U256, Contract> = BTreeMap::new();
//                     contracts.insert(U256::from(1 as u64), contract);
//                     let mut mock_runtime = MockRuntime {
//                         block_hashes: BTreeMap::new(),
//                         block_number: U256::from(0 as u64),
//                         block_coinbase: U256::from(0 as u64),
//                         block_timestamp: U256::from(0 as u64),
//                         block_difficulty: U256::from(0 as u64),
//                         block_randomness: U256::from(0 as u64),
//                         block_gas_limit: U256::from(100000 as u64),
//                         block_base_fee_per_gas: U256::from(1 as u64),
//                         chain_id: U256::from(0 as u64),
//                         contracts: contracts,
//                     };

//                     let mut context = EVMContext::create_sub_context(
//                         U256::from(1 as u64),
//                         Message {
//                             caller: U256::from(0 as u64),
//                             value: U256::from(10 as u64),
//                             data: Memory::new(),
//                         },
//                         1000,
//                         mock_runtime.contracts[&U256::from(1 as u64)].code.clone(),
//                         Transaction {
//                             origin: U256::from(0 as u64),
//                             gas_price: U256::from(1 as u64),
//                         },
//                         U256::from(1 as u64),
//                     );
//                     let result = context.execute(&mut mock_runtime);
//                     assert_eq!(result, true);
//                     assert_eq!(*mock_runtime.storage(U256::from(1 as u64)).get(&(U256::from_str(#result_address).unwrap())).unwrap(), U256::from_str(#result_value).unwrap());
//                 }
//             });
//         }
//     }
//     // Combine all generated tests into a single TokenStream
//     let expanded = quote! {
//         #(#tests)*
//     };

//     TokenStream::from(expanded)
// }

// #[proc_macro]
// pub fn generate_official_tests_from_folder(input: TokenStream) -> TokenStream {
//     let folder_name_lit = parse_macro_input!(input as LitStr);
//     let folder_name = folder_name_lit.value();
//     let mut tests = Vec::new();
//     for entry in fs::read_dir(folder_name).unwrap() {
//         let entry = entry.unwrap().path();
//         let file_name = String::from(entry.to_str().unwrap());
//         let test_name = entry.file_stem().unwrap().to_str().unwrap();

//         // println!("File: {}", file_name);
//         // Read the content of the specified JSON file
//         let input_str = match std::fs::read_to_string(&file_name) {
//             Ok(content) => content,
//             Err(err) => {
//                 // Handle file reading error here
//                 panic!("Failed to read file {}: {:?}", file_name, err);
//             }
//         };

//         let json_data: HashMap<String, Value> = serde_json::from_str(&input_str).unwrap();
//         let mut num_tests: usize = 0;
//         match json_data[test_name].clone() {
//             Value::Object(obj) => match obj["post"].clone() {
//                 Value::Object(post) => match post["Berlin"].clone() {
//                     Value::Object(berlin) => {
//                         println!("Berlin: {:?}", berlin);
//                     }

//                     Value::Array(arr) => {
//                         num_tests = arr.len();
//                     }
//                     _ => {
//                         panic!("Expected a JSON object at the root");
//                     }
//                 },
//                 _ => {
//                     panic!("Expected a JSON object at the root");
//                 }
//             },
//             _ => {
//                 panic!("Expected a JSON object at the root");
//             }
//         }

//         for i in 0..num_tests {
//             let test_name = Ident::new(
//                 format!(
//                     "run_test_{}_{}_{}",
//                     folder_name_lit.value().replace("/", "_").replace(".", ""),
//                     test_name.replace("+", "pos").replace("-", "min"),
//                     i
//                 )
//                 .as_str(),
//                 proc_macro2::Span::call_site(),
//             );
//             tests.push(quote! {
//                 #[test]
//                 fn #test_name() {
//                     let filename = #file_name;
//                     run_test_file(filename.to_string(), true, #i);
//                 }
//             });
//         }
//     }
//     // Combine all generated tests into a single TokenStream
//     let expanded = quote! {
//         #(#tests)*
//     };

//     TokenStream::from(expanded)
//     // TokenStream::from(quote!{})
// }

// #[proc_macro]
// pub fn generate_official_tests_from_file(input: TokenStream) -> TokenStream {
//     let file_name_lit = parse_macro_input!(input as LitStr);
//     let file_name = file_name_lit.value();
//     let mut tests = Vec::new();
//     // let entry = entry.unwrap().path();
//     // let test_name = entry.file_stem().unwrap().to_str().unwrap();

//     // println!("File: {}", file_name);
//     // Read the content of the specified JSON file
//     let input_str = match std::fs::read_to_string(&file_name) {
//         Ok(content) => content,
//         Err(err) => {
//             // Handle file reading error here
//             panic!("Failed to read file {}: {:?}", file_name, err);
//         }
//     };

//     let json_data: HashMap<String, Value> = serde_json::from_str(&input_str).unwrap();
//     let mut num_tests: usize = 0;
//     let test_name = Path::new(&file_name)
//         .file_stem()
//         .and_then(|s| s.to_str())
//         .unwrap_or("invalid_file_name");
//     match json_data[test_name].clone() {
//         Value::Object(obj) => match obj["post"].clone() {
//             Value::Object(post) => match post["Berlin"].clone() {
//                 Value::Object(berlin) => {
//                     println!("Berlin: {:?}", berlin);
//                 }

//                 Value::Array(arr) => {
//                     num_tests = arr.len();
//                 }
//                 _ => {
//                     panic!("Expected a JSON object at the root");
//                 }
//             },
//             _ => {
//                 panic!("Expected a JSON object at the root");
//             }
//         },
//         _ => {
//             panic!("Expected a JSON object at the root");
//         }
//     }
//     for i in 0..num_tests {
//         // for i in 43..44 {
//         let test_name = Ident::new(
//             format!("run_test_{}", i).as_str(),
//             proc_macro2::Span::call_site(),
//         );
//         tests.push(quote! {
//             #[test]
//             fn #test_name() {
//                 let filename = #file_name;
//                 run_test_file(filename.to_string(), true, #i  as usize);
//             }
//         });
//     }

//     // Combine all generated tests into a single TokenStream
//     let expanded = quote! {
//         #(#tests)*
//     };

//     TokenStream::from(expanded)
//     // TokenStream::from(quote!{})
// }
