//! Implements the OpenZeppelin system configuration for a Runtime.
//!
//! This macro sets up the necessary configurations for the following pallets:
//! - `frame_system`
//! - `pallet_timestamp`
//! - `parachain_info`
//! - `pallet_scheduler`
//! - `pallet_preimage`
//! - `pallet_proxy`
//! - `pallet_balances`
//! - `pallet_utility`
//! - `cumulus_pallet_parachain_system`
//! - `pallet_multisig`
//!
//! # Parameters
//! - `$t`: A type that implements the `SystemConfig` trait, providing the necessary associated types
//!   and configurations for core system functionality.
//!
//! # Important
//! Rerun benchmarks if making changes to runtime configuration, as weight calculations
//! may need to be updated.

#[macro_export]
macro_rules! impl_openzeppelin_system {
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

        parameter_types!{
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
            type Lookup = <$t as SystemConfig>::Lookup;
            /// The maximum number of consumers allowed on a single account.
            type MaxConsumers = <$t as SystemConfig>::MaxConsumers;
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
            type MinimumPeriod = ConstU64<0>;
            /// A timestamp: milliseconds since the unix epoch.
            type Moment = u64;
            type OnTimestampSet = Aura;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
        }

        impl parachain_info::Config for Runtime {}

        parameter_types! {
            pub MaximumSchedulerWeight: frame_support::weights::Weight = Perbill::from_percent(80) *
                RuntimeBlockWeights::get().max_block;
            pub const MaxScheduledRuntimeCallsPerBlock: u32 = 50;
        }

        impl pallet_scheduler::Config for Runtime {
            type MaxScheduledPerBlock = MaxScheduledRuntimeCallsPerBlock;
            type MaximumWeight = MaximumSchedulerWeight;
            type OriginPrivilegeCmp = frame_support::traits::EqualPrivilegeOnly;
            type PalletsOrigin = OriginCaller;
            type Preimages = Preimage;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            type RuntimeOrigin = RuntimeOrigin;
            type ScheduleOrigin = <$t as SystemConfig>::ScheduleOrigin;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
        }

        parameter_types! {
            pub const PreimageBaseDeposit: Balance = deposit(2, 64);
            pub const PreimageByteDeposit: Balance = deposit(0, 1);
            pub const PreimageHoldReason: RuntimeHoldReason = RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
        }

        impl pallet_preimage::Config for Runtime {
            type Consideration = frame_support::traits::fungible::HoldConsideration<
                AccountId,
                Balances,
                PreimageHoldReason,
                frame_support::traits::LinearStoragePrice<
                    PreimageBaseDeposit,
                    PreimageByteDeposit,
                    Balance,
                >,
            >;
            type Currency = Balances;
            type ManagerOrigin = <$t as SystemConfig>::PreimageOrigin;
            type RuntimeEvent = RuntimeEvent;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
        }

        parameter_types! {
            pub const ProxyDepositBase: Balance = deposit(1, 40);
            pub const AnnouncementDepositBase: Balance = deposit(1, 48);
            pub const ProxyDepositFactor: Balance = deposit(0, 33);
            pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
        }

        impl pallet_proxy::Config for Runtime {
            type AnnouncementDepositBase = AnnouncementDepositBase;
            type AnnouncementDepositFactor = AnnouncementDepositFactor;
            type CallHasher = BlakeTwo256;
            type Currency = Balances;
            type MaxPending = <$t as SystemConfig>::MaxPendingProxies;
            type MaxProxies = <$t as SystemConfig>::MaxProxies;
            type ProxyDepositBase = ProxyDepositBase;
            type ProxyDepositFactor = ProxyDepositFactor;
            type ProxyType = <$t as SystemConfig>::ProxyType;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_proxy::WeightInfo<Runtime>;
        }

        impl pallet_balances::Config for Runtime {
            type AccountStore = System;
            /// The type for recording an account's balance.
            type Balance = Balance;
            type DustRemoval = ();
            type ExistentialDeposit = <$t as SystemConfig>::ExistentialDeposit;
            type FreezeIdentifier = ();
            type MaxFreezes = <$t as SystemConfig>::MaxFreezes;
            type MaxLocks = <$t as SystemConfig>::MaxLocks;
            type MaxReserves = <$t as SystemConfig>::MaxReserves;
            type ReserveIdentifier = [u8; 8];
            /// The ubiquitous event type.
            type RuntimeEvent = RuntimeEvent;
            type RuntimeFreezeReason = RuntimeFreezeReason;
            type RuntimeHoldReason = RuntimeHoldReason;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
        }

        impl pallet_utility::Config for Runtime {
            type PalletsOrigin = OriginCaller;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
        }

        parameter_types! {
            pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
            pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
            pub const RelayOrigin: AggregateMessageOrigin = AggregateMessageOrigin::Parent;
        }

        impl cumulus_pallet_parachain_system::Config for Runtime {
            #[cfg(not(feature = "async-backing"))]
            type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
            #[cfg(feature = "async-backing")]
            type CheckAssociatedRelayNumber = RelayNumberMonotonicallyIncreases;
            type ConsensusHook = ConsensusHook;
            type DmpQueue = frame_support::traits::EnqueueWithOrigin<MessageQueue, RelayOrigin>;
            type OnSystemEvent = ();
            type OutboundXcmpMessageSource = XcmpQueue;
            type ReservedDmpWeight = ReservedDmpWeight;
            type ReservedXcmpWeight = ReservedXcmpWeight;
            type RuntimeEvent = RuntimeEvent;
            type SelfParaId = parachain_info::Pallet<Runtime>;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::cumulus_pallet_parachain_system::WeightInfo<Runtime>;
            type XcmpMessageHandler = XcmpQueue;
        }

        parameter_types! {
            // One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
            pub const DepositBase: Balance = deposit(1, 88);
            // Additional storage item size of 32 bytes.
            pub const DepositFactor: Balance = deposit(0, 32);
        }

        impl pallet_multisig::Config for Runtime {
            type Currency = Balances;
            type DepositBase = DepositBase;
            type DepositFactor = DepositFactor;
            type MaxSignatories = <$t as SystemConfig>::MaxSignatories;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_multisig::WeightInfo<Runtime>;
        }
    };
}
