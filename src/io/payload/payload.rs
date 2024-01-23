use crate::io::payload::builder::PayloadBuilder;
use crate::io::payload::payloads::Initial;
use crate::io::payload::PayloadAction;
use crate::io::{PipeRead, PipeWrite, TcpPipe};
use std::marker::PhantomData;

pub struct Payload {}

pub struct UnknownArch;
pub struct X86;
pub struct X64;

impl Payload {
    pub fn builder() -> PayloadBuilder<Initial, UnknownArch> {
        PayloadBuilder::<Initial, UnknownArch>::new()
    }
}

async fn test() {
    let payload = Payload::builder()
        .recv_regex_utf8(r"HTB\{[^\}]+\}")
        .print()
        //.payload(|_data| Payload::builder().recv_line().build())
        .build();

    let mut pipe = TcpPipe::connect("123.123.123.123:80").await.unwrap();
    //aaa(&mut pipe);
    payload.execute(&mut pipe).await.unwrap();
}

async fn aaa<T: PipeWrite + PipeRead + Unpin>(_pipe: &mut T) {}

// use crate::io::payload::write::PayloadWrite;
// use crate::io::payload::write::PayloadWriteExt;
// use crate::io::Payload;
// use crossterm::Command;
// use std::marker::PhantomData;
//
// #[derive(Default, Clone)]
// pub struct Initial;
//
// pub struct PayloadBuilder<T> {
//     payload: Payload,
//     _phantom: PhantomData<T>,
// }
//
// impl<T> PayloadBuilder<T> {
//     pub(crate) fn new(payload: Payload) -> PayloadBuilder<T> {
//         PayloadBuilder::<T> {
//             payload,
//             _phantom: PhantomData::default(),
//         }
//     }
// }
//
// macro_rules! reading_fns {
//     () => {
//         pub fn recv_until<T: AsRef<[u8]>>(self, data: T) -> PayloadBuilder<Reading> {
//             let mut payload = self.payload;
//             payload.recv_until(data);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn recv_until_regex(self, pattern: &str) -> PayloadBuilder<Reading> {
//             let mut payload = self.payload;
//             payload.recv_until_regex(pattern);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn recv_regex(self, pattern: &str) -> PayloadBuilder<Reading> {
//             let mut payload = self.payload;
//             payload.recv_regex(pattern);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn recv_line(self) -> PayloadBuilder<Reading> {
//             let mut payload = self.payload;
//             payload.recv_line();
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn recv_line_crlf(self) -> PayloadBuilder<Reading> {
//             let mut payload = self.payload;
//             payload.recv_line_crlf();
//             PayloadBuilder::new(payload)
//         }
//     };
// }
//
// macro_rules! numeric_push_methods {
//     ($name:ident, $type:ty) => {
//         paste::item! {
//             pub fn [<$name _le>](self, value: $type) -> PayloadBuilder<Sending> {
//                 let mut payload = self.payload;
//                 payload.[<$name _le>](value);
//                 PayloadBuilder::new(payload)
//             }
//
//             pub fn [<$name _be>](self, value: $type) -> PayloadBuilder<Sending>  {
//                 let mut payload = self.payload;
//                 payload.[<$name _be>](value);
//                 PayloadBuilder::new(payload)
//             }
//         }
//     };
// }
//
// macro_rules! build_fn {
//     () => {
//         pub fn build(self) -> Payload {
//             self.payload
//         }
//     }
// }
//
// macro_rules! sending_fns {
//     () => {
//         pub fn push<T: AsRef<[u8]>>(self, data: T) -> PayloadBuilder<Sending> {
//             let mut payload = self.payload;
//             payload.push(data);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn push_line<T: AsRef<[u8]>>(self, data: T) -> PayloadBuilder<Sending> {
//             let mut payload = self.payload;
//             payload.push_line(data);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn push_line_crlf<T: AsRef<[u8]>>(self, data: T) -> PayloadBuilder<Sending> {
//             let mut payload = self.payload;
//             payload.push_line(data);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn fill_byte(self, byte: u8, count: usize) -> PayloadBuilder<Sending> {
//             let mut payload = self.payload;
//             payload.fill_byte(byte, count);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn fill<T: AsRef<[u8]>>(self, data: T, count: usize) -> PayloadBuilder<Sending> {
//             let mut payload = self.payload;
//             payload.fill(data, count);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn push_ansi_command<T: Command>(self, command: T) -> PayloadBuilder<Sending> {
//             let mut payload = self.payload;
//             payload.push_ansi_command(command);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn push_p64_le(self, ptr: u64) -> PayloadBuilder<Sending> {
//             let mut payload = self.payload;
//             payload.push_p64_le(ptr);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn push_p32_le(self, ptr: u32) -> PayloadBuilder<Sending> {
//             let mut payload = self.payload;
//             payload.push_p32_le(ptr);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn push_p64_be(self, ptr: u64) -> PayloadBuilder<Sending> {
//             let mut payload = self.payload;
//             payload.push_p64_be(ptr);
//             PayloadBuilder::new(payload)
//         }
//
//         pub fn push_p32_be(self, ptr: u32) -> PayloadBuilder<Sending> {
//             let mut payload = self.payload;
//             payload.push_p32_be(ptr);
//             PayloadBuilder::new(payload)
//         }
//
//         // For unsigned integers
//         numeric_push_methods!(push_u8, u8);
//         numeric_push_methods!(push_u16, u16);
//         numeric_push_methods!(push_u32, u32);
//         numeric_push_methods!(push_u64, u64);
//         numeric_push_methods!(push_u128, u128);
//         numeric_push_methods!(push_usize, usize);
//
//         // For signed integers
//         numeric_push_methods!(push_i8, i8);
//         numeric_push_methods!(push_i16, i16);
//         numeric_push_methods!(push_i32, i32);
//         numeric_push_methods!(push_i64, i64);
//         numeric_push_methods!(push_i128, i128);
//         numeric_push_methods!(push_isize, isize);
//
//         // For floating numbers
//         numeric_push_methods!(push_f32, f32);
//         numeric_push_methods!(push_f64, f64);
//     };
// }
//
// impl PayloadBuilder<Initial> {
//     reading_fns! {}
//     sending_fns! {}
// }
//
//
// impl PayloadBuilder<Stateless> {
//     reading_fns! {}
//     sending_fns! {}
//     build_fn! {}
// }
//
// impl PayloadBuilder<Reading> {
//     reading_fns! {}
//     sending_fns! {}
//     build_fn! {}
//
//     pub fn print(self) -> PayloadBuilder<Stateless> {
//         let mut payload = self.payload;
//         payload.print();
//         PayloadBuilder::new(payload)
//     }
// }
//
// impl PayloadBuilder<Sending> {
//     sending_fns! {}
//
//     pub fn send(self) -> PayloadBuilder<Stateless> {
//         let mut payload = self.payload;
//         payload.send();
//         PayloadBuilder::new(payload)
//     }
// }

// use std::marker::PhantomData;
// use crate::io::payload::steps::{PayloadStep, ReadPayload, SendPayload};
//
// #[derive(Debug, Clone)]
// pub struct StSending;
// #[derive(Debug, Clone)]
// pub struct StReading;
// #[derive(Debug, Clone)]
// pub struct StDefault;
// #[derive(Debug, Clone)]
// pub struct OptDefault;
// #[derive(Debug, Clone)]
// pub struct OptDebug;
//
//
// #[derive(Default, Clone)]
// pub struct Payload<S, O, R> {
//     //steps: Vec<Box<PayloadStep<_>>>,
//     _phantom_ret: PhantomData<R>,
//     _phantom_options: PhantomData<O>,
//     _phantom_state: PhantomData<S>,
// }
//
// impl<S,O,R> Payload<S,O,R>
// {
//     pub(crate) fn transform<SOut, OOut, ROut>(self)-> Payload<SOut, OOut, ROut> {
//         let steps = self.steps;
//         Payload {
//             steps,
//             _phantom_ret: PhantomData::default(),
//             _phantom_options: PhantomData::default(),
//             _phantom_state: PhantomData::default(),
//         }
//     }
//     pub(crate) fn get_or_init_send_payload(&mut self) -> &mut SendPayload
//     {
//         if let Some(PayloadStep::Send(_)) = self.steps.last() {} else
//         {
//             self.steps.push(PayloadStep::Send(SendPayload::default()));
//         }
//
//         match self.steps.last_mut()
//         {
//             Some(PayloadStep::Send(data)) => data,
//             _ => unreachable!()
//         }
//     }
// }
//
// impl<O,R> Payload<StReading,O,R>
// {
//     // pub(crate) fn get_read_payload(&mut self) -> &mut ReadPayload
//     // {
//     //     match self.steps.last_mut()
//     //     {
//     //         Some(PayloadStep::Read(data)) => data,
//     //         _ => unreachable!()
//     //     }
//     // }
// }
//
// impl<S,R> Payload<S, OptDefault, R>
// {
//     pub fn debug(self) -> Payload<S, OptDebug, R>
//     {
//         self.transform()
//     }
// }
