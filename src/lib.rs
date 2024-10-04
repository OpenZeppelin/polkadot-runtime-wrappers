#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_defaults)]

mod assets;
mod consensus;
mod governance;
mod system;
mod xcm;

use frame_support::pallet_prelude::{ConstU32, Get};
use sp_version::RuntimeVersion;

pub trait SystemConfig {
    type AccountId;
    type SS58Prefix;
    type Version: Get<RuntimeVersion>;
    type ExistentialDeposit;
    type ScheduleOrigin;
    type PreimageOrigin;
}

pub trait ConsensusConfig {
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<100_000>;
    type MaxCandidates = ConstU32<100>;
    type MaxInvulnerables = ConstU32<20>;
    type MinEligibleCollators = ConstU32<20>;
    type CollatorSelectionUpdateOrigin;
}

pub trait AssetsConfig {
    type ApprovalDeposit;
    type AssetAccountDeposit;
    type AssetDeposit;
    type CreateOrigin;
    type ForceOrigin;
}

pub trait GovernanceConfig {
    type TreasuryBurn = ();
    type TreasurySpendFunds = ();
    type TreasuryBurnDestination = ();
    type TreasuryMaxApprovals = ConstU32<100>;
    type TreasuryInteriorLocation;
    type TreasuryPalletId;
    type TreasuryProposalBond;
    type TreasuryProposalBondMinimum;
    type TreasuryProposalBondMaximum;
    type TreasurySpendPeriod;
    type TreasuryPayoutSpendPeriod;
    type TreasuryOnSlash;
    type TreasuryApproveOrigin;
    type TreasuryRejectOrigin;
    type TreasurySpendOrigin;
    type ConvictionVoteLockingPeriod;
    type ConvictionMaxVotes = ConstU32<512>;
    type DispatchWhitelistedOrigin;
    type WhitelistOrigin;
    type ReferendaAlarmInterval;
    type ReferendaCancelOrigin;
    type ReferendaKillOrigin;
    type ReferendaMaxQueued = ConstU32<20>;
    type ReferendaSlash;
    type ReferendaSubmissionDeposit;
    type ReferendaSubmitOrigin;
    type ReferendaUndecidingTimeout;
}

pub trait XcmConfig {
    type MessageQueueHeapSize;
    type MessageQueueMaxStale;
    type MessageQueueServiceWeight;
    type XcmpQueueControllerOrigin;
    type XcmpQueueMaxInboundSuspended;
}
