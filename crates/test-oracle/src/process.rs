//! Bounded execution of the external Kotlin oracle.

use std::{
    ffi::{OsStr, OsString},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use thiserror::Error;

/// Successful oracle process output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessOutput {
    /// UTF-8 standard output without trailing whitespace.
    pub stdout: String,
    /// UTF-8 standard error without trailing whitespace.
    pub stderr: String,
}

/// External-process failure.
#[derive(Debug, Error)]
pub enum ProcessError {
    /// Process could not be started or observed.
    #[error("failed to execute {program}: {source}")]
    Io {
        /// Program name.
        program: String,
        /// Underlying process error.
        #[source]
        source: std::io::Error,
    },
    /// Process exceeded its deadline.
    #[error("process {program} exceeded {timeout:?}")]
    Timeout {
        /// Program name.
        program: String,
        /// Configured deadline.
        timeout: Duration,
    },
    /// Process exited unsuccessfully.
    #[error("process {program} exited with {code:?}: {stderr}")]
    Exit {
        /// Program name.
        program: String,
        /// Platform exit code.
        code: Option<i32>,
        /// Captured error stream.
        stderr: String,
    },
    /// Output was not valid UTF-8.
    #[error("process {program} emitted non-UTF-8 {stream}")]
    Utf8 {
        /// Program name.
        program: String,
        /// Stream name.
        stream: &'static str,
    },
}

/// Builder for a bounded oracle command.
#[derive(Debug, Clone)]
pub struct OracleCommand {
    program: OsString,
    args: Vec<OsString>,
    timeout: Duration,
}

impl OracleCommand {
    /// Creates a command with a 60-second default deadline.
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            program: program.as_ref().to_owned(),
            args: Vec::new(),
            timeout: Duration::from_mins(1),
        }
    }

    /// Appends arguments.
    #[must_use]
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.args
            .extend(args.into_iter().map(|arg| arg.as_ref().to_owned()));
        self
    }

    /// Overrides the execution deadline.
    #[must_use]
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Executes the process and captures both streams.
    ///
    /// # Errors
    ///
    /// Returns [`ProcessError`] when the process cannot start, exceeds its deadline,
    /// exits unsuccessfully, or emits output that is not UTF-8.
    pub fn run(&self) -> Result<ProcessOutput, ProcessError> {
        let program = self.program.to_string_lossy().into_owned();
        let mut child = Command::new(&self.program)
            .args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|source| ProcessError::Io {
                program: program.clone(),
                source,
            })?;
        let started = Instant::now();
        loop {
            match child.try_wait().map_err(|source| ProcessError::Io {
                program: program.clone(),
                source,
            })? {
                Some(_) => break,
                None if started.elapsed() >= self.timeout => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(ProcessError::Timeout {
                        program,
                        timeout: self.timeout,
                    });
                }
                None => thread::sleep(Duration::from_millis(5)),
            }
        }
        let output = child
            .wait_with_output()
            .map_err(|source| ProcessError::Io {
                program: program.clone(),
                source,
            })?;
        let stdout = String::from_utf8(output.stdout)
            .map_err(|_| ProcessError::Utf8 {
                program: program.clone(),
                stream: "stdout",
            })?
            .trim()
            .to_owned();
        let stderr = String::from_utf8(output.stderr)
            .map_err(|_| ProcessError::Utf8 {
                program: program.clone(),
                stream: "stderr",
            })?
            .trim()
            .to_owned();
        if !output.status.success() {
            return Err(ProcessError::Exit {
                program,
                code: output.status.code(),
                stderr,
            });
        }
        Ok(ProcessOutput { stdout, stderr })
    }
}
