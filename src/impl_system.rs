//! OpenZeppelin System Pallets Wrapper

#[macro_export]
macro_rules! impl_oz_system {
    ($t:ty) => {
        pub struct NormalFilter;
        impl Contains<RuntimeCall> for NormalFilter {
            fn contains(c: &RuntimeCall) -> bool {
                match c {
                    // We filter anonymous proxy as they make "reserve" inconsistent
                    // See: https://github.com/paritytech/polkadot-sdk/blob/v1.9.0-rc2/substrate/frame/proxy/src/lib.rs#L260
                    RuntimeCall::Proxy(method) => !matches!(
                        method,
                        pallet_proxy::Call::create_pure { .. }
                            | pallet_proxy::Call::kill_pure { .. }
                            | pallet_proxy::Call::remove_proxies { .. }
                    ),
                    _ => true,
                }
            }
        }

        /// The default types are being injected by [`derive_impl`](`frame_support::derive_impl`) from
        /// [`ParaChainDefaultConfig`](`struct@frame_system::config_preludes::ParaChainDefaultConfig`),
        /// but overridden as needed.
        #[derive_impl(frame_system::config_preludes::ParaChainDefaultConfig as frame_system::DefaultConfig)]
        impl frame_system::Config for Runtime {
            /// The data to be stored in an account.
            type AccountData = pallet_balances::AccountData<Balance>;
            /// The identifier used to distinguish between accounts.
            type AccountId = <$t as SystemConfig>::AccountId;
            /// The basic call filter to use in dispatchable.
            type BaseCallFilter = NormalFilter;
            /// The block type.
            type Block = Block;
            /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
            type BlockHashCount = BlockHashCount;
            /// The maximum length of a block (in bytes).
            type BlockLength = RuntimeBlockLength;
            /// Block & extrinsics weights: base values and limits.
            type BlockWeights = RuntimeBlockWeights;
            /// The weight of database operations that the runtime can invoke.
            type DbWeight = RocksDbWeight;
            /// The type for hashing blocks and tries.
            type Hash = Hash;
            /// The lookup mechanism to get account ID from whatever is passed in
            /// dispatchers.
            type Lookup = AccountIdLookup<Self::AccountId, ()>;
            /// The maximum number of consumers allowed on a single account.
            type MaxConsumers = ConstU32<16>;
            /// The index type for storing how many extrinsics an account has signed.
            type Nonce = Nonce;
            /// The action to take on a Runtime Upgrade
            type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
            /// Converts a module to an index of this module in the runtime.
            type PalletInfo = PalletInfo;
            /// The aggregated dispatch type that is available for extrinsics.
            type RuntimeCall = RuntimeCall;
            /// The ubiquitous event type.
            type RuntimeEvent = RuntimeEvent;
            /// The ubiquitous origin type.
            type RuntimeOrigin = RuntimeOrigin;
            /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
            type SS58Prefix = <$t as SystemConfig>::SS58Prefix;
            /// Runtime version.
            type Version = <$t as SystemConfig>::Version;
        }

        impl pallet_timestamp::Config for Runtime {
            #[cfg(feature = "experimental")]
            type MinimumPeriod = ConstU64<0>;
            #[cfg(not(feature = "experimental"))]
            type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
            /// A timestamp: milliseconds since the unix epoch.
            type Moment = u64;
            type OnTimestampSet = Aura;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
        }
    };
}
