
#[macro_export]
macro_rules! impl_tanssi {
    ($t:ty) => {
        impl pallet_author_inherent::Config for Runtime {
            type AuthorId = NimbusId;
            type AccountLookup = dp_consensus::NimbusLookUp;
            type CanAuthor = pallet_cc_authorities_noting::CanAuthor<Runtime>;
            type SlotBeacon = dp_consensus::AuraDigestSlotBeacon<Runtime>;
            type WeightInfo = 
                pallet_author_inherent::weights::SubstrateWeight<Runtime>;
        }
        
        impl pallet_cc_authorities_noting::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type SelfParaId = parachain_info::Pallet<Runtime>;
            type RelayChainStateProvider = 
                cumulus_pallet_parachain_system::RelaychainDataProvider<Self>;
            type AuthorityId = NimbusId;
            type WeightInfo = 
                pallet_cc_authorities_noting::weights::SubstrateWeight<Runtime>;
        }
    };
}

pub fn pallet_name_list() -> Vec<(&'static str, &'static str)> {
    vec![
        ("AuthorInherent", "pallet_author_inherent"),
        ("AuthoritiesNoting", "pallet_cc_authorities_noting")
    ]
}