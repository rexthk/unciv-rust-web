//! Strongly typed identifiers for mutable game entities.
//!
//! Runtime entity references use `u32`, which is compact and crosses the JavaScript boundary
//! without numeric precision loss. Persistent game identity retains Unciv's UUID representation.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifies a game across saves, workers, and multiplayer exchanges.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct GameId(Uuid);

impl GameId {
    /// Creates a game identifier from an existing UUID.
    #[must_use]
    pub const fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns the underlying UUID.
    #[must_use]
    pub const fn get(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for GameId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<GameId> for Uuid {
    fn from(value: GameId) -> Self {
        value.get()
    }
}

impl fmt::Display for GameId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for GameId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value).map(Self::new)
    }
}

macro_rules! entity_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(
            Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
        )]
        #[repr(transparent)]
        #[serde(transparent)]
        pub struct $name(u32);

        impl $name {
            /// Creates an identifier from its stable numeric representation.
            #[must_use]
            pub const fn new(value: u32) -> Self {
                Self(value)
            }

            /// Returns the stable numeric representation.
            #[must_use]
            pub const fn get(self) -> u32 {
                self.0
            }
        }

        impl From<u32> for $name {
            fn from(value: u32) -> Self {
                Self::new(value)
            }
        }

        impl From<$name> for u32 {
            fn from(value: $name) -> Self {
                value.get()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

entity_id!(
    /// Identifies a civilization within a game.
    CivilizationId
);
entity_id!(
    /// Identifies a city within a game.
    CityId
);
entity_id!(
    /// Identifies a unit within a game.
    UnitId
);
