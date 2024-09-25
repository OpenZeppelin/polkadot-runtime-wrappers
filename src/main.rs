#![cfg_attr(not(feature = "std"), no_std)]

mod api;
mod construct_runtime;
mod system;

use frame_support::pallet_prelude::Get;
use sp_version::RuntimeVersion;

pub trait SystemConfig {
    type AccountId;
    type SS58Prefix;
    type Version: Get<RuntimeVersion>;
    type ScheduleOrigin;
    type PreimageOrigin;
}

use tester::test_macro_output;

fn main() {
    println!("{}", to_pascal_case!(frame_system));
    println!("{}", pallet_id!(frame_system));
    test_macro_output! {
        pallet_id!(frame_system);
    }
}
