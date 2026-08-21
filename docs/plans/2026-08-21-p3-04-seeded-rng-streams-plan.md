# Plan: P3-04 Seeded RNG Stream Identifiers

**Goal**: Add stable RNG stream identifiers and a reproducible cross-platform seeded sequence.
**Architecture**: A `u64` stream ID with a JavaScript-safe hexadecimal wire form selects a
SplitMix64 stream derived from an `i64` root seed with fixed domain separation.
**Tech Stack**: Rust, Serde, SplitMix64 wrapping arithmetic.

## Step 1: Specify identifier and sequence contracts

**File**: `crates/engine/tests/random.rs`

Add focused contract tests importing `unciv_engine::random::{RngStreamId, SeededRng}`. Verify:

```rust
let stream = RngStreamId::new(0x0123_4567_89ab_cdef);
assert_eq!(stream.get(), 0x0123_4567_89ab_cdef);
assert_eq!(serde_json::to_string(&stream).unwrap(), r#""0123456789abcdef""#);

let mut rng = SeededRng::new(-42, stream);
assert_eq!(
    (0..5).map(|_| rng.next_u64()).collect::<Vec<_>>(),
    [
        0xecc2_ae24_7554_c283,
        0xf081_9020_4e02_12a0,
        0x0f16_d892_34f4_6253,
        0x360b_c19f_0f0f_7207,
        0xdbd9_d245_a610_0c36,
    ],
);
```

Also assert that malformed wire values are rejected, reconstructing the same seed/stream
reproduces the sequence, and adjacent stream identifiers do not produce the same prefix.

Run `cargo test -p unciv-engine --test random` and confirm compilation fails because the `random`
module does not exist.

## Step 2: Add the deterministic RNG module

**Files**: `crates/engine/src/random.rs`, `crates/engine/src/lib.rs`

Add a documented `RngStreamId(u64)` with `new`, `get`, conversions, display, ordering, hashing, and
Serde as exactly 16 hexadecimal string digits. Add `SeededRng` with:

```rust
pub const fn new(seed: i64, stream_id: RngStreamId) -> Self;
pub fn next_u64(&mut self) -> u64;
```

Derive the initial state by applying the SplitMix64 finalizer to the stream ID XOR
`0x554e_4349_5652_4e47`, then mixing that with the root seed's two's-complement bits. Advance with
the SplitMix64 Weyl increment `0x9e37_79b9_7f4a_7c15` and standard finalizer multipliers. Export the
module from `lib.rs`.

Rerun `cargo test -p unciv-engine --test random` and confirm all focused tests pass.

## Step 3: Update the task ledger and verify end to end

**File**: `docs/plans/2026-08-19-unciv-rust-web-implementation-plan.md`

Mark P3-04 done. Run:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
RUSTDOCFLAGS="-D warnings" cargo doc -p unciv-engine --no-deps
cargo check -p unciv-wasm-api --target wasm32-unknown-unknown
```

Inspect the final diff and status so only P3-04 files are included.

| Group | Steps | Can Parallelize |
|---|---|---|
| 1 | Step 1 | No |
| 2 | Step 2 | No; depends on Step 1 |
| 3 | Step 3 | No; depends on Step 2 |
