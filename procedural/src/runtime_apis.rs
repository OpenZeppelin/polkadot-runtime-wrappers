use crate::{
    apis::{
        self, construct_benchmarking_api, fetch_ident, AbstractionState, AssetAPIFields,
        ConsensusAPIFields, EVMAPIFields, SystemAPIFields,
    },
    models::Abstractions,
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
                // TODO: check abstraction attribute
                abstractions.push(m)
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

    let benchmarks = construct_benchmarking_api(state);

    let expanded = quote! {
        use sp_api::impl_runtime_apis;
        use sp_consensus_aura::sr25519::AuthorityId as AuraId;
        use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
        use frame_support::{
            genesis_builder_helper::{build_state, get_preset},
            weights::Weight,
        };
        use sp_runtime::{
            traits::Block as BlockT,
            transaction_validity::{TransactionSource, TransactionValidity},
            ApplyExtrinsicResult,
        };

        use sp_std::prelude::Vec;
        use sp_version::RuntimeVersion;

        impl_runtime_apis! {
            #inner

            #benchmarks
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
    let abstraction = Abstractions::try_from(item.ident).expect("Wrong Abstraction Struct");
    let (_, content) = item
        .content
        .expect("`mod assets` does not have any content.");

    match abstraction {
        Abstractions::EVM => {
            let EVMAPIFields {
                call,
                executive,
                ethereum,
            } = EVMAPIFields::try_from(content.as_slice()).expect("Error while parsing EVM config");

            apis::evm_apis(runtime, block, &call, &executive, &ethereum)
        }
        Abstractions::Assets => {
            state.assets = true;

            let AssetAPIFields {
                transaction_payment,
                balance,
                call,
            } = AssetAPIFields::try_from(content.as_slice())
                .expect("Error while parsing assets config");

            apis::assets_apis(runtime, block, &transaction_payment, &balance, &call)
        }
        Abstractions::Consensus => {
            state.assets = true;

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
        Abstractions::System => {
            let SystemAPIFields {
                executive,
                system,
                parachain_system,
                version,
                account_id,
                nonce,
                genesis,
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
            )
        }
        Abstractions::XCM => {
            state.xcm = true;
            quote! {}
        }
        _ => quote! {},
    }
}
