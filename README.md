# Blockchain

A minimal Rust blockchain runtime playground that demonstrates how block execution and pallet-style modules can work together.

## What this repository includes

- **`/tmp/workspace/Dan-Mach/Blockchain/blockchain`**: Main crate containing a small runtime and pallets.
- **`/tmp/workspace/Dan-Mach/Blockchain/blockchain/macros`**: Procedural macro crate used to generate call-dispatch boilerplate.
- **`/tmp/workspace/Dan-Mach/Blockchain/server`**: Scaffolded server crate (currently minimal/empty).

## Core functionality

The runtime composes multiple pallets and executes blocks of extrinsics:

- **System pallet (`system.rs`)**
  - Tracks block number.
  - Tracks account nonces.
- **Balances pallet (`balances.rs`)**
  - Stores balances by account.
  - Supports token transfers with underflow/overflow checks.
- **Proof of Existence pallet (`proof_of_existence.rs`)**
  - Allows users to create and revoke ownership claims for content.
  - Prevents duplicate claims and unauthorized revocations.
- **Support types (`support.rs`)**
  - Defines `Block`, `Header`, `Extrinsic`, and `Dispatch` primitives.

The example runtime in `blockchain/src/main.rs` creates sample accounts, builds two blocks, dispatches balance transfers and proof-of-existence calls, and prints resulting state.

## Build and run

From repository root:

```bash
cargo build
cargo run -p blockchain
```

## Test

```bash
cargo test
```

## Notes

- This project is educational and intentionally minimal.
- Some crates/files are scaffolding for future expansion.
