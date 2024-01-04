use crate::io::stdio::TerminalBridge;

use tokio::io::{AsyncRead, AsyncWrite};

pub struct NcursesTerminalBridge {}

impl TerminalBridge for NcursesTerminalBridge {
    async fn bridge<R: AsyncRead, W: AsyncWrite>(_reader: &mut R, _writer: &mut W) {
        todo!()
    }
}
