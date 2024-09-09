Configuring and creating a runtime in Substrate may be an intimidating task for the newcomers. As OpenZeppelin, we are aiming to ease this process and lower the entry-barrier for Runtime Configuration by abstracting away the most common (yet still intricate) parts of the process for the new projects, while still preserving the full customizability that Substrate provides.

# Before Polkadot-Runtime-Wrappers


### 10s or 100s of pallet Config implementations looking like this:
```rust
parameter_types! {
	pub const ExistentialDeposit: Balance = 0;
}

impl pallet_balances::Config for Runtime {
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 4];
	type MaxLocks = ConstU32<50>;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type FreezeIdentifier = ();
	type MaxFreezes = ConstU32<0>;
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type WeightInfo = moonbeam_weights::pallet_balances::WeightInfo<Runtime>;
}
```

### 10s or 100s of RuntimeApi implementations looking like this:

```rust
impl_runtime_apis! {
    /*
    ... redacted
    */
    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }
    /*
    ... redacted
    */
}
```

### Very long list of pallets in Runtime Macro:
```rust
construct_runtime!(
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

        /*
        ... redacted
        */
    }
);
```

### And as a result of these, very long list of imports and `use` statements

```rust
/*
... redacted
*/

pub use frame_support::traits::Get;
use frame_support::{
	construct_runtime,
	dispatch::{DispatchClass, GetDispatchInfo, PostDispatchInfo},
	ensure,
	pallet_prelude::DispatchResult,
	parameter_types,
	traits::{
		fungible::{Balanced, Credit, HoldConsideration, Inspect},
		tokens::imbalance::ResolveTo,
		tokens::{PayFromAccount, UnityAssetBalanceConversion},
		ConstBool, ConstU128, ConstU16, ConstU32, ConstU64, ConstU8, Contains, EitherOfDiverse,
		EqualPrivilegeOnly, Imbalance, InstanceFilter, LinearStoragePrice, OnFinalize,
		OnUnbalanced,
	},
	weights::{
		constants::WEIGHT_REF_TIME_PER_SECOND, ConstantMultiplier, Weight, WeightToFeeCoefficient,
		WeightToFeeCoefficients, WeightToFeePolynomial,
	},
	PalletId,
};
use frame_system::{EnsureRoot, EnsureSigned};

/*
... redacted
*/
```

# After Polkadot-Runtime-Wrappers

1. Grouped pallets under `system`, `monetary`, `governance`, `collator support`, and `xcm` as it currently is in our runtime code.
2. For each group, we are exposing a trait. These traits help configuring the pallets with minimal efforts:
    1. associated types will be the things that users will want to configure the most.
    2. all the other details will be hidden from the user, hence, runtime configuration will be much easier.
    3. `exclude_pallets_from_default_config` function provides a way to opt-out of defaults for specific. pallets. This will be an advanced feature for users that know what they are doing.
3. If the users want to opt-out of the whole subgroup of pallets (say, `monetary`) instead of individual pallets, they still can do this.
4. If the users want to add additional pallets (any pallets, including custom ones), they can do so via `CustomPallets` trait.
5. `OzConfig` struct ensures the type-safety.


### The whole Runtime configuration is now free of boilerplate, and focuses on each runtime's specific business logic:

1. no more 10s or 100s of pallet config configurations.
2. no more 10s or 100s of runtime api implementations.
3. no more a long intimidating list of pallet lists for constructing the runtime.
4. no more 100s of lines of `use` statements.

```rust
use oz_pallets::{
		OzConfig,
		traits::{OzSystem, OzMonetary, OzGovernance, OzCollator, OzXcm, CustomPallets}
};

/* configuration for our subgroupings */
struct MySystem;

impl OzSystem for MySystem {
    type account = Config::AccountId,
    type prefix = Config::SS58Prefix,
    type version = Config::Version,

    fn exclude_pallets_from_default_config() -> impl IntoIterator<AsRef<str>> {
        return vec!["pallet_multisig".to_string()];
    }
}

struct MyMonetary;

impl OzMonetary for MyMonetary {
		// same structure as above
}


/* configuration for additional/custom pallets */
struct MyPallets;

impl CustomPallets for MyPallets {
		// it is the user's responsibility to include the necessary
		// `impl custom_pallet::Config for Runtime` code block
		// to the codebase for the pallets added here
		fn include_pallets() -> impl IntoIterator<AsRef<str>> {
				return vec!["XTokens: orml_xtokens".to_string()];
		}
}

/* creating the `OzConfig` struct and generating the runtime */
let MyConfig = OzConfig {
		system:        Some(MySystem),
		monetary:      Some(MyMonetary),
		governance:    None,
		collator:      Some(MyCollator),
		xcm:           Some(MyXcm),
		extra_pallets: Some(MyPallets),
};


construct_runtime_oz!(MyConfig);
// the above will expand to:
// - regular `construct_runtime!` macro, or
// - the more modern `#[frame_support::runtime]` macro



/* optional stuff */

// Since `pallet_multisig` is excluded from the default config in the trait implementation,
// we expect the configuration for `pallet_multisig` to be present in the codebase
impl pallet_multisig::Config for Runtime {
    type Currency = Balances;
    type DepositBase = DepositBase;
    type DepositFactor = DepositFactor;
    type MaxSignatories = MaxSignatories;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    /// Rerun benchmarks if you are making changes to runtime configuration.
    type WeightInfo = weights::pallet_multisig::WeightInfo<Runtime>;
}

// orml_xtokens is an extra pallet, hence it's definition should be provided by the user
impl orml_xtokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type CurrencyIdConvert = BifrostCurrencyIdConvert<ParachainInfo>;
	type AccountIdToLocation = BifrostAccountIdToLocation;
	type SelfLocation = SelfRelativeLocation;
	type LocationsFilter = Everything;
	type MinXcmFee = ParachainMinFee;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type UniversalLocation = UniversalLocation;
	type MaxAssetsForTransfer = MaxAssetsForTransfer;
	type ReserveProvider = RelativeReserveProvider;
	type RateLimiter = ();
	type RateLimiterId = ();
}
```
