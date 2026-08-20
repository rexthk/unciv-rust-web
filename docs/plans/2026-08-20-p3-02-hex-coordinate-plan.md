# Plan: P3-02 Hex Coordinate Type

**Goal**: Complete P3-02 with an Unciv-compatible hex coordinate value and deterministic geometry.
**Architecture**: Signed `i32` `HexCoord` components using Unciv's native two-axis basis, with
clockwise neighbors and overflow-safe unwrapped distance.
**Tech Stack**: Rust, Serde, Proptest.

## Step 1: Specify the geometry contract

**File**: `crates/engine/tests/hex.rs`

Add focused examples for the six neighbors and representative distance cases, plus properties for
neighbor distance, symmetry, and translation invariance. Add an extreme-coordinate example that
requires widened arithmetic and verify neighbor overflow is deterministic. Run
`cargo test -p unciv-engine --test hex` and confirm it fails because the `hex` module does not
exist.

## Step 2: Implement `HexCoord`

**Files**: `crates/engine/src/hex.rs`, `crates/engine/src/lib.rs`

Add the documented coordinate value, component access, `ZERO`, deterministic direction and
neighbor arrays, and overflow-safe distance calculation. Export the module from the engine crate.
Rerun the focused test and confirm it passes.

## Step 3: Verify and record completion

Run formatting, workspace Clippy with warnings denied, workspace tests, strict engine Rustdoc, and
the wasm32 API check. Mark P3-02 done in the implementation plan, review the focused diff, and
commit the task.

| Group | Steps | Can Parallelize |
|---|---|---|
| 1 | Step 1 | No |
| 2 | Step 2 | No; depends on Step 1 |
| 3 | Step 3 | No; depends on Step 2 |
