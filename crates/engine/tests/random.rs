//! Contract tests for deterministic seeded random streams.

use unciv_engine::random::{RngStreamId, SeededRng};

const STREAM: RngStreamId = RngStreamId::new(0x0123_4567_89ab_cdef);

#[test]
fn stream_id_preserves_its_raw_value() {
    assert_eq!(STREAM.get(), 0x0123_4567_89ab_cdef);
    assert_eq!(u64::from(STREAM), 0x0123_4567_89ab_cdef);
    assert_eq!(STREAM.to_string(), "81985529216486895");
}

#[test]
fn stream_id_serializes_as_fixed_width_hexadecimal() {
    let encoded = serde_json::to_string(&STREAM).expect("serialize RNG stream identifier");
    assert_eq!(encoded, r#""0123456789abcdef""#);
    assert_eq!(
        serde_json::from_str::<RngStreamId>(&encoded).expect("deserialize RNG stream identifier"),
        STREAM,
    );

    let maximum = RngStreamId::new(u64::MAX);
    let maximum_encoded =
        serde_json::to_string(&maximum).expect("serialize maximum RNG stream identifier");
    assert_eq!(maximum_encoded, r#""ffffffffffffffff""#);
    assert_eq!(
        serde_json::from_str::<RngStreamId>(&maximum_encoded)
            .expect("deserialize maximum RNG stream identifier"),
        maximum,
    );
}

#[test]
fn stream_id_rejects_malformed_serialized_values() {
    for invalid in [
        "7",
        r#""7""#,
        r#""0123456789abcde""#,
        r#""0123456789abcdef0""#,
        r#""0123456789abcdeg""#,
    ] {
        assert!(serde_json::from_str::<RngStreamId>(invalid).is_err());
    }
}

#[test]
fn seeded_stream_matches_the_golden_sequence() {
    let mut rng = SeededRng::new(-42, STREAM);

    let sequence = [
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
    ];

    assert_eq!(
        sequence,
        [
            0xecc2_ae24_7554_c283,
            0xf081_9020_4e02_12a0,
            0x0f16_d892_34f4_6253,
            0x360b_c19f_0f0f_7207,
            0xdbd9_d245_a610_0c36,
        ],
    );
}

#[test]
fn reconstructing_a_stream_reproduces_its_sequence() {
    let mut first = SeededRng::new(i64::MIN, STREAM);
    let mut second = SeededRng::new(i64::MIN, STREAM);

    for _ in 0..16 {
        assert_eq!(first.next_u64(), second.next_u64());
    }
}

#[test]
fn distinct_stream_ids_produce_distinct_sequences() {
    let mut first = SeededRng::new(42, RngStreamId::new(7));
    let mut second = SeededRng::new(42, RngStreamId::new(8));

    let first_prefix = [first.next_u64(), first.next_u64(), first.next_u64()];
    let second_prefix = [second.next_u64(), second.next_u64(), second.next_u64()];

    assert_ne!(first_prefix, second_prefix);
}
