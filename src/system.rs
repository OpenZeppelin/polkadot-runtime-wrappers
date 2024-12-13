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

        // The default types are being injected by [`derive_impl`](`frame_support::derive_impl`) from
        // [`ParaChainDefaultConfig`](`struct@frame_system::config_preludes::ParaChainDefaultConfig`),
        // but overridden as needed.
        #[derive_impl(frame_system::config_preludes::ParaChainDefaultConfig as frame_system::DefaultConfig)]
        impl frame_system::Config for Runtime {
            // The data to be stored in an account.
            type AccountData = pallet_balances::AccountData<Balance>;
            // The identifier used to distinguish between accounts.
            type AccountId = <$t as SystemConfig>::AccountId;
            // The basic call filter to use in dispatchable.
            type BaseCallFilter = NormalFilter;
            // The block type.
            type Block = Block;
            // Maximum number of block number to block hash mappings to keep (oldest pruned first).
            type BlockHashCount = BlockHashCount;
            // The maximum length of a block (in bytes).
            type BlockLength = RuntimeBlockLength;
            // Block & extrinsics weights: base values and limits.
            type BlockWeights = RuntimeBlockWeights;
            // The weight of database operations that the runtime can invoke.
            type DbWeight = <$t as SystemWeight>::DbWeight;
            // The type for hashing blocks and tries.
            type Hash = Hash;
            // The lookup mechanism to get account ID from whatever is passed in
            // dispatchers.
            type Lookup = <$t as SystemConfig>::Lookup;
            // The maximum number of consumers allowed on a single account.
            type MaxConsumers = <$t as SystemConfig>::MaxConsumers;
            // The index type for storing how many extrinsics an account has signed.
            type Nonce = Nonce;
            // The action to take on a Runtime Upgrade
            type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
            // Converts a module to an index of this module in the runtime.
            type PalletInfo = PalletInfo;
            // The aggregated dispatch type that is available for extrinsics.
            type RuntimeCall = RuntimeCall;
            // The ubiquitous event type.
            type RuntimeEvent = RuntimeEvent;
            // The ubiquitous origin type.
            type RuntimeOrigin = RuntimeOrigin;
            // This is used as an identifier of the chain. 42 is the generic substrate prefix.
            type SS58Prefix = <$t as SystemConfig>::SS58Prefix;
            // Runtime version.
            type Version = <$t as SystemConfig>::Version;
        }

        // A pallet that provides a way for consensus systems to set and check the onchain time.
        impl pallet_timestamp::Config for Runtime {
            // Timestamp must increment by at least <MinimumPeriod> between sequential blocks
            type MinimumPeriod = <$t as SystemConfig>::SlotDuration;
            // A timestamp: milliseconds since the unix epoch.
            type Moment = u64;
            // The Config::OnTimestampSet configuration trait can be set to another pallet we want to notify that the
            // timestamp has been updated, as long as it implements OnTimestampSet.
            type OnTimestampSet = <$t as SystemConfig>::OnTimestampSet;
            // Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = <$t as SystemWeight>::Timestamp;
        }

        impl parachain_info::Config for Runtime {}

        parameter_types! {
            pub MaximumSchedulerWeight: frame_support::weights::Weight = Perbill::from_percent(80) *
                RuntimeBlockWeights::get().max_block;
            pub const MaxScheduledRuntimeCallsPerBlock: u32 = 50;
        }

        // A Pallet for scheduling runtime calls.
        impl pallet_scheduler::Config for Runtime {
            // The maximum number of scheduled calls in the queue for a single block.
            type MaxScheduledPerBlock = MaxScheduledRuntimeCallsPerBlock;
            // The maximum weight that may be scheduled per block for any dispatchables.
            type MaximumWeight = MaximumSchedulerWeight;
            // Compare the privileges of origins. This will be used when canceling a task,
            // to ensure that the origin that tries to cancel has greater or equal privileges as the origin that created the scheduled task.
            type OriginPrivilegeCmp = frame_support::traits::EqualPrivilegeOnly;
            // The caller origin, overarching type of all pallets origins.
            type PalletsOrigin = OriginCaller;
            // The preimage provider with which we look up call hashes to get the call.
            type Preimages = Preimage;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            // The aggregated origin which the dispatch will take.
            type RuntimeOrigin = RuntimeOrigin;
            // Required origin to schedule or cancel calls.
            type ScheduleOrigin = <$t as SystemConfig>::ScheduleOrigin;
            type WeightInfo = <$t as SystemWeight>::Scheduler;
        }

        parameter_types! {
            pub const PreimageBaseDeposit: Balance = deposit(2, 64);
            pub const PreimageByteDeposit: Balance = deposit(0, 1);
            pub const PreimageHoldReason: RuntimeHoldReason = RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
        }

        // The Preimage pallet allows for the users and the runtime to store the preimage of a hash on chain.
        // This can be used by other pallets for storing and managing large byte-blobs.
        impl pallet_preimage::Config for Runtime {
            // A means of providing some cost while data is stored on-chain.
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
            // Currency type for this pallet.
            type Currency = Balances;
            // An origin that can request a preimage be placed on-chain without a deposit or fee, or manage existing preimages.
            type ManagerOrigin = <$t as SystemConfig>::PreimageOrigin;
            // The overarching event type.
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as SystemWeight>::Preimage;
        }

        parameter_types! {
            pub const ProxyDepositBase: Balance = deposit(1, 40);
            pub const AnnouncementDepositBase: Balance = deposit(1, 48);
            pub const ProxyDepositFactor: Balance = deposit(0, 33);
            pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
        }
        // A pallet allowing accounts to give permission to other accounts to dispatch types of calls from their signed origin.
        impl pallet_proxy::Config for Runtime {
            // The base amount of currency needed to reserve for creating an announcement.
            type AnnouncementDepositBase = AnnouncementDepositBase;
            // The amount of currency needed per announcement made.
            type AnnouncementDepositFactor = AnnouncementDepositFactor;
            // The type of hash used for hashing the call.
            type CallHasher = BlakeTwo256;
            // The currency mechanism.
            type Currency = Balances;
            // The maximum amount of time-delayed announcements that are allowed to be pending.
            type MaxPending = <$t as SystemConfig>::MaxPendingProxies;
            // The maximum amount of proxies allowed for a single account.
            type MaxProxies = <$t as SystemConfig>::MaxProxies;
            // The base amount of currency needed to reserve for creating a proxy.
            type ProxyDepositBase = ProxyDepositBase;
            // The amount of currency needed per proxy added.
            type ProxyDepositFactor = ProxyDepositFactor;
            // A kind of proxy; specified with the proxy and passed in to the `IsProxyable` filter.
            type ProxyType = <$t as SystemConfig>::ProxyType;
            // The overarching call type.
            type RuntimeCall = RuntimeCall;
            // The overarching event type.
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as SystemWeight>::Proxy;
        }


        // The Balances pallet provides functionality for handling accounts and balances for a single token.
        impl pallet_balances::Config for Runtime {
            // The means of storing the balances of an account.
            type AccountStore = System;
            // The type for recording an account's balance.
            type Balance = Balance;
            // Handler for the unbalanced reduction when removing a dust account.
            type DustRemoval = ();
            // The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!
            type ExistentialDeposit = <$t as SystemConfig>::ExistentialDeposit;
            // The ID type for freezes.
            type FreezeIdentifier = ();
            // The maximum number of individual freeze locks that can exist on an account at any time.
            type MaxFreezes = <$t as SystemConfig>::MaxFreezes;
            // The maximum number of locks that should exist on an account. Not strictly enforced but used for weight estimation.
            type MaxLocks = <$t as SystemConfig>::MaxLocks;
            // The maximum number of named reserves that can exist on an account.
            type MaxReserves = <$t as SystemConfig>::MaxReserves;
            // The ID type for reserves. Use of reserves is deprecated in favour of holds.
            type ReserveIdentifier = [u8; 8];
            // The ubiquitous event type.
            type RuntimeEvent = RuntimeEvent;
            // The overarching freeze reason.
            type RuntimeFreezeReason = RuntimeFreezeReason;
            // The overarching hold reason.
            type RuntimeHoldReason = RuntimeHoldReason;
            type WeightInfo = <$t as SystemWeight>::Balances;
        }

        // A stateless pallet with helpers for dispatch management which does no re-authentication.
        impl pallet_utility::Config for Runtime {
            // The caller origin, overarching type of all pallets origins.
            type PalletsOrigin = OriginCaller;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as SystemWeight>::Utility;
        }

        parameter_types! {
            pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
            pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
            pub const RelayOrigin: AggregateMessageOrigin = AggregateMessageOrigin::Parent;
        }

        // base pallet for Cumulus-based parachains.
        impl cumulus_pallet_parachain_system::Config for Runtime {
            // Checks if the associated relay parent block number is valid. Depending on the feature, it ensures the relay number increases as expected.
            #[cfg(not(feature = "async-backing"))]
            type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
            #[cfg(feature = "async-backing")]
            type CheckAssociatedRelayNumber = RelayNumberMonotonicallyIncreases;
            // An entry-point for managing the backlog of unincluded parachain blocks and authorship rights for those blocks.
            type ConsensusHook = <$t as SystemConfig>::ConsensusHook;
            // Queues inbound downward messages for delayed processing.
            // All inbound DMP messages from the relay are pushed into this.
            // The handler is expected to eventually process all the messages that are pushed to it.
            type DmpQueue = frame_support::traits::EnqueueWithOrigin<MessageQueue, RelayOrigin>;
            // Something which can be notified when the validation data is set.
            type OnSystemEvent = ();
            // The place where outbound XCMP messages come from. This is queried in `finalize_block`.
            type OutboundXcmpMessageSource = XcmpQueue;
            // The weight reserved at the beginning of the block for processing DMP messages.
            type ReservedDmpWeight = ReservedDmpWeight;
            // The weight reserved at the beginning of the block for processing XCMP messages.
            type ReservedXcmpWeight = ReservedXcmpWeight;
            // The overarching event type.
            type RuntimeEvent = RuntimeEvent;
            // Returns the parachain ID we are running with.
            type SelfParaId = parachain_info::Pallet<Runtime>;
            type WeightInfo = <$t as SystemWeight>::ParachainSystem;
            // The message handler that will be invoked when messages are received via XCMP.
            type XcmpMessageHandler = XcmpQueue;
        }


        parameter_types! {
            // One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
            pub const DepositBase: Balance = deposit(1, 88);
            // Additional storage item size of 32 bytes.
            pub const DepositFactor: Balance = deposit(0, 32);
        }

        // A pallet for doing multisig dispatch.
        impl pallet_multisig::Config for Runtime {
            // The currency mechanism.
            type Currency = Balances;
            // The base amount of currency needed to reserve for creating a multisig execution or to store a dispatch call for later.
            type DepositBase = DepositBase;
            // The amount of currency needed per unit threshold when creating a multisig execution.
            type DepositFactor = DepositFactor;
            // The maximum amount of signatories allowed in the multisig.
            type MaxSignatories = <$t as SystemConfig>::MaxSignatories;
            // The overarching call type.
            type RuntimeCall = RuntimeCall;
            // The overarching event type.
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as SystemWeight>::Multisig;
        }

    };
}

pub const PALLET_NAMES: [(&str, &str); 10] = [
    ("System", "frame_system"),
    ("Timestamp", "pallet_timestamp"),
    ("ParachainInfo", "parachain_info"),
    ("Scheduler", "pallet_scheduler"),
    ("Preimage", "pallet_preimage"),
    ("Proxy", "pallet_proxy"),
    ("Balances", "pallet_balances"),
    ("Utility", "pallet_utility"),
    ("ParachainSystem", "cumulus_pallet_parachain_system"),
    ("Multisig", "pallet_multisig"),
];
