
// Expects input to be list of tuples of form (pallet_identifier, pallet_index)
#[macro_export]
macro_rules! construct_openzeppelin_runtime {
    ($(($pallet:ident, $index:expr)),* $(,)?) => {
        ::frame_support::construct_runtime!(
            pub enum Runtime {
                $(
                    pallet_id!($pallet): $pallet = $index,
                )*
            }
        );
    };
}

#[macro_export]
macro_rules! to_pascal_case {
    ($($part:ident)+) => {{
        fn to_pascal_case_impl(s: &str) -> String {
            let mut result = String::new();
            let mut capitalize_next = true;

            for c in s.chars() {
                if c == '_' {
                    capitalize_next = true;
                } else if capitalize_next {
                    result.push(c.to_ascii_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(c.to_ascii_lowercase());
                }
            }

            result
        }

        let parts = vec![$(stringify!($part)),+];
        parts.into_iter()
            .map(to_pascal_case_impl)
            // TODO: filter not working fix it
            //.filter(|p| *p != "Frame" && *p != "Pallet" && *p != "CumulusPallet" )
            .collect::<Vec<_>>()
            .join("")
    }};
}

#[macro_export]
macro_rules! pallet_id {
    (pallet_custom_origins) => {
        "Origins"
    };
    (cumulus_pallet_$($name:ident)+) => {
        to_pascal_case!($($name)+)
    };
    (pallet_$($name:ident)+) => {
        to_pascal_case!($($name)+)
    };
    (frame_$($name:ident)+) => {
        to_pascal_case!($($name)+)
    };
    ($($name:ident)+) => {
        to_pascal_case!($($name)+)
    };
}

#[macro_export]
macro_rules! generic_runtime_pallet_indices {
    () => {
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
    };
}
