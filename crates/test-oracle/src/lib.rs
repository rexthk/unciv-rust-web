//! Kotlin-oracle process integration and normalized-state comparison.

pub mod process;
pub mod report;
pub mod scenario;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{path::Path, time::Duration};
use thiserror::Error;

use crate::process::{OracleCommand, ProcessError};

const ORACLE_MARKER: &str = "@@UNCIV_ORACLE_JSON@@";

/// One observable mismatch between normalized states.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Difference {
    /// JSON Pointer locating the mismatch.
    pub pointer: String,
    /// Expected normalized value.
    pub expected: Value,
    /// Actual normalized value.
    pub actual: Value,
}

/// Kotlin oracle invocation or decoding failure.
#[derive(Debug, Error)]
pub enum OracleError {
    /// External process failure.
    #[error(transparent)]
    Process(#[from] ProcessError),
    /// Successful output did not contain the protocol marker.
    #[error("Kotlin oracle output did not contain {ORACLE_MARKER}")]
    MissingMarker,
    /// Marked output was not valid JSON.
    #[error("invalid normalized oracle JSON: {0}")]
    Json(#[from] serde_json::Error),
}

/// Runs the repository's Kotlin oracle for one scenario and decodes normalized state.
///
/// # Errors
///
/// Returns [`OracleError`] when the process fails, the protocol marker is absent,
/// or the marked payload is not valid JSON.
pub fn run_oracle_scenario(
    repository_root: &Path,
    scenario: &Path,
    timeout: Duration,
) -> Result<Value, OracleError> {
    let script = repository_root.join("oracle-kotlin/run-oracle.sh");
    let output = OracleCommand::new(script)
        .args([scenario.as_os_str()])
        .timeout(timeout)
        .run()?;
    let payload = output
        .stdout
        .lines()
        .find_map(|line| line.strip_prefix(ORACLE_MARKER))
        .ok_or(OracleError::MissingMarker)?;
    Ok(serde_json::from_str(payload)?)
}

/// Recursively compares normalized JSON values in deterministic key order.
#[must_use]
pub fn diff_values(expected: &Value, actual: &Value) -> Vec<Difference> {
    let mut differences = Vec::new();
    compare("", expected, actual, &mut differences);
    differences
}

fn compare(pointer: &str, expected: &Value, actual: &Value, differences: &mut Vec<Difference>) {
    match (expected, actual) {
        (Value::Object(expected_map), Value::Object(actual_map)) => {
            let mut keys = expected_map
                .keys()
                .chain(actual_map.keys())
                .collect::<Vec<_>>();
            keys.sort_unstable();
            keys.dedup();
            for key in keys {
                let child_pointer = format!("{pointer}/{}", escape_pointer(key));
                compare(
                    &child_pointer,
                    expected_map.get(key).unwrap_or(&Value::Null),
                    actual_map.get(key).unwrap_or(&Value::Null),
                    differences,
                );
            }
        }
        (Value::Array(expected_items), Value::Array(actual_items)) => {
            for index in 0..expected_items.len().max(actual_items.len()) {
                compare(
                    &format!("{pointer}/{index}"),
                    expected_items.get(index).unwrap_or(&Value::Null),
                    actual_items.get(index).unwrap_or(&Value::Null),
                    differences,
                );
            }
        }
        _ if expected != actual => differences.push(Difference {
            pointer: pointer.to_owned(),
            expected: expected.clone(),
            actual: actual.clone(),
        }),
        _ => {}
    }
}

fn escape_pointer(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}
