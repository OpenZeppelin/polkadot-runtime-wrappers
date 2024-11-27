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

        // Allow multiple blocks per slot based on the async backing feature.
        #[cfg(not(feature = "async-backing"))]
        parameter_types! {
            pub const AllowMultipleBlocksPerSlot: bool = false;
        }

        #[cfg(feature = "async-backing")]
        parameter_types! {
            pub const AllowMultipleBlocksPerSlot: bool = true;
        }

        // The Aura module extends Aura consensus by managing offline reporting.
        impl pallet_aura::Config for Runtime {
            type AllowMultipleBlocksPerSlot = AllowMultipleBlocksPerSlot;
            // The identifier type for an authority.
            type AuthorityId = AuraId;
            // A way to check whether a given validator is disabled and should not be authoring blocks.
            type DisabledValidators = <$t as ConsensusConfig>::DisabledValidators;
            // Max number of authorities allowed
            type MaxAuthorities = <$t as ConsensusConfig>::MaxAuthorities;
            // A slot duration provider which infers the slot duration from the [pallet_timestamp::Config::MinimumPeriod] by multiplying
            // it by two, to ensure that authors have the majority of their slot to author within.
            type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Self>;
        }

        impl cumulus_pallet_aura_ext::Config for Runtime {}

        parameter_types! {
            pub const PotId: PalletId = PalletId(*b"PotStake");
            // A session is a period of time that has a constant set of validators. Validators can only join or
            // exit the validator set at a session change. It is measured in block numbers.
            pub const SessionLength: BlockNumber = 6 * HOURS;
            // StakingAdmin pluralistic body.
            pub const StakingAdminBodyId: BodyId = BodyId::Defense;
        }

        // A pallet to manage collators in a parachain.
        impl pallet_collator_selection::Config for Runtime {
            // The currency mechanism.
            type Currency = Balances;
            // should be a multiple of session or things will get inconsistent
            type KickThreshold = Period;
            // Maximum number of candidates that we should have without taking into account the invulnerables.
            type MaxCandidates = <$t as ConsensusConfig>::MaxCandidates;
            // Maximum number of invulnerables (a set of collators appointed by governance. These accounts will always be collators.)
            type MaxInvulnerables = <$t as ConsensusConfig>::MaxInvulnerables;
            // Minimum number eligible collators. Should always be greater than zero.
            // This ensures that there will always be one collator who can produce a block.
            type MinEligibleCollators = <$t as ConsensusConfig>::MinEligibleCollators;
            // Account Identifier from which the internal Pot is generated.
            type PotId = PotId;
            // The overarching event type.
            type RuntimeEvent = RuntimeEvent;
            // Origin that can dictate updating parameters of this pallet.
            type UpdateOrigin = <$t as ConsensusConfig>::CollatorSelectionUpdateOrigin;
            // A stable ID for a validator.
            type ValidatorId = <Self as frame_system::Config>::AccountId;
            // A conversion from account ID to validator ID.
            type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
            // Validate a user is registered
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

        // The Session pallet allows validators to manage their session keys, provides a function for
        // changing the session length, and handles session rotation.
        impl pallet_session::Config for Runtime {
            // A session key is actually several keys kept together that provide the various
            // signing functions required by network authorities/validators in pursuit of their duties.
            type Keys = SessionKeys;
            // Something that can predict the next session rotation
            type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
            type RuntimeEvent = RuntimeEvent;
            // Handler when a session has changed
            type SessionHandler =
                <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
            // Handler for managing new session.
            type SessionManager = CollatorSelection;
            // Indicator for when to end the session.
            type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
            // Every account has an associated validator ID. For some simple staking
            // systems, this may just be the same as the account ID. For staking systems using a
            // stash/controller model, the validator ID would be the stash account ID of the controller.
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
