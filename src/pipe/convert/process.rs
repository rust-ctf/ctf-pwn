use super::*;

use std::{ffi::OsStr, process::Stdio, result};
use thiserror::*;
use tokio::{io::*, process::*};

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

pub type Result<T> = result::Result<T, ProcessPipeError>;

type ProcessPipe = Pipe<MergedOutput2<ChildStdout, ChildStderr>, ChildStdin>;
type StdoutPipe = Pipe<ChildStdout, ChildStdin>;
type StderrPipe = Pipe<ChildStderr, ChildStdin>;

impl<R, W> Pipe<R, W>
where
    R: AsyncRead,
    W: AsyncWrite,
{
    pub async fn from_app<S: AsRef<OsStr>>(program: S) -> Result<ProcessPipe> {
        let command = Command::new(program);
        Self::spawn_command(command)
    }

    pub async fn from_app_args<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
        program: S,
        args: I,
    ) -> Result<ProcessPipe> {
        let mut command = Command::new(program);
        let _ = command.args(args);
        Self::spawn_command(command)
    }

    pub fn spawn_command(mut value: Command) -> Result<ProcessPipe> {
        let process = value
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        process.try_into()
    }

    pub async fn from_app_stdout<S: AsRef<OsStr>>(program: S) -> Result<StdoutPipe> {
        let command = Command::new(program);
        Self::spawn_command_stdout(command)
    }

    pub async fn from_app_args_stdout<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
        program: S,
        args: I,
    ) -> Result<StdoutPipe> {
        let mut command = Command::new(program);
        let _ = command.args(args);
        Self::spawn_command_stdout(command)
    }

    pub fn spawn_command_stdout(mut value: Command) -> Result<StdoutPipe> {
        let process = value.stdout(Stdio::piped()).stdin(Stdio::piped()).spawn()?;

        let stdin = process.stdin.ok_or(ProcessPipeError::StdinNotSet)?;
        let stdout = process.stdout.ok_or(ProcessPipeError::StdoutNotSet)?;
        Ok((stdin, stdout).into())
    }

    pub async fn from_app_stderr<S: AsRef<OsStr>>(program: S) -> Result<StderrPipe> {
        let command = Command::new(program);
        Self::spawn_command_stderr(command)
    }

    pub async fn from_app_args_stderr<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
        program: S,
        args: I,
    ) -> Result<StderrPipe> {
        let mut command = Command::new(program);
        let _ = command.args(args);
        Self::spawn_command_stderr(command)
    }

    pub fn spawn_command_stderr(mut value: Command) -> Result<StderrPipe> {
        let process = value.stderr(Stdio::piped()).stdin(Stdio::piped()).spawn()?;

        let stdin = process.stdin.ok_or(ProcessPipeError::StdinNotSet)?;
        let stderr = process.stderr.ok_or(ProcessPipeError::StdErrNotSet)?;
        Ok((stdin, stderr).into())
    }
}

impl TryFrom<Child> for ProcessPipe {
    type Error = ProcessPipeError;

    fn try_from(value: Child) -> Result<Self> {
        let stdin = value.stdin.ok_or(ProcessPipeError::StdinNotSet)?;
        let stdout = value.stdout.ok_or(ProcessPipeError::StdoutNotSet)?;
        let stderr = value.stderr.ok_or(ProcessPipeError::StdErrNotSet)?;
        Ok((stdin, stdout, stderr).into())
    }
}

impl From<(ChildStdin, ChildStdout, ChildStderr)> for ProcessPipe {
    fn from(value: (ChildStdin, ChildStdout, ChildStderr)) -> Self {
        let (stdin, stdout, stderr) = value;
        let read_stream = MergedOutput2 {
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
