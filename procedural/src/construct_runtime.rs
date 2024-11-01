use crate::models::ConstructAbstractions;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{parse_macro_input, Ident, Item, ItemStruct, ItemType, Type};

pub fn construct_openzeppelin_runtime(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as syn::ItemMod);
    let Some((_, items)) = input.content else {
        panic!("no content");
    };
    let mut inner = quote! {};
    let mut outer = quote! {};
    let mut pallet_index = 0;

    for item in items {
        match item {
            Item::Struct(m) => {
                let (abstraction, append) = parse_abstraction(m, &mut pallet_index);
                inner.extend(abstraction);
                outer.extend(append);
            }
            Item::Type(item) => {
                let pallet = parse_pallet(item, &mut pallet_index);
                inner.extend(pallet);
            }
            _ => (),
        }
    }

    let expanded = quote! {
        #[frame_support::runtime]
        mod runtime {
            #[runtime::runtime]
            #[runtime::derive(
                RuntimeCall,
                RuntimeEvent,
                RuntimeError,
                RuntimeOrigin,
                RuntimeFreezeReason,
                RuntimeHoldReason,
                RuntimeSlashReason,
                RuntimeLockId,
                RuntimeTask
            )]
            pub struct Runtime;

            #inner
        }

        #outer
    };
    TokenStream::from(expanded)
}

fn parse_abstraction(
    item: ItemStruct,
    index: &mut u32,
) -> (proc_macro2::TokenStream, Option<proc_macro2::TokenStream>) {
    let abstraction_name = ConstructAbstractions::try_from(item).expect("Wrong Struct");

    match abstraction_name {
        ConstructAbstractions::System => (
            construct_system(index),
            Some(quote! {
                cumulus_pallet_parachain_system::register_validate_block! {
                    Runtime = Runtime,
                    BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
                }
            }),
        ),
        ConstructAbstractions::Assets => (construct_assets(index), None),
        ConstructAbstractions::Consensus => (construct_consensus(index), None),
        ConstructAbstractions::Governance => (construct_governance(index), None),
        ConstructAbstractions::Xcm => (construct_xcm(index), None),
        ConstructAbstractions::Evm => (construct_evm(index), None),
    }
}

fn construct_xcm(index: &mut u32) -> proc_macro2::TokenStream {
    construct_abstraction(index, &openzeppelin_polkadot_wrappers::xcm::PALLET_NAMES)
}

fn construct_governance(index: &mut u32) -> proc_macro2::TokenStream {
    construct_abstraction(
        index,
        &openzeppelin_polkadot_wrappers::governance::PALLET_NAMES,
    )
}

fn construct_consensus(index: &mut u32) -> proc_macro2::TokenStream {
    construct_abstraction(
        index,
        &openzeppelin_polkadot_wrappers::consensus::PALLET_NAMES,
    )
}

fn construct_evm(index: &mut u32) -> proc_macro2::TokenStream {
    construct_abstraction(index, &openzeppelin_polkadot_wrappers::evm::PALLET_NAMES)
}

fn construct_assets(index: &mut u32) -> proc_macro2::TokenStream {
    construct_abstraction(index, &openzeppelin_polkadot_wrappers::assets::PALLET_NAMES)
}

fn construct_system(index: &mut u32) -> proc_macro2::TokenStream {
    construct_abstraction(index, &openzeppelin_polkadot_wrappers::system::PALLET_NAMES)
}

fn construct_abstraction(index: &mut u32, pallets: &[(&str, &str)]) -> proc_macro2::TokenStream {
    let mut res = quote! {};
    for (name, module) in pallets {
        res.extend(construct_pallet(
            index,
            construct_ident(name),
            construct_ident(module),
        ));
    }
    res
}

fn construct_pallet(index: &mut u32, name: Ident, ty: Ident) -> proc_macro2::TokenStream {
    let index_literal = Literal::u32_unsuffixed(*index);
    *index += 1;
    quote! {
        #[runtime::pallet_index(#index_literal)]
        pub type #name = #ty;
    }
}

fn construct_ident(name: &str) -> syn::Ident {
    Ident::new(name, Span::call_site())
}

fn parse_pallet(item: ItemType, index: &mut u32) -> proc_macro2::TokenStream {
    let is_pallet = item.attrs.iter().any(|f| {
        let Ok(path) = f.meta.require_path_only() else {
            return false;
        };
        let Ok(ident) = path.require_ident() else {
            return false;
        };
        ident == "pallet"
    });
    if !is_pallet {
        panic!("`pallet` attribute is missing");
    }
    let name = &item.ident;

    let ty = match *item.ty {
        Type::Path(path) => {
            let Some(ident) = path.path.get_ident() else {
                panic!("Malformed type, found: {:?}", path.path);
            };
            ident.clone()
        }
        _ => panic!("Malformed type, found {:?}", item.ty),
    };
    let index_literal = Literal::u32_unsuffixed(*index);
    *index += 1;
    quote! {
        #[runtime::pallet_index(#index_literal)]
        pub type #name = #ty;
    }
}
