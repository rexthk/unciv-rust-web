//! Monotonic game-state revision for optimistic concurrency and atomic commands.
//!
//! Every successful command increments the revision exactly once. Clients capture the current
//! revision and send it as the expected revision with their next command; the engine rejects
//! stale commands without mutating state.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

/// Error returned when a game revision cannot be incremented further.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[error("game revision overflow")]
pub struct RevisionOverflow;

/// A monotonic counter identifying a specific version of game state.
///
/// The initial revision is `0`. Each successful command creates the next revision (`1`, `2`, …).
/// Serde uses a canonical decimal string so all 64 bits survive JSON and JavaScript round trips
/// without exceeding JavaScript's safe-integer range (`2^53 - 1`); `u64` serialized as a JSON
/// number would lose precision in JS and via `wasm-bindgen`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GameRevision(u64);

impl GameRevision {
    /// The revision of a newly created game before any command has been applied.
    pub const INITIAL: Self = Self::new(0);

    /// The largest representable revision.
    pub const MAX: Self = Self::new(u64::MAX);

    /// Creates a revision from its stable numeric representation.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the stable numeric representation.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the next revision, or an error if this is the maximum revision.
    ///
    /// # Errors
    ///
    /// Returns [`RevisionOverflow`] when `self` is [`Self::MAX`].
    pub fn next(self) -> Result<Self, RevisionOverflow> {
        self.checked_next().ok_or(RevisionOverflow)
    }

    /// Returns the next revision, or `None` if this is the maximum revision.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self::new(value)),
            None => None,
        }
    }

    /// Increments this revision in place, or returns an error on overflow.
    ///
    /// On error the revision is left unchanged.
    ///
    /// # Errors
    ///
    /// Returns [`RevisionOverflow`] when `self` is [`Self::MAX`].
    pub fn increment(&mut self) -> Result<(), RevisionOverflow> {
        *self = self.next()?;
        Ok(())
    }
}

impl From<u64> for GameRevision {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<GameRevision> for u64 {
    fn from(value: GameRevision) -> Self {
        value.get()
    }
}

impl fmt::Display for GameRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Serialize for GameRevision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for GameRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GameRevisionVisitor;

        impl Visitor<'_> for GameRevisionVisitor {
            type Value = GameRevision;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a game revision as a numeric string")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(GameRevision::new(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                value
                    .parse::<u64>()
                    .map(GameRevision::new)
                    .map_err(E::custom)
            }
        }

        deserializer.deserialize_any(GameRevisionVisitor)
    }
}
