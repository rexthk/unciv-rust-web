//! Collection aliases with deterministic iteration and serialization order.

use indexmap::{IndexMap, IndexSet};

/// A map that iterates and serializes entries in insertion order.
///
/// Callers must insert entries in a deterministic order; this collection preserves order but does
/// not canonicalize values received from unordered sources.
pub type OrderedMap<K, V> = IndexMap<K, V>;

/// A set that iterates and serializes values in insertion order.
///
/// Callers must insert values in a deterministic order; this collection preserves order but does
/// not canonicalize values received from unordered sources.
pub type OrderedSet<T> = IndexSet<T>;
