#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_defaults)]

mod api;
mod consensus;
mod runtime;
mod system;

use frame_support::pallet_prelude::{ConstU32, Get};
use sp_version::RuntimeVersion;

pub trait SystemConfig {
    type AccountId;
    type SS58Prefix;
    type Version: Get<RuntimeVersion>;
    type ScheduleOrigin;
    type PreimageOrigin;
}

pub trait ConsensusConfig {
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<100_000>;
    type MaxCandidates = ConstU32<100>;
    type MaxInvulnerables = ConstU32<20>;
    type MinEligibleCollators = ConstU32<20>;
    type CollatorSelectionUpdateOrigin;
}
