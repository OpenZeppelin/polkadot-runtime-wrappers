#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_defaults)]

mod api;
mod assets;
mod consensus;
mod runtime;
mod system;

use frame_support::pallet_prelude::{ConstU32, Get};
use sp_version::RuntimeVersion;

pub trait SystemConfig {
    type AccountId;
    type SS58Prefix;
    type Version: Get<RuntimeVersion>;
    type ExistentialDeposit;
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

pub trait AssetsConfig {
    type ApprovalDeposit;
    type AssetAccountDeposit;
    type AssetDeposit;
    type CreateOrigin;
    type ForceOrigin;
}
