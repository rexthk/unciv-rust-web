# P3-05 Game Revision Design

**Date:** 2026-08-24

## Design

The engine exposes `GameRevision`, a transparent `u64` newtype that identifies a specific version
of game state. A newly created game starts at revision `0` (`INITIAL`); each successful command
increments the revision by exactly one and returns the new revision with its events and
presentation updates. Failed or stale commands never change the revision, preserving atomicity.

`GameRevision` supports optimistic concurrency: a command carries the client's expected revision,
and the engine rejects the command when the expected revision does not match the current
revision. This prevents lost updates when multiple UI surfaces, workers, or multiplayer peers
attempt to mutate the same game concurrently. Multiplayer conflict handling, persistence
envelopes, and diagnostic bundles all reference this revision, so its representation must be
stable across native, Wasm, and JSON boundaries.

The type provides `new`, `get`, numeric conversions, `Display`, ordering, hashing, and
Serde as a canonical decimal string. Serializing a `u64` as a JSON number would exceed
JavaScript's safe-integer range (`2^53 - 1`) and lose precision through JSON / `wasm-bindgen` /
JS, so the wire format must be a string to preserve all 64 bits (consistent with
`RngStreamId`'s use of a string for JS safety). Deserialization accepts both decimal strings
(new format) and JSON numbers (legacy/tolerant) for compatibility. Using `u64` gives an
effectively unbounded monotonic counter; the implementation explicitly checks for overflow rather
than wrapping, returning a dedicated `RevisionOverflow` error. Although overflow is unreachable
in realistic gameplay (it would require 2^64 successful commands), the engine must not silently
reuse a revision.

Only `next`, `checked_next`, and `increment` are included in this task. The typed command
error enum that wraps a stale-revision failure, the command envelope that carries the expected
revision, and the state root that owns the current revision are deferred to P3-06 through
P3-10.

## Alternatives Considered

- `u32` would match entity IDs and JavaScript safe integers, but truncates sooner and forces an
  earlier overflow policy; `u64` aligns with the SplitMix64 seed width and leaves headroom for
  long-running or automated games.
- Wrapping increment (`wrapping_add`) would silently reuse revisions and break optimistic
  concurrency; checked increment with an explicit error preserves the invariant that revisions
  are unique.
- Serializing as a JSON number would preserve simplicity for small values but would break
  JavaScript for values above `2^53 - 1` and via `wasm-bindgen`; the engine must not rely on
  "realistic" game lengths staying below that threshold.
- Serializing as a fixed-width hex string like `RngStreamId` would preserve all 64 bits but is
  less readable for a monotonic counter and mismatches its decimal `Display`; decimal string is
  the natural JS-safe representation for a revision.
- Using an external crate such as `revision` or `versioned` introduces dependency coupling without
  matching the engine's transparent `u64` contract.

## Verification

Contract tests lock the initial revision (`0`), raw-value preservation, JavaScript-safe decimal
string JSON round-trips (including values above `2^53` and `u64::MAX`), ordering, and boundary
values, plus malformed-input rejection and tolerant deserialization from both string and number.
Overflow tests verify that incrementing `u64::MAX` returns `RevisionOverflow` with a stable
message, does not panic, does not wrap, and leaves the original value unchanged. Property tests
verify increment-by-one for all non-maximum values and that `checked_next` agrees with `next`.
Workspace formatting, Clippy, tests, Rustdoc, and the Wasm API build must pass.
