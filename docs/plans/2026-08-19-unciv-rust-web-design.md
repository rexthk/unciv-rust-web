# Unciv Rust/Web Reimplementation Design

**Date:** 2026-08-19  
**Compatibility baseline:** Unciv 4.21.10  
**Product target:** Full feature parity, including mods, map editor, audio, save compatibility, and multiplayer  
**Clients:** Desktop Chrome, Android Chrome, and iPhone Chrome/WebKit  
**Frontend:** React + TypeScript + PixiJS  
**Game engine:** Rust compiled to WebAssembly  
**Backend:** Rust + Axum

## Goals

Build a browser-first reimplementation of Unciv 4.21.10 whose gameplay, rulesets, mods, saves, and multiplayer protocol are compatible with the pinned Kotlin release. The finished product must be installable as a PWA, playable offline in single-player, usable with mouse, keyboard, and touch, and deployable with a Rust multiplayer backend.

Compatibility is defined by observable behavior and serialized formats. The Rust version does not need to reproduce internal Kotlin class structure, but the same fixed seed and command sequence should produce equivalent normalized game state unless a documented compatibility exception exists.

## Non-goals

- Tracking every future Unciv release automatically.
- Supporting browsers other than the named Chrome/WebKit targets during the initial program.
- Reusing the LibGDX Scene2D interface.
- Making multiplayer authoritative in the first release.
- Requiring WebGPU; WebGL2 is the baseline.
- Recreating copyrighted Firaxis assets.

## System Architecture

The repository is a Rust workspace plus a TypeScript web application:

- `crates/engine`: deterministic game state, commands, events, and turn processing.
- `crates/rules`: Unciv JSON ruleset/mod parsing, validation, and compiled rule indexes.
- `crates/ai`: pathfinding and civilization, city, and unit automation.
- `crates/compat`: Unciv 4.21.10 save, map, mod, and multiplayer wire compatibility.
- `crates/wasm-api`: narrow `wasm-bindgen` boundary for commands and presentation views.
- `crates/server`: Axum HTTP/WebSocket multiplayer service.
- `crates/test-oracle`: normalized-state comparison and differential-test support.
- `crates/tools`: fixture generation, asset processing, save conversion, and diagnostics.
- `web`: React, TypeScript, PixiJS, IndexedDB, workers, PWA, and browser tests.
- `fixtures/unciv-4.21.10`: pinned rulesets, saves, maps, mods, and expected normalized states.

The engine must not depend on browser APIs, React, PixiJS, Axum, SQL, or filesystem implementations. Platform-specific behavior enters through small ports for clocks, storage, networking, entropy, and diagnostics.

## Command and Presentation Model

React never mutates game state. User input becomes a typed command such as `MoveUnit`, `FoundCity`, `ChooseTechnology`, or `EndTurn`. Each command carries stable entity identifiers and an expected state revision. Rust validates all preconditions before mutation, applies the command atomically, emits domain events, increments the revision, and returns compact presentation updates.

The JavaScript boundary exposes screen-specific immutable views instead of serializing the complete game after each action. Examples include `WorldViewportView`, `SelectedUnitView`, `CityScreenView`, and `DiplomacyView`. Large map data uses typed arrays and stable numeric IDs.

Map generation and AI execute in Web Workers containing separate Wasm instances. A worker receives a versioned snapshot and returns generated data or proposed commands. The authoritative engine validates worker results before applying them.

## Persistence and Compatibility

IndexedDB stores settings, saves, installed mods, cached indexes, and recovery checkpoints. Desktop and Android Chrome may use the File System Access API. iPhone uses standard upload/download controls.

Every stored object has a versioned envelope containing format version, Unciv compatibility version, compression type, payload length, and content hash. Save writes are transactional: write a new revision, read and validate it, then update the active pointer. The prior valid autosave is retained.

Import follows four stages: parse compatibility representation, apply Unciv migrations, validate all references and invariants, then construct live engine state. Export performs the reverse and is checked against pinned compatibility fixtures.

## Rules and Mods

Unciv rulesets remain data-driven. JSON files are parsed into compatibility DTOs, validated, then compiled into indexed runtime structures. Unique effects are implemented through a registry keyed by the pinned Unciv unique type. Each supported unique has parsing, validation, evaluation, fixture, and parity tests.

Mods are installed into isolated namespaces. ZIP input is bounded, path-sanitized, checksummed, and validated before activation. Invalid mods are quarantined without changing the active ruleset. Mod order and dependency resolution are deterministic.

## Rendering and User Interface

React implements navigation, dialogs, forms, lists, Civilopedia, city management, diplomacy, technology, policies, religion, multiplayer, mod management, saves, settings, and accessibility semantics. PixiJS owns the world-map canvas, tile layers, units, effects, selection, minimap, camera, and hit testing.

WebGL2 is the required renderer. Texture atlases and screen-specific assets load lazily. The interface must support responsive phone layouts, touch gestures, safe areas, orientation changes, virtual-keyboard resizing, mouse, keyboard shortcuts, and reduced-motion preferences.

## Multiplayer Backend

The Axum server preserves Unciv 4.21.10 asynchronous whole-state exchange. PostgreSQL stores accounts, friendships, lobbies, game metadata, chat metadata, revisions, and audit information. S3-compatible object storage holds compressed game states and mod archives.

Writes use revision hashes and idempotency keys. Conflicts return the current server revision instead of overwriting it. WebSockets deliver notifications and chat; HTTP remains the durable state-transfer path. Reconnect, polling fallback, and resumable uploads account for mobile suspension.

## Resilience and Diagnostics

Commands are atomic and failures are typed. Expected failures never partially mutate state. Unexpected engine failures produce a privacy-filtered diagnostic bundle containing command type, seed, state revision, enabled mods, compatibility version, and stack trace.

The PWA service worker uses versioned caches. A new release becomes active only after the application shell and required assets pass integrity checks. A failed update retains the previous working cache.

Rendering failures are isolated from engine state. React error boundaries can recreate screens, and PixiJS can rebuild its renderer and texture cache. Worker crashes receive one controlled restart before falling back to main-thread execution.

## Testing Strategy

1. Rust unit and property tests cover calculations, commands, rules, invariants, serialization, map operations, combat, cities, diplomacy, AI, and victory.
2. Pinned compatibility fixtures cover representative Unciv 4.21.10 saves, maps, rulesets, mods, and wire payloads.
3. Differential tests execute fixed seeds and scripted commands in Kotlin and Rust, comparing normalized state after every operation.
4. Vitest and Playwright cover the Wasm boundary, IndexedDB, workers, PWA updates, offline behavior, import/export, React screens, PixiJS, and touch input.
5. Containerized end-to-end tests launch Axum, PostgreSQL, object storage, and two browser clients to exercise full multiplayer flows.
6. Performance gates cover startup size and latency, world rendering, memory, autosave latency, AI turns, map generation, and synchronization.

## Delivery Principle

The project advances through vertical slices. A phase is complete only when its code, compatibility fixtures, browser behavior, failure paths, and performance budget pass. UI appearance alone is never considered feature completion.
