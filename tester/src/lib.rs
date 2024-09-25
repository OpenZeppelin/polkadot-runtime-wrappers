extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn test_macro_output(input: TokenStream) -> TokenStream {
    // Simply log or print the input tokens
    for token in input.clone() {
        println!("{:?}", token);
    }

    // Return the original tokens unmodified
    input
}