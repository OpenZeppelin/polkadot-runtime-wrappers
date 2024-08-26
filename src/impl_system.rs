//! OpenZeppelin System Pallets Wrapper

#[macro_export]
macro_rules! impl_oz_system {
	($runtime:ty, $pallet_info:ty, $runtime_call:ty, $runtime_event:ty) => {
        use frame_support::{
            derive_impl,
            dispatch::DispatchClass,
            parameter_types,
            traits::{
                AsEnsureOriginWithArg, ConstU32, ConstU64, Contains, EitherOfDiverse, InstanceFilter,
                TransformOrigin,
            },
            weights::{ConstantMultiplier, Weight},
            PalletId,
        };
        use frame_system::{
            limits::{BlockLength, BlockWeights},
            EnsureRoot, EnsureSigned,
        };

        parameter_types! {
            pub const Version: RuntimeVersion = VERSION;

            // This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
            //  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
            // `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
            // the lazy contract deletion.
            pub RuntimeBlockLength: BlockLength =
                BlockLength::max_with_normal_ratio(MAX_BLOCK_LENGTH, NORMAL_DISPATCH_RATIO);
            pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
                .base_block(BlockExecutionWeight::get())
                .for_class(DispatchClass::all(), |weights| {
                    weights.base_extrinsic = ExtrinsicBaseWeight::get();
                })
                .for_class(DispatchClass::Normal, |weights| {
                    weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
                })
                .for_class(DispatchClass::Operational, |weights| {
                    weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
                    // Operational transactions have some extra reserved space, so that they
                    // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
                    weights.reserved = Some(
                        MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
                    );
                })
                .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
                .build_or_panic();
            // generic substrate prefix. For more info, see: [Polkadot Accounts In-Depth](https://wiki.polkadot.network/docs/learn-account-advanced#:~:text=The%20address%20format%20used%20in,belonging%20to%20a%20specific%20network)
            pub const SS58Prefix: u16 = 42;
        }

        pub struct NormalFilter;
        impl Contains<$runtime_call> for NormalFilter {
            fn contains(c: &$runtime_call) -> bool {
                match c {
                    // We filter anonymous proxy as they make "reserve" inconsistent
                    // See: https://github.com/paritytech/polkadot-sdk/blob/v1.9.0-rc2/substrate/frame/proxy/src/lib.rs#L260
                    $runtime_call::Proxy(method) => !matches!(
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
        impl frame_system::Config for $runtime {
            /// The data to be stored in an account.
            type AccountData = pallet_balances::AccountData<Balance>;
            /// The identifier used to distinguish between accounts.
            type AccountId = AccountId;
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
            type Lookup = AccountIdLookup<AccountId, ()>;
            /// The maximum number of consumers allowed on a single account.
            type MaxConsumers = ConstU32<16>;
            /// The index type for storing how many extrinsics an account has signed.
            type Nonce = Nonce;
            /// The action to take on a Runtime Upgrade
            type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
            /// Converts a module to an index of this module in the runtime.
            type PalletInfo = $pallet_info;
            /// The aggregated dispatch type that is available for extrinsics.
            type RuntimeCall = $runtime_call;
            /// The ubiquitous event type.
            type RuntimeEvent = $runtime_event;
            /// The ubiquitous origin type.
            type RuntimeOrigin = $runtime_origin;
            /// TODO: PASS IN AS INPUT
            /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
            type SS58Prefix = SS58Prefix;
            /// TODO: PASS IN AS INPUT
            /// Runtime version.
            type Version = Version;
        }
    }
}
