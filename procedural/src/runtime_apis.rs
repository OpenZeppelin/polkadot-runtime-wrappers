use crate::{
    apis::{
        self, fetch_ident, AbstractionState, AssetAPIFields, BenchmarkAPIFields,
        ConsensusAPIFields, EVMAPIFields, SystemAPIFields, TanssiAPIFields,
    },
    models::APIAbstractions,
};
use core::panic;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, Item, ItemMod};

pub fn impl_openzeppelin_runtime_apis(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemMod);
    let Some((_, items)) = input.content else {
        panic!("no content");
    };

    let mut inner = quote! {};

    let mut abstractions = vec![];

    let mut runtime: Option<Ident> = None;
    let mut block: Option<Ident> = None;

    for item in items {
        match item {
            Item::Type(ty) => {
                if ty.ident == "Runtime" {
                    runtime = Some(fetch_ident(&ty.ty));
                } else if ty.ident == "Block" {
                    block = Some(fetch_ident(&ty.ty));
                }
            }
            Item::Mod(m) => {
                let is_abstraction = m.attrs.iter().any(|f| {
                    let Ok(path) = f.meta.require_path_only() else {
                        return false;
                    };
                    let Ok(ident) = path.require_ident() else {
                        return false;
                    };
                    ident == "abstraction"
                });
                if is_abstraction {
                    abstractions.push(m)
                }
            }
            Item::Impl(im) => {
                inner.extend(im.to_token_stream());
            }
            _ => (),
        }
    }

    let runtime = runtime.expect(
        "Runtime is missing. Please, add `type Runtime = /* Reference to generated runtime */` to the root of the module",
    );
    let block = block.expect(
        "Block is missing. Please, add `type Block = /* Reference to generated runtime */` to the root of the module",
    );

    let mut state = AbstractionState::default();

    for abstraction in abstractions {
        inner.extend(construct_abstraction(
            abstraction,
            &mut state,
            &runtime,
            &block,
        ));
    }

    if let AbstractionState {
        benchmark_fields: Some(fields),
        consensus,
    } = state
    {
        inner.extend(apis::construct_benchmarking_api(
            consensus, &runtime, fields,
        ));
    }

    let expanded = quote! {
        sp_api::impl_runtime_apis! {
            #inner
        }
    };

    TokenStream::from(expanded)
}

fn construct_abstraction(
    item: ItemMod,
    state: &mut AbstractionState,
    runtime: &Ident,
    block: &Ident,
) -> proc_macro2::TokenStream {
    let abstraction = APIAbstractions::try_from(item.ident).expect("Wrong Abstraction Struct");
    let (_, content) = item
        .content
        .expect("`mod assets` does not have any content.");

    match abstraction {
        APIAbstractions::Evm => {
            let EVMAPIFields {
                call,
                executive,
                ethereum,
            } = EVMAPIFields::try_from(content.as_slice()).expect("Error while parsing EVM config");

            apis::evm_apis(runtime, block, &call, &executive, &ethereum)
        }
        APIAbstractions::Assets => {
            let AssetAPIFields {
                transaction_payment,
                balance,
                call,
            } = AssetAPIFields::try_from(content.as_slice())
                .expect("Error while parsing assets config");

            apis::assets_apis(runtime, block, &transaction_payment, &balance, &call)
        }
        APIAbstractions::Consensus => {
            state.consensus = true;
            #[cfg(not(feature = "async-backing"))]
            {
                let ConsensusAPIFields { session_keys, aura } =
                    ConsensusAPIFields::try_from(content.as_slice())
                        .expect("Error while parsing Consensus config");
                apis::consensus_apis(runtime, block, &session_keys, &aura)
            }
            #[cfg(feature = "async-backing")]
            {
                let ConsensusAPIFields {
                    session_keys,
                    slot_duration,
                    consensus_hook,
                } = ConsensusAPIFields::try_from(content.as_slice())
                    .expect("Error while parsing Consensus config");
                apis::consensus_apis(
                    runtime,
                    block,
                    &session_keys,
                    &slot_duration,
                    &consensus_hook,
                )
            }
        }
        APIAbstractions::System => {
            let SystemAPIFields {
                executive,
                system,
                parachain_system,
                version,
                account_id,
                nonce,
                genesis,
                runtime_block_weights,
            } = SystemAPIFields::try_from(content.as_slice())
                .expect("Error while parsing system config");

            apis::system_apis(
                runtime,
                block,
                &executive,
                &system,
                &parachain_system,
                &version,
                &account_id,
                &nonce,
                &genesis,
                &runtime_block_weights,
            )
        }
        APIAbstractions::Benchmarks => {
            let api_fields = BenchmarkAPIFields::try_from(content.as_slice())
                .expect("Error while parsing benchmarking config");

            state.benchmark_fields = Some(api_fields);
            quote! {}
        }
        APIAbstractions::Tanssi => {
            let TanssiAPIFields { session_keys } = TanssiAPIFields::try_from(content.as_slice())
                .expect("Error while parsing Tanssi config");
            apis::tanssi_apis(runtime, block, &session_keys)
        }
    }
}
