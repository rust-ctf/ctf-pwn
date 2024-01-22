// use crate::io::Payload;
//
// #[derive(Debug, Clone)]
// pub(crate) enum PayloadStep<R> {
//     Send(SendPayload),
//     Read(ReadPayload<R>),
// }
//
// #[derive(Debug,Default,Clone)]
// pub(crate) struct SendPayload
// {
//     data: Vec<u8>
// }
//
// pub struct PayloadCtx
// {
//
// }
//
// #[derive(Debug, Clone)]
// pub(crate) struct ReadPayload<R>
// {
//     action: PayloadReadAction,
//     print: bool,
//     //dyn_payload: Option<fn(PayloadCtx, R) -> impl Payload<_,_,_>>
// }
//
// #[derive(Debug, Copy, Clone)]
// pub(crate) enum ReadEncoding
// {
//     Bytes,
//     Utf8,
//     Ascii,
// }
//
// #[derive(Debug, Clone)]
// pub(crate) enum PayloadReadAction {
//     RecvUntil(Vec<u8>),
//     RecvUntilRegex(String),
//     RecvRegex(String),
//     RecvLine(),
//     RecvLineCrlf(),
// }
//
