use core::panic;
use crate::{apis, models::Abstractions};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, Item, ItemMod, Type};

#[derive(Default)]
struct AbstractionState {
    pub assets: bool,
    pub xcm: bool,
    pub consensus: bool,
}

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
        Abstractions::Assets => {
            state.assets = true;
            let mut transaction_payment = None;
            let mut call = None;
            let mut balance = None;

            for item in content {
                match item {
                    Item::Type(ty) => {
                        if ty.ident == "TransactionPayment" {
                            transaction_payment = Some(fetch_ident(&ty.ty))
                        } else if ty.ident == "RuntimeCall" {
                            call = Some(fetch_ident(&ty.ty))
                        } else if ty.ident == "Balance" {
                            balance = Some(fetch_ident(&ty.ty))
                        }
                    }
                    _ => (),
                }
            }

            let transaction_payment =
                transaction_payment.expect("`type TransactionPayment` not specified, but required");
            let balance = balance.expect("`type Balance` not specified, but required");
            let call = call.expect("`type RuntimeCall` not specified, but required");

            apis::assets_apis(runtime, block, &transaction_payment, &balance, &call)
        }
        Abstractions::Consensus => {
            state.assets = true;

            let mut session_keys = None;

            #[cfg(not(feature = "async-backing"))]
            let mut aura = None;
            #[cfg(feature = "async-backing")]
            let mut slot_duration = None;
            #[cfg(feature = "async-backing")]
            let mut consensus_hook = None;

            for item in content {
                match item {
                    Item::Type(ty) => {
                        let typ = *ty.ty;
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
                            slot_duration = Some(fetch_ident(&typ))
                        }
                    }
                    _ => (),
                }
            }
            let session_keys = session_keys.expect("type SessionKeys` not specified, but required");

            #[cfg(not(feature = "async-backing"))]
            {
                let aura = aura.expect("type Aura` not specified, but required");
                apis::consensus_assets(runtime, block, &session_keys, &aura)
            }
            #[cfg(feature = "async-backing")]
            {
                let slot_duration =
                    slot_duration.expect("type SlotDuration` not specified, but required");
                let consensus_hook =
                    consensus_hook.expect("type SlotDuration` not specified, but required");
                apis::consensus_assets(
                    runtime,
                    block,
                    &session_keys,
                    &slot_duration,
                    &consensus_hook,
                )
            }
        }
        Abstractions::System => {
            let mut executive = None;
            let mut system = None;
            let mut parachain_system = None;
            let mut version = None;
            let mut account_id = None;
            let mut nonce = None;
            let mut genesis = None;

            for item in content {
                match item {
                    Item::Type(ty) => {
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
                        }
                    }
                    _ => (),
                }
            }

            let executive = executive.expect("`type Executive` not specified, but required");
            let system = system.expect("`type System` not specified, but required");
            let parachain_system =
                parachain_system.expect("`type ParachainSystem` not specified, but required");
            let version = version.expect("`type RuntimeVersion` not specified, but required");
            let account_id = account_id.expect("`type AccountId` not specified, but required");
            let nonce = nonce.expect("`type Nonce` not specified, but required");
            let genesis = genesis.expect("`type RuntimeGenesisConfig` not specified, but required");
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

fn construct_benchmarking_api(state: AbstractionState) -> proc_macro2::TokenStream {
    let mut xcm_dispatch = quote! {};
    let mut xcm_metadata = quote! {};
    let mut consensus_dispatch = quote! {};
    let mut consensus_metadata = quote! {};

    if state.consensus {
        consensus_dispatch = construct_consensus_dispatch_benchmarking();
        consensus_metadata = construct_consensus_metadata_benchmarking();
    }

    if state.assets && state.xcm {
        xcm_metadata = construct_xcm_metadata_benchmarking();
        xcm_dispatch = construct_xcm_dispatch_benchmarking();
    }

    quote! {
        #[cfg(feature = "runtime-benchmarks")]
        impl frame_benchmarking::Benchmark<Block> for Runtime {
            fn benchmark_metadata(extra: bool) -> (
                Vec<frame_benchmarking::BenchmarkList>,
                Vec<frame_support::traits::StorageInfo>,
            ) {
                use frame_benchmarking::{Benchmarking, BenchmarkList};
                use frame_support::traits::StorageInfoTrait;
                use frame_system_benchmarking::Pallet as SystemBench;

                #xcm_metadata
                #consensus_metadata

                use super::*;

                let mut list = Vec::<BenchmarkList>::new();
                list_benchmarks!(list, extra);

                let storage_info = AllPalletsWithSystem::storage_info();
                (list, storage_info)
            }

            fn dispatch_benchmark(
                config: frame_benchmarking::BenchmarkConfig
            ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
                use frame_benchmarking::{BenchmarkError, Benchmarking, BenchmarkBatch};
                use frame_system_benchmarking::Pallet as SystemBench;

                use super::{*, types::*, configs::*, constants::currency::CENTS};

                #xcm_dispatch
                #consensus_dispatch

                use frame_support::traits::WhitelistedStorageKeys;
                let whitelist = AllPalletsWithSystem::whitelisted_storage_keys();

                let mut batches = Vec::<BenchmarkBatch>::new();
                let params = (&config, &whitelist);
                add_benchmarks!(params, batches);

                if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
                Ok(batches)
            }
        }
    }
}

fn construct_consensus_metadata_benchmarking() -> proc_macro2::TokenStream {
    quote! {
        use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
    }
}

