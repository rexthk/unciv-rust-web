# P3-04 Seeded RNG Stream Identifiers Design

**Date:** 2026-08-21

## Design

The engine exposes `RngStreamId`, a transparent `u64` newtype used to assign stable numeric
identities to independent deterministic-random consumers. Subsystems will declare explicit
constants rather than deriving identifiers from Rust names or caller strings, so refactoring a
module cannot silently change gameplay. Once an identifier contributes to persisted or observable
behavior, it must not be renumbered or reused. The type supports ordering, hashing, display,
conversion, and transparent Serde representation like the existing engine ID types.

`SeededRng` combines an Unciv-compatible signed 64-bit root seed with an `RngStreamId`. The signed
seed is reinterpreted as its two's-complement `u64` bit pattern, then both inputs are mixed with the
standard SplitMix64 finalizer and a fixed domain-separation constant. The resulting state drives a
SplitMix64 generator whose wrapping arithmetic is identical on native and Wasm targets. Calls to
one stream therefore cannot consume values from another stream, and reconstructing a stream from
the same seed and identifier always restarts the same sequence.

Only `next_u64` is included in this task. Bounded integers, floating-point sampling, shuffling, and
domain-specific stream constants will be added with their consuming features, avoiding APIs whose
compatibility requirements are not yet known. The generator state is intentionally not serialized;
later game state should store stable seeds and identifiers or explicit domain state rather than an
opaque implementation cursor.

## Alternatives Considered

- PCG32 has a native stream selector, but limits stream identity and adds a second output width
  before consumers require bounded sampling.
- `rand_chacha` offers strong independent streams, but introduces dependency-version coupling and
  more machinery than this foundational task needs.
- Hashing caller strings resembles Unciv's `stateBasedRandom`, but makes renames gameplay-affecting
  and admits accidental runtime identifiers.

## Verification

Contract tests lock the transparent identifier representation, the exact golden sequence for a
fixed signed seed and stream, reproducibility, and separation between distinct stream identifiers.
Formatting, Clippy, workspace tests, strict engine Rustdoc, and the Wasm API build must pass.
