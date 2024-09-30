//! Consensus pallet grouping wrapper

#[macro_export]
macro_rules! impl_openzeppelin_consensus {
    ($t:ty) => {
        impl pallet_authorship::Config for Runtime {
            type EventHandler = (CollatorSelection,);
            type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
        }

        #[cfg(not(feature = "async-backing"))]
        parameter_types! {
            pub const AllowMultipleBlocksPerSlot: bool = false;
        }

        #[cfg(feature = "async-backing")]
        parameter_types! {
            pub const AllowMultipleBlocksPerSlot: bool = true;
        }

        impl pallet_aura::Config for Runtime {
            type AllowMultipleBlocksPerSlot = AllowMultipleBlocksPerSlot;
            type AuthorityId = AuraId;
            type DisabledValidators = <$t as ConsensusConfig>::DisabledValidators;
            type MaxAuthorities = <$t as ConsensusConfig>::MaxAuthorities;
            type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Self>;
        }

        impl cumulus_pallet_aura_ext::Config for Runtime {}

        parameter_types! {
            pub const PotId: PalletId = PalletId(*b"PotStake");
            pub const SessionLength: BlockNumber = 6 * HOURS;
            // StakingAdmin pluralistic body.
            pub const StakingAdminBodyId: BodyId = BodyId::Defense;
        }

        impl pallet_collator_selection::Config for Runtime {
            type Currency = Balances;
            // should be a multiple of session or things will get inconsistent
            type KickThreshold = Period;
            type MaxCandidates = <$t as ConsensusConfig>::MaxCandidates;
            type MaxInvulnerables = <$t as ConsensusConfig>::MaxInvulnerables;
            type MinEligibleCollators = <$t as ConsensusConfig>::MinEligibleCollators;
            type PotId = PotId;
            type RuntimeEvent = RuntimeEvent;
            type UpdateOrigin = <$t as ConsensusConfig>::CollatorSelectionUpdateOrigin;
            type ValidatorId = <Self as frame_system::Config>::AccountId;
            type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
            type ValidatorRegistration = Session;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_collator_selection::WeightInfo<Runtime>;
        }
    };
}
