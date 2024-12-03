//! Implements the OpenZeppelin assets configuration for a Runtime.
//!
//! This macro sets up the necessary configurations for the following pallets:
//! - `pallet_assets`
//! - `pallet_transaction_payment`
//! - `pallet_asset_manager`
//!
//! # Parameters
//! - `$t`: A type that implements the `AssetsConfig` trait, providing the necessary associated types
//!   and configurations.
//!
//! # Important
//! Rerun benchmarks if making changes to runtime configuration, as weight calculations
//! may need to be updated.

#[macro_export]
macro_rules! impl_openzeppelin_assets {
    ($t:ty) => {
        // Constants for assets configuration
        parameter_types! {
            // The maximum length of a name or symbol stored on-chain.
            pub const StringLimit: u32 = 50;
            // The basic amount of funds that must be reserved when adding metadata to your asset.
            pub const MetadataDepositBase: Balance = deposit(1, 68);
            // The additional funds that must be reserved for the number of bytes you store in your metadata.
            pub const MetadataDepositPerByte: Balance = deposit(0, 1);
            // Maximum number of items that can be removed in a single operation.
            pub const RemoveItemsLimit: u32 = 1000;
        }

        // Helper struct and implementation for runtime benchmarks
        // Only enabled when the `runtime-benchmarks` feature is active
        pallet_assets::runtime_benchmarks_enabled! {
            pub struct BenchmarkHelper;
            impl<AssetIdParameter> pallet_assets::BenchmarkHelper<AssetIdParameter> for BenchmarkHelper
            where
            AssetIdParameter: From<<$t as AssetsConfig>::AssetId>,
            {
            fn create_asset_id_parameter(id: u32) -> AssetIdParameter {
                (id as <$t as AssetsConfig>::AssetId).into()
            }
            }
        }

        impl pallet_assets::Config for Runtime {
            // The amount of funds that must be reserved when creating a new approval.
            type ApprovalDeposit = <$t as AssetsConfig>::ApprovalDeposit;
            // The amount of funds that must be reserved for a non-provider asset account to be maintained.
            type AssetAccountDeposit = <$t as AssetsConfig>::AssetAccountDeposit;
            // The basic amount of funds that must be reserved for an asset.
            type AssetDeposit = <$t as AssetsConfig>::AssetDeposit;
            // Identifier for the class of asset.
            type AssetId = <$t as AssetsConfig>::AssetId;
            // Wrapper around `AssetId` to use in dispatchable call signatures.
            type AssetIdParameter = parity_scale_codec::Compact<<$t as AssetsConfig>::AssetId>;
            // The units in which we record balances.
            type Balance = Balance;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkHelper = BenchmarkHelper;
            type CallbackHandle = ();
            // Standard asset class creation is only allowed if the origin attempting it and the
            // asset class are in this set.
            type CreateOrigin = <$t as AssetsConfig>::CreateOrigin;
            type Currency = Balances;
            type Extra = ();
            // The origin which may forcibly create or destroy an asset or otherwise alter privileged
	// attributes.
            type ForceOrigin = <$t as AssetsConfig>::ForceOrigin;
            type Freezer = ();
            type MetadataDepositBase = MetadataDepositBase;
            type MetadataDepositPerByte = MetadataDepositPerByte;
            type RemoveItemsLimit = RemoveItemsLimit;
            // The overarching event type
            type RuntimeEvent = RuntimeEvent;
            type StringLimit = StringLimit;
            type WeightInfo = <$t as AssetsWeight>::Assets;
        }

        parameter_types! {
            // Relay Chain `TransactionByteFee` / 10
            pub const TransactionByteFee: Balance = 10 * MICROCENTS;
            pub const OperationalFeeMultiplier: u8 = 5;
        }

        impl pallet_transaction_payment::Config for Runtime {
            // Fees stay almost constant over the short term and adjust slowly over time.
            // Spikes in transaction volume in the short term lead to long transaction inclusion times so tipping is allowed
            // to enable prioritization in proportion to tip amount.
            type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
            // Convert a length value into a deductible fee based on the currency type.
            type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
            // Handler for withdrawing, refunding and depositing the transaction fee.
            type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
            // A fee multiplier for `Operational` extrinsics to compute "virtual tip" to boost their
		// `priority`
            type OperationalFeeMultiplier = OperationalFeeMultiplier;
            type RuntimeEvent = RuntimeEvent;
            type WeightToFee = <$t as AssetsConfig>::WeightToFee;
        }

        impl pallet_asset_manager::Config for Runtime {
            type AssetId = AssetId;
            type AssetRegistrar = <$t as AssetsConfig>::AssetRegistrar;
            type AssetRegistrarMetadata = <$t as AssetsConfig>::AssetRegistrarMetadata;
            type Balance = Balance;
            type ForeignAssetModifierOrigin = <$t as AssetsConfig>::ForeignAssetModifierOrigin;
            type ForeignAssetType = <$t as AssetsConfig>::AssetType;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as AssetsWeight>::AssetManager;
        }
    };
}

pub const PALLET_NAMES: [(&str, &str); 3] = [
    ("Assets", "pallet_assets"),
    ("TransactionPayment", "pallet_transaction_payment"),
    ("AssetManager", "pallet_asset_manager"),
];
