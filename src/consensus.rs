//! This macro sets up the necessary configurations for the following pallets:
//! - `pallet_authorship`
//! - `pallet_aura`
//! - `cumulus_pallet_aura_ext`
//! - `pallet_collator_selection`
//! - `pallet_session`
//!
//! # Parameters
//! - `$t`: A type that implements the `ConsensusConfig` trait, providing the necessary associated types
//!   and configurations.
//!
//! # Important
//! Rerun benchmarks if making changes to runtime configuration, as weight calculations
//! may need to be updated.

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
            type WeightInfo = <$t as ConsensusWeight>::CollatorSelection;
        }

        parameter_types! {
            // pallet_session ends the session after a fixed period of blocks.
            // The first session will have length of Offset,
            // and the following sessions will have length of Period.
            // By setting Offset to zero we allow the chain to start processing blocks immediately.
            // And then rotate parachain validators every 6 hours.
            pub const Period: u32 = 6 * HOURS;
            pub const Offset: u32 = 0;
        }

        impl pallet_session::Config for Runtime {
            type Keys = SessionKeys;
            type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
            type RuntimeEvent = RuntimeEvent;
            // Essentially just Aura, but let's be pedantic.
            type SessionHandler =
                <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
            type SessionManager = CollatorSelection;
            type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
            type ValidatorId = <Self as frame_system::Config>::AccountId;
            // we don't have stash and controller, thus we don't need the convert as well.
            type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
            type WeightInfo = <$t as ConsensusWeight>::Session;
        }
    };
}

pub const PALLET_NAMES: [(&str, &str); 5] = [
    ("Authorship", "pallet_authorship"),
    ("Aura", "pallet_aura"),
    ("AuraExt", "cumulus_pallet_aura_ext"),
    ("CollatorSelection", "pallet_collator_selection"),
    ("Session", "pallet_session"),
];
