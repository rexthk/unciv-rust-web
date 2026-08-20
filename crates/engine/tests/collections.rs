//! Contract tests for deterministic ordered collection aliases.

use unciv_engine::collections::{OrderedMap, OrderedSet};

#[test]
fn ordered_map_serializes_in_insertion_order() {
    let mut map = OrderedMap::new();
    map.insert("third", 3);
    map.insert("first", 1);
    map.insert("second", 2);
    map.insert("first", 10);

    let encoded = serde_json::to_string(&map).expect("serialize ordered map");

    assert_eq!(encoded, r#"{"third":3,"first":10,"second":2}"#);
}

#[test]
fn ordered_set_serializes_in_insertion_order() {
    let mut set = OrderedSet::new();
    set.insert("third");
    set.insert("first");
    set.insert("second");
    set.insert("first");

    let encoded = serde_json::to_string(&set).expect("serialize ordered set");

    assert_eq!(encoded, r#"["third","first","second"]"#);
}
