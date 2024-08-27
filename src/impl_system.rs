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
            pub const MaxProxies: u32 = 32;
            pub const MaxPending: u32 = 32;
            pub const ProxyDepositBase: Balance = deposit(1, 40);
            pub const AnnouncementDepositBase: Balance = deposit(1, 48);
            pub const ProxyDepositFactor: Balance = deposit(0, 33);
            pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
        }

        /// The type used to represent the kinds of proxying allowed.
        /// If you are adding new pallets, consider adding new ProxyType variant
        #[derive(
            Copy,
            Clone,
            Decode,
            Default,
            Encode,
            Eq,
            MaxEncodedLen,
            Ord,
            PartialEq,
            PartialOrd,
            RuntimeDebug,
            TypeInfo,
        )]
        pub enum ProxyType {
            /// Allows to proxy all calls
            #[default]
            Any,
            /// Allows all non-transfer calls
            NonTransfer,
            /// Allows to finish the proxy
            CancelProxy,
            /// Allows to operate with collators list (invulnerables, candidates, etc.)
            Collator,
        }

        impl InstanceFilter<RuntimeCall> for ProxyType {
            fn filter(&self, c: &RuntimeCall) -> bool {
                match self {
                    ProxyType::Any => true,
                    ProxyType::NonTransfer => !matches!(c, RuntimeCall::Balances { .. }),
                    ProxyType::CancelProxy => matches!(
                        c,
                        RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })
                            | RuntimeCall::Multisig { .. }
                    ),
                    ProxyType::Collator => {
                        matches!(c, RuntimeCall::CollatorSelection { .. } | RuntimeCall::Multisig { .. })
                    }
                }
            }
        }

        impl pallet_proxy::Config for Runtime {
            type AnnouncementDepositBase = AnnouncementDepositBase;
            type AnnouncementDepositFactor = AnnouncementDepositFactor;
            type CallHasher = BlakeTwo256;
            type Currency = Balances;
            type MaxPending = MaxPending;
            type MaxProxies = MaxProxies;
            type ProxyDepositBase = ProxyDepositBase;
            type ProxyDepositFactor = ProxyDepositFactor;
            type ProxyType = ProxyType;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_proxy::WeightInfo<Runtime>;
        }

        parameter_types! {
            pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
            pub const MaxFreezes: u32 = 0;
            pub const MaxLocks: u32 = 50;
            pub const MaxReserves: u32 = 50;
        }

        impl pallet_balances::Config for Runtime {
            type AccountStore = System;
            /// The type for recording an account's balance.
            type Balance = Balance;
            type DustRemoval = ();
            type ExistentialDeposit = ExistentialDeposit;
            type FreezeIdentifier = ();
            type MaxFreezes = MaxFreezes;
            type MaxLocks = MaxLocks;
            type MaxReserves = MaxReserves;
            type ReserveIdentifier = [u8; 8];
            /// The ubiquitous event type.
            type RuntimeEvent = RuntimeEvent;
            type RuntimeFreezeReason = RuntimeFreezeReason;
            type RuntimeHoldReason = RuntimeHoldReason;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
        }
    };
}
