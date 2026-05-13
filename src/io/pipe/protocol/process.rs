use crate::io::pipe::{OwnedPipe, PipeError};
use std::{ffi::OsStr, process::Stdio};
use tokio::process::*;

/// Pipe backed by a child process's stdin/stdout.
pub type StdoutPipe = OwnedPipe<Child, ChildStdout, ChildStdin>;

impl StdoutPipe {
    /// Spawn a process by program name and return a pipe to it.
    ///
    /// # Errors
    ///
    /// Returns `PipeError` if the process fails to spawn or its stdio handles are unavailable.
    pub fn from_app<S: AsRef<OsStr>>(program: S) -> Result<Self, PipeError> {
        let command = Command::new(program);
        Self::spawn_command(command)
    }

    /// Spawn a process with arguments and return a pipe to it.
    ///
    /// # Errors
    ///
    /// Returns `PipeError` if the process fails to spawn or its stdio handles are unavailable.
    pub fn from_app_args<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
        program: S,
        args: I,
    ) -> Result<Self, PipeError> {
        let mut command = Command::new(program);
        let _ = command.args(args);
        Self::spawn_command(command)
    }

    /// Spawn a `Command` and return a pipe connected to its stdin/stdout.
    ///
    /// # Errors
    ///
    /// Returns `PipeError` if the process fails to spawn or its stdio handles are unavailable.
    pub fn spawn_command(mut value: Command) -> Result<Self, PipeError> {
        let mut process = value
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = process.stdin.take().ok_or(PipeError::Unknown)?;
        let stdout = process.stdout.take().ok_or(PipeError::Unknown)?;
        Ok(Self::new_with_handle(stdout, stdin, process))
    }
}
