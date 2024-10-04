//! XCM pallet groupings wrapper

#[macro_export]
macro_rules! impl_openzeppelin_xcm {
    ($t:ty) => {
        impl pallet_message_queue::Config for Runtime {
            type HeapSize = <$t as XcmConfig>::MessageQueueHeapSize;
            type IdleMaxServiceWeight = <$t as XcmConfig>::MessageQueueServiceWeight;
            type MaxStale = <$t as XcmConfig>::MessageQueueMaxStale;
            #[cfg(feature = "runtime-benchmarks")]
            type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor<
                cumulus_primitives_core::AggregateMessageOrigin,
            >;
            #[cfg(not(feature = "runtime-benchmarks"))]
            type MessageProcessor = ProcessXcmMessage<
                AggregateMessageOrigin,
                xcm_executor::XcmExecutor<XcmExecutorConfig>,
                RuntimeCall,
            >;
            // The XCMP queue pallet is only ever able to handle the `Sibling(ParaId)` origin:
            type QueueChangeHandler = NarrowOriginToSibling<XcmpQueue>;
            type QueuePausedQuery = NarrowOriginToSibling<XcmpQueue>;
            type RuntimeEvent = RuntimeEvent;
            type ServiceWeight = <$t as XcmConfig>::MessageQueueServiceWeight;
            type Size = u32;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_message_queue::WeightInfo<Runtime>;
        }

        parameter_types! {
            /// The asset ID for the asset that we use to pay for message delivery fees.
            pub FeeAssetId: AssetId = AssetId(RelayLocation::get());
            /// The base fee for the message delivery fees. Kusama is based for the reference.
            pub const ToSiblingBaseDeliveryFee: u128 = CENTS.saturating_mul(3);
        }

        impl cumulus_pallet_xcmp_queue::Config for Runtime {
            type ChannelInfo = ParachainSystem;
            type ControllerOrigin = <$t as XcmConfig>::XcmpQueueControllerOrigin;
            type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
            // TODO
            type MaxActiveOutboundChannels = ConstU32<128>;
            type MaxInboundSuspended = <$t as XcmConfig>::XcmpQueueMaxInboundSuspended;
            // TODO
            type MaxPageSize = ConstU32<{ 1 << 16 }>;
            /// Ensure that this value is not set to null (or NoPriceForMessageDelivery) to prevent spamming
            type PriceForSiblingDelivery = PriceForSiblingParachainDelivery;
            type RuntimeEvent = RuntimeEvent;
            type VersionWrapper = ();
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::cumulus_pallet_xcmp_queue::WeightInfo<Runtime>;
            // Enqueue XCMP messages from siblings for later processing.
            type XcmpQueue =
                TransformOrigin<MessageQueue, AggregateMessageOrigin, ParaId, ParaIdToSibling>;
        }

        parameter_types! {
            pub const RelayLocation: Location = Location::parent();
            pub const RelayNetwork: Option<NetworkId> = None;
            pub PlaceholderAccount: AccountId = PolkadotXcm::check_account();
            pub AssetsPalletLocation: Location =
                PalletInstance(<Assets as PalletInfoAccess>::index() as u8).into();
            pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
            pub UniversalLocation: InteriorLocation = Parachain(ParachainInfo::parachain_id().into()).into();
        }

        /// `AssetId/Balancer` converter for `TrustBackedAssets`
        pub type TrustBackedAssetsConvertedConcreteId =
            assets_common::TrustBackedAssetsConvertedConcreteId<AssetsPalletLocation, Balance>;

        /// Type for specifying how a `Location` can be converted into an
        /// `AccountId`. This is used when determining ownership of accounts for asset
        /// transacting and when attempting to use XCM `Transact` in order to determine
        /// the dispatch Origin.
        pub type LocationToAccountId = (
            // The parent (Relay-chain) origin converts to the parent `AccountId`.
            ParentIsPreset<AccountId>,
            // Sibling parachain origins convert to AccountId via the `ParaId::into`.
            SiblingParachainConvertsVia<Sibling, AccountId>,
            // Straight up local `AccountId32` origins just alias directly to `AccountId`.
            AccountId32Aliases<RelayNetwork, AccountId>,
        );

        /// Means for transacting assets on this chain.
        pub type LocalAssetTransactor = FungibleAdapter<
            // Use this currency:
            Balances,
            // Use this currency when it is a fungible asset matching the given location or name:
            IsConcrete<RelayLocation>,
            // Do a simple punn to convert an AccountId32 Location into a native chain account ID:
            LocationToAccountId,
            // Our chain's account ID type (we can't get away without mentioning it explicitly):
            AccountId,
            // We don't track any teleports.
            (),
        >;

        /// Means for transacting assets besides the native currency on this chain.
        pub type LocalFungiblesTransactor = FungiblesAdapter<
            // Use this fungibles implementation:
            Assets,
            // Use this currency when it is a fungible asset matching the given location or name:
            TrustBackedAssetsConvertedConcreteId,
            // Convert an XCM MultiLocation into a local account id:
            LocationToAccountId,
            // Our chain's account ID type (we can't get away without mentioning it explicitly):
            AccountId,
            // We don't track any teleports of `Assets`.
            NoChecking,
            // We don't track any teleports of `Assets`, but a placeholder account is provided due to trait
            // bounds.
            PlaceholderAccount,
        >;

        /// Means for transacting assets on this chain.
        pub type AssetTransactors = (LocalAssetTransactor, LocalFungiblesTransactor);

        /// This is the type we use to convert an (incoming) XCM origin into a local
        /// `Origin` instance, ready for dispatching a transaction with Xcm's
        /// `Transact`. There is an `OriginKind` which can biases the kind of local
        /// `Origin` it will become.
        pub type XcmOriginToTransactDispatchOrigin = (
            // Sovereign account converter; this attempts to derive an `AccountId` from the origin location
            // using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
            // foreign chains who want to have a local sovereign account on this chain which they control.
            SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
            // Native converter for Relay-chain (Parent) location; will convert to a `Relay` origin when
            // recognized.
            RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
            // Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
            // recognized.
            SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
            // Native signed account converter; this just converts an `AccountId32` origin into a normal
            // `RuntimeOrigin::Signed` origin of the same 32-byte value.
            SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
            // Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
            XcmPassthrough<RuntimeOrigin>,
        );

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
            type AssetClaims = PolkadotXcm;
            type AssetExchanger = ();
            type AssetLocker = ();
            // How to withdraw and deposit an asset.
            type AssetTransactor = AssetTransactors;
            type AssetTrap = PolkadotXcm;
            type Barrier = Barrier;
            type CallDispatcher = RuntimeCall;
            /// When changing this config, keep in mind, that you should collect fees.
            type FeeManager = XcmFeeManagerFromComponents<
                IsChildSystemParachain<primitives::Id>,
                XcmFeeToAccount<Self::AssetTransactor, AccountId, TreasuryAccount>,
            >;
            type HrmpChannelAcceptedHandler = ();
            type HrmpChannelClosingHandler = ();
            type HrmpNewChannelOpenRequestHandler = ();
            /// Please, keep these two configs (`IsReserve` and `IsTeleporter`) mutually exclusive
            type IsReserve = NativeAsset;
            type IsTeleporter = ();
            type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
            type MessageExporter = ();
            type OriginConverter = XcmOriginToTransactDispatchOrigin;
            type PalletInstancesInfo = AllPalletsWithSystem;
            type ResponseHandler = PolkadotXcm;
            type RuntimeCall = RuntimeCall;
            type SafeCallFilter = Everything;
            type SubscriptionService = PolkadotXcm;
            type Trader =
                UsingComponents<WeightToFee, RelayLocation, AccountId, Balances, ToAuthor<Runtime>>;
            type TransactionalProcessor = FrameTransactionalProcessor;
            type UniversalAliases = Nothing;
            // Teleporting is disabled.
            type UniversalLocation = UniversalLocation;
            type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
            type XcmRecorder = PolkadotXcm;
            type XcmSender = XcmRouter;
        }

        /// No local origins on this chain are allowed to dispatch XCM sends/executions.
        pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

        /// The means for routing XCM messages which are not for local execution into
        /// the right message queues.
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

        impl pallet_xcm::Config for Runtime {
            type AdminOrigin = <$t as XcmConfig>::XcmAdminOrigin;
            // ^ Override for AdvertisedXcmVersion default
            type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
            type Currency = Balances;
            type CurrencyMatcher = ();
            type ExecuteXcmOrigin = <$t as XcmConfig>::ExecuteXcmOrigin;
            type MaxLockers = MaxLockers;
            type MaxRemoteLockConsumers = MaxRemoteLockConsumers;
            type RemoteLockConsumerIdentifier = ();
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            type RuntimeOrigin = RuntimeOrigin;
            type SendXcmOrigin = <$t as XcmConfig>::SendXcmOrigin;
            type SovereignAccountOf = LocationToAccountId;
            type TrustedLockers = ();
            type UniversalLocation = UniversalLocation;
            type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_xcm::WeightInfo<Runtime>;
            type XcmExecuteFilter = Nothing;
            // ^ Disable dispatchable execute on the XCM pallet.
            // Needs to be `Everything` for local testing.
            type XcmExecutor = XcmExecutor<XcmExecutorConfig>;
            type XcmReserveTransferFilter = Nothing;
            type XcmRouter = XcmRouter;
            type XcmTeleportFilter = Nothing;

            const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
        }

        impl cumulus_pallet_xcm::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type XcmExecutor = XcmExecutor<XcmExecutorConfig>;
        }
    };
}
