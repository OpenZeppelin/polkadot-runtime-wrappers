//! Governance pallet groupings wrapper

#[macro_export]
macro_rules! impl_openzeppelin_governance {
    ($t:ty) => {
        impl pallet_sudo::Config for Runtime {
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_sudo::WeightInfo<Runtime>;
        }

        #[cfg(feature = "runtime-benchmarks")]
        parameter_types! {
            pub LocationParents: u8 = 1;
            pub BenchmarkParaId: u8 = 0;
        }

        parameter_types! {
            pub TreasuryAccount: AccountId = Treasury::account_id();
        }

        impl pallet_treasury::Config for Runtime {
            type ApproveOrigin = <$t as GovernanceConfig>::TreasuryApproveOrigin;
            type AssetKind = AssetKind;
            type BalanceConverter = frame_support::traits::tokens::UnityAssetBalanceConversion;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkHelper = polkadot_runtime_common::impls::benchmarks::TreasuryArguments<
                LocationParents,
                BenchmarkParaId,
            >;
            type Beneficiary = Beneficiary;
            type BeneficiaryLookup = IdentityLookup<Self::Beneficiary>;
            type Burn = <$t as GovernanceConfig>::TreasuryBurn;
            type BurnDestination = <$t as GovernanceConfig>::TreasuryBurnDestination;
            type Currency = Balances;
            type MaxApprovals = <$t as GovernanceConfig>::TreasuryMaxApprovals;
            type OnSlash = <$t as GovernanceConfig>::TreasuryOnSlash;
            type PalletId = <$t as GovernanceConfig>::TreasuryPalletId;
            #[cfg(feature = "runtime-benchmarks")]
            type Paymaster = PayWithEnsure<TreasuryPaymaster, OpenHrmpChannel<BenchmarkParaId>>;
            #[cfg(not(feature = "runtime-benchmarks"))]
            type Paymaster = TreasuryPaymaster;
            type PayoutPeriod = <$t as GovernanceConfig>::TreasuryPayoutSpendPeriod;
            type ProposalBond = <$t as GovernanceConfig>::TreasuryProposalBond;
            type ProposalBondMaximum = <$t as GovernanceConfig>::TreasuryProposalBondMaximum;
            type ProposalBondMinimum = <$t as GovernanceConfig>::TreasuryProposalBondMinimum;
            type RejectOrigin = <$t as GovernanceConfig>::TreasuryRejectOrigin;
            type RuntimeEvent = RuntimeEvent;
            type SpendFunds = <$t as GovernanceConfig>::TreasurySpendFunds;
            type SpendOrigin = <$t as GovernanceConfig>::TreasurySpendOrigin;
            type SpendPeriod = <$t as GovernanceConfig>::TreasurySpendPeriod;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
        }

        impl pallet_conviction_voting::Config for Runtime {
            type Currency = Balances;
            type MaxTurnout = frame_support::traits::tokens::currency::ActiveIssuanceOf<
                Balances,
                Self::AccountId,
            >;
            type MaxVotes = <$t as GovernanceConfig>::ConvictionMaxVotes;
            type Polls = Referenda;
            type RuntimeEvent = RuntimeEvent;
            type VoteLockingPeriod = <$t as GovernanceConfig>::ConvictionVoteLockingPeriod;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_conviction_voting::WeightInfo<Runtime>;
        }

        impl pallet_whitelist::Config for Runtime {
            type DispatchWhitelistedOrigin = <$t as GovernanceConfig>::DispatchWhitelistedOrigin;
            type Preimages = Preimage;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_whitelist::WeightInfo<Runtime>;
            type WhitelistOrigin = <$t as GovernanceConfig>::WhitelistOrigin;
        }

        impl pallet_custom_origins::Config for Runtime {}

        parameter_types! {
            pub const MaxBalance: Balance = Balance::MAX;
        }
        pub type TreasurySpender = EitherOf<EnsureRootWithSuccess<AccountId, MaxBalance>, Spender>;

        impl pallet_referenda::Config for Runtime {
            type AlarmInterval = <$t as GovernanceConfig>::ReferendaAlarmInterval;
            type CancelOrigin = <$t as GovernanceConfig>::ReferendaCancelOrigin;
            type Currency = Balances;
            type KillOrigin = <$t as GovernanceConfig>::ReferendaKillOrigin;
            type MaxQueued = <$t as GovernanceConfig>::ReferendaMaxQueued;
            type Preimages = Preimage;
            type RuntimeCall = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            type Scheduler = Scheduler;
            type Slash = <$t as GovernanceConfig>::ReferendaSlash;
            type SubmissionDeposit = <$t as GovernanceConfig>::ReferendaSubmissionDeposit;
            type SubmitOrigin = <$t as GovernanceConfig>::ReferendaSubmitOrigin;
            type Tally = pallet_conviction_voting::TallyOf<Runtime>;
            type Tracks = tracks::TracksInfo;
            type UndecidingTimeout = <$t as GovernanceConfig>::ReferendaUndecidingTimeout;
            type Votes = pallet_conviction_voting::VotesOf<Runtime>;
            /// Rerun benchmarks if you are making changes to runtime configuration.
            type WeightInfo = weights::pallet_referenda::WeightInfo<Runtime>;
        }
    };
}