extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn generate_map(_input: TokenStream) -> TokenStream {

    let input_string = include_str!("../../data/input.txt").split("\n\n");
    
    let mut code = quote! {};
    for parts in input_string.skip(1){

        let mut lines = parts.lines();

        let name = lines.next().unwrap().replace(" ", "_").replace(":", "").replace("-", "_");

        let branches = lines.map(|line| {
            let values: Vec<usize> = line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
            let dest_start = values[0];
            let src_start = values[1];
            let src_end = values[1] + values[2];
            quote!{
                #src_start..=#src_end => #dest_start + i - #src_start,
            }
        });

        

        code = quote! {

            fn #name(i: usize) -> usize {
                /*match i {
                    #(#branches)*
                    _ => return i,
                }*/
                i
            }
        };
    }

    code.into()
}
