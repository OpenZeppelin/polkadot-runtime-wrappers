//! Construct runtime wrapper macros

// WIP not working as expected
// Input is tuples of form (PalletName, pallet, index)
#[macro_export]
macro_rules! construct_openzeppelin_runtime {
    ($(($name:ident, $pallet:ident, $index:expr)),* $(,)?) => {
        ::frame_support::construct_runtime!(
            pub enum Runtime {
                $(
                    $name: $pallet = $index,
                )*
            }
        );
    };
}

// Hardcoded temporary solutions below
// need to be combined into solution shown above

#[macro_export]
macro_rules! construct_openzeppelin_generic_runtime {
    () => {
        ::frame_support::construct_runtime!(
            pub enum Runtime
            {
                // System Support
                System: frame_system = 0,
                ParachainSystem: cumulus_pallet_parachain_system = 1,
                Timestamp: pallet_timestamp = 2,
                ParachainInfo: parachain_info = 3,
                Proxy: pallet_proxy = 4,
                Utility: pallet_utility = 5,
                Multisig: pallet_multisig = 6,
                Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 7,
                Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>, HoldReason} = 8,

                // Monetary
                Balances: pallet_balances = 10,
                TransactionPayment: pallet_transaction_payment = 11,
                Assets: pallet_assets = 12,
                Treasury: pallet_treasury::{Pallet, Call, Storage, Config<T>, Event<T>} = 13,

                // Governance
                Sudo: pallet_sudo = 15,
                ConvictionVoting: pallet_conviction_voting::{Pallet, Call, Storage, Event<T>} = 16,
                Referenda: pallet_referenda::{Pallet, Call, Storage, Event<T>} = 17,
                Origins: pallet_custom_origins::{Origin} = 18,
                Whitelist: pallet_whitelist::{Pallet, Call, Storage, Event<T>} = 19,

                // Collator Support. The order of these 4 are important and shall not change.
                Authorship: pallet_authorship = 20,
                CollatorSelection: pallet_collator_selection = 21,
                Session: pallet_session = 22,
                Aura: pallet_aura = 23,
                AuraExt: cumulus_pallet_aura_ext = 24,

                // XCM Helpers
                XcmpQueue: cumulus_pallet_xcmp_queue = 30,
                PolkadotXcm: pallet_xcm = 31,
                CumulusXcm: cumulus_pallet_xcm = 32,
                MessageQueue: pallet_message_queue = 33,
            }
        );
    }
}

#[macro_export]
macro_rules! construct_openzeppelin_evm_runtime {
    () => {
        ::frame_support::construct_runtime!(
            pub enum Runtime
            {
                // System Support
                System: frame_system = 0,
                ParachainSystem: cumulus_pallet_parachain_system = 1,
                Timestamp: pallet_timestamp = 2,
                ParachainInfo: parachain_info = 3,
                Proxy: pallet_proxy = 4,
                Utility: pallet_utility = 5,
                Multisig: pallet_multisig = 6,
                Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 7,
                Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>, HoldReason} = 8,

                // Monetary
                Balances: pallet_balances = 10,
                TransactionPayment: pallet_transaction_payment = 11,
                Assets: pallet_assets = 12,
                Treasury: pallet_treasury::{Pallet, Call, Storage, Config<T>, Event<T>} = 13,
                AssetManager: pallet_asset_manager = 14,

                // Governance
                Sudo: pallet_sudo = 15,
                ConvictionVoting: pallet_conviction_voting::{Pallet, Call, Storage, Event<T>} = 16,
                Referenda: pallet_referenda::{Pallet, Call, Storage, Event<T>} = 17,
                Origins: pallet_custom_origins::{Origin} = 18,
                Whitelist: pallet_whitelist::{Pallet, Call, Storage, Event<T>} = 19,

                // Collator Support. The order of these 4 are important and shall not change.
                Authorship: pallet_authorship = 20,
                CollatorSelection: pallet_collator_selection = 21,
                Session: pallet_session = 22,
                Aura: pallet_aura = 23,
                AuraExt: cumulus_pallet_aura_ext = 24,

                // XCM Helpers
                XcmpQueue: cumulus_pallet_xcmp_queue = 30,
                PolkadotXcm: pallet_xcm = 31,
                CumulusXcm: cumulus_pallet_xcm = 32,
                MessageQueue: pallet_message_queue = 33,

                // EVM
                Ethereum: pallet_ethereum = 40,
                EVM: pallet_evm = 41,
                BaseFee: pallet_base_fee = 42,
                EVMChainId: pallet_evm_chain_id = 43,
            }
        );
    }
}
