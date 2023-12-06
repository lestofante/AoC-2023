extern crate proc_macro;

use std::{fs, env, path::PathBuf};

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn generate_map(input: TokenStream) -> TokenStream {
    let binding = parse_macro_input!(input as syn::LitStr).value();

    let srcdir = PathBuf::from(file!());
    let srcdir = srcdir.parent().unwrap();
    let binding = srcdir.join(binding);

    let binding = fs::read_to_string(&binding).expect(&format!("file erro {binding:?} {}", file!()));

    let input_string = binding.split("\n\n");
    
    let mut all_code = quote! {};
    for parts in input_string.skip(1){

        let mut lines = parts.lines();

        let name = Ident::new(&lines.next().unwrap().trim().replace(" ", "_").replace(":", "").replace("-", "_"), proc_macro2::Span::call_site());

        let branches = lines.map(|line| {
            let values: Vec<usize> = line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
            let dest_start = values[0];
            let src_start = values[1];
            let src_end = values[1] + values[2] - 1;
            quote!{
                #src_start..=#src_end => #dest_start + i - #src_start,
            }
        });

        let code = quote!{
            fn #name(i: usize) -> usize {
                match i {
                    #(#branches)*
                    _ => return i,
                }
            }
        };

        all_code.extend(code);
    }

    all_code.into()
}
