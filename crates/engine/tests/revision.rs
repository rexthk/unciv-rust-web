//! Contract tests for the game revision type.

use proptest::prelude::*;
use unciv_engine::revision::{GameRevision, RevisionOverflow};

#[test]
fn initial_revision_is_zero() {
    assert_eq!(GameRevision::INITIAL.get(), 0);
    assert_eq!(GameRevision::default(), GameRevision::INITIAL);
}

#[test]
fn revision_preserves_its_raw_value() {
    let revision = GameRevision::new(42);
    assert_eq!(revision.get(), 42);
    assert_eq!(u64::from(revision), 42);
    assert_eq!(GameRevision::from(42u64), revision);
    assert_eq!(revision.to_string(), "42");
}

#[test]
fn revision_round_trips_through_json() {
    let revision = GameRevision::new(7);
    let encoded = serde_json::to_string(&revision).expect("serialize revision");
    // Serialized as a decimal string to preserve full u64 through JS / wasm-bindgen.
    assert_eq!(encoded, r#""7""#);
    let decoded = serde_json::from_str::<GameRevision>(&encoded).expect("deserialize revision");
    assert_eq!(decoded, revision);
}

#[test]
fn revision_serializes_as_string_for_javascript_safety() {
    // Values above 2^53-1 would lose precision as JSON numbers in JavaScript.
    let unsafe_js = GameRevision::new((1u64 << 53) + 1);
    let encoded = serde_json::to_string(&unsafe_js).expect("serialize large revision");
    // Must be a JSON string, not a number.
    assert_eq!(encoded, format!(r#""{}""#, (1u64 << 53) + 1));
    assert_eq!(
        serde_json::from_str::<GameRevision>(&encoded).expect("deserialize large revision"),
        unsafe_js
    );

    let maximum = GameRevision::new(u64::MAX);
    let maximum_encoded = serde_json::to_string(&maximum).expect("serialize maximum revision");
    assert_eq!(maximum_encoded, format!(r#""{}""#, u64::MAX));
    assert_eq!(
        serde_json::from_str::<GameRevision>(&maximum_encoded)
            .expect("deserialize maximum revision"),
        maximum
    );
}

#[test]
fn revision_deserializes_from_number_and_string_for_compatibility() {
    // New wire format: string.
    let from_string: GameRevision =
        serde_json::from_str(r#""42""#).expect("deserialize revision from string");
    assert_eq!(from_string, GameRevision::new(42));

    // Legacy / tolerant: JSON number (e.g. from older saves or manual fixtures).
    let from_number: GameRevision =
        serde_json::from_str("42").expect("deserialize revision from number");
    assert_eq!(from_number, GameRevision::new(42));

    // Max via string (cannot be reliably represented as JS number but can be in JSON tests).
    let max_from_string: GameRevision =
        serde_json::from_str(&format!(r#""{}""#, u64::MAX)).expect("deserialize max from string");
    assert_eq!(max_from_string, GameRevision::MAX);
}

#[test]
fn revision_rejects_malformed_serialized_values() {
    for invalid in [
        "7.5",
        r#""7.5""#,
        r#""-1""#,
        r#""abc""#,
        r#""18446744073709551616""#, // u64::MAX + 1
        r#""0x10""#,
        r"null",
        r"{}",
        r"[]",
    ] {
        assert!(
            serde_json::from_str::<GameRevision>(invalid).is_err(),
            "should reject {invalid}"
        );
    }
}

#[test]
fn next_increments_revision_by_one() {
    let initial = GameRevision::INITIAL;
    let first = initial.next().expect("first increment succeeds");
    assert_eq!(first.get(), 1);

    let second = first.next().expect("second increment succeeds");
    assert_eq!(second.get(), 2);

    // Chaining increments.
    let mut revision = GameRevision::INITIAL;
    for expected in 1..=10 {
        revision = revision.next().expect("increment within range");
        assert_eq!(revision.get(), expected);
    }
}

#[test]
fn next_returns_overflow_error_at_maximum() {
    let maximum = GameRevision::new(u64::MAX);
    assert_eq!(maximum, GameRevision::MAX);

    let result = maximum.next();
    assert!(result.is_err());

    let error = result.expect_err("maximum revision must overflow");
    assert_eq!(error, RevisionOverflow);
    assert_eq!(error.to_string(), "game revision overflow");
}

#[test]
fn next_does_not_wrap_or_panic_on_overflow() {
    let maximum = GameRevision::MAX;
    // Ensure repeated attempts also fail without side effects.
    for _ in 0..3 {
        assert!(maximum.next().is_err());
        // Original remains unchanged.
        assert_eq!(maximum.get(), u64::MAX);
    }
}

#[test]
fn checked_next_returns_none_on_overflow() {
    assert_eq!(GameRevision::MAX.checked_next(), None);
    assert_eq!(
        GameRevision::new(41).checked_next(),
        Some(GameRevision::new(42))
    );
    assert_eq!(
        GameRevision::INITIAL.checked_next(),
        Some(GameRevision::new(1))
    );
}

#[test]
fn revision_increment_mutates_on_success_and_preserves_on_error() {
    let mut revision = GameRevision::new(5);
    revision.increment().expect("increment succeeds");
    assert_eq!(revision.get(), 6);

    let mut maximum = GameRevision::MAX;
    let result = maximum.increment();
    assert!(result.is_err());
    assert_eq!(result.expect_err("overflow"), RevisionOverflow);
    // Value unchanged on error.
    assert_eq!(maximum.get(), u64::MAX);
}

#[test]
fn revision_error_implements_std_error() {
    let error = RevisionOverflow;
    let std_error: &dyn std::error::Error = &error;
    assert_eq!(std_error.to_string(), "game revision overflow");
    // Debug representation is available.
    assert!(!format!("{error:?}").is_empty());
}

#[test]
fn revision_ordering_matches_numeric_ordering() {
    assert!(GameRevision::new(1) < GameRevision::new(2));
    assert!(GameRevision::new(10) > GameRevision::new(9));
    assert_eq!(GameRevision::new(5), GameRevision::new(5));
    assert!(GameRevision::INITIAL < GameRevision::MAX);
}

proptest! {
    #[test]
    fn revisions_round_trip_for_arbitrary_raw_values(raw in any::<u64>()) {
        let revision = GameRevision::new(raw);
        prop_assert_eq!(revision.get(), raw);
        prop_assert_eq!(u64::from(revision), raw);
        prop_assert_eq!(revision.to_string(), raw.to_string());

        let encoded = serde_json::to_string(&revision).expect("serialize revision");
        // Wire format is a quoted decimal string for JS safety.
        prop_assert_eq!(&encoded, &format!("\"{raw}\""));

        let decoded = serde_json::from_str::<GameRevision>(&encoded).expect("deserialize revision");
        prop_assert_eq!(decoded, revision);
    }

    #[test]
    fn next_increments_by_one_for_non_maximum_values(raw in 0..u64::MAX) {
        let revision = GameRevision::new(raw);
        let next = revision.next().expect("non-maximum revision increments");
        prop_assert_eq!(next.get(), raw + 1);
    }

    #[test]
    fn checked_next_matches_next(raw in any::<u64>()) {
        let revision = GameRevision::new(raw);
        if raw == u64::MAX {
            prop_assert_eq!(revision.checked_next(), None);
            prop_assert!(revision.next().is_err());
        } else {
            prop_assert_eq!(revision.checked_next(), Some(GameRevision::new(raw + 1)));
            prop_assert_eq!(revision.next().expect("increment").get(), raw + 1);
        }
    }
}

#[test]
fn revision_round_trips_at_boundaries() {
    for raw in [u64::MIN, 1, u64::MAX - 1, u64::MAX] {
        let revision = GameRevision::new(raw);
        let encoded = serde_json::to_string(&revision).expect("serialize revision");
        let decoded = serde_json::from_str::<GameRevision>(&encoded).expect("deserialize revision");
        assert_eq!(decoded, revision);
    }
}
