use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn assets_apis(
    runtime: &Ident,
    block: &Ident,
    transaction_payment: &Ident,
    balance: &Ident,
    call: &Ident,
) -> TokenStream {
    quote! {
        impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<#block, #balance>
            for #runtime
        {
            fn query_info(
                uxt: <#block as BlockT>::Extrinsic,
                len: u32,
            ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<#balance> {
                #transaction_payment::query_info(uxt, len)
            }
            fn query_fee_details(
                uxt: <#block as BlockT>::Extrinsic,
                len: u32,
            ) -> pallet_transaction_payment::FeeDetails<#balance> {
                #transaction_payment::query_fee_details(uxt, len)
            }
            fn query_weight_to_fee(weight: Weight) -> #balance {
                #transaction_payment::weight_to_fee(weight)
            }
            fn query_length_to_fee(length: u32) -> #balance {
                #transaction_payment::length_to_fee(length)
            }
        }

        impl
            pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<
                #block,
                #balance,
                #call,
            > for #runtime
        {
            fn query_call_info(
                call: #call,
                len: u32,
            ) -> pallet_transaction_payment::RuntimeDispatchInfo<#balance> {
                #transaction_payment::query_call_info(call, len)
            }
            fn query_call_fee_details(
                call: #call,
                len: u32,
            ) -> pallet_transaction_payment::FeeDetails<#balance> {
                #transaction_payment::query_call_fee_details(call, len)
            }
            fn query_weight_to_fee(weight: Weight) -> #balance {
                #transaction_payment::weight_to_fee(weight)
            }
            fn query_length_to_fee(length: u32) -> #balance {
                #transaction_payment::length_to_fee(length)
            }
        }
    }
}
