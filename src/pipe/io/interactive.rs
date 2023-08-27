use std::sync::Arc;
use read::Result;
use super::*;
use tokio::{io::*, sync::RwLock, join};

enum InteractiveMode {
    Shell,
    Key,
}

impl<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> Pipe<R, W> {
    pub async fn interactive(&self) -> Result<()>
    {
        self.interactiv_iternal(InteractiveMode::Shell).await
    }

    pub async fn interactive_key_mode(&self) -> Result<()>
    {
        self.interactiv_iternal(InteractiveMode::Key).await
    }

    async fn interactive_read_task(&self, end_signal: Arc<RwLock<bool>>) -> Result<()>
    {
        let mut buf = [0u8; 1024];
        while !*end_signal.read().await {
            match self.read(&mut buf).await {
                Ok(0) => break, //EOF
                Ok(len) => 
                { 
                    Self::print_read(&buf[..len]);
                    println!();
                },
                Err(PipeReadError::TimeoutError(_)) => continue,
                Err(PipeReadError::IoError(e)) if e.kind() == io::ErrorKind::TimedOut => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    async fn interactive_write_task(&self, end_signal: Arc<RwLock<bool>>) -> Result<()>
    {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            println!("You wrote: {}", line);
            self.write(line.as_bytes()).await?;
        }
        *end_signal.write().await = true;
        Ok(())
    }

    async fn interactiv_iternal(&self, _mode: InteractiveMode) -> Result<()>
    {
        let finished_write = Arc::new(RwLock::new(false)); 
        let finished_read = finished_write.clone();
        let read_thread = async move
        {
            self.interactive_read_task(finished_read).await
        };

        let write_thread= async move
        {
            self.interactive_write_task(finished_write).await
        };

        let (read_result, write_result) = join!(read_thread, write_thread);
        let _ = read_result?;
        let _ = write_result?;

        Ok(())
    }

    fn print_read(buf: &[u8])
    {
        for b in buf.iter()
        {
            print!("{}", *b as char)
        }
    }

}
