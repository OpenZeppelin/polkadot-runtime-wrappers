use proc_macro2::Ident;
use quote::quote;
use syn::Item;

use super::fetch_ident;

#[derive(Default)]
pub struct AbstractionState {
    pub benchmark_fields: Option<BenchmarkAPIFields>,
    pub consensus: bool,
}

pub struct BenchmarkAPIFields {
    pub all_pallets_with_system: Ident,
    pub parachain_system: Ident,
    pub system: Ident,
    pub xcm_fields: Option<XCMBenchmarkAPIFields>,
}

impl TryFrom<&[Item]> for BenchmarkAPIFields {
    type Error = &'static str;

    fn try_from(value: &[Item]) -> Result<Self, Self::Error> {
        let mut all_pallets_with_system = None;
        let mut parachain_system = None;
        let mut system = None;

        for item in value {
            if let Item::Type(ty) = item {
                if ty.ident == "AllPalletsWithSystem" {
                    all_pallets_with_system = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "ParachainSystem" {
                    parachain_system = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "System" {
                    system = Some(fetch_ident(&ty.ty))
                }
            }
        }

        let all_pallets_with_system = all_pallets_with_system
            .ok_or("type `AllPalletsWithSystem` not specified, but required")?;
        let parachain_system =
            parachain_system.ok_or("type `ParachainSystem` not specified, but required")?;
        let system = system.ok_or("type `System` not specified, but required")?;
        let xcm_fields = XCMBenchmarkAPIFields::try_from(value)
            .map_err(|e| println!("{e:?}"))
            .ok();

        Ok(BenchmarkAPIFields {
            all_pallets_with_system,
            parachain_system,
            system,
            xcm_fields,
        })
    }
}

pub struct XCMBenchmarkAPIFields {
    pub assets: Ident,
    pub asset_manager: Ident,
    pub asset_type: Ident,
    pub runtime_origin: Ident,
    pub relay_location: Ident,
    pub existential_deposit: Ident,
    pub asset_id: Ident,
    pub xcm_config: Ident,
    pub account_id: Ident,
    pub cents: Ident,
    pub fee_asset_id: Ident,
    pub transaction_byte_fee: Ident,
    pub address: Ident,
    pub balances: Ident,
}

impl TryFrom<&[Item]> for XCMBenchmarkAPIFields {
    type Error = &'static str;
    fn try_from(value: &[Item]) -> Result<Self, Self::Error> {
        let mut assets = None;
        let mut asset_manager = None;
        let mut asset_type = None;
        let mut runtime_origin = None;
        let mut relay_location = None;
        let mut existential_deposit = None;
        let mut asset_id = None;
        let mut xcm_config = None;
        let mut account_id = None;
        let mut cents = None;
        let mut fee_asset_id = None;
        let mut transaction_byte_fee = None;
        let mut address = None;
        let mut balances = None;

        for item in value {
            if let Item::Type(ty) = item {
                if ty.ident == "Assets" {
                    assets = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "AssetManager" {
                    asset_manager = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "AssetType" {
                    asset_type = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "RuntimeOrigin" {
                    runtime_origin = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "RelayLocation" {
                    relay_location = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "ExistentialDeposit" {
                    existential_deposit = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "AssetId" {
                    asset_id = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "XCMConfig" {
                    xcm_config = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "AccountId" {
                    account_id = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "Cents" {
                    cents = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "FeeAssetId" {
                    fee_asset_id = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "TransactionByteFee" {
                    transaction_byte_fee = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "Address" {
                    address = Some(fetch_ident(&ty.ty))
                } else if ty.ident == "Balances" {
                    balances = Some(fetch_ident(&ty.ty))
                }
            }
        }

        let assets = assets.ok_or("type `Assets` not specified, but required")?;
        let asset_manager =
            asset_manager.ok_or("type `AssetManager` not specified, but required")?;
        let asset_type = asset_type.ok_or("type `AssetType` not specified, but required")?;
        let runtime_origin =
            runtime_origin.ok_or("type `RuntimeOrigin` not specified, but required")?;
        let relay_location =
            relay_location.ok_or("type `RelayLocation` not specified, but required")?;
        let existential_deposit =
            existential_deposit.ok_or("type `ExistentialDeposit` not specified, but required")?;
        let asset_id = asset_id.ok_or("type `AssetId` not specified, but required")?;
        let xcm_config = xcm_config.ok_or("type `XCMConfig` not specified, but required")?;
        let account_id = account_id.ok_or("type `AccountId` not specified, but required")?;
        let cents = cents.ok_or("type `Cents` not specified, but required")?;
        let fee_asset_id = fee_asset_id.ok_or("type `FeeAssetId` not specified, but required")?;
        let transaction_byte_fee =
            transaction_byte_fee.ok_or("type `TransactionByteFee` not specified, but required")?;
        let address = address.ok_or("type `Address` not specified, but required")?;
        let balances = balances.ok_or("type `Balances` not specified, but required")?;

        Ok(XCMBenchmarkAPIFields {
            assets,
            asset_manager,
            asset_type,
            runtime_origin,
            relay_location,
            existential_deposit,
            asset_id,
            xcm_config,
            account_id,
            cents,
            fee_asset_id,
            transaction_byte_fee,
            address,
            balances
        })
    }
}

pub fn construct_benchmarking_api(
    consensus_benchmarking: bool,
    runtime: &Ident,
    api_fields: BenchmarkAPIFields,
) -> proc_macro2::TokenStream {
    let mut xcm_dispatch = quote! {};
    let mut xcm_metadata = quote! {};
    let mut consensus_dispatch = quote! {};
    let mut consensus_metadata = quote! {};

    if consensus_benchmarking {
        consensus_dispatch = construct_consensus_dispatch_benchmarking(runtime);
        consensus_metadata = construct_consensus_metadata_benchmarking();
    }

    let BenchmarkAPIFields {
        all_pallets_with_system,
        system,
        parachain_system,
        xcm_fields,
    } = api_fields;

    if let Some(XCMBenchmarkAPIFields {
        assets,
        asset_manager,
        asset_type,
        runtime_origin,
        relay_location,
        existential_deposit,
        asset_id,
        xcm_config,
        account_id,
        cents,
        fee_asset_id,
        transaction_byte_fee,
        address,
    }) = xcm_fields
    {
        xcm_metadata = construct_xcm_metadata_benchmarking();
        xcm_dispatch = construct_xcm_dispatch_benchmarking(
            runtime,
            assets,
            asset_manager,
            asset_type,
            runtime_origin,
            relay_location,
            &parachain_system,
            existential_deposit,
            asset_id,
            xcm_config,
            account_id,
            cents,
            fee_asset_id,
            transaction_byte_fee,
            address,
        );
    }

    quote! {
        #[cfg(feature = "runtime-benchmarks")]
        impl frame_benchmarking::Benchmark<Block> for #runtime {
            fn benchmark_metadata(extra: bool) -> (
                sp_std::prelude::Vec<frame_benchmarking::BenchmarkList>,
                sp_std::prelude::Vec<frame_support::traits::StorageInfo>,
            ) {
                use frame_benchmarking::{Benchmarking, BenchmarkList};
                use frame_support::traits::StorageInfoTrait;
                use frame_system_benchmarking::Pallet as SystemBench;
                use crate::*;

                #xcm_metadata
                #consensus_metadata

                let mut list = sp_std::prelude::Vec::<BenchmarkList>::new();
                list_benchmarks!(list, extra);

                let storage_info = #all_pallets_with_system::storage_info();
                (list, storage_info)
            }

            fn dispatch_benchmark(
                config: frame_benchmarking::BenchmarkConfig
            ) -> Result<sp_std::prelude::Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
                use frame_benchmarking::{BenchmarkError, Benchmarking, BenchmarkBatch};
                use frame_system_benchmarking::Pallet as SystemBench;
                use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;

                use crate::{*, types::*, configs::*};

                #[cfg(feature = "runtime-benchmarks")]
                impl frame_system_benchmarking::Config for #runtime {
                    fn setup_set_code_requirements(
                        code: &sp_std::vec::Vec<u8>,
                    ) -> Result<(), BenchmarkError> {
                        #parachain_system::initialize_for_set_code_benchmark(code.len() as u32);
                        Ok(())
                    }

                    fn verify_set_code() {
                        #system::assert_last_event(
                            cumulus_pallet_parachain_system::Event::<#runtime>::ValidationFunctionStored
                                .into(),
                        );
                    }
                }

                #xcm_dispatch
                #consensus_dispatch

                use frame_support::traits::WhitelistedStorageKeys;
                let whitelist = #all_pallets_with_system::whitelisted_storage_keys();

                let mut batches = sp_std::prelude::Vec::<BenchmarkBatch>::new();
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

fn construct_consensus_dispatch_benchmarking(runtime: &Ident) -> proc_macro2::TokenStream {
    quote! {
        use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
        impl cumulus_pallet_session_benchmarking::Config for #runtime {}
    }
}

fn construct_xcm_metadata_benchmarking() -> proc_macro2::TokenStream {
    quote! {
        use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;
    }
}

#[allow(clippy::too_many_arguments)]
fn construct_xcm_dispatch_benchmarking(
    runtime: &Ident,
    assets: Ident,
    asset_manager: Ident,
    asset_type: Ident,
    runtime_origin: Ident,
    relay_location: Ident,
    parachain_system: &Ident,
    existential_deposit: Ident,
    asset_id: Ident,
    xcm_config: Ident,
    account_id: Ident,
    cents: Ident,
    fee_asset_id: Ident,
    transaction_byte_fee: Ident,
    address: Ident,
    balances: Ident,
) -> proc_macro2::TokenStream {
    quote! {
        use cumulus_primitives_core::ParaId;
        use frame_support::parameter_types;
        use xcm::latest::prelude::{Asset, AssetId as XcmAssetId, Assets as AssetList, Fungible, Location, Parachain, Parent, ParentThen, PalletInstance, GeneralIndex};

        parameter_types! {
            pub const RandomParaId: ParaId = ParaId::new(43211234);
            pub ExistentialDepositAsset: Option<Asset> = Some((
                #relay_location::get(),
                <#existential_deposit as sp_core::Get<u128>>::get()
            ).into());
            /// The base fee for the message delivery fees. Kusama is based for the reference.
            pub const ToParentBaseDeliveryFee: u128 = #cents.saturating_mul(3);
            pub const InitialTransferAssetAmount: u128 = 4001070000100;
        }

        pub type PriceForParentDelivery = polkadot_runtime_common::xcm_sender::ExponentialPrice<
            #fee_asset_id,
            ToParentBaseDeliveryFee,
            #transaction_byte_fee,
            #parachain_system,
        >;

        impl pallet_xcm::benchmarking::Config for #runtime {
            type DeliveryHelper = cumulus_primitives_utility::ToParentDeliveryHelper<
                #xcm_config,
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
                use xcm_primitives::AssetTypeGetter;
                use frame_system::RawOrigin;

                // set up fee asset
                let fee_location = #relay_location::get();
                let who: #account_id = frame_benchmarking::whitelisted_caller();

                let Some(location_v3) = xcm::v3::Location::try_from(fee_location.clone()).ok() else {
                    return None;
                };
                let asset_type = #asset_type::Xcm(location_v3);

                let balance = 3001070000000;
                let who = frame_benchmarking::whitelisted_caller();
                let _ =
                    <#balances as frame_support::traits::Currency<_>>::make_free_balance_be(&who, balance);

                let local_asset_id: #asset_id = asset_type.clone().into();
                let manager_id = #asset_manager::account_id();
                let _ = #assets::force_create(#runtime_origin::root(), local_asset_id.clone().into(), #address::from(manager_id.clone()), true, 1);
                let _ = #assets::mint(
                    RawOrigin::Signed(manager_id.clone()).into(),
                    local_asset_id.into(),
                    #address::from(who),
                    InitialTransferAssetAmount::get(),
                );
                #asset_manager::set_asset_type_asset_id(asset_type.clone(), local_asset_id.into());

                // open a mock parachain channel
                #parachain_system::open_outbound_hrmp_channel_for_benchmarks_or_tests(
                    RandomParaId::get().into()
                );

                // set up transfer asset
                let initial_asset_amount: u128 = InitialTransferAssetAmount::get();
                let (asset_id, _, _) = pallet_assets::benchmarking::create_default_minted_asset::<
                    #runtime,
                    ()
                >(true, initial_asset_amount);

                let local_asset_id: #asset_id = asset_id.into();
                let self_reserve = Location {
                    parents: 0,
                    interior: [
                        PalletInstance(<#assets as PalletInfoAccess>::index() as u8), GeneralIndex(local_asset_id as u128)
                    ].into()
                };

                let Some(location_v3) = xcm::v3::Location::try_from(self_reserve.clone()).ok() else {
                    return None;
                };
                let asset_type = #asset_type::Xcm(location_v3);
                #asset_manager::set_asset_type_asset_id(asset_type.clone(), local_asset_id);

                let asset = Asset {
                    fun: Fungible(<#existential_deposit as sp_core::Get<u128>>::get()),
                    id: XcmAssetId(self_reserve.into())
                }.into();
                Some((
                    asset,
                    ParentThen(Parachain(RandomParaId::get().into()).into()).into(),
                ))
            }

            fn set_up_complex_asset_transfer(
            ) -> Option<(AssetList, u32, Location, Box<dyn FnOnce()>)> {
                use frame_support::traits::PalletInfoAccess;
                use xcm_primitives::AssetTypeGetter;
                // set up local asset
                let initial_asset_amount: u128 = 1000000011;

                let (asset_id, _, _) = pallet_assets::benchmarking::create_default_minted_asset::<
                    #runtime,
                    ()
                >(true, initial_asset_amount);

                let local_asset_id: #asset_id = asset_id.into();

                let self_reserve = Location {
                    parents:0,
                    interior: [
                        PalletInstance(<#assets as PalletInfoAccess>::index() as u8), GeneralIndex(local_asset_id as u128)
                    ].into()
                };

                let Some(location_v3) = xcm::v3::Location::try_from(self_reserve.clone()).ok() else {
                    return None;
                };
                let asset_type = #asset_type::Xcm(location_v3);
                #asset_manager::set_asset_type_asset_id(asset_type.clone(), local_asset_id);

                let destination: xcm::v4::Location = Parent.into();

                // set up fee asset
                let fee_amount: u128 = <#existential_deposit as sp_core::Get<u128>>::get();
                let asset_amount: u128 = 10;
                let fee_asset: Asset = (self_reserve.clone(), fee_amount).into();
                let transfer_asset: Asset = (self_reserve.clone(), asset_amount).into();

                let assets: cumulus_primitives_core::Assets = sp_std::vec![fee_asset.clone(), transfer_asset].into();
                let fee_index: u32 = 0;

                let who = frame_benchmarking::whitelisted_caller();

                let verify: Box<dyn FnOnce()> = Box::new(move || {
                    // verify balance after transfer, decreased by
                    // transferred amount (and delivery fees)
                    assert!(#assets::balance(local_asset_id, &who) <= initial_asset_amount - fee_amount);
                });

                Some((assets, fee_index, destination, verify))
            }

            fn get_asset() -> Asset {
                use xcm_primitives::AssetTypeGetter;
                let location = Location::parent();
                let asset_id = XcmAssetId(location.clone());
                let asset = Asset {
                    id: asset_id.clone(),
                    fun: Fungible(<#existential_deposit as sp_core::Get<u128>>::get()),
                };
                let Some(location_v3) = xcm::v3::Location::try_from(location).ok() else {
                    return asset;
                };
                let asset_type = #asset_type::Xcm(location_v3);
                let local_asset_id: #asset_id = asset_type.clone().into();
                let manager_id = #asset_manager::account_id();
                let _ = #assets::force_create(#runtime_origin::root(), local_asset_id.clone().into(), #address::from(manager_id), true, 1);
                #asset_manager::set_asset_type_asset_id(asset_type.clone(), local_asset_id);
                asset
            }
        }
    }
}
