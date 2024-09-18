#![cfg_attr(not(feature = "std"), no_std)]

mod api;
mod construct_runtime;
mod system;

pub use construct_runtime::GENERIC_RUNTIME_PALLET_INDICES;

use frame_support::pallet_prelude::Get;
use sp_version::RuntimeVersion;

pub trait SystemConfig {
    type AccountId;
    type SS58Prefix;
    type Version: Get<RuntimeVersion>;
    type ScheduleOrigin;
    type PreimageOrigin;
}
