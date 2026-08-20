//! Contract tests for Unciv-compatible hex coordinates.

use proptest::prelude::*;
use unciv_engine::hex::HexCoord;

#[test]
fn components_retain_signed_unciv_coordinates() {
    let coordinate = HexCoord::new(-7, 11);

    assert_eq!(coordinate.x(), -7);
    assert_eq!(coordinate.y(), 11);
}

#[test]
fn neighbors_follow_unciv_clockwise_order() {
    let origin = HexCoord::new(3, -2);

    assert_eq!(
        origin.neighbors(),
        [
            HexCoord::new(4, -1),
            HexCoord::new(3, -1),
            HexCoord::new(2, -2),
            HexCoord::new(2, -3),
            HexCoord::new(3, -3),
            HexCoord::new(4, -2),
        ]
    );
}

#[test]
fn neighbors_reject_every_unrepresentable_coordinate_boundary() {
    for coordinate in [
        HexCoord::new(i32::MAX, 0),
        HexCoord::new(i32::MIN, 0),
        HexCoord::new(0, i32::MAX),
        HexCoord::new(0, i32::MIN),
    ] {
        let panic = std::panic::catch_unwind(|| coordinate.neighbors());

        assert!(
            panic.is_err(),
            "{coordinate:?} should reject neighbor overflow"
        );
    }
}

#[test]
fn distance_matches_unciv_coordinate_axes() {
    let origin = HexCoord::ZERO;

    assert_eq!(origin.distance_to(origin), 0);
    assert_eq!(origin.distance_to(HexCoord::new(3, 3)), 3);
    assert_eq!(origin.distance_to(HexCoord::new(-4, -2)), 4);
    assert_eq!(origin.distance_to(HexCoord::new(3, -2)), 5);
}

#[test]
fn distance_handles_the_full_i32_coordinate_range() {
    let low_high = HexCoord::new(i32::MIN, i32::MAX);
    let high_low = HexCoord::new(i32::MAX, i32::MIN);

    assert_eq!(low_high.distance_to(high_low), 8_589_934_590);
}

#[test]
fn coordinates_serialize_as_named_integer_components() {
    let coordinate = HexCoord::new(-7, 11);
    let encoded = serde_json::to_string(&coordinate).expect("serialize hex coordinate");

    assert_eq!(encoded, r#"{"x":-7,"y":11}"#);
    assert_eq!(
        serde_json::from_str::<HexCoord>(&encoded).expect("deserialize hex coordinate"),
        coordinate
    );
}

proptest! {
    #[test]
    fn distance_to_self_is_zero(x in any::<i32>(), y in any::<i32>()) {
        let coordinate = HexCoord::new(x, y);

        prop_assert_eq!(coordinate.distance_to(coordinate), 0);
    }

    #[test]
    fn every_neighbor_is_one_step_away(x in -1_000_000i32..=1_000_000, y in -1_000_000i32..=1_000_000) {
        let origin = HexCoord::new(x, y);

        for neighbor in origin.neighbors() {
            prop_assert_eq!(origin.distance_to(neighbor), 1);
        }
    }

    #[test]
    fn distance_is_symmetric(
        ax in any::<i32>(),
        ay in any::<i32>(),
        bx in any::<i32>(),
        by in any::<i32>(),
    ) {
        let a = HexCoord::new(ax, ay);
        let b = HexCoord::new(bx, by);

        prop_assert_eq!(a.distance_to(b), b.distance_to(a));
    }

    #[test]
    fn distance_satisfies_the_triangle_inequality(
        ax in any::<i32>(),
        ay in any::<i32>(),
        bx in any::<i32>(),
        by in any::<i32>(),
        cx in any::<i32>(),
        cy in any::<i32>(),
    ) {
        let a = HexCoord::new(ax, ay);
        let b = HexCoord::new(bx, by);
        let c = HexCoord::new(cx, cy);

        prop_assert!(a.distance_to(c) <= a.distance_to(b) + b.distance_to(c));
    }

    #[test]
    fn distance_is_translation_invariant(
        ax in -1_000_000i32..=1_000_000,
        ay in -1_000_000i32..=1_000_000,
        bx in -1_000_000i32..=1_000_000,
        by in -1_000_000i32..=1_000_000,
        dx in -1_000_000i32..=1_000_000,
        dy in -1_000_000i32..=1_000_000,
    ) {
        let a = HexCoord::new(ax, ay);
        let b = HexCoord::new(bx, by);
        let translated_a = HexCoord::new(ax + dx, ay + dy);
        let translated_b = HexCoord::new(bx + dx, by + dy);

        prop_assert_eq!(a.distance_to(b), translated_a.distance_to(translated_b));
    }

    #[test]
    fn coordinates_round_trip_through_json(x in any::<i32>(), y in any::<i32>()) {
        let coordinate = HexCoord::new(x, y);
        let encoded = serde_json::to_string(&coordinate).expect("serialize hex coordinate");
        let decoded = serde_json::from_str::<HexCoord>(&encoded).expect("deserialize hex coordinate");

        prop_assert_eq!(decoded, coordinate);
    }
}
