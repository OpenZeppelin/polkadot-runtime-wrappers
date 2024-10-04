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
                xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
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
            type MaxInboundSuspended = <$t as XcmConfig>::XcmpQueueMaxInboundSuspended;
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
    };
}
