## OpenZeppelin Wrappers procedural macros

### `construct_runtime!`

We have made a wrapper over the `construct_runtime!` to support the abstractions. The macro itself have changed, supporting both abstractions and regular pallets:

```rust
#[construct_openzeppelin_runtime]
mod runtime {
    #[abstraction]
    struct System; // Available names are System, Consensus, XCM, Assets, Governance. EVM is in development.
    #[pallet]
    type Pallet = pallet_crate; // It mimicks the second version of construct runtime macro, but without the pallet_index assignment
}
```

Pallet index assignment is hidden from this API. If you want to use it, please create an issue.