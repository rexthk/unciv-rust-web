# Plan: P3-01 Hybrid Entity IDs

**Goal**: Complete P3-01 with UUID game identity and compact typed runtime entity IDs.
**Architecture**: `GameId(Uuid)` for persistent aggregate identity; distinct `u32` newtypes for
runtime civilization, city, and unit references.
**Tech Stack**: Rust, Serde, UUID, Proptest.

## Step 1: Specify the `GameId` contract

**File**: `crates/engine/tests/id.rs`

Add a property test covering construction, conversion, display/parsing, and transparent JSON
round-trips for arbitrary UUID bytes. Run `cargo test -p unciv-engine --test id` and confirm it
fails because `GameId` does not exist.

## Step 2: Implement `GameId`

**Files**: `crates/engine/src/id.rs`, `crates/engine/Cargo.toml`

Add the engine's workspace UUID dependency and implement a documented transparent UUID newtype
with `new`, `get`, `From` conversions, `Display`, `FromStr`, ordering/hash traits, and Serde.
Rerun the focused ID tests and confirm they pass.

## Step 3: Verify the complete change

Run formatting, workspace Clippy with warnings denied, workspace tests, strict engine Rustdoc, the
wasm32 API check, and a final code review.

| Group | Steps | Can Parallelize |
|---|---|---|
| 1 | Step 1 | No |
| 2 | Step 2 | No; depends on Step 1 |
| 3 | Step 3 | No; depends on Step 2 |
