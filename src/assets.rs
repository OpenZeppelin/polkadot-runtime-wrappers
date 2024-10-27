//! Assets pallet groupings wrapper

#[macro_export]
macro_rules! impl_openzeppelin_assets {
    ($t:ty) => {
        parameter_types! {
            pub const StringLimit: u32 = 50;
            pub const MetadataDepositBase: Balance = deposit(1, 68);
            pub const MetadataDepositPerByte: Balance = deposit(0, 1);
            pub const RemoveItemsLimit: u32 = 1000;
        }

        impl pallet_assets::Config for Runtime {
            type ApprovalDeposit = <$t as AssetsConfig>::ApprovalDeposit;
            type AssetAccountDeposit = <$t as AssetsConfig>::ApprovalDeposit;
            type AssetDeposit = <$t as AssetsConfig>::AssetDeposit;
            type AssetId = <$t as AssetsConfig>::AssetId;
            type AssetIdParameter = parity_scale_codec::Compact<<$t as AssetsConfig>::AssetId>;
            type Balance = Balance;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkHelper = ();
            type CallbackHandle = ();
            type CreateOrigin = <$t as AssetsConfig>::CreateOrigin;
            type Currency = Balances;
            type Extra = ();
            type ForceOrigin = <$t as AssetsConfig>::ForceOrigin;
            type Freezer = ();
            type MetadataDepositBase = MetadataDepositBase;
            type MetadataDepositPerByte = MetadataDepositPerByte;
            type RemoveItemsLimit = RemoveItemsLimit;
            type RuntimeEvent = RuntimeEvent;
            type StringLimit = StringLimit;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_assets::WeightInfo<Runtime>;
        }

        parameter_types! {
            /// Relay Chain `TransactionByteFee` / 10
            pub const TransactionByteFee: Balance = 10 * MICROCENTS;
            pub const OperationalFeeMultiplier: u8 = 5;
        }

        impl pallet_transaction_payment::Config for Runtime {
            /// There are two possible mechanisms available: slow and fast adjusting.
            /// With slow adjusting fees stay almost constant in short periods of time, changing only in long term.
            /// It may lead to long inclusion times during spikes, therefore tipping is enabled.
            /// With fast adjusting fees change rapidly, but fixed for all users at each block (no tipping)
            type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
            type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
            type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
            type OperationalFeeMultiplier = OperationalFeeMultiplier;
            type RuntimeEvent = RuntimeEvent;
            type WeightToFee = WeightToFee;
        }

        // We instruct how to register the Assets
        // In this case, we tell it to Create an Asset in pallet-assets
        pub struct AssetRegistrar;
        use frame_support::{pallet_prelude::DispatchResult, transactional};

        impl pallet_asset_manager::AssetRegistrar<Runtime> for AssetRegistrar {
            #[transactional]
            fn create_foreign_asset(
                asset: AssetId,
                min_balance: Balance,
                metadata: AssetRegistrarMetadata,
                is_sufficient: bool,
            ) -> DispatchResult {
                Assets::force_create(
                    RuntimeOrigin::root(),
                    asset.into(),
                    sp_runtime::MultiAddress::Id(AssetManager::account_id()),
                    is_sufficient,
                    min_balance,
                )?;

                // Lastly, the metadata
                Assets::force_set_metadata(
                    RuntimeOrigin::root(),
                    asset.into(),
                    metadata.name,
                    metadata.symbol,
                    metadata.decimals,
                    metadata.is_frozen,
                )
            }

            #[transactional]
            fn destroy_foreign_asset(asset: AssetId) -> DispatchResult {
                // Mark the asset as destroying
                Assets::start_destroy(RuntimeOrigin::root(), asset.into())
            }

            fn destroy_asset_dispatch_info_weight(asset: AssetId) -> Weight {
                // For us both of them (Foreign and Local) have the same annotated weight for a given
                // witness
                // We need to take the dispatch info from the destroy call, which is already annotated in
                // the assets pallet

                // This is the dispatch info of destroy
                RuntimeCall::Assets(pallet_assets::Call::<Runtime>::start_destroy {
                    id: asset.into(),
                })
                .get_dispatch_info()
                .weight
            }
        }

        #[derive(
            Clone, Default, Eq, Debug, PartialEq, Ord, PartialOrd, Encode, Decode, TypeInfo,
        )]
        pub struct AssetRegistrarMetadata {
            pub name: Vec<u8>,
            pub symbol: Vec<u8>,
            pub decimals: u8,
            pub is_frozen: bool,
        }

        impl pallet_asset_manager::Config for Runtime {
            type AssetId = AssetId;
            type AssetRegistrar = AssetRegistrar;
            type AssetRegistrarMetadata = AssetRegistrarMetadata;
            type Balance = Balance;
            type ForeignAssetModifierOrigin = <$t as AssetsConfig>::ForeignAssetModifierOrigin;
            type ForeignAssetType = <$t as AssetsConfig>::AssetType;
            type RuntimeEvent = RuntimeEvent;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_asset_manager::WeightInfo<Runtime>;
        }
    };
}
