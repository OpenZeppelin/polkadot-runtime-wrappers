//! Exclude macros

pub trait ExampleConfig {
    type ExcludedPallets = ();
}

impl ExampleConfig for () {}

#[macro_export]
macro_rules! impl_openzeppelin_example {
    ($t:ty) => {
        maybe_impl_config!(frame_support, $t)
    };
}

// Implement the Config trait only if the pallet is NOT in the exclusion list.
#[macro_export]
macro_rules! maybe_impl_config {
    ($pallet_name:ident, $t:ty) => {{
        let fake_excluded_list = ("frame_support");
        let excluded = fake_excluded_list.contains(&stringify!($pallet_name));
        // let excluded = <<$t as ExampleConfig >::ExcludedPallets as ::frame_support::traits::Contains<_>>::contains(&stringify!($pallet_name).as_bytes().to_vec());
        if !excluded {
            impl_config!(pallet_timestamp)
        } else {
            "EXCLUDED SUCCESSFULLY"
        }
    }};
}

#[macro_export]
macro_rules! impl_config {
    (pallet_timestamp) => {
        "impl pallet_timestamp::Config for Runtime {
            type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
            /// A timestamp: milliseconds since the unix epoch.
            type Moment = u64;
            type OnTimestampSet = Aura;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
        }"
    };
}
