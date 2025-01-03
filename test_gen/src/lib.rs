use official_test_types::types::TestStateMulti;
use proc_macro::TokenStream;
use quote::quote;
use serde::{ Deserialize, Deserializer, Serialize };
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::{ self, File };
use std::io::BufReader;
use std::vec;
use std::{ collections::HashMap, path::Path };
use syn::{ parse_macro_input, Ident, LitStr };

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
                match generate_tests_from_file(path) {
                    Ok(tests) => {
                        return tests;
                    }
                    Err(_) => {
                        return vec![];
                    }
                };
            } else if metadata.is_dir() {
                let mut tests = Vec::new();
                for entry in fs::read_dir(path).unwrap() {
                    tests.append(
                        &mut generate_tests_from_path(entry.unwrap().path().to_str().unwrap())
                    );
                }
                return tests;
            }
        }
        Err(_) => println!("Failed to get metadata for {}", path),
    }
    return vec![];
}

fn generate_tests_from_file(
    file_path: &str
) -> Result<Vec<proc_macro2::TokenStream>, serde_json::Error> {
    // If problem with passing return null
    let mut test_functions = vec![];
    if !file_path.ends_with(".json") {
        return Ok(test_functions);
    }
    let parsed_tests: BTreeMap<String, TestStateMulti> = serde_json::from_reader(
        BufReader::new(File::open(file_path).unwrap())
    )?;
    let clean_file_path = clean_path(file_path);
    for (index, test) in parsed_tests.iter().nth(0).unwrap().1.tests().iter().enumerate() {
        let test_name = Ident::new(
            &(clean_file_path.clone() + &index.to_string()),
            proc_macro2::Span::call_site()
        );
        let test_str = serde_json::to_string(test)?;
        let test_str = LitStr::new(&test_str, proc_macro2::Span::call_site());
        test_functions.push(
            quote! {
            #[test]
            fn #test_name() {
                let test_string = #test_str;
                let test: TestState = serde_json::from_str(test_string).unwrap();

                // match test {
                //     Ok(test) => run_test(&test, true),
                //     Err(_) => {}
                // }
                run_test(&test, true);
            }
        }
        );
    }
    Ok(test_functions)
}

fn clean_path(path: &str) -> String {
    path.replace("/", "_")
        .replace(".", "")
        .replace("-", "_sub_")
        .replace("+", "_add_")
        .replace("^", "_pow_")
}
