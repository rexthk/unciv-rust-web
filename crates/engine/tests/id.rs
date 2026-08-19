//! Contract tests for strongly typed entity identifiers.

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use proptest::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
use unciv_engine::id::{CityId, CivilizationId, GameId, UnitId};
use uuid::Uuid;

fn assert_id_round_trip<Id>(raw: u32)
where
    Id: Copy + Debug + Display + Eq + From<u32> + Into<u32> + Serialize + DeserializeOwned,
{
    let id = Id::from(raw);

    assert_eq!(id.into(), raw);
    assert_eq!(id.to_string(), raw.to_string());

    let encoded = serde_json::to_string(&id).expect("serialize entity ID");
    assert_eq!(encoded, raw.to_string());

    let decoded = serde_json::from_str::<Id>(&encoded).expect("deserialize entity ID");
    assert_eq!(decoded, id);
}

proptest! {
    #[test]
    fn entity_ids_round_trip_for_arbitrary_raw_values(raw in any::<u32>()) {
        assert_id_round_trip::<CivilizationId>(raw);
        assert_id_round_trip::<CityId>(raw);
        assert_id_round_trip::<UnitId>(raw);
    }

    #[test]
    fn game_ids_round_trip_for_arbitrary_uuid_bytes(bytes in any::<[u8; 16]>()) {
        let uuid = Uuid::from_bytes(bytes);
        let id = GameId::new(uuid);

        assert_eq!(id.get(), uuid);
        assert_eq!(Uuid::from(id), uuid);
        assert_eq!(GameId::from_str(&id.to_string()).expect("parse game ID"), id);

        let encoded = serde_json::to_string(&id).expect("serialize game ID");
        assert_eq!(encoded, format!("\"{uuid}\""));

        let decoded = serde_json::from_str::<GameId>(&encoded).expect("deserialize game ID");
        assert_eq!(decoded, id);
    }
}

#[test]
fn entity_ids_round_trip_at_raw_value_boundaries() {
    for raw in [u32::MIN, 1, u32::MAX] {
        assert_id_round_trip::<CivilizationId>(raw);
        assert_id_round_trip::<CityId>(raw);
        assert_id_round_trip::<UnitId>(raw);
    }
}
