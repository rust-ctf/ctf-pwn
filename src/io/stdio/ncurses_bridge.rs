use crate::io::stdio::TerminalBridge;
use crate::io::{CacheReader, TimeoutReader, TimeoutWriter};
use tokio::io::{AsyncRead, AsyncWrite};

pub struct NcursesTerminalBridge {}

impl TerminalBridge for NcursesTerminalBridge {
    async fn bridge<R: AsyncRead, W: AsyncWrite>(reader: &mut R, writer: &mut W) {
        todo!()
    }
}
