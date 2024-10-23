use quote::quote;

#[derive(Default)]
pub struct AbstractionState {
    pub assets: bool,
    pub xcm: bool,
    pub consensus: bool,
}

pub fn construct_benchmarking_api(state: AbstractionState) -> proc_macro2::TokenStream {
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
