# Plan: Unciv 4.21.10 Rust/Web Reimplementation

**Goal**: Deliver full Unciv 4.21.10-compatible gameplay in desktop Chrome, Android Chrome, and iPhone Chrome/WebKit, using a Rust/Wasm engine, React/PixiJS client, and Rust/Axum multiplayer server.

**Architecture**: A deterministic platform-neutral Rust engine receives typed commands and produces events and compact presentation views. React owns application UI, PixiJS owns the world canvas, IndexedDB owns local persistence, workers run AI/map generation, and Axum/PostgreSQL/S3-compatible storage implement multiplayer.

**Tech Stack**: Rust stable, Cargo workspace, serde, wasm-bindgen, TypeScript, React, Vite, PixiJS, Vitest, Playwright, Axum, SQLx, PostgreSQL, MinIO/S3, Docker Compose, GitHub Actions.

**Design**: `docs/plans/2026-08-19-unciv-rust-web-design.md`

## Planning Rules

- Each normal task is budgeted at 20-40 minutes, centered on 30 minutes.
- Every task starts with a failing test, fixture, lint check, or explicit observable assertion.
- If a task cannot pass within 40 minutes, stop and split the remaining work into new tasks.
- One task should normally touch one component and no more than three production files.
- Every completed task ends with focused tests and one Git commit.
- Generated task families create one task per source item; they are not single bulk tasks.
- Phase gates are integration checkpoints and may occupy several separately listed 30-minute tasks.

