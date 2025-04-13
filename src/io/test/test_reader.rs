use pin_project_lite::pin_project;
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    io::AsyncRead,
    time::{sleep, Sleep},
};

use super::TestAction;

pin_project! {
    pub struct AsyncTestReader {
        queue: VecDeque<TestAction>,
        #[pin]
        sleeper: Option<Sleep>,
    }
}

impl AsyncTestReader {
    pub fn new(queue: &[TestAction]) -> Self {
        Self {
            queue: queue.into(),
            sleeper: None,
        }
    }
}

impl AsyncRead for AsyncTestReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let mut this = self.project();

        loop {
            if let Some(sleeper) = this.sleeper.as_mut().as_pin_mut() {
                match sleeper.poll(cx) {
                    Poll::Ready(_) => {
                        this.sleeper.set(None);
                    }
                    Poll::Pending => return Poll::Pending,
                }
            }

            return match this.queue.pop_front() {
                Some(TestAction::Data(mut data)) => {
                    let len = usize::min(buf.remaining(), data.len());
                    buf.put_slice(&data[..len]);
                    data.drain(..len);
                    if !data.is_empty() {
                        this.queue.push_front(TestAction::Data(data));
                    }
                    Poll::Ready(Ok(()))
                }
                Some(TestAction::Sleep(duration)) => {
                    this.sleeper.set(Some(sleep(duration)));
                    continue;
                }
                None => Poll::Ready(Ok(())),
            };
        }
    }
}
