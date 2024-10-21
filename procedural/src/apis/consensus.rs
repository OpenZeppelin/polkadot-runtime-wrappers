use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn consensus_assets(
    runtime: &Ident,
    block: &Ident,
    session_keys: &Ident,
    #[cfg(not(feature = "async-backing"))] aura: &Ident,
    #[cfg(feature = "async-backing")] slot_duration: &Ident,
    #[cfg(feature = "async-backing")] consensus_hook: &Ident,
) -> TokenStream {
    #[cfg(feature = "async-backing")]
    let slot_duration = quote! {
        return sp_consensus_aura::SlotDuration::from_millis(<#api_ident as SystemAPI>::SLOT_DURATION);
    };
    #[cfg(not(feature = "async-backing"))]
    let slot_duration = quote! {
        sp_consensus_aura::SlotDuration::from_millis(#aura::slot_duration())
    };

    let res = quote! {
        impl sp_consensus_aura::AuraApi<#block, AuraId> for #runtime {
            fn slot_duration() -> sp_consensus_aura::SlotDuration {
                #slot_duration
            }

            fn authorities() -> Vec<AuraId> {
                pallet_aura::Authorities::<#runtime>::get().into_inner()
            }
        }

        impl sp_session::SessionKeys<#block> for #runtime {
            fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
                #session_keys::generate(seed)
            }

            fn decode_session_keys(encoded: Vec<u8>) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
                #session_keys::decode_into_raw_public_keys(&encoded)
            }
        }
    };
    #[cfg(feature = "async-backing")]
    res.extend(quote! {
        impl cumulus_primitives_aura::AuraUnincludedSegmentApi<#block> for #runtime {
            fn can_build_upon(
                included_hash: <#block as BlockT>::Hash,
                slot: cumulus_primitives_aura::Slot,
            ) -> bool {
                #consensus_hook::can_build_upon(included_hash, slot)
            }
        }
    });

    res
}
