use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Item};

use super::fetch_ident;

#[derive(Debug)]
pub struct SystemAPIFields {
    pub executive: Ident,
    pub system: Ident,
    pub parachain_system: Ident,
    pub version: Ident,
    pub account_id: Ident,
    pub nonce: Ident,
    pub genesis: Ident,
    pub runtime_block_weights: Ident,
}

impl TryFrom<&[Item]> for SystemAPIFields {
    type Error = &'static str;

    fn try_from(value: &[Item]) -> Result<Self, Self::Error> {
        let mut executive = None;
        let mut system = None;
        let mut parachain_system = None;
        let mut version = None;
        let mut account_id = None;
        let mut nonce = None;
        let mut genesis = None;
        let mut runtime_block_weights = None;

        for item in value {
            if let Item::Type(ty) = item {
                if ty.ident == "Executive" {
                    executive = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "System" {
                    system = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "ParachainSystem" {
                    parachain_system = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "RuntimeVersion" {
                    version = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "AccountId" {
                    account_id = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "Nonce" {
                    nonce = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "RuntimeGenesisConfig" {
                    genesis = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "RuntimeBlockWeights" {
                    runtime_block_weights = Some(fetch_ident(&ty.ty))
                }
            }
        }

        let executive = executive.ok_or("`type Executive` not specified, but required")?;
        let system = system.ok_or("`type System` not specified, but required")?;
        let parachain_system =
            parachain_system.ok_or("`type ParachainSystem` not specified, but required")?;
        let version = version.ok_or("`type RuntimeVersion` not specified, but required")?;
        let account_id = account_id.ok_or("`type AccountId` not specified, but required")?;
        let nonce = nonce.ok_or("`type Nonce` not specified, but required")?;
        let genesis = genesis.ok_or("`type RuntimeGenesisConfig` not specified, but required")?;
        let runtime_block_weights = runtime_block_weights
            .ok_or("`type RuntimeBlockWeights` not specified, but required")?;

        Ok(SystemAPIFields {
            executive,
            system,
            parachain_system,
            version,
            account_id,
            nonce,
            genesis,
            runtime_block_weights,
        })
    }
}

#[allow(clippy::too_many_arguments)]
pub fn system_apis(
    runtime: &Ident,
    block: &Ident,
    executive: &Ident,
    system: &Ident,
    parachain_system: &Ident,
    version: &Ident,
    account_id: &Ident,
    nonce: &Ident,
    genesis: &Ident,
    runtime_block_weights: &Ident,
) -> TokenStream {
    quote! {
        impl sp_api::Core<Block> for #runtime {
            fn version() -> sp_version::RuntimeVersion {
                #version
            }

            fn execute_block(block: #block) {
                #executive::execute_block(block)
            }

            fn initialize_block(
                header: &<#block as sp_runtime::traits::Block>::Header,
            ) -> sp_runtime::ExtrinsicInclusionMode {
                #executive::initialize_block(header)
            }
        }

        impl sp_api::Metadata<#block> for #runtime {
            fn metadata() -> sp_core::OpaqueMetadata {
                sp_core::OpaqueMetadata::new(#runtime::metadata().into())
            }

            fn metadata_at_version(version: u32) -> Option<sp_core::OpaqueMetadata> {
                #runtime::metadata_at_version(version)
            }

            fn metadata_versions() -> sp_std::vec::Vec<u32> {
                #runtime::metadata_versions()
            }
        }

        impl sp_block_builder::BlockBuilder<#block> for #runtime {
            fn apply_extrinsic(extrinsic: <#block as sp_runtime::traits::Block>::Extrinsic) -> sp_runtime::ApplyExtrinsicResult {
                #executive::apply_extrinsic(extrinsic)
            }

            fn finalize_block() -> <#block as sp_runtime::traits::Block>::Header {
                #executive::finalize_block()
            }

            fn inherent_extrinsics(
                data: sp_inherents::InherentData,
            ) -> sp_std::prelude::Vec<<#block as sp_runtime::traits::Block>::Extrinsic> {
                data.create_extrinsics()
            }

            fn check_inherents(
                block: #block,
                data: sp_inherents::InherentData,
            ) -> sp_inherents::CheckInherentsResult {
                data.check_extrinsics(&block)
            }
        }

        impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<#block> for #runtime {
            fn validate_transaction(
                source: sp_runtime::transaction_validity::TransactionSource,
                tx: <#block as sp_runtime::traits::Block>::Extrinsic,
                block_hash: <#block as sp_runtime::traits::Block>::Hash,
            ) -> sp_runtime::transaction_validity::TransactionValidity {
                #executive::validate_transaction(source, tx, block_hash)
            }
        }

        impl sp_offchain::OffchainWorkerApi<#block> for #runtime {
            fn offchain_worker(header: &<#block as sp_runtime::traits::Block>::Header) {
                #executive::offchain_worker(header)
            }
        }

        impl frame_system_rpc_runtime_api::AccountNonceApi<#block, #account_id, #nonce> for #runtime {
            fn account_nonce(account: #account_id) -> #nonce {
                #system::account_nonce(account)
            }
        }

        impl cumulus_primitives_core::CollectCollationInfo<#block> for #runtime {
            fn collect_collation_info(
                header: &<#block as sp_runtime::traits::Block>::Header,
            ) -> cumulus_primitives_core::CollationInfo {
                #parachain_system::collect_collation_info(header)
            }
        }

        #[cfg(feature = "try-runtime")]
        impl frame_try_runtime::TryRuntime<#block> for #runtime {
            fn on_runtime_upgrade(
                checks: frame_try_runtime::UpgradeCheckSelect,
            ) -> (frame_support::weights::Weight, frame_support::weights::Weight) {
                let weight = #executive::try_runtime_upgrade(checks).unwrap();
                (weight, #runtime_block_weights::get().max_block)
            }

            fn execute_block(
                block: #block,
                state_root_check: bool,
                signature_check: bool,
                select: frame_try_runtime::TryStateSelect,
            ) -> frame_support::weights::Weight {
                // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
                // have a backtrace here.
                #executive::try_execute_block(block, state_root_check, signature_check, select)
                    .unwrap()
            }
        }

        impl sp_genesis_builder::GenesisBuilder<#block> for #runtime {
            fn build_state(config: sp_std::prelude::Vec<u8>) -> sp_genesis_builder::Result {
                frame_support::genesis_builder_helper::build_state::<#genesis>(config)
            }

            fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<sp_std::prelude::Vec<u8>> {
                frame_support::genesis_builder_helper::get_preset::<#genesis>(id, |_| None)
            }

            fn preset_names() -> sp_std::prelude::Vec<sp_genesis_builder::PresetId> {
                Default::default()
            }
        }
    }
}
