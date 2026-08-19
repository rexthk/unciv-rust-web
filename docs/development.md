# Development Setup

## Required tools

- Rust 1.95.0 through rustup
- Node.js 22 or newer
- Corepack with pnpm 10.15.0
- Java 21 for the Kotlin oracle
- Docker for future PostgreSQL/MinIO integration tests
- `just` and `cargo-deny`

## Bootstrap

```bash
rustup show
COREPACK_INTEGRITY_KEYS=0 corepack pnpm@10.15.0 --dir web install --frozen-lockfile
COREPACK_INTEGRITY_KEYS=0 corepack pnpm@10.15.0 --dir web exec playwright install chromium webkit
cargo install just cargo-deny --locked
just check
```

`COREPACK_INTEGRITY_KEYS=0` is only needed by older Corepack installations whose bundled signing-key list predates the pinned pnpm release. Upgrade Corepack instead when possible.

In environments where `registry.npmjs.org` is unavailable, set both `COREPACK_NPM_REGISTRY` and `npm_config_registry` to an approved registry mirror for installation. The committed lockfile remains authoritative.

## Compatibility source

The sibling checkout `/Users/rexthk/repo/Unciv` is used only as the pinned 4.21.10 source while creating fixtures. Verify it before regenerating inventories:

```bash
just verify-upstream
```

Override its location with `UNCIV_UPSTREAM_DIR` when necessary.

## Test layers

```bash
just check              # Rust and browser unit/build checks
just oracle-smoke       # Kotlin/Rust normalized-state smoke comparison
just e2e                # Desktop Chrome, Android Chrome, iPhone WebKit
just verify             # Complete local verification
```
