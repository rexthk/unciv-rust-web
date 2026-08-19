//! Versioned cross-engine scenario contracts.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use thiserror::Error;

/// Cross-engine scenario executed by both implementations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Scenario {
    /// Contract version.
    pub schema_version: u32,
    /// Human-readable stable scenario name.
    pub name: String,
    /// Deterministic random seed.
    pub seed: i64,
    /// Ordered instructions.
    pub instructions: Vec<Instruction>,
}

/// One language-neutral scenario operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Instruction {
    /// Stable instruction identifier.
    #[serde(rename = "type")]
    pub kind: String,
    /// Instruction-specific values.
    #[serde(default)]
    pub payload: Map<String, Value>,
}

/// Scenario decoding or contract error.
#[derive(Debug, Error)]
pub enum ScenarioError {
    /// JSON could not be decoded.
    #[error("invalid scenario JSON: {0}")]
    Json(#[from] serde_json::Error),
    /// Contract version is unsupported.
    #[error("unsupported scenario schema version {0}")]
    UnsupportedVersion(u32),
    /// Required textual value is empty.
    #[error("scenario name and instruction types must not be empty")]
    EmptyName,
}

impl Scenario {
    /// Decodes and validates one scenario document.
    ///
    /// # Errors
    ///
    /// Returns [`ScenarioError`] for malformed JSON, unsupported schema versions,
    /// or empty scenario/instruction names.
    pub fn from_json(source: &str) -> Result<Self, ScenarioError> {
        let scenario: Self = serde_json::from_str(source)?;
        if scenario.schema_version != 1 {
            return Err(ScenarioError::UnsupportedVersion(scenario.schema_version));
        }
        if scenario.name.trim().is_empty()
            || scenario
                .instructions
                .iter()
                .any(|instruction| instruction.kind.trim().is_empty())
        {
            return Err(ScenarioError::EmptyName);
        }
        Ok(scenario)
    }
}