## Standard Commands

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
pnpm --dir web lint
pnpm --dir web test
pnpm --dir web test:e2e
docker compose -f infra/docker-compose.test.yml up -d
```

For each task, use a focused command where possible:

```bash
cargo test -p <crate> <test_name>
pnpm --dir web vitest run <test-file>
pnpm --dir web playwright test <spec-file>
```

Commit format:

```bash
git add <focused-files>
git commit -m "<area>: <task outcome>"
```

## Execution Dependencies

| Group | Phases | Can Parallelize |
|---|---|---|
| A | 0-2: repository, oracle, contracts | Limited |
| B | 3-5: rules, uniques, map | Yes after core contracts |
| C | 6-14: gameplay systems and AI | By subsystem after shared state stabilizes |
| D | 15-17: compatibility, Wasm, web shell | Yes after serialization contracts |
| E | 18-21: rendering, screens, mods, media | By screen/feature with shared UI primitives frozen |
| F | 22-23: server and multiplayer client | Yes after protocol contracts |
| G | 24-26: mobile, hardening, release | By test matrix; final gate is sequential |

---

## Phase 0: Repository and Toolchain Bootstrap

**Status**: Complete (2026-08-19)

**Exit gate**: Empty engine and web shell build, test, lint, and run in CI on every commit.

| ID | Status | ~30-minute task | Files | Verification |
|---|---|---|---|---|
| P0-01 | Done | Add root README with scope and compatibility baseline | `README.md` | Link checker/manual review |
| P0-02 | Done | Add Rust toolchain pin and formatter/lint settings | `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml` | `cargo fmt --check` |
| P0-03 | Done | Create Cargo workspace and placeholder crates | `Cargo.toml`, `crates/*/Cargo.toml` | `cargo metadata --no-deps` |
| P0-04 | Done | Add minimal library entry points for all crates | `crates/*/src/lib.rs` | `cargo check --workspace` |
| P0-05 | Done | Scaffold Vite React TypeScript application | `web/package.json`, `web/vite.config.ts`, `web/src/main.tsx` | `pnpm --dir web build` |
| P0-06 | Done | Add Vitest smoke test | `web/src/App.test.tsx` | Observe fail, implement shell, observe pass |
| P0-07 | Done | Add Playwright Chrome smoke configuration | `web/playwright.config.ts`, `web/e2e/smoke.spec.ts` | `pnpm --dir web test:e2e` |
| P0-08 | Done | Add WebKit Playwright project for iPhone compatibility | Same config, `web/e2e/smoke.spec.ts` | WebKit test passes |
| P0-09 | Done | Add workspace Makefile/task runner commands | `justfile` | `just check` |
| P0-10 | Done | Add `.gitignore`, editor config, and license notices | `.gitignore`, `.editorconfig`, `LICENSES/` | Clean `git status` after builds |
| P0-11 | Done | Add GitHub Actions Rust job | `.github/workflows/ci.yml` | Local equivalent passes |
| P0-12 | Done | Add GitHub Actions web job | `.github/workflows/ci.yml` | Web build/test command passes |
| P0-13 | Done | Add dependency-license audit | `deny.toml`, CI | `cargo deny check` |
| P0-14 | Done | Add JavaScript dependency audit policy | `web/package.json`, CI | `pnpm audit` policy passes |
| P0-15 | Done | Record reproducible developer setup | `docs/development.md` | Fresh-directory setup walkthrough |

## Phase 1: Pin Upstream and Build the Compatibility Inventory

**Status**: Complete (2026-08-19)

**Exit gate**: Unciv 4.21.10 source, data, formats, screens, effects, and scenarios are represented by machine-readable manifests.

| ID | Status | Task | Files | Verification |
|---|---|---|---|---|
| P1-01 | Done | Record upstream repository URL, commit, version, and license | `upstream/manifest.toml` | Manifest unit test |
| P1-02 | Done | Add script that verifies the pinned source checksum | `scripts/verify-upstream.sh` | Corrupt SHA fixture causes failure |
| P1-03 | Done | Copy permitted built-in ruleset fixtures with attribution | `fixtures/unciv-4.21.10/rulesets/` | Fixture checksum test |
| P1-04 | Done | Inventory all JSON asset categories | `fixtures/unciv-4.21.10/inventory/ruleset-files.json` | Inventory count snapshot |
| P1-05 | Done | Inventory every `UniqueType` entry | `.../inventory/unique-types.json` | Expected count/checksum test |
| P1-06 | Done | Inventory save-visible Kotlin model fields | `.../inventory/save-fields.json` | No duplicate type/field keys |
| P1-07 | Done | Inventory multiplayer endpoints and payloads | `.../inventory/multiplayer-api.json` | JSON schema validation |
| P1-08 | Done | Inventory user-visible screens and flows | `.../inventory/screens.json` | Every screen has category and parity owner |
| P1-09 | Done | Inventory keyboard, mouse, and touch actions | `.../inventory/input-actions.json` | Stable action IDs test |
| P1-10 | Done | Inventory bundled translations and sound assets | `.../inventory/media.json` | File existence/checksum test |
| P1-11 | Done | Inventory map editor operations | `.../inventory/map-editor.json` | Operation IDs unique |
| P1-12 | Done | Define parity status schema | `schemas/parity-status.schema.json` | Schema validates sample |
| P1-13 | Done | Generate initial task ledger from inventories | `scripts/generate-inventories.mjs`, `docs/task-ledger/` | Deterministic output test |
| P1-14 | Done | Add task-ledger completeness CI check | `scripts/check-task-ledger.mjs`, CI | Missing inventory item fails CI |
| P1-15 | Done | Document compatibility exception process | `docs/compatibility-policy.md` | Template lint test |

## Phase 2: Kotlin Oracle and Differential Harness

**Status**: Complete (2026-08-19)

**Exit gate**: A command-line Kotlin oracle and Rust comparator can execute a tiny deterministic scenario and report exact normalized differences.

| ID | Status | Task | Files | Verification |
|---|---|---|---|---|
| P2-01 | Done | Define language-neutral scenario JSON schema | `schemas/scenario.schema.json` | Valid/invalid samples |
| P2-02 | Done | Define normalized-state JSON schema | `schemas/normalized-state.schema.json` | Schema tests |
| P2-03 | Done | Add Kotlin oracle wrapper project | `oracle-kotlin/` | Wrapper emits versioned normalized JSON |
| P2-04 | Done | Load pinned rulesets in oracle | `oracle-kotlin/src/main/kotlin/com/unciv/oracle/OracleMain.kt` | Ruleset count in normalized output |
| P2-05 | Done | Execute `new_game` scenario instruction | Oracle command runner | Golden fixture |
| P2-06 | Done | Execute one unit-selection/no-op instruction | Oracle runner | State remains deterministic |
| P2-07 | Done | Normalize map coordinates and tile ordering | Oracle normalizer | Repeated output byte-identical |
| P2-08 | Done | Normalize civilization and unit IDs | Oracle normalizer | Stable canonical unit descriptors |
| P2-09 | Done | Create Rust scenario reader | `crates/test-oracle/src/scenario.rs` | Schema fixture test |
| P2-10 | Done | Create normalized diff engine | `crates/test-oracle/src/lib.rs` | Intentional mismatch test |
| P2-11 | Done | Add process runner for Kotlin oracle | `crates/test-oracle/src/process.rs` | Captures success/error/timeout |
| P2-12 | Done | Add first cross-language smoke comparison | `crates/test-oracle/tests/smoke.rs` | Comparator passes |
| P2-13 | Done | Add minimized mismatch artifact writer | `crates/test-oracle/src/report.rs` | Artifact test |
| P2-14 | Done | Add oracle job to CI | `.github/workflows/ci.yml` | Pinned smoke comparison passes |

## Phase 3: Core Types, IDs, Commands, Events, and Revisions

**Exit gate**: Platform-neutral engine state can accept a no-op command atomically and serialize deterministically.

| ID | Task | Files | Verification |
|---|---|---|---|
| P3-01 | Add strongly typed entity IDs | `crates/engine/src/id.rs` | Round-trip/property tests |
| P3-02 | Add hex coordinate type | `crates/engine/src/hex.rs` | Neighbor/distance tests |
| P3-03 | Add deterministic ordered map/set aliases | `crates/engine/src/collections.rs` | Stable serialization test |
| P3-04 | Add seeded RNG stream identifiers | `crates/engine/src/random.rs` | Golden sequence test |
| P3-05 | Add game revision type | `crates/engine/src/revision.rs` | Overflow/error test |
| P3-06 | Define command envelope | `crates/engine/src/command.rs` | serde snapshot |
| P3-07 | Define domain-event envelope | `crates/engine/src/event.rs` | serde snapshot |
| P3-08 | Add typed command error enum | `crates/engine/src/error.rs` | Stable error-code test |
| P3-09 | Add minimal game-state root | `crates/engine/src/state.rs` | Deterministic serialization |
| P3-10 | Implement atomic no-op command | `crates/engine/src/apply.rs` | Stale/current revision tests |
| P3-11 | Add invariant-check trait | `crates/engine/src/invariant.rs` | Broken fixture detected |
| P3-12 | Add state-content hashing | `crates/engine/src/hash.rs` | Golden hash test |
| P3-13 | Add command journal | `crates/engine/src/journal.rs` | Replay equals original state |
| P3-14 | Add engine prelude and API documentation | `crates/engine/src/lib.rs` | Rustdoc builds |

## Phase 4: Ruleset Parser, Validation, and Compiled Indexes

**Exit gate**: Every built-in Unciv 4.21.10 ruleset file parses, validates, and compiles without ignored fields.

Initial tasks:

| ID | Task | Files | Verification |
|---|---|---|---|
| P4-01 | Add JSON-with-comments preprocessing | `crates/rules/src/json.rs` | Comment/trailing-comma fixtures |
| P4-02 | Add source-location-aware parse errors | `crates/rules/src/error.rs` | Error snapshot |
| P4-03 | Define ruleset root DTO | `crates/compat/src/ruleset/mod.rs` | serde fixture |
| P4-04 | Add unknown-field accounting | `crates/rules/src/audit.rs` | Unknown field fails strict mode |
| P4-05 | Add cross-reference validator framework | `crates/rules/src/validate.rs` | Missing reference test |
| P4-06 | Add deterministic compiled string IDs | `crates/rules/src/string_id.rs` | Stable-ID snapshot |
| P4-07 | Add compiled name indexes | `crates/rules/src/index.rs` | Duplicate-name test |
| P4-08 | Add base-ruleset merge pipeline | `crates/rules/src/merge.rs` | Merge-order fixture |
| P4-09 | Add mod override pipeline | Same module | Replacement/deletion fixtures |
| P4-10 | Add all-built-in-rulesets integration test | `crates/rules/tests/builtins.rs` | Zero ignored fields |

Generated family `RULE-*`: create one 30-minute task per ruleset category in `ruleset-files.json`. Each task adds its compatibility DTO, parser fixture, validation rules, compiled representation, and focused test. Categories include units, buildings, technologies, policies, terrains, resources, improvements, nations, eras, speeds, difficulties, promotions, beliefs, religions, quests, events, victories, specialists, personalities, ruins, and global uniques.

## Phase 5: Unique Effect Language

**Exit gate**: Every pinned `UniqueType` has an explicit parity status; all implemented types pass parsing and behavioral differential fixtures.

| ID | Task | Files | Verification |
|---|---|---|---|
| P5-01 | Define unique token/placeholder AST | `crates/rules/src/unique/ast.rs` | Parser fixtures |
| P5-02 | Parse bracket placeholders and conditionals | `.../parser.rs` | Golden examples |
| P5-03 | Add unique target enum | `.../target.rs` | Compatibility snapshot |
| P5-04 | Add condition evaluation context | `.../condition.rs` | Truth-table tests |
| P5-05 | Add effect registry and stable IDs | `.../registry.rs` | Duplicate/missing tests |
| P5-06 | Add unsupported-effect diagnostic | `.../error.rs` | Error snapshot |
| P5-07 | Generate unique parity matrix | `scripts/generate-unique-matrix.ts` | 100% inventory coverage |

Generated family `UNIQUE-0001..N`: one task per `unique-types.json` entry. Each task must:

1. Add one failing parse fixture.
2. Add one failing validation or evaluation fixture.
3. Implement exactly one registry handler or documented alias/deprecation.
4. Add at least one Kotlin-oracle differential scenario.
5. Mark only that unique entry implemented in the parity matrix.
6. Run `cargo test -p rules unique::<stable_id>` and its oracle scenario.

Do not combine unrelated unique effects. The current inventory is expected to produce roughly 638 tasks before aliases and deprecated entries are classified.

## Phase 6: Hex Map Model and Map Generation

**Exit gate**: Pinned map files load/export, and fixed-seed generated maps match normalized structural invariants and approved oracle fixtures.

| ID | Task | Files | Verification |
|---|---|---|---|
| P6-01 | Implement axial/offset conversion | `crates/engine/src/map/coord.rs` | Table tests |
| P6-02 | Implement hex rings and spirals | `.../geometry.rs` | Property tests |
| P6-03 | Add tile storage and lookup | `.../tile_map.rs` | Bounds tests |
| P6-04 | Add terrain/features/resources model | `.../tile.rs` | serde fixture |
| P6-05 | Add map wrapping | `.../wrap.rs` | Edge-neighbor tests |
| P6-06 | Add map-file compatibility DTO | `crates/compat/src/map.rs` | Import/export fixture |
| P6-07 | Implement continent detection | `crates/engine/src/map/regions.rs` | Golden map fixture |
| P6-08 | Implement water-body detection | Same module | Golden map fixture |
| P6-09 | Implement elevation/land generation stage | `.../generator/land.rs` | Seed snapshot |
| P6-10 | Implement temperature/moisture stage | `.../generator/climate.rs` | Seed snapshot |
| P6-11 | Implement terrain assignment | `.../generator/terrain.rs` | Distribution test |
| P6-12 | Implement natural wonders | `.../generator/wonders.rs` | Constraint tests |
| P6-13 | Implement resources | `.../generator/resources.rs` | Diversity tests |
| P6-14 | Implement starting locations | `.../generator/starts.rs` | Fairness invariants |
| P6-15 | Add complete map-generation oracle scenarios | `fixtures/.../scenarios/map/` | Differential reports approved |

Generated family `MAPGEN-*`: one task per pinned generator option, map shape, resource setting, size, and edge case found in the inventory.

## Phase 7: Game Lifecycle and Turn Processing

**Exit gate**: A new game can be created, advanced through turns, autosaved, replayed, and compared with the Kotlin oracle.

| ID | Task | Files | Verification |
|---|---|---|---|
| P7-01 | Define game parameters DTO/runtime model | `crates/engine/src/game/parameters.rs` | Compatibility fixture |
| P7-02 | Implement player/civilization creation | `.../start.rs` | Fixed-seed fixture |
| P7-03 | Implement starting units/cities assignment | Same module | Oracle comparison |
| P7-04 | Add turn phase enum/state machine | `.../turn.rs` | Transition tests |
| P7-05 | Add `EndTurn` command validation | `.../commands/end_turn.rs` | Invalid-state tests |
| P7-06 | Process civilization start-of-turn effects | `.../turn/start.rs` | Event snapshot |
| P7-07 | Process civilization end-of-turn effects | `.../turn/end.rs` | Event snapshot |
| P7-08 | Implement year/turn conversion | `.../calendar.rs` | Speed/era tables |
| P7-09 | Implement game cloning/snapshot | `.../snapshot.rs` | Hash equality test |
| P7-10 | Implement command replay | `.../replay.rs` | Multi-turn equality |
| P7-11 | Add 25-turn oracle smoke scenario | Fixtures | State diff passes |
| P7-12 | Add 250-turn headless stability scenario | Integration test | No panic/invariant failure |

## Phase 8: Civilization Economy and Global State

**Exit gate**: Civilization stats, resources, happiness, golden ages, notifications, and visibility match oracle scenarios.

Create one 30-minute task for each item below, with a focused module and oracle fixture:

- Civilization identity and nation traits.
- Stat reserve accounting: gold, science, culture, faith.
- Per-turn stat calculation pipeline.
- Strategic and luxury resource accounting.
- Happiness and unhappiness sources.
- Golden-age accumulation and activation.
- Era progression.
- Great-person point pools.
- Known-civilization/contact state.
- Fog of war and explored tiles.
- Natural-wonder discovery.
- Notifications and action links.
- Defeat and resurrection state.
- Spectator and view-as behavior.

Files live under `crates/engine/src/civilization/`; focused tests live beside modules and differential scenarios under `fixtures/unciv-4.21.10/scenarios/civilization/`.

Generated family `CIV-*`: one task per civilization manager and serialized field in the compatibility inventory.

## Phase 9: Cities and Construction

**Exit gate**: City founding through conquest, production, growth, borders, specialists, and buildings match pinned saves and scripted scenarios.

Task sequence, each one task unless marked generated:

1. City identity, location, owner, and name model.
2. Found-city command and placement validation.
3. Population and food storage.
4. Citizen tile assignment.
5. Specialist assignment.
6. Base tile yields.
7. Building/specialist yield modifiers.
8. Production queue model.
9. Add/remove/reorder queue commands.
10. Unit construction completion.
11. Building construction completion.
12. Wonder uniqueness enforcement.
13. Purchase with gold.
14. Purchase with faith.
15. Border growth and tile acquisition.
16. City connections.
17. Resistance, puppet, annex, and raze states.
18. City capture transfer.
19. Capital relocation.
20. We-love-the-king-day state.
21. City-state-specific behavior.
22. City serialization and normalized view.

Primary path: `crates/engine/src/city/`. Generated families `CITY-BUILDING-*` and `CITY-UNIQUE-*` create one behavioral parity task per construction-specific rule interaction.

## Phase 10: Units, Movement, and Pathfinding

**Exit gate**: Every unit action and movement rule has parity coverage, and representative paths match the oracle.

Initial tasks:

- Unit model, owner, health, movement, promotions, and transport state.
- Unit placement and occupancy invariants.
- Neighbor movement cost.
- Roads, rails, rivers, embarkation, and domain transitions.
- Dijkstra/A* path API with deterministic tie-breaking.
- Zone-of-control handling.
- Civilian/military stacking.
- Move command and partial path execution.
- Swap units.
- Fortify, sleep, explore, automate, and wake.
- Found city, build improvement, repair, pillage, paradrop, air sweep, setup, and disband.
- Upgrade and gift unit.
- Great-person actions.
- Unit capture and civilian recapture.
- Visibility and line-of-sight contribution.
- Unit serialization and presentation views.

Primary paths: `crates/engine/src/unit/` and `crates/engine/src/pathfinding/`.

Generated family `UNIT-ACTION-*`: one task per action in the pinned unit-action inventory. Generated family `PROMOTION-*`: one task per promotion behavior requiring code rather than generic unique evaluation.

## Phase 11: Combat

**Exit gate**: Melee, ranged, city, naval, air, nuclear, interception, capture, and post-combat behavior pass differential fixtures.

| Task family | One-task unit |
|---|---|
| `COMBAT-CALC-*` | One damage modifier or strength source |
| `COMBAT-ACTION-*` | One attack mode or command path |
| `COMBAT-AFTER-*` | One post-combat effect |
| `COMBAT-EDGE-*` | One edge case from existing Kotlin tests/issues |

Foundation tasks implement combatants, attack validation, deterministic damage preview, melee resolution, ranged resolution, city combat, experience/promotions, capture/destruction, withdrawal, interception, air missions, nuclear effects, civilian capture, and combat event logs under `crates/engine/src/combat/`.

Each task runs a focused Rust test and a small oracle scenario. Add property tests ensuring health bounds, ownership invariants, and preview/resolution consistency.

## Phase 12: Technology, Policies, Religion, and Great People

**Exit gate**: All progression trees, choices, costs, unlocks, beliefs, religion spread, and great-person mechanics match pinned scenarios.

Use one task per bullet and generated tasks per rule interaction:

- Technology graph and prerequisite validation.
- Research selection, overflow, free technologies, and completion.
- Era unlocks and obsolescence.
- Policy branches, adoption costs, completion, free policies, and switching rules.
- Faith accumulation and pantheon founding.
- Religion founding, naming, and belief selection.
- Passive pressure and active spread.
- Missionary, inquisitor, and prophet actions.
- Religious city-state and diplomacy effects.
- Great-person generation thresholds.
- Scientist, engineer, merchant, artist, general, and admiral actions.

Paths: `crates/engine/src/progression/` and `crates/engine/src/religion/`. Generated families `TECH-*`, `POLICY-*`, `BELIEF-*`, and `GREAT-PERSON-*` are created from the inventory.

## Phase 13: Diplomacy, Trade, City-States, Quests, and Victory

**Exit gate**: Major-civilization diplomacy, city-state relations, deals, quests, war/peace, and every victory path complete in headless scenarios.

Thirty-minute tasks cover:

- Relationship levels and modifiers.
- First contact and introductions.
- Denouncement, friendship, defensive pact, research agreement, and open borders.
- Declare-war validation and effects.
- Peace negotiation and treaty duration.
- Trade offer model and valuation hooks.
- Gold, resources, cities, agreements, and treaty items.
- Deal expiry and cancellation.
- City-state influence, resting points, allies, friends, and gifts.
- City-state bullying and protection.
- Quest generation, progress, completion, and expiry.
- Barbarian and encampment diplomacy effects.
- Scientific, cultural, domination, diplomatic, and time victories.
- Defeat and one-more-turn behavior.
- Replay/history events required by the victory screen.

Paths: `crates/engine/src/diplomacy/`, `trade/`, `city_state/`, `quest/`, and `victory/`. Generate one task per serialized manager field and unique rule interaction.

## Phase 14: AI and Automation

**Exit gate**: AI can complete multiple full games without invariant failures, severe stalls, or unbounded turn time, and key decisions match oracle fixtures where deterministic comparison is meaningful.

Foundation tasks:

- AI evaluation context and deterministic tie-breaking.
- Threat map and reachable-target cache.
- City production selection.
- Citizen assignment.
- Worker improvement selection.
- Settler city-site selection.
- Civilian safety movement.
- Military target selection.
- Tactical movement and attack sequencing.
- Ranged positioning.
- Naval and embarked behavior.
- Air-unit behavior.
- Technology selection.
- Policy selection.
- Religion choices.
- Diplomacy decisions.
- Trade valuation and offer generation.
- City-state interactions.
- Victory-strategy selection.
- Automation cancellation and recovery.

Generated family `AI-*`: one task per Kotlin automation function/class identified in the inventory. Each task adds either an oracle scenario or a behavior invariant. Performance tasks add budgets for small, standard, and large maps.

## Phase 15: Save, Map, Settings, and Migration Compatibility

**Exit gate**: The Rust client imports and exports the compatibility corpus without data loss and can resume representative games at every era.

| ID | Task | Files | Verification |
|---|---|---|---|
| P15-01 | Define versioned storage envelope | `crates/compat/src/envelope.rs` | Corruption tests |
| P15-02 | Implement gzip/base64 codecs | `crates/compat/src/codec.rs` | Kotlin fixture round trip |
| P15-03 | Implement game save reader | `crates/compat/src/save/read.rs` | Corpus imports |
| P15-04 | Implement game save writer | `.../write.rs` | Oracle accepts export |
| P15-05 | Implement preview extraction | `.../preview.rs` | Preview fixture |
| P15-06 | Implement settings compatibility subset | `crates/compat/src/settings.rs` | Fixture round trip |
| P15-07 | Implement map import/export | `crates/compat/src/map.rs` | Corpus round trip |
| P15-08 | Add migration registry | `crates/compat/src/migration.rs` | Ordered migration test |
| P15-09 | Add transactional-save abstraction | `crates/engine/src/storage.rs` | Failure injection tests |
| P15-10 | Add save-corpus report | `crates/tools/src/bin/check_saves.rs` | Zero unexplained fields |

Generated family `MIGRATION-*`: one task per migration required to load saves accepted by Unciv 4.21.10. Generated family `SAVE-FIELD-*`: one task per unresolved serialized field in the inventory.

## Phase 16: Wasm Boundary and Workers

**Exit gate**: The web app creates a game, sends commands, receives views, saves state, and delegates one AI turn through a worker without leaking memory.

Tasks:

- Configure wasm-bindgen build and TypeScript declarations.
- Add Wasm initialization and version handshake.
- Add structured command input API.
- Add structured error output API.
- Add revision and state-hash API.
- Add world viewport typed-array view.
- Add selected-unit view.
- Add city-screen view.
- Add generic menu/list view DTO patterns.
- Add save import/export byte APIs.
- Add worker message schema.
- Start/stop/restart worker lifecycle.
- Transfer snapshots to workers.
- Run map generation in worker.
- Run one AI civilization turn in worker.
- Add cancellation and progress events.
- Add browser memory-leak harness.

Files: `crates/wasm-api/`, `web/src/engine/`, and `web/src/workers/`. Every boundary type has Rust serialization tests and TypeScript contract tests.

## Phase 17: Web Shell, IndexedDB, and PWA

**Exit gate**: Installable offline shell starts on all target browsers, stores settings and saves transactionally, and survives upgrade/reload tests.

Thirty-minute tasks:

- Application router and screen registry.
- Global error boundary.
- Engine provider and loading state.
- IndexedDB schema v1.
- Settings repository.
- Save metadata repository.
- Transactional save blobs and active pointers.
- Previous-autosave recovery.
- Storage usage/cleanup screen.
- File upload adapter.
- File download adapter.
- File System Access enhancement for supported Chrome.
- Service worker registration.
- Versioned application-shell cache.
- Ruleset/asset cache.
- Safe update activation.
- Offline-start test.
- Interrupted-update recovery test.
- PWA manifest and icons.

Paths: `web/src/app/`, `web/src/storage/`, `web/src/files/`, `web/src/pwa/`, and `web/public/`.

## Phase 18: PixiJS World Renderer and Input

**Exit gate**: Large maps render within budgets and support equivalent mouse, keyboard, and touch navigation and unit interaction.

One task each:

- Pixi application lifecycle and canvas sizing.
- Camera transform and coordinate conversion.
- Terrain base layer.
- Terrain feature layer.
- Rivers and borders.
- Roads and improvements.
- Resources and yields.
- Cities and labels.
- Units and health/movement indicators.
- Fog of war.
- Selection/highlight overlays.
- Movement/attack path preview.
- Combat effects.
- Minimap.
- Tile hit testing.
- Mouse pan/zoom/select.
- Keyboard pan and shortcuts.
- Touch pan, pinch zoom, tap, long press, and double tap.
- Safe-area and orientation handling.
- Texture atlas loader and eviction.
- Renderer context-loss recovery.
- Large-map frame-time benchmark.

Paths: `web/src/map/`. Every visual layer gets a deterministic screenshot fixture and hit-test unit tests.

## Phase 19: React Screen Parity

**Exit gate**: Every screen/flow inventory entry is implemented or has an approved compatibility exception, with responsive and accessibility tests.

First build shared primitives as individual tasks:

- Buttons, icon buttons, toggles, selectors, sliders, text inputs, tabs, tables, tooltips, dialogs, toasts, progress indicators, virtualized lists, split panes, collapsible sections, keyboard-focus management, touch target sizing, and responsive screen scaffolds.

Then generate family `SCREEN-*`: one task per `screens.json` entry. Each screen task must:

1. Add a failing component or Playwright test for one user flow.
2. Add the smallest React component/view adapter needed.
3. Wire typed engine commands without direct mutation.
4. Cover loading, empty, error, phone, and desktop states where applicable.
5. Add keyboard/touch semantics and accessible names.
6. Mark exactly one inventory entry complete.

Required screen groups include main menu, new game, nation/map/mod options, world HUD, unit panel, city screen, technology, policies, religion, diplomacy, trade, overviews, Civilopedia, victory/replay, saves, settings, multiplayer, mod manager, map editor, language picker, tutorials, crash recovery, and developer diagnostics.

## Phase 20: Mods and Map Editor

**Exit gate**: Compatible mods install/enable/disable safely, and map-editor files round-trip with representative editing workflows.

Tasks:

- ZIP bounded decompression.
- ZIP path traversal rejection.
- Mod manifest discovery.
- Mod checksum and namespace.
- Mod dependency and order resolution.
- Ruleset validation report UI.
- Install from upload.
- Install from URL/GitHub-compatible endpoint.
- Enable/disable and rollback.
- Missing-mod save recovery UI.
- Mod asset registration and cache eviction.
- Map editor state/undo stack.
- Terrain paint tool.
- Feature/resource/improvement tools.
- Unit/city placement tools.
- Starting-location tools.
- Selection/area tools.
- Map resize and transform operations.
- Import/export and scenario metadata.
- Desktop and phone editor workflows.

Generated families `MOD-FIXTURE-*` and `EDITOR-OP-*` create one task per representative mod and inventoried editor operation.

## Phase 21: Localization, Fonts, Audio, Tutorials, and Accessibility

**Exit gate**: All pinned translations load, Unicode text renders, music/sounds obey browser policies, tutorials work, and core flows pass accessibility audits.

Tasks:

- Properties-file translation parser.
- Placeholder/plural/markup compatibility.
- Translation fallback chain.
- Lazy language pack loading.
- Locale-aware sorting through JavaScript `Intl`.
- Font fallback registry.
- Dynamic glyph atlas or browser text strategy validation.
- Right-to-left layout audit and fixes.
- Web Audio context unlock flow.
- Sound-effect mixer and volume controls.
- Music playlist/mood controller.
- Background/suspension audio behavior.
- Mod audio loading.
- Tutorial trigger engine compatibility.
- Tutorial overlay and progress storage.
- Keyboard-only navigation audit.
- Screen-reader semantic audit for React UI.
- Contrast, reduced motion, and scalable text.

Generated family `LANG-*`: one task per bundled translation, consisting of parse validation, missing-key report, font smoke test, and one representative screen screenshot.

## Phase 22: Rust/Axum Multiplayer Server

**Exit gate**: Server protocol fixtures pass; users can create/join a lobby and exchange versioned game states and chat through containerized infrastructure.

| Task group | Thirty-minute units |
|---|---|
| Bootstrap | Axum app, config, tracing, health, graceful shutdown |
| Database | SQLx setup, migrations, test transaction helper, each table/repository separately |
| Auth | Password/token model, login, refresh, logout, authorization middleware |
| Social | Friends, requests, blocks, lookup, privacy checks |
| Lobby | Create, join, leave, update options, ready state, start game |
| Game state | Metadata, blob upload, blob download, revision conflict, idempotency |
| Chat | Persist message, history, WebSocket publish, reconnect cursor |
| Notifications | Turn events, WebSocket subscription, polling fallback |
| Object store | MinIO adapter, checksum, retention, orphan cleanup |
| Operations | Rate limits, request IDs, metrics, backups, restore drill |

Primary paths: `crates/server/src/`, `migrations/`, and `infra/`. Generated family `API-*` creates one task per endpoint in `multiplayer-api.json`, including request fixture, response fixture, authorization test, handler, and compatibility status update.

## Phase 23: Multiplayer Web Client

**Exit gate**: Two real browser clients complete lobby creation, game start, several turn exchanges, chat, suspension, conflict recovery, and reconnect.

Tasks:

- Server-profile configuration.
- Authentication storage and refresh.
- Friend list and requests.
- Lobby list/create/join/leave.
- Player slot and ready-state UI.
- Start-game synchronization.
- Upload/download compressed state.
- Revision-conflict presentation and recovery.
- Idempotent retry queue.
- WebSocket lifecycle.
- Polling fallback.
- Turn notification UI.
- Chat history/send/reconnect.
- Offline pending-action presentation.
- Mobile background/resume behavior.
- Two-client Playwright fixture.
- Host/guest full-flow test.
- Upgrade compatibility test against pinned payloads.

Paths: `web/src/multiplayer/` and `web/e2e/multiplayer/`.

## Phase 24: Mobile and WebKit Hardening

**Exit gate**: Required end-to-end flows pass on Android Chrome and iPhone Chrome/WebKit emulation plus physical-device smoke runs.

One task per item:

- Define supported viewport/device matrix.
- Add Android Playwright projects.
- Add iPhone/WebKit Playwright projects.
- Add safe-area layout fixtures.
- Add portrait/landscape transition tests.
- Add virtual-keyboard resize tests.
- Audit all touch targets.
- Audit scrolling/nested scrolling.
- Audit gesture conflicts with browser navigation.
- Test IndexedDB quotas and eviction messaging.
- Test iPhone upload/download save path.
- Test PWA install/update on Android.
- Document iPhone home-screen limitations.
- Test suspension during AI turn.
- Test suspension during save.
- Test suspension during multiplayer upload.
- Add low-memory renderer recovery.
- Add physical Android smoke checklist.
- Add physical iPhone smoke checklist.
- Record and triage device-specific defects as one task each.

## Phase 25: Security, Performance, and Reliability

**Exit gate**: Threat model controls, performance budgets, soak tests, backup restore, and failure-injection suites pass.

Tasks:

- Threat model browser, mod, save, and server surfaces.
- Add maximum save/map/mod sizes.
- Add decompression-ratio limits.
- Add parser fuzz targets.
- Add command-sequence fuzz target.
- Add map/pathfinding property suites.
- Add server authorization matrix tests.
- Add upload content-hash verification.
- Add CSP and security headers.
- Add dependency and secret scanning.
- Define startup/download budgets.
- Split/lazy-load language and sound assets.
- Profile Wasm size and remove major contributors.
- Profile world render and reduce allocations.
- Profile AI/map generation and add worker budgets.
- Profile IndexedDB save latency.
- Add 1,000-turn AI soak.
- Add repeated import/export soak.
- Add multiplayer reconnect storm test.
- Add database backup and restore test.
- Add object-store integrity scan.
- Add chaos tests for network, worker, storage, and renderer failures.

Every discovered hotspot or vulnerability becomes its own 30-minute remediation task rather than expanding the profiling task.

## Phase 26: Release Candidate and End-to-End Verification

**Exit gate**: The release candidate satisfies every compatibility inventory, test matrix, operational runbook, and full-game acceptance criterion.

Final tasks, executed sequentially:

1. Freeze Unciv 4.21.10 compatibility manifests and regenerate the task ledger.
2. Verify zero unexplained ruleset fields.
3. Verify every unique type is implemented, aliased, deprecated, or approved as an exception.
4. Verify every screen inventory entry.
5. Verify every multiplayer endpoint inventory entry.
6. Run all Rust unit/property/fuzz smoke suites.
7. Run the complete save/map/mod compatibility corpus.
8. Run Kotlin/Rust differential scenarios.
9. Complete a standard-map science victory in automated browser play.
10. Complete a domination victory.
11. Complete cultural, diplomatic, and time-victory scenarios.
12. Complete a modded-game scenario.
13. Complete a map-editor create/export/import/play scenario.
14. Complete two-player multiplayer from lobby through victory fixture.
15. Run desktop Chrome acceptance.
16. Run Android Chrome physical-device acceptance.
17. Run iPhone Chrome/WebKit physical-device acceptance.
18. Verify offline install, restart, upgrade, and rollback.
19. Verify backup restore and disaster-recovery runbook.
20. Produce license/attribution bundle and source-offer compliance review.
21. Produce release notes and known compatibility exceptions.
22. Tag the release candidate and deploy staging.
23. Run staging smoke and 24-hour soak.
24. Promote the exact tested artifacts to production.
25. Verify production health, new game, save recovery, and multiplayer exchange.

## Estimated Task Volume

The named tasks above are the structural minimum. Full parity expands through generated inventories:

| Family | Expected order of magnitude |
|---|---:|
| Repository/contracts/tooling | 80-120 tasks |
| Ruleset categories and validation | 100-180 tasks |
| Unique effects | Approximately 638 tasks before classification |
| Map/gameplay subsystems | 350-600 tasks |
| AI and automation | 150-300 tasks |
| Save/migration compatibility | 100-250 tasks |
| React screens and flows | 200-400 tasks |
| Rendering/mobile/accessibility | 150-300 tasks |
| Server/client multiplayer | 180-320 tasks |
| Fixtures, differential tests, defects, hardening | 400-900 tasks |

Expected total: roughly **2,300-4,000 focused tasks**, or **1,150-2,000 nominal engineering hours** before coordination, review, discovery, and rework. Calendar time will be materially longer. The ledger generated in Phase 1 is the authoritative executable backlog.

## Definition of Done for Every Task

- The task has one explicit compatibility or product assertion.
- A failing automated check was observed first.
- The smallest implementation required for that assertion was added.
- Focused tests pass.
- Formatting and linting pass for touched components.
- No unrelated inventory entries are marked complete.
- Paths, compatibility status, and documentation are updated when applicable.
- The change is committed with a focused message.

## Final End-to-End Verification Command

Phase 26 will provide `just release-verify`; its target contract is:

```bash
just release-verify
# Runs Rust checks, web checks, compatibility corpus, differential tests,
# containerized multiplayer tests, browser matrices, asset/license checks,
# performance budgets, and release artifact integrity checks.
```

No production release is considered complete until this command and the three physical-device acceptance checklists pass against the exact deployment artifacts.
