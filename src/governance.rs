//! Implements the OpenZeppelin governance configuration for a Runtime.
//!
//! This macro sets up the necessary configurations for the following pallets:
//! - `pallet_sudo`
//! - `pallet_treasury`
//! - `pallet_conviction_voting`
//! - `pallet_whitelist`
//! - `pallet_custom_origins`
//! - `pallet_referenda`
//!
//! # Parameters
//! - `$t`: A type that implements the `GovernanceConfig` trait, providing the necessary associated types
//!   and configurations.
//!
//! # Important
//! Rerun benchmarks if making changes to runtime configuration, as weight calculations
//! may need to be updated.

#[macro_export]
macro_rules! impl_openzeppelin_governance {
    ($t:ty) => {
        // A pallet to provide a way to execute privileged runtime calls using a specified sudo (“superuser do”) account.
        impl pallet_sudo::Config for Runtime {
            // The overarching call type.
            type RuntimeCall = RuntimeCall;
            // The overarching event type.
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as GovernanceWeight>::Sudo;
        }

        #[cfg(feature = "runtime-benchmarks")]
        parameter_types! {
            pub LocationParents: u8 = 1;
            pub BenchmarkParaId: u8 = 0;
        }

        // The Treasury pallet provides a “pot” of funds that can be managed by stakeholders in the
        // system and a structure for making spending proposals from this pot.
        impl pallet_treasury::Config for Runtime {
            // The kind of asset to be spent.
            type AssetKind = AssetKind;
            // Type for converting the balance of an [Self::AssetKind] to the balance of the native asset.
            type BalanceConverter = frame_support::traits::tokens::UnityAssetBalanceConversion;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkHelper = polkadot_runtime_common::impls::benchmarks::TreasuryArguments<
                LocationParents,
                BenchmarkParaId,
            >;
            // Used to identify the beneficiaries eligible to receive treasury spends.
            type Beneficiary = Beneficiary;
            // Converting trait to take a source type and convert to [`Self::Beneficiary`].
            type BeneficiaryLookup = IdentityLookup<Self::Beneficiary>;
            // Percentage of spare funds (if any) that are burnt per spend period.
            type Burn = <$t as GovernanceConfig>::TreasuryBurn;
            // Handler for the unbalanced decrease when treasury funds are burned.
            type BurnDestination = <$t as GovernanceConfig>::TreasuryBurnDestination;
            // The staking balance.
            type Currency = Balances;
            // The maximum number of approvals that can wait in the spending queue.
            type MaxApprovals = <$t as GovernanceConfig>::TreasuryMaxApprovals;
            // The treasury's pallet id, used for deriving its sovereign account ID.
            type PalletId = <$t as GovernanceConfig>::TreasuryPalletId;
            #[cfg(feature = "runtime-benchmarks")]
            // Type for processing spends of [Self::AssetKind] in favor of [`Self::Beneficiary`].
            type Paymaster = PayWithEnsure<TreasuryPaymaster, OpenHrmpChannel<BenchmarkParaId>>;
            #[cfg(not(feature = "runtime-benchmarks"))]
            type Paymaster = TreasuryPaymaster;
            // The period during which an approved treasury spend has to be claimed.
            type PayoutPeriod = <$t as GovernanceConfig>::TreasuryPayoutSpendPeriod;
            // Origin from which rejections must come.
            type RejectOrigin = <$t as GovernanceConfig>::TreasuryRejectOrigin;
            type RuntimeEvent = RuntimeEvent;
            // Runtime hooks to external pallet using treasury to compute spend funds.
            type SpendFunds = <$t as GovernanceConfig>::TreasurySpendFunds;
            // The origin required for approving spends from the treasury outside of the proposal process.
            type SpendOrigin = <$t as GovernanceConfig>::TreasurySpendOrigin;
            // Period between successive spends.
            type SpendPeriod = <$t as GovernanceConfig>::TreasurySpendPeriod;
            type WeightInfo = <$t as GovernanceWeight>::Treasury;
        }

        // Pallet for managing actual voting in polls.
        impl pallet_conviction_voting::Config for Runtime {
            // Currency type with which voting happens.
            type Currency = Balances;
            // The maximum amount of tokens which may be used for voting.
            type MaxTurnout = frame_support::traits::tokens::currency::ActiveIssuanceOf<
                Balances,
                Self::AccountId,
            >;
            // The maximum number of concurrent votes an account may have.
            type MaxVotes = <$t as GovernanceConfig>::ConvictionMaxVotes;
            // The implementation of the logic which conducts polls.
            type Polls = Referenda;
            // The overarching event type.
            type RuntimeEvent = RuntimeEvent;
            // The minimum period of vote locking.
            type VoteLockingPeriod = <$t as GovernanceConfig>::ConvictionVoteLockingPeriod;
            type WeightInfo = <$t as GovernanceWeight>::ConvictionVoting;
        }

        // Pallet to allow some configurable origin: Config::WhitelistOrigin to whitelist some hash of a call, and
        // allow another configurable origin: Config::DispatchWhitelistedOrigin to dispatch them with the root origin.
        impl pallet_whitelist::Config for Runtime {
            // Required origin for dispatching whitelisted call with root origin.
            type DispatchWhitelistedOrigin = <$t as GovernanceConfig>::DispatchWhitelistedOrigin;
            // The handler of pre-images.
            type Preimages = Preimage;
            // The overarching call type.
            type RuntimeCall = RuntimeCall;
            // The overarching event type.
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = <$t as GovernanceWeight>::Whitelist;
            // Required origin for whitelisting a call.
            type WhitelistOrigin = <$t as GovernanceConfig>::WhitelistOrigin;
        }

        impl pallet_custom_origins::Config for Runtime {}

        parameter_types! {
            pub const MaxBalance: Balance = Balance::MAX;
        }
        pub type TreasurySpender = EitherOf<EnsureRootWithSuccess<AccountId, MaxBalance>, Spender>;

        // A pallet for executing referenda. A referendum is a vote on whether a proposal should be dispatched from a particular origin.
        impl pallet_referenda::Config for Runtime {
            // Quantization level for the referendum wakeup scheduler.
            type AlarmInterval = <$t as GovernanceConfig>::ReferendaAlarmInterval;
            // Origin from which any vote may be cancelled.
            type CancelOrigin = <$t as GovernanceConfig>::ReferendaCancelOrigin;
            // Currency type for this pallet.
            type Currency = Balances;
            // Origin from which any vote may be killed.
            type KillOrigin = <$t as GovernanceConfig>::ReferendaKillOrigin;
            // Maximum size of the referendum queue for a single track.
            type MaxQueued = <$t as GovernanceConfig>::ReferendaMaxQueued;
            // The preimage provider.
            type Preimages = Preimage;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            // The Scheduler.
            type Scheduler = Scheduler;
            // Handler for the unbalanced reduction when slashing a preimage deposit.
            type Slash = <$t as GovernanceConfig>::ReferendaSlash;
            // The minimum amount to be used as a deposit for a public referendum proposal.
            type SubmissionDeposit = <$t as GovernanceConfig>::ReferendaSubmissionDeposit;
            // Origin from which proposals may be submitted.
            type SubmitOrigin = <$t as GovernanceConfig>::ReferendaSubmitOrigin;
            // The tallying type.
            type Tally = pallet_conviction_voting::TallyOf<Runtime>;
            // Information concerning the different referendum tracks.
            type Tracks = tracks::TracksInfo;
            // The number of blocks after submission that a referendum must begin being decided by.
            // Once this passes, then anyone may cancel the referendum.
            type UndecidingTimeout = <$t as GovernanceConfig>::ReferendaUndecidingTimeout;
            // The counting type for votes. Usually just balance.
            type Votes = pallet_conviction_voting::VotesOf<Runtime>;
            type WeightInfo = <$t as GovernanceWeight>::Referenda;
        }
    };
}

pub const PALLET_NAMES: [(&str, &str); 6] = [
    ("Sudo", "pallet_sudo"),
    ("Treasury", "pallet_treasury"),
    ("ConvictionVoting", "pallet_conviction_voting"),
    ("Whitelist", "pallet_whitelist"),
    ("Origins", "pallet_custom_origins"),
    ("Referenda", "pallet_referenda"),
];
