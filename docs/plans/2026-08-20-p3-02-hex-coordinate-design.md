# P3-02 Hex Coordinate Design

**Date:** 2026-08-20

## Design

The engine uses an immutable `HexCoord` value with signed 32-bit `x` and `y` components matching
Unciv 4.21.10's native coordinate basis: increasing `x` moves toward 10 o'clock and increasing `y`
moves toward 2 o'clock. Matching the upstream basis avoids conversion at save, oracle, and rules
boundaries. Offset-row and world-coordinate conversions remain separate Phase 6 concerns.

`HexCoord` exposes construction, component access, the origin constant, clockwise neighbor
enumeration beginning at 12 o'clock, and unwrapped hex distance. Neighbor order is deterministic:
`(1, 1)`, `(0, 1)`, `(-1, 0)`, `(-1, -1)`, `(0, -1)`, `(1, 0)` relative to the origin.
Neighbor construction rejects an unrepresentable result consistently in debug and release builds.
Distance follows Unciv's two-axis formula and uses widened intermediate arithmetic so subtraction,
absolute values, and addition are defined across the complete `i32` coordinate range.

The type derives equality, hashing, ordering, and Serde traits for later use as deterministic map
keys and serialized state. Arithmetic operators, rings, spirals, wrapping, offset conversion, and
screen-space conversion are deliberately deferred until their planned map tasks require them.

## Alternatives Considered

- Standard axial `q`/`r` names have familiar terminology but obscure the direct relationship to
  Unciv's serialized `x`/`y` fields.
- Three-component cube coordinates make the distance formula visually symmetric but store a
  redundant component and require conversion at every compatibility boundary.

## Verification

Focused tests cover the exact six neighbor coordinates and order, zero/aligned/opposed-axis
distances, symmetry, translation invariance, neighbor overflow, and full-range distance arithmetic.
Workspace formatting, Clippy, tests, Rustdoc, and the wasm32 API check must pass.
