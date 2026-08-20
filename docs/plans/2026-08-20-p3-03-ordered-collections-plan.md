# Plan: P3-03 Deterministic Ordered Collection Aliases

**Goal**: Complete P3-03 with engine-wide aliases for insertion-ordered maps and sets.
**Architecture**: Public aliases over Serde-enabled `IndexMap` and `IndexSet`.
**Tech Stack**: Rust, IndexMap, Serde JSON.

## Step 1: Specify stable serialization

**File**: `crates/engine/tests/collections.rs`

Add tests that build map and set values in deliberately non-sorted order and assert their exact
JSON representation. Include replacement and duplicate insertion to verify existing positions stay
stable. Run `cargo test -p unciv-engine --test collections` and confirm it fails because the
`collections` module does not exist.

## Step 2: Add the aliases

**Files**: `crates/engine/src/collections.rs`, `crates/engine/src/lib.rs`

Define documented `OrderedMap<K, V>` and `OrderedSet<T>` aliases and export the module. Rerun the
focused tests and confirm they pass.

## Step 3: Verify and prepare for review

Run formatting, workspace Clippy with warnings denied, workspace tests, strict engine Rustdoc, and
the wasm32 API check. Mark P3-03 done in the implementation plan and leave the diff uncommitted for
review.

| Group | Steps | Can Parallelize |
|---|---|---|
| 1 | Step 1 | No |
| 2 | Step 2 | No; depends on Step 1 |
| 3 | Step 3 | No; depends on Step 2 |
