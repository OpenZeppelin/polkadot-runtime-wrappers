use models::Abstractions;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{parse_macro_input, Ident, Item, ItemStruct, ItemType, Type};

mod models;

#[proc_macro_attribute]
pub fn construct_openzeppelin_runtime(_: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as syn::ItemMod);
    let Some((_, items)) = input.content else {
        panic!("no content");
    };
    let mut inner = quote! {};
    let mut pallet_index = 0;

    for item in items {
        match item {
            Item::Struct(m) => {
                let abstraction = parse_abstraction(m, &mut pallet_index);
                inner.extend(abstraction);
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
    };
    TokenStream::from(expanded)
}

fn parse_abstraction(item: ItemStruct, index: &mut u32) -> proc_macro2::TokenStream {
    let is_pallet = item.attrs.iter().any(|f| {
        let Ok(path) = f.meta.require_path_only() else {
            return false;
        };
        let Ok(ident) = path.require_ident() else {
            return false;
        };
        ident == "abstraction"
    });
    if !is_pallet {
        panic!("`abstraction` attribute is missing");
    }

    let Ok(abstraction_name) = Abstractions::try_from(item.ident) else {
        panic!("unknown abstraction");
    };

    match abstraction_name {
        Abstractions::System => construct_system(index),
        Abstractions::Assets => construct_assets(index),
        Abstractions::Consensus => construct_consensus(index),
        Abstractions::Governance => construct_governance(index),
        Abstractions::XCM => construct_xcm(index),
        Abstractions::EVM => construct_evm(index),
    }
}

fn construct_xcm(index: &mut u32) -> proc_macro2::TokenStream {
    let mut res = quote! {};
    for (name, module) in openzeppelin_polkadot_wrappers::xcm::PALLET_NAMES {
        res.extend(construct_pallet(
            index,
            construct_ident(name),
            construct_ident(module),
        ));
    }
    res
}

fn construct_governance(index: &mut u32) -> proc_macro2::TokenStream {
    let mut res = quote! {};
    for (name, module) in openzeppelin_polkadot_wrappers::governance::PALLET_NAMES {
        res.extend(construct_pallet(
            index,
            construct_ident(name),
            construct_ident(module),
        ));
    }
    res
}

fn construct_consensus(index: &mut u32) -> proc_macro2::TokenStream {
    let mut res = quote! {};
    for (name, module) in openzeppelin_polkadot_wrappers::consensus::PALLET_NAMES {
        res.extend(construct_pallet(
            index,
            construct_ident(name),
            construct_ident(module),
        ));
    }
    res
}

fn construct_assets(index: &mut u32) -> proc_macro2::TokenStream {
    let mut res = quote! {};
    for (name, module) in openzeppelin_polkadot_wrappers::assets::PALLET_NAMES {
        res.extend(construct_pallet(
            index,
            construct_ident(name),
            construct_ident(module),
        ));
    }
    res
}

fn construct_system(index: &mut u32) -> proc_macro2::TokenStream {
    let mut res = quote! {};
    for (name, module) in openzeppelin_polkadot_wrappers::system::PALLET_NAMES {
        res.extend(construct_pallet(
            index,
            construct_ident(name),
            construct_ident(module),
        ));
    }
    res
}

fn construct_evm(index: &mut u32) -> proc_macro2::TokenStream {
    let mut res = quote! {};
    for (name, module) in openzeppelin_polkadot_wrappers::evm::PALLET_NAMES {
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
