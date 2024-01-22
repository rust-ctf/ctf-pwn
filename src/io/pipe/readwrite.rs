// use crate::io::{Payload, PipeError, PipeReadExt, PipeWriteExt};
// use tokio::io::AsyncWriteExt;
//
// impl<T> PipeReadWriteExt for T where T: PipeReadExt + PipeWriteExt {}
//
// pub trait PipeReadWriteExt: PipeReadExt + PipeWriteExt {
//     async fn payload(&mut self, payload: &Payload) -> Result<usize, PipeError>
//     where
//         Self: Unpin,
//     {
//         payload_internal(self, payload, false).await
//     }
//
//     async fn payload_debug(&mut self, payload: &Payload) -> Result<usize, PipeError>
//     where
//         Self: Unpin,
//     {
//         payload_internal(self, payload, true).await
//     }
// }
//
// async fn payload_internal<T: PipeReadWriteExt + Unpin + ?Sized>(
//     this: &mut T,
//     payload: &Payload,
//     debug: bool,
// ) -> Result<usize, PipeError> {
//     let mut size = 0usize;
//     let mut last_data = None;
//     for step in payload.steps() {
//         match step {
//             PayloadStep::Data(data) => {
//                 this.write_all(&data).await?;
//                 size += data.len();
//                 print_debug(&data, debug).await?;
//             }
//             PayloadStep::Send() => {
//                 this.flush().await?;
//             }
//             PayloadStep::ReadUntil(data) => {
//                 let data = this.recv_until(data, false).await?;
//                 print_debug(&data, debug).await?;
//                 last_data = Some(data);
//             }
//             PayloadStep::ReadUntilRegex(pattern) => {
//                 let data = this.recv_until_regex(pattern, false).await?;
//                 print_debug(&data, debug).await?;
//                 last_data = Some(data);
//             }
//             PayloadStep::ReadRegex(pattern) => {
//                 if debug {
//                     let (before, data) = this.recv_until_regex_split(pattern).await?;
//                     print_to_stdout(&before).await?;
//                     print_to_stdout(&data).await?;
//                     last_data = Some(data);
//                 } else {
//                     let data = this.recv_regex(pattern).await?;
//                     last_data = Some(data);
//                 }
//             }
//             PayloadStep::ReadLine() => {
//                 let data = this.recv_line().await?;
//                 print_debug(&data, debug).await?;
//                 last_data = Some(data);
//             }
//             PayloadStep::ReadLineCrlf() => {
//                 let data = this.recv_line().await?;
//                 print_debug(&data, debug).await?;
//                 last_data = Some(data);
//             }
//             PayloadStep::Print() => {
//                 if let Some(data) = &last_data {
//                     print_to_stdout(&data).await?;
//                 }
//             }
//         }
//     }
//
//     this.flush().await?;
//     Ok(size)
// }
//
// async fn print_debug(data: &[u8], debug: bool) -> tokio::io::Result<()> {
//     if debug {
//         print_to_stdout(&data).await?;
//     }
//     Ok(())
// }
//
// async fn print_to_stdout(data: &[u8]) -> tokio::io::Result<()> {
//     tokio::io::stdout().write_all(&data).await?;
//     tokio::io::stdout().flush().await?;
//     println!();
//     Ok(())
// }
