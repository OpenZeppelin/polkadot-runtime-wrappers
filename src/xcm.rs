//! Implements the OpenZeppelin XCM configuration for a Runtime.
//!
//! This macro sets up the necessary configurations for the following pallets:
//! - `pallet_message_queue`
//! - `cumulus_pallet_xcmp_queue`
//! - `pallet_xcm`
//! - `cumulus_pallet_xcm`
//! - `pallet_xcm_weight_trader`
//! - `orml_xtokens`
//! - `pallet_xcm_transactor`
//!
//! # Parameters
//! - `$t`: A type that implements the `XcmConfig` trait, providing the necessary associated types
//!   and configurations for cross-chain messaging functionality.
//!
//! # Important
//! Rerun benchmarks if making changes to runtime configuration, as weight calculations
//! may need to be updated.

#[macro_export]
macro_rules! impl_openzeppelin_xcm {
    ($t:ty) => {
        // Provides generalized message queuing and processing capabilities on a per-queue basis for arbitrary use-cases.
        impl pallet_message_queue::Config for Runtime {
            // The size of the page; this implies the maximum message size which can be sent.
            type HeapSize = <$t as XcmConfig>::MessageQueueHeapSize;
            // The maximum amount of weight (if any) to be used from remaining weight `on_idle` to service enqueued items.
            type IdleMaxServiceWeight = <$t as XcmConfig>::MessageQueueServiceWeight;
            // The maximum number of stale pages (i.e., of overweight messages) allowed before culling can happen.
            type MaxStale = <$t as XcmConfig>::MessageQueueMaxStale;
            // Processor for a message. Storage changes are not rolled back on error.
            #[cfg(feature = "runtime-benchmarks")]
            type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor<cumulus_primitives_core::AggregateMessageOrigin>;
            #[cfg(not(feature = "runtime-benchmarks"))]
            type MessageProcessor = ProcessXcmMessage<AggregateMessageOrigin, xcm_executor::XcmExecutor<XcmExecutorConfig>, RuntimeCall>;
            // Code to be called when a message queue changes - either with items introduced or removed.
            type QueueChangeHandler = NarrowOriginToSibling<XcmpQueue>;
            // Queried by the pallet to check whether a queue can be serviced.
            type QueuePausedQuery = NarrowOriginToSibling<XcmpQueue>;
            // The overarching event type.
            type RuntimeEvent = RuntimeEvent;
            // The amount of weight (if any) provided to the message queue for servicing enqueued items `on_initialize`.
            type ServiceWeight = <$t as XcmConfig>::MessageQueueServiceWeight;
            // Page/heap size type.
            type Size = u32;
            type WeightInfo = <$t as XcmWeight>::MessageQueue;
        }


        parameter_types! {
            // The asset ID for the asset that we use to pay for message delivery fees.
            pub FeeAssetId: cumulus_primitives_core::AssetId = cumulus_primitives_core::AssetId(Location::parent());
            // The base fee for the message delivery fees. Kusama is based for the reference.
            pub const ToSiblingBaseDeliveryFee: u128 = CENTS.saturating_mul(3);
        }

        // A pallet which uses the XCMP transport layer to handle both incoming and outgoing XCM message sending and dispatch,
        // queuing, signalling and backpressure.
        impl cumulus_pallet_xcmp_queue::Config for Runtime {
            // Information on the available XCMP channels.
            type ChannelInfo = ParachainSystem;
            // The origin that is allowed to resume or suspend the XCMP queue.
            type ControllerOrigin = <$t as XcmConfig>::XcmpQueueControllerOrigin;
            // Conversion function to convert an XCM `Location` origin to a superuser origin.
            type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
            // Maximal number of outbound XCMP channels that can have messages queued at the same time.
            type MaxActiveOutboundChannels = <$t as XcmConfig>::MaxActiveOutboundChannels;
            // The maximum number of inbound XCMP channels that can be suspended simultaneously.
            type MaxInboundSuspended = <$t as XcmConfig>::XcmpQueueMaxInboundSuspended;
            // The maximal page size for HRMP message pages, determining the upper limit for the PoV worst-case size.
            type MaxPageSize = <$t as XcmConfig>::MaxPageSize;
            // Price model for delivering an XCM to a sibling parachain destination.
            // This ensures that messages incur a cost to prevent spamming.
            type PriceForSiblingDelivery = PriceForSiblingParachainDelivery;
            // The overarching event type for the pallet.
            type RuntimeEvent = RuntimeEvent;
            // Means of converting an `Xcm` into a `VersionedXcm`.
            // This can be updated for runtime-specific handling, or left as a no-op `()` as used here.
            type VersionWrapper = ();
            type WeightInfo = <$t as XcmWeight>::XcmpQueue;
            // Handles enqueuing XCMP messages from sibling parachains for later processing.
            type XcmpQueue =
                TransformOrigin<MessageQueue, AggregateMessageOrigin, ParaId, ParaIdToSibling>;
        }


        parameter_types! {
            pub PlaceholderAccount: AccountId = PolkadotXcm::check_account();
            pub UniversalLocation: InteriorLocation = Parachain(ParachainInfo::parachain_id().into()).into();
        }

        parameter_types! {
            // One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
            pub const UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
            pub const MaxInstructions: u32 = 100;
            pub const MaxAssetsIntoHolding: u32 = 64;
        }

        pub struct ParentOrParentsExecutivePlurality;
        impl Contains<Location> for ParentOrParentsExecutivePlurality {
            fn contains(location: &Location) -> bool {
                matches!(location.unpack(), (1, []) | (1, [Plurality { id: BodyId::Executive, .. }]))
            }
        }

        pub type Barrier = TrailingSetTopicAsId<
            DenyThenTry<
                DenyReserveTransferToRelayChain,
                (
                    TakeWeightCredit,
                    WithComputedOrigin<
                        (
                            AllowTopLevelPaidExecutionFrom<Everything>,
                            AllowExplicitUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
                            // ^^^ Parent and its exec plurality get free execution
                        ),
                        UniversalLocation,
                        ConstU32<8>,
                    >,
                ),
            >,
        >;

        pub struct XcmExecutorConfig;
        impl xcm_executor::Config for XcmExecutorConfig {
            type Aliasers = Nothing;
            // Handles asset claims, integrated with PolkadotXcm for cross-chain asset handling.
            type AssetClaims = PolkadotXcm;
            type AssetExchanger = ();
            type AssetLocker = ();
            // Handles asset transactions, such as deposits and withdrawals.
            type AssetTransactor = <$t as XcmConfig>::AssetTransactors;
            // Drops assets left in the Holding Register at the end of XCM execution.
            type AssetTrap = PolkadotXcm;
            // Barrier that decides whether an XCM can be executed.
            type Barrier = Barrier;
            // Dispatches runtime calls specified in the XCM.
            type CallDispatcher = RuntimeCall;
            // Fee management logic, to ensure costs are collected during transactions.
            type FeeManager = <$t as XcmConfig>::FeeManager;
            type HrmpChannelAcceptedHandler = ();
            type HrmpChannelClosingHandler = ();
            type HrmpNewChannelOpenRequestHandler = ();
            // Filters which combinations of locations and assets are considered reserves.
            type IsReserve = <$t as XcmConfig>::Reserves;
            // Determines which combinations of locations and assets are teleporters, currently disabled.
            type IsTeleporter = ();
            // Limits the maximum number of assets that can be placed in the Holding Register.
            type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
            // Exports messages
            type MessageExporter = ();
            // Converts XCM origin to runtime dispatch origin.
            type OriginConverter = <$t as XcmConfig>::XcmOriginToTransactDispatchOrigin;
            // Provides information about all runtime pallets.
            type PalletInstancesInfo = AllPalletsWithSystem;
            // Handles responses to XCM queries.
            type ResponseHandler = PolkadotXcm;
            // Represents a runtime call type.
            type RuntimeCall = RuntimeCall;
            // Filter to determine whether a runtime call is allowed; here, all calls are allowed.
            type SafeCallFilter = Everything;
            // Handles subscription requests for XCM version changes.
            type SubscriptionService = PolkadotXcm;
            // Determines how to trade weight for message execution costs.
            type Trader = <$t as XcmConfig>::Trader;
            // Processes XCM transactions with rollback capability in case of failure.
            type TransactionalProcessor = FrameTransactionalProcessor;
            type UniversalAliases = Nothing;
            // Defines the universal location for the XCM.
            type UniversalLocation = UniversalLocation;
            // Provides weight measurement logic for XCM execution.
            type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
            // Records XCM execution for diagnostic or auditing purposes.
            type XcmRecorder = PolkadotXcm;
            // Sends XCM messages, using a router to determine the appropriate destination.
            type XcmSender = XcmRouter;
        }


        // The means for routing XCM messages which are not for local execution into
        // the right message queues.
        pub type XcmRouter = WithUniqueTopic<(
            // Two routers - use UMP to communicate with the relay chain:
            cumulus_primitives_utility::ParentAsUmp<ParachainSystem, (), ()>,
            // ..and XCMP to communicate with the sibling chains.
            XcmpQueue,
        )>;

        parameter_types! {
            pub const MaxLockers: u32 = 8;
            pub const MaxRemoteLockConsumers: u32 = 0;
        }

        // Pallet to handle XCM messages.
        impl pallet_xcm::Config for Runtime {
            // Origin authorized for privileged XCM operations.
            type AdminOrigin = <$t as XcmConfig>::XcmAdminOrigin;
            // Advertised XCM version to other chains.
            type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
            // Lockable currency for managing tokens.
            type Currency = Balances;
            // Matcher for fungible assets, unused here.
            type CurrencyMatcher = ();
            // Origin allowed to execute XCM messages.
            type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, <$t as XcmConfig>::LocalOriginToLocation>;
            // Maximum number of local XCM locks per account.
            type MaxLockers = MaxLockers;
            // Maximum number of consumers for a single remote lock.
            type MaxRemoteLockConsumers = MaxRemoteLockConsumers;
            // Identifier for remote lock consumers, unused here.
            type RemoteLockConsumerIdentifier = ();
            // Runtime call type.
            type RuntimeCall = RuntimeCall;
            // Runtime event type.
            type RuntimeEvent = RuntimeEvent;
            // Runtime origin type.
            type RuntimeOrigin = RuntimeOrigin;
            // Origin allowed to send XCM messages.
            type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, <$t as XcmConfig>::LocalOriginToLocation>;
            // Converts XCM locations to sovereign account IDs.
            type SovereignAccountOf = <$t as XcmConfig>::LocationToAccountId;
            // Assets trusted to have locks by an origin, unused here.
            type TrustedLockers = ();
            // This chain's universal location for XCM purposes.
            type UniversalLocation = UniversalLocation;
            // Determines weight for XCM execution.
            type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
            type WeightInfo = <$t as XcmWeight>::Xcm;
            // Filter for executable XCM messages, adjusted for testing and benchmarks.
            #[cfg(feature = "runtime-benchmarks")]
            type XcmExecuteFilter = Everything;
            #[cfg(not(feature = "runtime-benchmarks"))]
            type XcmExecuteFilter = Nothing;
            // XCM executor configuration.
            type XcmExecutor = XcmExecutor<XcmExecutorConfig>;
            // Filter for reserve-transferable XCM messages.
            type XcmReserveTransferFilter = Everything;
            // Router to send XCM messages to their destinations.
            type XcmRouter = XcmRouter;
            // Filter for teleportable XCM messages, disabled here.
            type XcmTeleportFilter = Nothing;

            // Maximum number of queued version discovery requests.
            const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
        }


        impl cumulus_pallet_xcm::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type XcmExecutor = XcmExecutor<XcmExecutorConfig>;
        }

        // A pallet to trade weight for XCM execution costs.
        impl pallet_xcm_weight_trader::Config for Runtime {
            // Conversion logic from AccountId to XCM Location.
            type AccountIdToLocation = <$t as XcmConfig>::AccountIdToLocation;
            // Origin that can register supported assets.
            type AddSupportedAssetOrigin = <$t as XcmConfig>::AddSupportedAssetOrigin;
            // Filter for asset locations that should be supported for fees.
            type AssetLocationFilter = <$t as XcmConfig>::AssetFeesFilter;
            // Mechanism for withdrawing and depositing assets.
            type AssetTransactor = <$t as XcmConfig>::AssetTransactors;
            // The balance type for handling asset amounts.
            type Balance = Balance;
            // Origin that can edit units per second of a supported asset.
            type EditSupportedAssetOrigin = <$t as XcmConfig>::EditSupportedAssetOrigin;
            // XCM Location that represents the native currency.
            type NativeLocation = <$t as XcmConfig>::SelfReserve;
            // For benchmarking, a location that passes the asset location filter.
            #[cfg(feature = "runtime-benchmarks")]
            type NotFilteredLocation = <$t as XcmConfig>::RelayLocation;
            // Origin that can pause a supported asset.
            type PauseSupportedAssetOrigin = <$t as XcmConfig>::PauseSupportedAssetOrigin;
            // Origin that can remove a supported asset.
            type RemoveSupportedAssetOrigin = <$t as XcmConfig>::RemoveSupportedAssetOrigin;
            // Origin that can unpause a supported asset.
            type ResumeSupportedAssetOrigin = <$t as XcmConfig>::ResumeSupportedAssetOrigin;
            // The event type for this pallet.
            type RuntimeEvent = RuntimeEvent;
            // Weight information for extrinsics in the pallet.
            type WeightInfo = <$t as XcmWeight>::XcmWeightTrader;
            // The mechanism to convert weight into fees.
            type WeightToFee = <$t as XcmConfig>::WeightToFee;
            // Account that will receive XCM fees.
            type XcmFeesAccount = <$t as XcmConfig>::XcmFeesAccount;
        }


        impl orml_xtokens::Config for Runtime {
            type AccountIdToLocation = <$t as XcmConfig>::AccountIdToLocation;
            type Balance = Balance;
            type BaseXcmWeight = <$t as XcmConfig>::BaseXcmWeight;
            type CurrencyId = <$t as XcmConfig>::CurrencyId;
            type CurrencyIdConvert = <$t as XcmConfig>::CurrencyIdToLocation;
            type LocationsFilter = Everything;
            type MaxAssetsForTransfer = <$t as XcmConfig>::MaxAssetsForTransfer;
            type MinXcmFee = <$t as XcmConfig>::ParachainMinFee;
            type RateLimiter = ();
            type RateLimiterId = ();
            type ReserveProvider = <$t as XcmConfig>::XtokensReserveProviders;
            type RuntimeEvent = RuntimeEvent;
            type SelfLocation = <$t as XcmConfig>::SelfLocation;
            type UniversalLocation = <$t as XcmConfig>::UniversalLocation;
            type Weigher = <$t as XcmConfig>::XcmWeigher;
            type XcmExecutor = XcmExecutor<XcmExecutorConfig>;
        }

        impl pallet_xcm_transactor::Config for Runtime {
            type AccountIdToLocation = <$t as XcmConfig>::AccountIdToLocation;
            type AssetTransactor = <$t as XcmConfig>::AssetTransactors;
            type Balance = Balance;
            type BaseXcmWeight = <$t as XcmConfig>::BaseXcmWeight;
            type CurrencyId = <$t as XcmConfig>::CurrencyId;
            type CurrencyIdToLocation = <$t as XcmConfig>::CurrencyIdToLocation;
            type DerivativeAddressRegistrationOrigin = <$t as XcmConfig>::DerivativeAddressRegistrationOrigin;
            type HrmpManipulatorOrigin = <$t as XcmConfig>::HrmpManipulatorOrigin;
            type HrmpOpenOrigin = <$t as XcmConfig>::HrmpOpenOrigin;
            type MaxHrmpFee = xcm_builder::Case<<$t as XcmConfig>::MaxHrmpRelayFee>;
            type ReserveProvider = <$t as XcmConfig>::TransactorReserveProvider;
            type RuntimeEvent = RuntimeEvent;
            type SelfLocation = <$t as XcmConfig>::SelfLocation;
            type SovereignAccountDispatcherOrigin = <$t as XcmConfig>::SovereignAccountDispatcherOrigin;
            type Transactor = <$t as XcmConfig>::Transactors;
            type UniversalLocation = <$t as XcmConfig>::UniversalLocation;
            type Weigher = <$t as XcmConfig>::XcmWeigher;
            type WeightInfo = <$t as XcmWeight>::XcmTransactor;
            type XcmSender = <$t as XcmConfig>::XcmSender;
        }
    };
}

pub const PALLET_NAMES: [(&str, &str); 7] = [
    ("MessageQueue", "pallet_message_queue"),
    ("XcmpQueue", "cumulus_pallet_xcmp_queue"),
    ("PolkadotXcm", "pallet_xcm"),
    ("CumulusXcm", "cumulus_pallet_xcm"),
    ("XcmWeightTrader", "pallet_xcm_weight_trader"),
    ("XTokens", "orml_xtokens"),
    ("XcmTransactor", "pallet_xcm_transactor"),
];
