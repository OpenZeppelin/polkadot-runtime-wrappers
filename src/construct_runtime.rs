#[macro_export]
macro_rules! construct_openzeppelin_runtime {
    ($( ($pallet:ident, $index:expr) ),* $(,)?) => {{
        ::frame_support::construct_runtime!(
            pub enum Runtime {
                $(
                    $crate::to_pascal_case!($pallet): $pallet = $index,
                )*
            }
        );
    }};
}

#[macro_export]
macro_rules! to_pascal_case {
    (pallet_custom_origins) => {
        Origins
    };
    (cumulus_pallet_$($name:ident)_+) => {
        stringify!($name).to_case(Case::Pascal)
    };
    (pallet_$($name:ident)_+) => {
        stringify!($name).to_case(Case::Pascal)
    };
    (frame_$($name:ident)_+) => {
        stringify!($name).to_case(Case::Pascal)
    };
    ($name:ident) => {
        stringify!($name).to_case(Case::Pascal)
    };
}

#[macro_export]
macro_rules! capitalize {
    ($part:ident) => {
        $crate::paste::paste! {
            [<$part:capitalize>]
        }
    };
}

use scale_info::prelude::string::{String, ToString};
use sp_std::vec::Vec;

#[macro_export]
macro_rules! generic_runtime_pallet_indices {
    () => {
        (
            (frame_system, 0),
            (cumulus_pallet_parachain_system, 1),
            (pallet_timestamp, 2),
            (parachain_info, 3),
            (pallet_proxy, 4),
            (pallet_utility, 5),
            (pallet_multisig, 6),
            (pallet_scheduler, 7),
            (pallet_preimage, 8),
            (pallet_balances, 10),
            (pallet_transaction_payment, 11),
            (pallet_assets, 12),
            (pallet_treasury, 13),
            (pallet_sudo, 15),
            (pallet_conviction_voting, 16),
            (pallet_referenda, 17),
            (pallet_custom_origins, 18),
            (pallet_whitelist, 19),
            (pallet_authorship, 20),
            (pallet_collator_selection, 21),
            (pallet_session, 22),
            (pallet_aura, 23),
            (cumulus_pallet_aura_ext, 24),
            (cumulus_pallet_xcmp_queue, 30),
            (pallet_xcm, 31),
            (cumulus_pallet_xcm, 32),
            (pallet_message_queue, 33),
        )
    };
}

pub const GENERIC_RUNTIME_PALLET_INDICES: [(&'static str, u8); 27] = [
    // System Support
    ("frame_system", 0),
    ("cumulus_pallet_parachain_system", 1),
    ("pallet_timestamp", 2),
    ("parachain_info", 3),
    ("pallet_proxy", 4),
    ("pallet_utility", 5),
    ("pallet_multisig", 6),
    ("pallet_scheduler", 7),
    ("pallet_preimage", 8),
    // Monetary
    ("pallet_balances", 10),
    ("pallet_transaction_payment", 11),
    ("pallet_assets", 12),
    ("pallet_treasury", 13),
    // Governance
    ("pallet_sudo", 15),
    ("pallet_conviction_voting", 16),
    ("pallet_referenda", 17),
    ("pallet_custom_origins", 18),
    ("pallet_whitelist", 19),
    // Collator Support
    ("pallet_authorship", 20),
    ("pallet_collator_selection", 21),
    ("pallet_session", 22),
    ("pallet_aura", 23),
    ("cumulus_pallet_aura_ext", 24),
    // XCM Helpers
    ("cumulus_pallet_xcmp_queue", 30),
    ("pallet_xcm", 31),
    ("cumulus_pallet_xcm", 32),
    ("pallet_message_queue", 33),
];
