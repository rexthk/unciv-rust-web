set dotenv-load := true

pnpm := "corepack pnpm@10.15.0"

default:
    @just --list

install:
    {{pnpm}} --dir web install --frozen-lockfile
    {{pnpm}} --dir web exec playwright install chromium webkit

fmt:
    cargo fmt --all
    {{pnpm}} --dir web exec prettier --write .

check:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace
    cargo check -p unciv-wasm-api --target wasm32-unknown-unknown
    node --test scripts/lib/inventory.test.mjs
    node scripts/check-inventories.mjs
    node scripts/check-task-ledger.mjs
    {{pnpm}} --dir web lint
    {{pnpm}} --dir web format:check
    {{pnpm}} --dir web test
    {{pnpm}} --dir web build

e2e:
    {{pnpm}} --dir web test:e2e

audit:
    cargo deny check
    {{pnpm}} --dir web audit --audit-level high

oracle-smoke:
    cargo test -p unciv-test-oracle --test smoke -- --ignored

verify-upstream:
    ./scripts/verify-upstream.sh

verify-inventories:
    node scripts/check-inventories.mjs
    node scripts/check-task-ledger.mjs

verify: check verify-upstream verify-inventories oracle-smoke e2e
