#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_defaults)]

mod assets;
mod consensus;
mod evm;
mod governance;
mod system;
mod xcm;

use frame_support::traits::{ConstU32, Get};
use sp_version::RuntimeVersion;

pub trait SystemConfig {
    type AccountId;
    type Lookup;
    type SS58Prefix;
    type Version: Get<RuntimeVersion>;
    type ExistentialDeposit;
    type ScheduleOrigin;
    type PreimageOrigin;
    type MaxConsumers = ConstU32<16>;
    type MaxSignatories = ConstU32<100>;
    type MaxPendingProxies = ConstU32<32>;
    type MaxProxies = ConstU32<32>;
    type MaxFreezes = ConstU32<0>;
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ConstU32<50>;
}

pub trait ConsensusConfig {
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<100_000>;
    type MaxCandidates = ConstU32<100>;
    type MaxInvulnerables = ConstU32<20>;
    type MinEligibleCollators = ConstU32<4>;
    type CollatorSelectionUpdateOrigin;
}

pub trait AssetsConfig {
    type ApprovalDeposit;
    type AssetAccountDeposit;
    type AssetDeposit;
    type AssetId;
    type BenchmarkHelper = ();
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
    type TreasurySpendPeriod;
    type TreasuryPayoutSpendPeriod;
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
    type LocationToAccountId;
    type LocalOriginToLocation;
    type AssetTransactors;
    type XcmOriginToTransactDispatchOrigin;
    type FeeManager;
    type Trader;
    type Reserves;
    type MessageQueueHeapSize;
    type MessageQueueMaxStale;
    type MessageQueueServiceWeight;
    type XcmpQueueControllerOrigin;
    type XcmpQueueMaxInboundSuspended;
    type XcmAdminOrigin;
    type MaxActiveOutboundChannels = ConstU32<128>;
    type MaxPageSize = ConstU32<{ 1 << 16 }>;
}

pub trait EvmConfig {
    type AddressMapping;
    type FindAuthor;
    type CallOrigin;
    type WithdrawOrigin;
    type PrecompilesType;
    type PrecompilesValue;
}
