//! Persistent mismatch reports for CI artifacts.

use std::{fs, io, path::Path};

use crate::Difference;

/// Writes pretty JSON differences, creating parent directories as needed.
///
/// # Errors
///
/// Returns an I/O error when directories or the report cannot be written, or
/// when the difference values cannot be encoded as JSON.
pub fn write_report(path: &Path, differences: &[Difference]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let bytes = serde_json::to_vec_pretty(differences).map_err(io::Error::other)?;
    fs::write(path, bytes)
}
