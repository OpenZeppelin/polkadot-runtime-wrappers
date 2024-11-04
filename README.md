# polkadot-runtime-wrappers

Wrappers with sensible defaults for constructing Polkadot Parachain Runtimes


## Example: EVM Parachain Template 

Example usage may be found in both runtime templates in `[openzeppelin/polkadot-runtime-templates]()`.

The configuration of the EVM Parachain Template at the time that this README was written is posted below:

```rust, ignore
// OpenZeppelin runtime wrappers configuration
pub struct OpenZeppelinRuntime;
impl SystemConfig for OpenZeppelinRuntime {
    type AccountId = AccountId;
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type Lookup = IdentityLookup<AccountId>;
    type PreimageOrigin = EnsureRoot<AccountId>;
    type ProxyType = ProxyType;
    type SS58Prefix = ConstU16<42>;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    type Version = Version;
}
impl ConsensusConfig for OpenZeppelinRuntime {
    type CollatorSelectionUpdateOrigin = CollatorSelectionUpdateOrigin;
}
impl GovernanceConfig for OpenZeppelinRuntime {
    type ConvictionVoteLockingPeriod = ConstU32<{ 7 * DAYS }>;
    type DispatchWhitelistedOrigin = EitherOf<EnsureRoot<AccountId>, WhitelistedCaller>;
    type ReferendaAlarmInterval = ConstU32<1>;
    type ReferendaCancelOrigin = EnsureRoot<AccountId>;
    type ReferendaKillOrigin = EnsureRoot<AccountId>;
    type ReferendaSlash = Treasury;
    type ReferendaSubmissionDeposit = ConstU128<{ 3 * CENTS }>;
    type ReferendaSubmitOrigin = EnsureSigned<AccountId>;
    type ReferendaUndecidingTimeout = ConstU32<{ 14 * DAYS }>;
    type TreasuryInteriorLocation = TreasuryInteriorLocation;
    type TreasuryPalletId = TreasuryPalletId;
    type TreasuryPayoutSpendPeriod = ConstU32<{ 30 * DAYS }>;
    type TreasuryRejectOrigin = EitherOfDiverse<EnsureRoot<AccountId>, Treasurer>;
    type TreasurySpendOrigin = TreasurySpender;
    type TreasurySpendPeriod = ConstU32<{ 6 * DAYS }>;
    type WhitelistOrigin = EnsureRoot<AccountId>;
}
impl XcmConfig for OpenZeppelinRuntime {
    type AccountIdToLocation = AccountIdToLocation<AccountId>;
    type AddSupportedAssetOrigin = EnsureRoot<AccountId>;
    type AssetFeesFilter = AssetFeesFilter;
    type AssetTransactors = AssetTransactors;
    type BaseXcmWeight = BaseXcmWeight;
    type CurrencyId = CurrencyId;
    type CurrencyIdToLocation = CurrencyIdToLocation<AsAssetType<AssetId, AssetType, AssetManager>>;
    type DerivativeAddressRegistrationOrigin = EnsureRoot<AccountId>;
    type EditSupportedAssetOrigin = EnsureRoot<AccountId>;
    type FeeManager = FeeManager;
    type HrmpManipulatorOrigin = EnsureRoot<AccountId>;
    type HrmpOpenOrigin = EnsureRoot<AccountId>;
    type LocalOriginToLocation = LocalOriginToLocation;
    type LocationToAccountId = LocationToAccountId;
    type MaxAssetsForTransfer = MaxAssetsForTransfer;
    type MaxHrmpRelayFee = MaxHrmpRelayFee;
    type MessageQueueHeapSize = ConstU32<{ 64 * 1024 }>;
    type MessageQueueMaxStale = ConstU32<8>;
    type MessageQueueServiceWeight = MessageQueueServiceWeight;
    type ParachainMinFee = ParachainMinFee;
    type PauseSupportedAssetOrigin = EnsureRoot<AccountId>;
    type RelayLocation = RelayLocation;
    type RemoveSupportedAssetOrigin = EnsureRoot<AccountId>;
    type Reserves = Reserves;
    type ResumeSupportedAssetOrigin = EnsureRoot<AccountId>;
    type SelfLocation = SelfLocation;
    type SelfReserve = SelfReserve;
    type SovereignAccountDispatcherOrigin = EnsureRoot<AccountId>;
    type Trader = pallet_xcm_weight_trader::Trader<Runtime>;
    type TransactorReserveProvider = AbsoluteAndRelativeReserve<SelfLocationAbsolute>;
    type Transactors = Transactors;
    type UniversalLocation = UniversalLocation;
    type WeightToFee = WeightToFee;
    type XcmAdminOrigin = EnsureRoot<AccountId>;
    type XcmFeesAccount = TreasuryAccount;
    type XcmOriginToTransactDispatchOrigin = XcmOriginToTransactDispatchOrigin;
    type XcmSender = XcmRouter;
    type XcmWeigher = XcmWeigher;
    type XcmpQueueControllerOrigin = EnsureRoot<AccountId>;
    type XcmpQueueMaxInboundSuspended = ConstU32<1000>;
    type XtokensReserveProviders = ReserveProviders;
}
impl EvmConfig for OpenZeppelinRuntime {
    type AddressMapping = IdentityAddressMapping;
    type CallOrigin = EnsureAccountId20;
    type Erc20XcmBridgeTransferGasLimit = Erc20XcmBridgeTransferGasLimit;
    type FindAuthor = FindAuthorSession<Aura>;
    type PrecompilesType = OpenZeppelinPrecompiles<Runtime>;
    type PrecompilesValue = PrecompilesValue;
    type WithdrawOrigin = EnsureAccountId20;
}
impl AssetsConfig for OpenZeppelinRuntime {
    type ApprovalDeposit = ConstU128<MILLICENTS>;
    type AssetAccountDeposit = ConstU128<{ deposit(1, 16) }>;
    type AssetDeposit = ConstU128<{ 10 * CENTS }>;
    type AssetId = AssetId;
    type AssetRegistrar = AssetRegistrar;
    type AssetRegistrarMetadata = AssetRegistrarMetadata;
    type AssetType = AssetType;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = BenchmarkHelper;
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
    type ForceOrigin = EnsureRoot<AccountId>;
    type ForeignAssetModifierOrigin = EnsureRoot<AccountId>;
    type WeightToFee = WeightToFee;
}
impl_openzeppelin_assets!(OpenZeppelinRuntime);
impl_openzeppelin_system!(OpenZeppelinRuntime);
impl_openzeppelin_consensus!(OpenZeppelinRuntime);
impl_openzeppelin_governance!(OpenZeppelinRuntime);
impl_openzeppelin_xcm!(OpenZeppelinRuntime);
impl_openzeppelin_evm!(OpenZeppelinRuntime);
```
