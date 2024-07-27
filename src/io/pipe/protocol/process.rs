use crate::io::pipe::{OwnedPipe, PipeError};
use std::{ffi::OsStr, process::Stdio};
use tokio::process::*;

pub type StdoutPipe = OwnedPipe<ChildStdout, ChildStdin>;

impl StdoutPipe {
    pub async fn from_app<S: AsRef<OsStr>>(program: S) -> Result<Self, PipeError> {
        let command = Command::new(program);
        Self::spawn_command(command)
    }

    pub async fn from_app_args<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
        program: S,
        args: I,
    ) -> Result<Self, PipeError> {
        let mut command = Command::new(program);
        let _ = command.args(args);
        Self::spawn_command(command)
    }

    pub fn spawn_command(mut value: Command) -> Result<Self, PipeError> {
        let process = value
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = process.stdin.ok_or(PipeError::Unknown)?;
        let stdout = process.stdout.ok_or(PipeError::Unknown)?;
        Ok((stdin, stdout).into())
    }
}

impl From<(ChildStdin, ChildStdout)> for StdoutPipe {
    fn from(value: (ChildStdin, ChildStdout)) -> Self {
        let (stdin, stdout) = value;
        let read_stream = stdout;
        let write_stream = stdin;
        OwnedPipe::new(read_stream, write_stream)
    }
}