fn construct_consensus_dispatch_benchmarking() -> proc_macro2::TokenStream {
    quote! {
        use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
        impl cumulus_pallet_session_benchmarking::Config for Runtime {}
    }
}

fn construct_xcm_metadata_benchmarking() -> proc_macro2::TokenStream {
    quote! {
        use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;
    }
}

// TODO: think how this should change if AssetManager is included
fn construct_xcm_dispatch_benchmarking() -> proc_macro2::TokenStream {
    quote! {
        use cumulus_primitives_core::ParaId;
        use frame_support::parameter_types;
        parameter_types! {
            pub const RandomParaId: ParaId = ParaId::new(43211234);
            pub ExistentialDepositAsset: Option<Asset> = Some((
                RelayLocation::get(),
                ExistentialDeposit::get()
            ).into());
            /// The base fee for the message delivery fees. Kusama is based for the reference.
            pub const ToParentBaseDeliveryFee: u128 = CENTS.saturating_mul(3);
        }
        pub type PriceForParentDelivery = polkadot_runtime_common::xcm_sender::ExponentialPrice<
            FeeAssetId,
            ToParentBaseDeliveryFee,
            TransactionByteFee,
            ParachainSystem,
        >;
        use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;
        use xcm::latest::prelude::{Asset, AssetId, Assets as AssetList, Fungible, Location, Parachain, Parent, ParentThen, PalletInstance, GeneralIndex};
        impl pallet_xcm::benchmarking::Config for Runtime {
            type DeliveryHelper = cumulus_primitives_utility::ToParentDeliveryHelper<
                xcm_config::XcmConfig,
                ExistentialDepositAsset,
                PriceForParentDelivery,
            >;

            fn reachable_dest() -> Option<Location> {
                Some(Parent.into())
            }

            fn teleportable_asset_and_dest() -> Option<(Asset, Location)> {
                None
            }

            fn reserve_transferable_asset_and_dest() -> Option<(Asset, Location)> {
                use frame_support::traits::PalletInfoAccess;
                ParachainSystem::open_outbound_hrmp_channel_for_benchmarks_or_tests(
                    RandomParaId::get().into()
                );
                let balance = 3001070000000;
                let who = frame_benchmarking::whitelisted_caller();
                let _ =
                    <Balances as frame_support::traits::Currency<_>>::make_free_balance_be(&who, balance);

                let asset_amount: u128 = 10u128;
                let initial_asset_amount: u128 = asset_amount * 10;

                let (asset_id, _, _) = pallet_assets::benchmarking::create_default_minted_asset::<
                    Runtime,
                    ()
                >(true, initial_asset_amount);

                let asset_id_u32: u32 = asset_id.into();

                let location = Location {parents: 0, interior: (PalletInstance(<Assets as PalletInfoAccess>::index() as u8), GeneralIndex(asset_id_u32 as u128)).into()};
                Some((
                    Asset {
                        fun: Fungible(ExistentialDeposit::get()),
                        id: AssetId(location.into())
                    }.into(),
                    ParentThen(Parachain(RandomParaId::get().into()).into()).into(),
                ))
            }

            fn set_up_complex_asset_transfer(
            ) -> Option<(AssetList, u32, Location, Box<dyn FnOnce()>)> {
                use frame_support::traits::PalletInfoAccess;
                // set up local asset
                let asset_amount: u128 = 10u128;
                let initial_asset_amount: u128 = 1000000011;
                let (asset_id, _, _) = pallet_assets::benchmarking::create_default_minted_asset::<
                    Runtime,
                    ()
                >(true, initial_asset_amount);
                let asset_id_u32: u32 = asset_id.into();

                let self_reserve = Location {
                    parents:0,
                    interior: [
                        PalletInstance(<Assets as PalletInfoAccess>::index() as u8), GeneralIndex(asset_id_u32 as u128)
                    ].into()
                };

                let destination: xcm::v4::Location = Parent.into();

                let fee_amount: u128 = <Runtime as pallet_balances::Config>::ExistentialDeposit::get();
                let fee_asset: Asset = (self_reserve.clone(), fee_amount).into();

                // Give some multiple of transferred amount
                let balance = fee_amount * 1000;
                let who = frame_benchmarking::whitelisted_caller();
                let _ =
                    <Balances as frame_support::traits::Currency<_>>::make_free_balance_be(&who, balance);

                // verify initial balance
                assert_eq!(Balances::free_balance(&who), balance);
                let transfer_asset: Asset = (self_reserve.clone(), asset_amount).into();

                let assets: cumulus_primitives_core::Assets = vec![fee_asset.clone(), transfer_asset].into();
                let fee_index: u32 = 0;

                let verify: Box<dyn FnOnce()> = Box::new(move || {
                    // verify balance after transfer, decreased by
                    // transferred amount (and delivery fees)
                    assert!(Assets::balance(asset_id_u32, &who) <= initial_asset_amount - fee_amount);
                });

                Some((assets, fee_index, destination, verify))
            }

            fn get_asset() -> Asset {
                use frame_support::traits::PalletInfoAccess;
                Asset {
                    id: AssetId((Location {parents: 0, interior: (PalletInstance(<Assets as PalletInfoAccess>::index() as u8), GeneralIndex(1)).into()}).into()),
                    fun: Fungible(ExistentialDeposit::get()),
                }
            }
        }
    }
}

fn fetch_ident(ty: &Type) -> Ident {
    match ty {
        Type::Path(p) => p
            .path
            .get_ident()
            .expect(&format!("Wrong type received: {:?}", p))
            .clone(),
        _ => panic!("Wrong type received: {:?}", ty),
    }
}
