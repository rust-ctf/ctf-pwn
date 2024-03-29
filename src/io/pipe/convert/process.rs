use crate::io::merge::MergedAsyncReader;
use crate::io::Pipe;
use std::{ffi::OsStr, process::Stdio, result};
use thiserror::*;
use tokio::process::*;

#[derive(Error, Debug)]
pub enum ProcessPipeError {
    #[error("stdin not set")]
    StdinNotSet,
    #[error("stdout not set")]
    StdoutNotSet,
    #[error("stderr not set")]
    StdErrNotSet,
    #[error("io error")]
    IoError(#[from] tokio::io::Error),
}

pub type ProcessPipeResult<T> = result::Result<T, ProcessPipeError>;

pub type ProcessPipe = Pipe<MergedAsyncReader<ChildStdout, ChildStderr>, ChildStdin>;
pub type StdoutPipe = Pipe<ChildStdout, ChildStdin>;
pub type StderrPipe = Pipe<ChildStderr, ChildStdin>;

impl ProcessPipe {
    pub async fn from_app<S: AsRef<OsStr>>(program: S) -> ProcessPipeResult<Self> {
        let command = Command::new(program);
        Self::spawn_command(command)
    }

    pub async fn from_app_args<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
        program: S,
        args: I,
    ) -> ProcessPipeResult<Self> {
        let mut command = Command::new(program);
        let _ = command.args(args);
        Self::spawn_command(command)
    }

    pub fn spawn_command(mut value: Command) -> ProcessPipeResult<Self> {
        let process = value
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        process.try_into()
    }
}

impl StdoutPipe {
    pub async fn from_app<S: AsRef<OsStr>>(program: S) -> ProcessPipeResult<Self> {
        let command = Command::new(program);
        Self::spawn_command(command)
    }

    pub async fn from_app_args<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
        program: S,
        args: I,
    ) -> ProcessPipeResult<Self> {
        let mut command = Command::new(program);
        let _ = command.args(args);
        Self::spawn_command(command)
    }

    pub fn spawn_command(mut value: Command) -> ProcessPipeResult<Self> {
        let process = value.stdout(Stdio::piped()).stdin(Stdio::piped()).spawn()?;

        let stdin = process.stdin.ok_or(ProcessPipeError::StdinNotSet)?;
        let stdout = process.stdout.ok_or(ProcessPipeError::StdoutNotSet)?;
        Ok((stdin, stdout).into())
    }
}

impl StderrPipe {
    pub async fn from_app<S: AsRef<OsStr>>(program: S) -> ProcessPipeResult<Self> {
        let command = Command::new(program);
        Self::spawn_command(command)
    }

    pub async fn from_app_args<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
        program: S,
        args: I,
    ) -> ProcessPipeResult<Self> {
        let mut command = Command::new(program);
        let _ = command.args(args);
        Self::spawn_command(command)
    }

    pub fn spawn_command(mut value: Command) -> ProcessPipeResult<Self> {
        let process = value.stderr(Stdio::piped()).stdin(Stdio::piped()).spawn()?;

        let stdin = process.stdin.ok_or(ProcessPipeError::StdinNotSet)?;
        let stderr = process.stderr.ok_or(ProcessPipeError::StdErrNotSet)?;
        Ok((stdin, stderr).into())
    }
}

impl TryFrom<Child> for ProcessPipe {
    type Error = ProcessPipeError;

    fn try_from(value: Child) -> ProcessPipeResult<Self> {
        let stdin = value.stdin.ok_or(ProcessPipeError::StdinNotSet)?;
        let stdout = value.stdout.ok_or(ProcessPipeError::StdoutNotSet)?;
        let stderr = value.stderr.ok_or(ProcessPipeError::StdErrNotSet)?;
        Ok((stdin, stdout, stderr).into())
    }
}

impl From<(ChildStdin, ChildStdout, ChildStderr)> for ProcessPipe {
    fn from(value: (ChildStdin, ChildStdout, ChildStderr)) -> Self {
        let (stdin, stdout, stderr) = value;
        let read_stream = MergedAsyncReader {
            stream0: stdout,
            stream1: stderr,
        };
        let write_stream = stdin;
        Pipe::new(read_stream, write_stream)
    }
}

impl From<(ChildStdin, ChildStdout)> for StdoutPipe {
    fn from(value: (ChildStdin, ChildStdout)) -> Self {
        let (stdin, stdout) = value;
        let read_stream = stdout;
        let write_stream = stdin;
        Pipe::new(read_stream, write_stream)
    }
}

impl From<(ChildStdin, ChildStderr)> for StderrPipe {
    fn from(value: (ChildStdin, ChildStderr)) -> Self {
        let (stdin, stderr) = value;
        let read_stream = stderr;
        let write_stream = stdin;
        Pipe::new(read_stream, write_stream)
    }
}
