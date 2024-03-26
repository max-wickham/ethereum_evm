extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Item, ItemStatic};
#[proc_macro]
pub fn opcode_map(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a Rust syntax tree
    let input = parse_macro_input!(input as syn::File);

    // Extract constant definitions from the input
    let mut opcode_names = Vec::new();
    let mut opcode_values = Vec::new();

    for item in input.items.clone() {
        if let Item::Const(const_item) = item {
            let byte = match *const_item.expr {
                syn::Expr::Lit(lit) => {
                    match lit.lit {
                        syn::Lit::Int(byte) => {println!("{:?}", byte.to_string());
                            byte}
                        _ => {
                            panic!("Expected byte literal");
                        }
                    }
                }
                _ => {
                    panic!("Expected integer literal");
                }
            };
            opcode_names.push(const_item.ident.to_string());
            opcode_values.push(byte);
        }
    }

    // Generate the PHF map
    let map_tokens = quote! {
        phf::phf_map! {
            #(
                #opcode_values => #opcode_names,
            )*
        }
    };

    // Generate the final output
    let expanded = quote! {
        #input

        lazy_static! {
            pub static ref OPCODE_MAP: phf::Map<u8, &'static str> = #map_tokens;
        }
    };

    println!("{:?}", expanded.to_string());

    // Convert the generated tokens back into a TokenStream
    TokenStream::from(expanded)
}
