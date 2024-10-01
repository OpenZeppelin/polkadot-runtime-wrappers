//! Exclude macros
use frame_support::traits::Contains;
use sp_std::vec;
use sp_std::vec::Vec;

pub trait ExampleConfig {
    type ExcludedPallets: Contains<Vec<u8>>;
}

pub struct ExcludedPallets;
impl Contains<Vec<u8>> for ExcludedPallets {
    fn contains(i: &Vec<u8>) -> bool {
        vec!["frame_support".as_bytes().to_vec()].contains(&i)
    }
}

impl ExampleConfig for () {
    type ExcludedPallets = ExcludedPallets;
}

#[macro_export]
macro_rules! impl_for_runtime {
    ($pallet:ident, $t:ty) => {
        $crate::maybe_impl_config!($pallet, $t);
    };
}

#[macro_export]
macro_rules! maybe_impl_config {
    ($pallet_name:ident, $t:ty) => {{
        let config: Vec<u8> = stringify!($pallet_name).as_bytes().to_vec();
        let excluded = <<$t as ExampleConfig>::ExcludedPallets as ::frame_support::traits::Contains<_>>::contains(&config);
        if !excluded {
            $crate::impl_config!(pallet_timestamp)
        } else {
            //"EXCLUDED SUCCESSFULLY"
            {}
        }
    }};
}

#[macro_export]
macro_rules! impl_config {
    (pallet_timestamp) => {
        impl pallet_timestamp::Config for Runtime {
            type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
            /// A timestamp: milliseconds since the unix epoch.
            type Moment = u64;
            type OnTimestampSet = Aura;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
        }
    };
}
