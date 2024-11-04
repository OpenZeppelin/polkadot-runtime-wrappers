//! Implements the OpenZeppelin EVM configuration for a Runtime.
//!
//! This macro sets up the necessary configurations for the following pallets:
//! - `pallet_ethereum`
//! - `pallet_evm`
//! - `pallet_evm_chain_id`
//! - `pallet_base_fee`
//! - `pallet_erc20_xcm_bridge`
//!
//! # Parameters
//! - `$t`: A type that implements the `EvmConfig` trait, providing the necessary associated types
//!   and configurations.
//!
//! # Important
//! Rerun benchmarks if making changes to runtime configuration, as weight calculations
//! may need to be updated.

#[macro_export]
macro_rules! impl_openzeppelin_evm {
    ($t:ty) => {
        parameter_types! {
            pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
        }

        impl pallet_ethereum::Config for Runtime {
            type ExtraDataLength = ConstU32<30>;
            type PostLogContent = PostBlockAndTxnHashes;
            type RuntimeEvent = RuntimeEvent;
            type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
        }

        parameter_types! {
            /// Block gas limit is calculated with target for 75% of block capacity and ratio of maximum block weight and weight per gas
            pub BlockGasLimit: U256 = U256::from(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS);
            /// To calculate ratio of Gas Limit to PoV size we take the BlockGasLimit we calculated before, and divide it on MAX_POV_SIZE
            pub GasLimitPovSizeRatio: u64 = BlockGasLimit::get().min(u64::MAX.into()).low_u64().saturating_div(cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64);
            pub WeightPerGas: Weight = Weight::from_parts(WEIGHT_PER_GAS, 0);
            pub SuicideQuickClearLimit: u32 = 0;
        }

        impl pallet_evm::Config for Runtime {
            type AddressMapping = <$t as EvmConfig>::AddressMapping;
            type BlockGasLimit = BlockGasLimit;
            type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
            type CallOrigin = <$t as EvmConfig>::CallOrigin;
            type ChainId = EVMChainId;
            type Currency = Balances;
            type FeeCalculator = BaseFee;
            type FindAuthor = <$t as EvmConfig>::FindAuthor;
            type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
            type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
            type OnChargeTransaction = EVMCurrencyAdapter<Balances, ()>;
            type OnCreate = ();
            type PrecompilesType = <$t as EvmConfig>::PrecompilesType;
            type PrecompilesValue = <$t as EvmConfig>::PrecompilesValue;
            type Runner = pallet_evm::runner::stack::Runner<Self>;
            type RuntimeEvent = RuntimeEvent;
            type SuicideQuickClearLimit = SuicideQuickClearLimit;
            type Timestamp = Timestamp;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_evm::WeightInfo<Self>;
            type WeightPerGas = WeightPerGas;
            type WithdrawOrigin = <$t as EvmConfig>::WithdrawOrigin;
        }

        impl pallet_evm_chain_id::Config for Runtime {}

        parameter_types! {
            /// Starting value for base fee. Set at the same value as in Ethereum.
            pub DefaultBaseFeePerGas: U256 = U256::from(1_000_000_000);
            /// Default elasticity rate. Set at the same value as in Ethereum.
            pub DefaultElasticity: Permill = Permill::from_parts(125_000);
        }

        /// The thresholds based on which the base fee will change.
        pub struct BaseFeeThreshold;
        impl pallet_base_fee::BaseFeeThreshold for BaseFeeThreshold {
            fn lower() -> Permill {
                Permill::zero()
            }

            fn ideal() -> Permill {
                Permill::from_parts(500_000)
            }

            fn upper() -> Permill {
                Permill::from_parts(1_000_000)
            }
        }
        impl pallet_base_fee::Config for Runtime {
            type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
            type DefaultElasticity = DefaultElasticity;
            type RuntimeEvent = RuntimeEvent;
            type Threshold = BaseFeeThreshold;
        }

        impl pallet_erc20_xcm_bridge::Config for Runtime {
            type AccountIdConverter = LocationToH160;
            type Erc20MultilocationPrefix = Erc20XcmBridgePalletLocation;
            type Erc20TransferGasLimit = <$t as EvmConfig>::Erc20XcmBridgeTransferGasLimit;
            type EvmRunner = pallet_evm::runner::stack::Runner<Self>;
        }
    };
}

pub const PALLET_NAMES: [(&str, &str); 4] = [
    ("Ethereum", "pallet_ethereum"),
    ("EVM", "pallet_evm"),
    ("BaseFee", "pallet_base_fee"),
    ("EVMChainId", "pallet_evm_chain_id"),
];
