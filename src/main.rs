#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_defaults)]

mod api;
mod assets;
mod consensus;
pub mod exclude;
pub use exclude::*;
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
    // tuple of pallets to exclude from implementations
    type ExcludeList = ();
}

pub trait ConsensusConfig {
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<100_000>;
    type MaxCandidates = ConstU32<100>;
    type MaxInvulnerables = ConstU32<20>;
    type MinEligibleCollators = ConstU32<20>;
    type CollatorSelectionUpdateOrigin;
    // tuple of pallets to exclude from implementations
    type ExcludeList = ();
}

pub trait AssetsConfig {
    type ApprovalDeposit;
    type AssetAccountDeposit;
    type AssetDeposit;
    type CreateOrigin;
    type ForceOrigin;
    // tuple of pallets to exclude from implementations
    type ExcludeList = ();
}

pub fn main() {
    println!("{}", impl_for_runtime!(frame_support, ()));
}
