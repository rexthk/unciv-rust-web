# Unciv Rust/Web

Browser-first Rust/WebAssembly reimplementation of Unciv, pinned for observable compatibility with Unciv 4.21.10.

## Targets

- Desktop Chrome
- Android Chrome
- iPhone Chrome/WebKit
- Offline single-player PWA
- Compatible Rust/Axum asynchronous multiplayer server

## Architecture

- Rust engine, rules, AI, compatibility, server, and tooling crates
- `wasm-bindgen` boundary for the browser engine
- React and TypeScript application UI
- PixiJS world-map renderer
- IndexedDB local persistence
- Kotlin Unciv oracle for differential parity tests

See the [validated design](docs/plans/2026-08-19-unciv-rust-web-design.md) and [implementation plan](docs/plans/2026-08-19-unciv-rust-web-implementation-plan.md).

## Development

```bash
corepack pnpm@10.15.0 --dir web install
cargo test --workspace
corepack pnpm --dir web test
```

The compatibility source is pinned in `upstream/manifest.toml`. Source-derived files retain their original license notices; see `LICENSES/`.
