#[macro_export]
macro_rules! impl_openzeppelin_tanssi {
    ($t:ty) => {
        impl pallet_author_inherent::Config for Runtime {
            type AuthorId = nimbus_primitives::NimbusId;
            type AccountLookup = dp_consensus::NimbusLookUp;
            type CanAuthor = pallet_cc_authorities_noting::CanAuthor<Runtime>;
            type SlotBeacon = dp_consensus::AuraDigestSlotBeacon<Runtime>;
            type WeightInfo = <$t as TanssiWeight>::AuthorInherent;
        }

        impl pallet_cc_authorities_noting::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type SelfParaId = parachain_info::Pallet<Runtime>;
            type RelayChainStateProvider =
                cumulus_pallet_parachain_system::RelaychainDataProvider<Self>;
            type AuthorityId = nimbus_primitives::NimbusId;
            type WeightInfo = <$t as TanssiWeight>::AuthoritiesNoting;
        }
    };
}

pub const PALLET_NAMES: [(&str, &str); 2] = [
    ("AuthorInherent", "pallet_author_inherent"),
    ("AuthoritiesNoting", "pallet_cc_authorities_noting"),
];
