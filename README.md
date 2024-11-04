# polkadot-runtime-wrappers

`polkadot-runtime-wrappers` is a set of Rust macros designed to streamline the configuration of Polkadot Parachain Runtimes. These macros reduce the lines of code (LOC) necessary for configuring a secure and optimized runtime, providing a balance between customizability and ease of use.

Features:
- Reduced LOC: Minimize boilerplate by using macros to handle repetitive configurations.
- Sensible Defaults: Default configurations that meet most standard use cases, ensuring security and functionality without requiring extensive customization.
- High Configurability: Enable advanced users to customize runtime configurations, offering flexible settings without sacrificing simplicity for common setups.
- Security-Focused: Built with security in mind, ensuring that configurations adhere to best practices for Polkadot parachains.

## Installation

To use `polkadot-runtime-wrappers`, add it to your Cargo.toml file:

```toml
[dependencies]
openzeppelin-polkadot-wrappers = "0.1.0"
```

Then, import the necessary macros in your runtime configuration file.

## Usage

The macros are intended to streamline runtime configuration for Polkadot parachains. Here’s a basic example from the EVM parachain runtime maintained in `[openzeppelin/polkadot-runtime-templates](https://github.com/OpenZeppelin/polkadot-runtime-templates)`:

```rust, ignore
use openzeppelin_polkadot_wrappers::{impl_openzeppelin_system, SystemConfig};

pub struct OpenZeppelinRuntime;
impl SystemConfig for OpenZeppelinRuntime {
    // Basic configuration options:
    type AccountId = AccountId;
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type PreimageOrigin = EnsureRoot<AccountId>;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    type Version = Version;
    //...
}
impl_openzeppelin_system!(OpenZeppelinRuntime);
```

The `openzeppelin_polkadot_wrappers::impl_openzeppelin_system` macro call expands to implement the system grouping pallets for the Runtime by using the user configuration specified in the `openzeppelin_polkadot_wrappers::SystemConfig` implementation by `OpenZeppelinRuntime`:
- `frame_system`
- `pallet_timestamp`
- `parachain_info`
- `pallet_scheduler`
- `pallet_preimage`
- `pallet_proxy`
- `pallet_balances`
- `pallet_utility`
- `cumulus_pallet_parachain_system`
- `pallet_multisig`

There are also macros for the following pallet groupings:
- Assets
- Consensus
- EVM
- Governance
- XCM

Here are their example configs in the EVM parachain runtime:
```rust, ignore
use openzeppelin_polkadot_wrappers::{
    impl_openzeppelin_assets, impl_openzeppelin_consensus, impl_openzeppelin_evm,
    impl_openzeppelin_governance, impl_openzeppelin_xcm, AssetsConfig,
    ConsensusConfig, EvmConfig, GovernanceConfig, XcmConfig,
};
//...other imported types used in the configuration

impl ConsensusConfig for OpenZeppelinRuntime {
    type CollatorSelectionUpdateOrigin = CollatorSelectionUpdateOrigin;
    // Some types may be left unassigned to use defaults
}
impl GovernanceConfig for OpenZeppelinRuntime {
    type ConvictionVoteLockingPeriod = ConstU32<{ 7 * DAYS }>;
    type DispatchWhitelistedOrigin = EitherOf<EnsureRoot<AccountId>, WhitelistedCaller>;
    //...
}
impl XcmConfig for OpenZeppelinRuntime {
    type BaseXcmWeight = BaseXcmWeight;
    type LocalOriginToLocation = LocalOriginToLocation;
    type LocationToAccountId = LocationToAccountId;
    type MessageQueueHeapSize = ConstU32<{ 64 * 1024 }>;
    type MessageQueueMaxStale = ConstU32<8>;
    //...
}
impl EvmConfig for OpenZeppelinRuntime {
    type AddressMapping = IdentityAddressMapping;
    type CallOrigin = EnsureAccountId20;
    type PrecompilesType = OpenZeppelinPrecompiles<Runtime>;
    type PrecompilesValue = PrecompilesValue;
    //...
}
impl AssetsConfig for OpenZeppelinRuntime {
    type AssetDeposit = ConstU128<{ 10 * CENTS }>;
    type AssetId = u128;
    type ForceOrigin = EnsureRoot<AccountId>;
    //...
}
impl_openzeppelin_assets!(OpenZeppelinRuntime);
impl_openzeppelin_consensus!(OpenZeppelinRuntime);
impl_openzeppelin_governance!(OpenZeppelinRuntime);
impl_openzeppelin_xcm!(OpenZeppelinRuntime);
impl_openzeppelin_evm!(OpenZeppelinRuntime);
```

Here are a few ways `polkadot-runtime-wrappers` simplifies parachain configuration:
- Basic Setup: Only a few LOC required to get a secure, functioning runtime.
- Advanced Configuration: Customize each aspect of the runtime by passing additional parameters to the macros.
- Default Overrides: Override defaults for specific settings while letting the wrapper handle the rest.

## Contributing

Contributions are welcome! Please see our [CONTRIBUTING.md](./CONTRIBUTING.MD) for guidelines.

## License

This project is licensed under the GPLv3 License. See the [LICENSE](./LICENSE.md) file for more information.
