//! End-to-end Kotlin oracle smoke comparison.

use std::{fs, path::PathBuf, time::Duration};

use unciv_test_oracle::{diff_values, run_oracle_scenario};

#[test]
#[ignore = "requires the pinned Unciv checkout and Java 21"]
fn pinned_kotlin_oracle_matches_smoke_fixture() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let scenario = root.join("fixtures/unciv-4.21.10/scenarios/smoke.json");
    let expected: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("fixtures/unciv-4.21.10/normalized/smoke.json"))
            .expect("expected state fixture"),
    )
    .expect("valid expected state");

    let actual = run_oracle_scenario(&root, &scenario, Duration::from_mins(3))
        .expect("Kotlin oracle execution");

    assert_eq!(diff_values(&expected, &actual), Vec::new());
}
