//! Deterministic seeded random streams for engine simulation.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

const STREAM_DOMAIN: u64 = 0x554e_4349_5652_4e47;
const SPLITMIX_INCREMENT: u64 = 0x9e37_79b9_7f4a_7c15;

/// Identifies an independent deterministic random stream.
///
/// Subsystems should declare explicit numeric constants and must not renumber or reuse an
/// identifier after it contributes to persisted or observable behavior.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RngStreamId(u64);

impl RngStreamId {
    /// Creates a stream identifier from its stable numeric representation.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the stable numeric representation.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for RngStreamId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<RngStreamId> for u64 {
    fn from(value: RngStreamId) -> Self {
        value.get()
    }
}

impl fmt::Display for RngStreamId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Serialize for RngStreamId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format_args!("{:016x}", self.0))
    }
}

impl<'de> Deserialize<'de> for RngStreamId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.len() != 16 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(serde::de::Error::custom(
                "RNG stream identifier must contain exactly 16 hexadecimal digits",
            ));
        }

        u64::from_str_radix(&value, 16)
            .map(Self::new)
            .map_err(serde::de::Error::custom)
    }
}

/// A reproducible `SplitMix64` stream derived from a root seed and stable stream identifier.
#[derive(Clone, Debug)]
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    /// Creates a stream at the beginning of the sequence for `seed` and `stream_id`.
    #[must_use]
    pub const fn new(seed: i64, stream_id: RngStreamId) -> Self {
        let stream_state = mix64(stream_id.get() ^ STREAM_DOMAIN);
        let state = mix64(seed.cast_unsigned() ^ stream_state);
        Self { state }
    }

    /// Advances the stream and returns the next unsigned 64-bit value.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(SPLITMIX_INCREMENT);
        mix64(self.state)
    }
}

const fn mix64(mut value: u64) -> u64 {
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}
