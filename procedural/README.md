## OpenZeppelin Wrappers procedural macros

### `construct_runtime!`

We have made a wrapper over the `construct_runtime!` to support the abstractions. The macro itself have changed, supporting both abstractions and regular pallets:

```rust
#[construct_openzeppelin_runtime]
mod runtime {
    #[abstraction]
    struct System; // Available names are System, Consensus, XCM, Assets, Governance. EVM is in development.
    #[pallet]
    type Pallet = pallet_crate; // It mimics the second version of construct runtime macro, but without the pallet_index assignment
}
```

Pallet index assignment is hidden from this API. If you want to use it, please create an issue.

#### Supported abstractions:

* `System` -- `frame_system`, `pallet_timestamp`, `parachain_info`, `pallet_scheduler`, `pallet_preimage`, `pallet_proxy`, `pallet_balances`, `pallet_utility`, `cumulus_pallet_parachain_system`, `pallet_multisig`, `pallet_session`
* `Assets` -- `pallet_assets`, `pallet_transaction_payment`
* `Consensus` -- `pallet_authorship`, `pallet_aura`, `cumulus_pallet_aura_ext`, `pallet_collator_selection`
* `Governance` -- `pallet_sudo`, `pallet_treasury`, `pallet_conviction_voting`, `pallet_whitelist`, `pallet_custom_origins`, `pallet_referenda`
* `XCM` -- `pallet_message_queue`, `cumulus_pallet_xcmp_queue`, `pallet_xcm`, `cumulus_pallet_xcm`
* `EVM` -- `pallet_ethereum`, `pallet_evm`, `pallet_base_fee`, `pallet_evm_chain_id`