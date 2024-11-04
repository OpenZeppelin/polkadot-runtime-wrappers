use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Item};

use super::fetch_ident;

#[derive(Debug)]
pub struct ConsensusAPIFields {
    pub session_keys: Ident,
    #[cfg(not(feature = "async-backing"))]
    pub aura: Ident,
    #[cfg(feature = "async-backing")]
    pub slot_duration: Ident,
    #[cfg(feature = "async-backing")]
    pub consensus_hook: Ident,
}

impl TryFrom<&[Item]> for ConsensusAPIFields {
    type Error = &'static str;

    fn try_from(value: &[Item]) -> Result<Self, Self::Error> {
        let mut session_keys = None;

        #[cfg(not(feature = "async-backing"))]
        let mut aura = None;
        #[cfg(feature = "async-backing")]
        let mut slot_duration = None;
        #[cfg(feature = "async-backing")]
        let mut consensus_hook = None;

        for item in value {
            if let Item::Type(ty) = item {
                let typ = ty.ty.clone();
                if ty.ident == "SessionKeys" {
                    session_keys = Some(fetch_ident(&typ))
                }

                #[cfg(not(feature = "async-backing"))]
                if ty.ident == "Aura" {
                    aura = Some(fetch_ident(&typ))
                }

                #[cfg(feature = "async-backing")]
                if ty.ident == "SlotDuration" {
                    slot_duration = Some(fetch_ident(&typ))
                } else if ty.ident == "ConsensusHook" {
                    consensus_hook = Some(fetch_ident(&typ))
                }
            }
        }
        let session_keys = session_keys.ok_or("type `SessionKeys` not specified, but required")?;

        #[cfg(not(feature = "async-backing"))]
        {
            let aura = aura.ok_or("type `Aura` not specified, but required")?;
            Ok(ConsensusAPIFields { session_keys, aura })
        }

        #[cfg(feature = "async-backing")]
        {
            let slot_duration =
                slot_duration.ok_or("type `SlotDuration` not specified, but required")?;
            let consensus_hook =
                consensus_hook.ok_or("type `ConsensusHook` not specified, but required")?;
            Ok(ConsensusAPIFields {
                session_keys,
                slot_duration,
                consensus_hook,
            })
        }
    }
}

pub fn consensus_apis(
    runtime: &Ident,
    block: &Ident,
    session_keys: &Ident,
    #[cfg(not(feature = "async-backing"))] aura: &Ident,
    #[cfg(feature = "async-backing")] slot_duration: &Ident,
    #[cfg(feature = "async-backing")] consensus_hook: &Ident,
) -> TokenStream {
    #[cfg(feature = "async-backing")]
    let slot_duration = quote! {
        return sp_consensus_aura::SlotDuration::from_millis(#slot_duration);
    };
    #[cfg(not(feature = "async-backing"))]
    let slot_duration = quote! {
        sp_consensus_aura::SlotDuration::from_millis(#aura::slot_duration())
    };

    let mut res = quote! {};

    res.extend(quote! {
        impl sp_consensus_aura::AuraApi<#block, sp_consensus_aura::sr25519::AuthorityId> for #runtime {
            fn slot_duration() -> sp_consensus_aura::SlotDuration {
                #slot_duration
            }

            fn authorities() -> sp_std::prelude::Vec<sp_consensus_aura::sr25519::AuthorityId> {
                pallet_aura::Authorities::<#runtime>::get().into_inner()
            }
        }

        impl sp_session::SessionKeys<#block> for #runtime {
            fn generate_session_keys(seed: Option<sp_std::prelude::Vec<u8>>) -> sp_std::prelude::Vec<u8> {
                #session_keys::generate(seed)
            }

            fn decode_session_keys(encoded: sp_std::prelude::Vec<u8>) -> Option<sp_std::prelude::Vec<(sp_std::prelude::Vec<u8>, sp_core::crypto::KeyTypeId)>> {
                #session_keys::decode_into_raw_public_keys(&encoded)
            }
        }
    });
    #[cfg(feature = "async-backing")]
    res.extend(quote! {
        impl cumulus_primitives_aura::AuraUnincludedSegmentApi<#block> for #runtime {
            fn can_build_upon(
                included_hash: <#block as sp_runtime::traits::Block>::Hash,
                slot: cumulus_primitives_aura::Slot,
            ) -> bool {
                #consensus_hook::can_build_upon(included_hash, slot)
            }
        }
    });

    res
}
