# P3-03 Ordered Collections Design

**Date:** 2026-08-20

## Design

The engine exposes `OrderedMap<K, V>` and `OrderedSet<T>` aliases backed by `indexmap::IndexMap`
and `indexmap::IndexSet`. Both collections preserve insertion order for iteration and Serde output,
which allows engine state and events assembled in a deterministic sequence to serialize to stable
bytes. The existing workspace dependency already enables IndexMap's Serde support.

These aliases make order-sensitive collection choices explicit at engine API boundaries without
adding wrapper types or duplicating the upstream crate's methods. They do not turn nondeterministic
input into deterministic output: callers collecting from unordered sources must sort or otherwise
stabilize those values before insertion.

## Alternatives Considered

- `BTreeMap` and `BTreeSet` provide canonical key order, but require `Ord` and discard meaningful
  source or ruleset insertion order.
- `HashMap` and `HashSet` with a fixed hasher would stabilize hashing but still would not promise a
  public iteration-order contract.
- Newtype wrappers could enforce additional invariants later, but add API surface without a current
  requirement.

## Verification

Focused JSON assertions prove maps serialize properties and sets serialize values in insertion
order. Replacing an existing map value and inserting a duplicate set value must retain the original
position. Workspace formatting, Clippy, tests, strict engine Rustdoc, and the wasm32 API check must
pass.
