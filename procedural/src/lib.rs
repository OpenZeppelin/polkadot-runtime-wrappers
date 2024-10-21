use proc_macro::TokenStream;

mod apis;
mod construct_runtime;
mod models;
mod runtime_apis;

#[proc_macro_attribute]
pub fn openzeppelin_construct_runtime(_: TokenStream, tokens: TokenStream) -> TokenStream {
    construct_runtime::construct_openzeppelin_runtime(tokens)
}

#[proc_macro_attribute]
pub fn openzeppelin_runtime_apis(_: TokenStream, input: TokenStream) -> TokenStream {
    runtime_apis::impl_openzeppelin_runtime_apis(input)
}
