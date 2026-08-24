# Plan: P3-05 Game Revision Type

**Goal**: Complete P3-05 with a monotonic game-state revision that fails explicitly on overflow
and is JS/wasm-safe.
**Architecture**: A transparent `u64` newtype with a checked increment, stable JS-safe decimal
string Serde (not a JSON number, to avoid exceeding JS safe-integer `2^53-1`), and a dedicated
overflow error.
**Tech Stack**: Rust, Serde, thiserror, Proptest.

## Step 1: Specify the revision contract

**File**: `crates/engine/tests/revision.rs`

Add focused contract tests importing `unciv_engine::revision::{GameRevision, RevisionOverflow}`.
Verify:

```rust
let revision = GameRevision::INITIAL;
assert_eq!(revision.get(), 0);
assert_eq!(serde_json::to_string(&revision).unwrap(), r#""0""#);

let next = revision.next().expect("initial increments");
assert_eq!(next.get(), 1);

let maximum = GameRevision::new(u64::MAX);
assert_eq!(maximum.next().unwrap_err(), RevisionOverflow);
assert_eq!(RevisionOverflow.to_string(), "game revision overflow");
```

Also assert that `checked_next` returns `None` on overflow, `increment` preserves the value on
error, ordering matches numeric ordering, JSON round-trips arbitrary values as decimal strings
(including >2^53 and MAX), rejects malformed values, and accepts both string and number on
deserialize for compatibility.

Run `cargo test -p unciv-engine --test revision` and confirm compilation fails because the
`revision` module does not exist.

## Step 2: Add the game revision module

**Files**: `crates/engine/src/revision.rs`, `crates/engine/src/lib.rs`

Add a documented `GameRevision(u64)` with `new`, `get`, `INITIAL`, `MAX`,
conversions, `Display`, ordering, hashing, and Serde as a JS-safe decimal string
(`collect_str`, not `serialize_u64`, to avoid JS number precision loss). Add:

```rust
pub fn next(self) -> Result<Self, RevisionOverflow>;
pub const fn checked_next(self) -> Option<Self>;
pub fn increment(&mut self) -> Result<(), RevisionOverflow>;
```

Implement overflow via `u64::checked_add(1)` and return `RevisionOverflow`, a `thiserror` type
with the stable message `"game revision overflow"`. Accept both string and number forms during
deserialization for compatibility. Export the module from `lib.rs`.

Rerun `cargo test -p unciv-engine --test revision` and confirm all focused tests pass.

## Step 3: Update the task ledger and verify end to end

**File**: `docs/plans/2026-08-19-unciv-rust-web-implementation-plan.md`

Mark P3-05 done. Run:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
RUSTDOCFLAGS="-D warnings" cargo doc -p unciv-engine --no-deps
cargo check -p unciv-wasm-api --target wasm32-unknown-unknown
```

Inspect the final diff and status so only P3-05 files are included.

| Group | Steps | Can Parallelize |
|---|---|---|
| 1 | Step 1 | No |
| 2 | Step 2 | No; depends on Step 1 |
| 3 | Step 3 | No; depends on Step 2 |
