use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::AsyncRead;

use super::TestAction;

type BoxSleep = Pin<Box<tokio::time::Sleep>>;

/// Async reader driven by a queue of `TestAction` values.
pub struct AsyncTestReader {
    queue: VecDeque<TestAction>,
    sleeper: Option<BoxSleep>,
}

impl std::fmt::Debug for AsyncTestReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncTestReader")
            .field("queue", &self.queue)
            .finish_non_exhaustive()
    }
}

impl AsyncTestReader {
    /// Create a new test reader from a slice of actions.
    pub fn new(queue: &[TestAction]) -> Self {
        Self {
            queue: VecDeque::from(queue.to_vec()),
            sleeper: None,
        }
    }
}

impl AsyncRead for AsyncTestReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        loop {
            if let Some(sleeper) = self.sleeper.as_mut() {
                match sleeper.as_mut().poll(cx) {
                    Poll::Ready(()) => {
                        self.sleeper = None;
                    }
                    Poll::Pending => return Poll::Pending,
                }
            }

            return match self.queue.pop_front() {
                Some(TestAction::Data(mut data)) => {
                    let len = usize::min(buf.remaining(), data.len());
                    buf.put_slice(&data[..len]);
                    data.drain(..len);
                    if !data.is_empty() {
                        self.queue.push_front(TestAction::Data(data));
                    }
                    self.sleeper = None;
                    Poll::Ready(Ok(()))
                }
                Some(TestAction::Sleep(duration)) => {
                    self.sleeper = Some(Box::pin(tokio::time::sleep(duration)));
                    continue;
                }
                None => Poll::Ready(Ok(())),
            };
        }
    }
}
