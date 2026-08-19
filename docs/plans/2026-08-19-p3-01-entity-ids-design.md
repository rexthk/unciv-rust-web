# P3-01 Entity ID Design

**Date:** 2026-08-19

## Design

The engine uses two identifier representations according to their domain role:

- `GameId` wraps `uuid::Uuid`, preserving the stable UUID identity used by Unciv saves,
  multiplayer, deep links, and worker snapshots. It serializes as the canonical UUID string and
  does not generate randomness inside the engine.
- `CivilizationId`, `CityId`, and `UnitId` remain distinct transparent `u32` newtypes. They are
  compact runtime references suitable for JavaScript numbers and `Uint32Array` presentation data.

Legacy sentinel and wire representations stay outside the engine model. In particular, Unciv's
unit ID `-1` means “unassigned”; compatibility code will translate that state to absence rather
than admitting negative engine IDs. Multiplayer account identity (`PlayerId`) belongs to the
server/compatibility layers, and map tile identity will use the Phase 3 hex-coordinate type.

## Verification

Property tests cover arbitrary numeric IDs and arbitrary UUID bytes. Boundary tests cover the full
numeric range. JSON tests verify numeric entity IDs remain transparent and game IDs use canonical
UUID strings. Workspace Clippy, tests, Rustdoc, and the wasm32 build must pass.
