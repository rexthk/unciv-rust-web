//! Contract tests for scenario, diff, process, and report behavior.

use std::time::Duration;

use serde_json::json;
use unciv_test_oracle::{
    diff_values,
    process::{OracleCommand, ProcessError},
    report::write_report,
    scenario::Scenario,
};

#[test]
fn scenario_reader_accepts_versioned_instruction_stream() {
    let scenario = Scenario::from_json(
        r#"{
          "schemaVersion": 1,
          "name": "smoke",
          "seed": 42,
          "instructions": [{"type": "new_game", "payload": {}}]
        }"#,
    )
    .expect("valid scenario");

    assert_eq!(scenario.name, "smoke");
    assert_eq!(scenario.seed, 42);
    assert_eq!(scenario.instructions[0].kind, "new_game");
}

#[test]
fn normalized_diff_reports_json_pointer_and_values() {
    let expected = json!({"turn": 0, "metadata": {"rulesets": 2}});
    let actual = json!({"turn": 1, "metadata": {"rulesets": 2}});

    let differences = diff_values(&expected, &actual);

    assert_eq!(differences.len(), 1);
    assert_eq!(differences[0].pointer, "/turn");
    assert_eq!(differences[0].expected, json!(0));
    assert_eq!(differences[0].actual, json!(1));
}

#[test]
fn process_runner_captures_success_failure_and_timeout() {
    let success = OracleCommand::new("/bin/sh")
        .args(["-c", "printf '{\"ok\":true}'"])
        .timeout(Duration::from_secs(1))
        .run()
        .expect("successful process");
    assert_eq!(success.stdout, "{\"ok\":true}");

    let failure = OracleCommand::new("/bin/sh")
        .args(["-c", "printf boom >&2; exit 7"])
        .timeout(Duration::from_secs(1))
        .run()
        .expect_err("nonzero process");
    assert!(matches!(failure, ProcessError::Exit { code: Some(7), .. }));

    let timeout = OracleCommand::new("/bin/sh")
        .args(["-c", "sleep 2"])
        .timeout(Duration::from_millis(20))
        .run()
        .expect_err("timed out process");
    assert!(matches!(timeout, ProcessError::Timeout { .. }));
}

#[test]
fn mismatch_report_creates_parent_directories_and_json() {
    let temporary = tempfile::tempdir().expect("temporary directory");
    let report = temporary.path().join("nested/differences.json");
    let differences = diff_values(&json!({"turn": 0}), &json!({"turn": 1}));

    write_report(&report, &differences).expect("write report");

    let decoded: serde_json::Value =
        serde_json::from_slice(&std::fs::read(report).expect("read report")).expect("valid JSON");
    assert_eq!(decoded[0]["pointer"], "/turn");
}
