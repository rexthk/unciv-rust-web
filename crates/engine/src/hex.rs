//! Integer coordinates and basic geometry for Unciv's hexagonal maps.

use serde::{Deserialize, Serialize};

/// A tile position in Unciv's two-axis hex coordinate system.
///
/// Increasing `x` moves toward 10 o'clock and increasing `y` moves toward 2 o'clock. The third
/// cube-coordinate component is derived as `y - x` and is therefore not stored.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct HexCoord {
    x: i32,
    y: i32,
}

impl HexCoord {
    /// The map origin.
    pub const ZERO: Self = Self::new(0, 0);

    /// The six unit directions, clockwise from 12 o'clock.
    pub const DIRECTIONS: [Self; 6] = [
        Self::new(1, 1),
        Self::new(0, 1),
        Self::new(-1, 0),
        Self::new(-1, -1),
        Self::new(0, -1),
        Self::new(1, 0),
    ];

    /// Creates a coordinate from its Unciv `x` and `y` components.
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns the coordinate's 10-o'clock-axis component.
    #[must_use]
    pub const fn x(self) -> i32 {
        self.x
    }

    /// Returns the coordinate's 2-o'clock-axis component.
    #[must_use]
    pub const fn y(self) -> i32 {
        self.y
    }

    /// Returns adjacent coordinates clockwise from 12 o'clock.
    ///
    /// # Panics
    ///
    /// Panics if a component is at an `i32` boundary and the adjacent coordinate cannot be
    /// represented. Valid game maps are far smaller than this numeric range.
    #[must_use]
    pub fn neighbors(self) -> [Self; 6] {
        Self::DIRECTIONS.map(|direction| {
            let x = self
                .x
                .checked_add(direction.x)
                .expect("hex coordinate neighbor exceeds i32 range");
            let y = self
                .y
                .checked_add(direction.y)
                .expect("hex coordinate neighbor exceeds i32 range");

            Self::new(x, y)
        })
    }

    /// Returns the unwrapped number of hex steps to `other`.
    #[must_use]
    pub fn distance_to(self, other: Self) -> u64 {
        let delta_x = i64::from(self.x) - i64::from(other.x);
        let delta_y = i64::from(self.y) - i64::from(other.y);
        let delta_z = delta_y - delta_x;

        delta_x
            .unsigned_abs()
            .max(delta_y.unsigned_abs())
            .max(delta_z.unsigned_abs())
    }
}
