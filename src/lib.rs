#![cfg_attr(not(feature = "std"), no_std)]

mod api;
mod system;
// mod runtime; // ignore for now, come back to this

use frame_support::pallet_prelude::Get;
use sp_version::RuntimeVersion;

pub trait SystemConfig {
    type AccountId;
    type SS58Prefix;
    type Version: Get<RuntimeVersion>;
    type ScheduleOrigin;
    type PreimageOrigin;
}
