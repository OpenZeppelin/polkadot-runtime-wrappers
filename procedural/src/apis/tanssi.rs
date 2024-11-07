use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Item};

use super::fetch_ident;

#[derive(Debug)]
pub struct TanssiAPIFields {
    pub session_keys: Ident,
}

impl TryFrom<&[Item]> for TanssiAPIFields {
    type Error = &'static str;

    fn try_from(value: &[Item]) -> Result<Self, Self::Error> {
        let mut session_keys = None;

        for item in value {
            match item {
                Item::Type(ty) => {
                    let typ = ty.ty.clone();
                    if ty.ident == "SessionKeys" {
                        session_keys = Some(fetch_ident(&typ))
                    }
                }
                _ => (),
            }
        }
        let session_keys = session_keys.ok_or("type `SessionKeys` not specified, but required")?;

        return Ok(TanssiAPIFields { session_keys });
    }
}

pub fn tanssi_apis(
    runtime: &Ident,
    block: &Ident,
    session_keys: &Ident,
) -> TokenStream {
    let mut res = quote! {};
    res.extend(quote! {
        impl sp_session::SessionKeys<#block> for #runtime {
            fn generate_session_keys(seed: Option<sp_std::prelude::Vec<u8>>) -> sp_std::prelude::Vec<u8> {
                #session_keys::generate(seed)
            }

            fn decode_session_keys(encoded: sp_std::prelude::Vec<u8>) -> Option<sp_std::prelude::Vec<(sp_std::prelude::Vec<u8>, sp_core::crypto::KeyTypeId)>> {
                #session_keys::decode_into_raw_public_keys(&encoded)
            }
        }
    });

    res
}