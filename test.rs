#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[allow(async_fn_in_trait)]
#[allow(unused_imports)]
pub mod io {
    mod payload {
        mod payload {
            use std::marker::PhantomData;
            use crate::io::payload::builder::PayloadBuilder;
            use crate::io::payload::payloads::Initial;
            pub struct Payload {}
            pub struct UnknownArch;
            pub struct X86;
            pub struct X64;
            impl Payload {
                pub fn builder() -> PayloadBuilder<Initial, UnknownArch> {
                    PayloadBuilder::<Initial, UnknownArch>::new()
                }
            }
            fn test() {
                let payload = Payload::builder()
                    .recv_regex(r"HTB\{[^\}]+\}")
                    .payload(|data| { Payload::builder().recv_line().build() })
                    .build();
            }
        }
        mod write {
            use crate::io::PayloadAction;
        }
        mod steps {}
        mod payloads {
            mod send {
                use std::marker::PhantomData;
                use crossterm::Command;
                use tokio::io::AsyncWriteExt;
                use crate::io::{PayloadAction, PipeError, PipeRead, PipeWrite};
                pub struct Building;
                pub struct Complete;
                pub struct SendPayload<T, A> {
                    data: Vec<u8>,
                    _phantom: PhantomData<T>,
                    _phantom_arch: PhantomData<A>,
                }
                impl<A> SendPayload<Building, A> {
                    pub fn new() -> Self {
                        SendPayload {
                            data: Vec::new(),
                            _phantom: PhantomData::default(),
                            _phantom_arch: PhantomData::default(),
                        }
                    }
                    pub fn complete(self) -> SendPayload<Complete, A> {
                        SendPayload {
                            data: self.data,
                            _phantom: PhantomData::default(),
                            _phantom_arch: PhantomData::default(),
                        }
                    }
                    pub fn push<T: AsRef<[u8]>>(mut self, data: T) -> Self {
                        self.data.extend_from_slice(data.as_ref());
                        Self {
                            data: self.data,
                            _phantom: PhantomData::default(),
                            _phantom_arch: PhantomData::default(),
                        }
                    }
                    pub fn push_line<T: AsRef<[u8]>>(self, data: T) -> Self {
                        self.push(data).push("\n")
                    }
                    pub fn push_line_crlf<T: AsRef<[u8]>>(self, data: T) -> Self {
                        self.push(data).push("\r\n")
                    }
                    pub fn fill_byte(self, byte: u8, count: usize) -> Self {
                        let mut payload = self;
                        for _ in 0..count {
                            payload = payload.push([byte]);
                        }
                        payload
                    }
                    pub fn fill<T: AsRef<[u8]>>(self, data: T, count: usize) -> Self {
                        let mut payload = self;
                        for _ in 0..count {
                            payload = payload.push(data.as_ref());
                        }
                        payload
                    }
                    pub fn push_ansi_command<T: Command>(mut self, command: T) -> Self {
                        let mut ansi = String::new();
                        command.write_ansi(&mut ansi).unwrap();
                        self.push(ansi)
                    }
                    pub fn push_u8_le(self, value: u8) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_u8_be(self, value: u8) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_u16_le(self, value: u16) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_u16_be(self, value: u16) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_u32_le(self, value: u32) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_u32_be(self, value: u32) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_u64_le(self, value: u64) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_u64_be(self, value: u64) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_u128_le(self, value: u128) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_u128_be(self, value: u128) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_usize_le(self, value: usize) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_usize_be(self, value: usize) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_i8_le(self, value: i8) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_i8_be(self, value: i8) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_i16_le(self, value: i16) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_i16_be(self, value: i16) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_i32_le(self, value: i32) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_i32_be(self, value: i32) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_i64_le(self, value: i64) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_i64_be(self, value: i64) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_i128_le(self, value: i128) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_i128_be(self, value: i128) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_isize_le(self, value: isize) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_isize_be(self, value: isize) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_f32_le(self, value: f32) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_f32_be(self, value: f32) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                    pub fn push_f64_le(self, value: f64) -> Self {
                        self.push(&value.to_le_bytes())
                    }
                    pub fn push_f64_be(self, value: f64) -> Self {
                        self.push(&value.to_be_bytes())
                    }
                }
                impl<A> PayloadAction for SendPayload<Complete, A> {
                    type ReturnType = ();
                    async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(
                        &self,
                        _reader: &mut R,
                        writer: &mut W,
                    ) -> Result<Self::ReturnType, PipeError> {
                        writer.write_all(&self.data).await?;
                        Ok(())
                    }
                }
                impl<A> PayloadAction for SendPayload<Building, A> {
                    type ReturnType = ();
                    async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(
                        &self,
                        _reader: &mut R,
                        writer: &mut W,
                    ) -> Result<Self::ReturnType, PipeError> {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            mod read {
                use std::marker::PhantomData;
                use ascii::AsciiString;
                use crate::io::{
                    PayloadAction, PipeError, PipeRead, PipeReadExt, PipeWrite,
                };
                pub(crate) struct Bytes;
                pub(crate) struct Utf8;
                pub(crate) struct Ascii;
                pub enum ReadPayloadType {
                    RecvUntil(Vec<u8>, bool),
                    RecvUntilRegex(String, bool),
                    RecvRegex(String),
                    RecvLine(),
                    RecvLineCrlf(),
                }
                pub struct ReadPayload<E> {
                    read_type: ReadPayloadType,
                    _phantom: PhantomData<E>,
                }
                impl<E> ReadPayload<E> {
                    pub fn new(read_type: ReadPayloadType) -> ReadPayload<E> {
                        ReadPayload {
                            read_type,
                            _phantom: PhantomData::default(),
                        }
                    }
                    pub fn new_utf8(read_type: ReadPayloadType) -> ReadPayload<Utf8> {
                        ReadPayload {
                            read_type,
                            _phantom: PhantomData::default(),
                        }
                    }
                    pub fn new_ascii(read_type: ReadPayloadType) -> ReadPayload<Ascii> {
                        ReadPayload {
                            read_type,
                            _phantom: PhantomData::default(),
                        }
                    }
                }
                async fn execute_internal<R: PipeRead + Unpin, W: PipeWrite + Unpin, T>(
                    this: &ReadPayload<T>,
                    reader: &mut R,
                    _writer: &mut W,
                ) -> Result<Vec<u8>, PipeError> {
                    let result = match &this.read_type {
                        ReadPayloadType::RecvUntil(delim, drop) => {
                            reader.recv_until(delim, *drop).await?
                        }
                        ReadPayloadType::RecvUntilRegex(pattern, drop) => {
                            reader.recv_until_regex(pattern, *drop).await?
                        }
                        ReadPayloadType::RecvRegex(pattern) => {
                            reader.recv_regex(pattern).await?
                        }
                        ReadPayloadType::RecvLine() => reader.recv_line().await?,
                        ReadPayloadType::RecvLineCrlf() => reader.recv_line_crlf().await?,
                    };
                    Ok(result)
                }
                impl PayloadAction for ReadPayload<Bytes> {
                    type ReturnType = Vec<u8>;
                    async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(
                        &self,
                        reader: &mut R,
                        writer: &mut W,
                    ) -> Result<Self::ReturnType, PipeError> {
                        execute_internal(self, reader, writer).await
                    }
                }
                impl PayloadAction for ReadPayload<Utf8> {
                    type ReturnType = String;
                    async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(
                        &self,
                        reader: &mut R,
                        writer: &mut W,
                    ) -> Result<Self::ReturnType, PipeError> {
                        let result = execute_internal(self, reader, writer).await?;
                        Ok(String::from_utf8(result)?)
                    }
                }
                impl PayloadAction for ReadPayload<Ascii> {
                    type ReturnType = AsciiString;
                    async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(
                        &self,
                        reader: &mut R,
                        writer: &mut W,
                    ) -> Result<Self::ReturnType, PipeError> {
                        let result = execute_internal(self, reader, writer).await?;
                        Ok(AsciiString::from_ascii(result)?)
                    }
                }
            }
            mod chain {
                use crate::io::{PayloadAction, PipeError, PipeRead, PipeWrite};
                pub struct Chain<P1, P2> {
                    pub payload1: P1,
                    pub payload2: P2,
                }
                impl<P1: PayloadAction, P2: PayloadAction> Chain<P1, P2> {
                    pub fn new(payload1: P1, payload2: P2) -> Chain<P1, P2> {
                        Chain { payload1, payload2 }
                    }
                }
                impl<P1: PayloadAction, P2: PayloadAction> PayloadAction
                for Chain<P1, P2> {
                    type ReturnType = P2::ReturnType;
                    async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(
                        &self,
                        reader: &mut R,
                        writer: &mut W,
                    ) -> Result<Self::ReturnType, PipeError> {
                        let _ = self.payload1.execute(reader, writer).await?;
                        self.payload2.execute(reader, writer).await
                    }
                }
            }
            mod dynamic_payload {
                use crate::io::{PayloadAction, PipeError, PipeRead, PipeWrite};
                pub struct DynamicPayload<P, E, R> {
                    prev_payload: P,
                    action: fn(E) -> R,
                }
                impl<P, E, R> DynamicPayload<P, E, R>
                where
                    P: PayloadAction<ReturnType = E>,
                    R: PayloadAction,
                {
                    pub fn new(
                        prev_payload: P,
                        action: fn(E) -> R,
                    ) -> DynamicPayload<P, E, R> {
                        DynamicPayload {
                            prev_payload,
                            action,
                        }
                    }
                }
                impl<P, E, T> PayloadAction for DynamicPayload<P, E, T>
                where
                    P: PayloadAction<ReturnType = E>,
                    T: PayloadAction,
                {
                    type ReturnType = T::ReturnType;
                    async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(
                        &self,
                        reader: &mut R,
                        writer: &mut W,
                    ) -> Result<Self::ReturnType, PipeError> {
                        let result = self.prev_payload.execute(reader, writer).await?;
                        let new_payload = (self.action)(result);
                        new_payload.execute(reader, writer).await
                    }
                }
            }
            mod initial {
                use crate::io::{PayloadAction, PipeError, PipeRead, PipeWrite};
                pub struct Initial {}
                impl Initial {
                    pub fn new() -> Initial {
                        Initial {}
                    }
                }
                impl PayloadAction for Initial {
                    type ReturnType = ();
                    async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(
                        &self,
                        reader: &mut R,
                        writer: &mut W,
                    ) -> Result<Self::ReturnType, PipeError> {
                        Ok(())
                    }
                }
            }
            pub(crate) use send::*;
            pub(crate) use read::*;
            pub(crate) use chain::*;
            pub(crate) use dynamic_payload::*;
            pub(crate) use initial::*;
        }
        mod builder {
            mod send {
                use crossterm::Command;
                use crate::io::payload::builder::PayloadBuilder;
                use crate::io::payload::payloads::{
                    Building, Chain, Complete, DynamicPayload, Initial, ReadPayload,
                    SendPayload,
                };
                use crate::io::PayloadAction;
                impl<A> PayloadBuilder<Initial, A> {
                    pub fn push<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new().push(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_line<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new().push_line(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_line_crlf<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_line_crlf(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn fill_byte(
                        self,
                        byte: u8,
                        count: usize,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .fill_byte(byte, count);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn fill<T: AsRef<[u8]>>(
                        self,
                        data: T,
                        count: usize,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .fill(data, count);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_ansi_command<T: Command>(
                        self,
                        command: T,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_ansi_command(command);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u8_le(
                        self,
                        value: u8,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u8_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u8_be(
                        self,
                        value: u8,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u8_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u16_le(
                        self,
                        value: u16,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u16_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u16_be(
                        self,
                        value: u16,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u16_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u32_le(
                        self,
                        value: u32,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u32_be(
                        self,
                        value: u32,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u64_le(
                        self,
                        value: u64,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u64_be(
                        self,
                        value: u64,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u128_le(
                        self,
                        value: u128,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u128_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u128_be(
                        self,
                        value: u128,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u128_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_usize_le(
                        self,
                        value: usize,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_usize_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_usize_be(
                        self,
                        value: usize,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_usize_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i8_le(
                        self,
                        value: i8,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i8_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i8_be(
                        self,
                        value: i8,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i8_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i16_le(
                        self,
                        value: i16,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i16_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i16_be(
                        self,
                        value: i16,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i16_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i32_le(
                        self,
                        value: i32,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i32_be(
                        self,
                        value: i32,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i64_le(
                        self,
                        value: i64,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i64_be(
                        self,
                        value: i64,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i128_le(
                        self,
                        value: i128,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i128_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i128_be(
                        self,
                        value: i128,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i128_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_isize_le(
                        self,
                        value: isize,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_isize_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_isize_be(
                        self,
                        value: isize,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_isize_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f32_le(
                        self,
                        value: f32,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f32_be(
                        self,
                        value: f32,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f64_le(
                        self,
                        value: f64,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f64_be(
                        self,
                        value: f64,
                    ) -> PayloadBuilder<Chain<Initial, SendPayload<Building, A>>, A> {
                        let payload1: Initial = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                }
                impl<
                    P1: PayloadAction,
                    RP,
                    A,
                > PayloadBuilder<Chain<P1, ReadPayload<RP>>, A>
                where
                    ReadPayload<RP>: PayloadAction,
                {
                    pub fn push<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new().push(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_line<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new().push_line(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_line_crlf<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_line_crlf(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn fill_byte(
                        self,
                        byte: u8,
                        count: usize,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .fill_byte(byte, count);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn fill<T: AsRef<[u8]>>(
                        self,
                        data: T,
                        count: usize,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .fill(data, count);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_ansi_command<T: Command>(
                        self,
                        command: T,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_ansi_command(command);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u8_le(
                        self,
                        value: u8,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u8_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u8_be(
                        self,
                        value: u8,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u8_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u16_le(
                        self,
                        value: u16,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u16_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u16_be(
                        self,
                        value: u16,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u16_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u32_le(
                        self,
                        value: u32,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u32_be(
                        self,
                        value: u32,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u64_le(
                        self,
                        value: u64,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u64_be(
                        self,
                        value: u64,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u128_le(
                        self,
                        value: u128,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u128_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u128_be(
                        self,
                        value: u128,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u128_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_usize_le(
                        self,
                        value: usize,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_usize_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_usize_be(
                        self,
                        value: usize,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_usize_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i8_le(
                        self,
                        value: i8,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i8_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i8_be(
                        self,
                        value: i8,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i8_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i16_le(
                        self,
                        value: i16,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i16_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i16_be(
                        self,
                        value: i16,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i16_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i32_le(
                        self,
                        value: i32,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i32_be(
                        self,
                        value: i32,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i64_le(
                        self,
                        value: i64,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i64_be(
                        self,
                        value: i64,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i128_le(
                        self,
                        value: i128,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i128_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i128_be(
                        self,
                        value: i128,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i128_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_isize_le(
                        self,
                        value: isize,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_isize_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_isize_be(
                        self,
                        value: isize,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_isize_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f32_le(
                        self,
                        value: f32,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f32_be(
                        self,
                        value: f32,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f64_le(
                        self,
                        value: f64,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f64_be(
                        self,
                        value: f64,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: Chain<P1, ReadPayload<RP>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                }
                impl<P, E, R, A> PayloadBuilder<DynamicPayload<P, E, R>, A>
                where
                    P: PayloadAction<ReturnType = E>,
                    R: PayloadAction<ReturnType = E>,
                {
                    pub fn push<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new().push(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_line<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new().push_line(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_line_crlf<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_line_crlf(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn fill_byte(
                        self,
                        byte: u8,
                        count: usize,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .fill_byte(byte, count);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn fill<T: AsRef<[u8]>>(
                        self,
                        data: T,
                        count: usize,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .fill(data, count);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_ansi_command<T: Command>(
                        self,
                        command: T,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_ansi_command(command);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u8_le(
                        self,
                        value: u8,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u8_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u8_be(
                        self,
                        value: u8,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u8_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u16_le(
                        self,
                        value: u16,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u16_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u16_be(
                        self,
                        value: u16,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u16_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u32_le(
                        self,
                        value: u32,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u32_be(
                        self,
                        value: u32,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u64_le(
                        self,
                        value: u64,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u64_be(
                        self,
                        value: u64,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u128_le(
                        self,
                        value: u128,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u128_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u128_be(
                        self,
                        value: u128,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u128_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_usize_le(
                        self,
                        value: usize,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_usize_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_usize_be(
                        self,
                        value: usize,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_usize_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i8_le(
                        self,
                        value: i8,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i8_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i8_be(
                        self,
                        value: i8,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i8_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i16_le(
                        self,
                        value: i16,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i16_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i16_be(
                        self,
                        value: i16,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i16_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i32_le(
                        self,
                        value: i32,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i32_be(
                        self,
                        value: i32,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i64_le(
                        self,
                        value: i64,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i64_be(
                        self,
                        value: i64,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i128_le(
                        self,
                        value: i128,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i128_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i128_be(
                        self,
                        value: i128,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i128_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_isize_le(
                        self,
                        value: isize,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_isize_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_isize_be(
                        self,
                        value: isize,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_isize_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f32_le(
                        self,
                        value: f32,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f32_be(
                        self,
                        value: f32,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f64_le(
                        self,
                        value: f64,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f64_be(
                        self,
                        value: f64,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, SendPayload<Building, A>>,
                        A,
                    > {
                        let payload1: DynamicPayload<P, E, R> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                }
                impl<
                    P1: PayloadAction,
                    A,
                > PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                    pub fn push<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_line<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_line(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_line_crlf<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_line_crlf(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn fill_byte(
                        self,
                        byte: u8,
                        count: usize,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.fill_byte(byte, count);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn fill<T: AsRef<[u8]>>(
                        self,
                        data: T,
                        count: usize,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.fill(data, count);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_ansi_command<T: Command>(
                        self,
                        command: T,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_ansi_command(command);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u8_le(
                        self,
                        value: u8,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_u8_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u8_be(
                        self,
                        value: u8,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_u8_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u16_le(
                        self,
                        value: u16,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_u16_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u16_be(
                        self,
                        value: u16,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_u16_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u32_le(
                        self,
                        value: u32,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_u32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u32_be(
                        self,
                        value: u32,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_u32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u64_le(
                        self,
                        value: u64,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_u64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u64_be(
                        self,
                        value: u64,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_u64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u128_le(
                        self,
                        value: u128,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_u128_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u128_be(
                        self,
                        value: u128,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_u128_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_usize_le(
                        self,
                        value: usize,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_usize_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_usize_be(
                        self,
                        value: usize,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_usize_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i8_le(
                        self,
                        value: i8,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_i8_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i8_be(
                        self,
                        value: i8,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_i8_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i16_le(
                        self,
                        value: i16,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_i16_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i16_be(
                        self,
                        value: i16,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_i16_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i32_le(
                        self,
                        value: i32,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_i32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i32_be(
                        self,
                        value: i32,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_i32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i64_le(
                        self,
                        value: i64,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_i64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i64_be(
                        self,
                        value: i64,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_i64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i128_le(
                        self,
                        value: i128,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_i128_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i128_be(
                        self,
                        value: i128,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_i128_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_isize_le(
                        self,
                        value: isize,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_isize_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_isize_be(
                        self,
                        value: isize,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_isize_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f32_le(
                        self,
                        value: f32,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_f32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f32_be(
                        self,
                        value: f32,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_f32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f64_le(
                        self,
                        value: f64,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_f64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f64_be(
                        self,
                        value: f64,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.push_f64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn send(
                        self,
                    ) -> PayloadBuilder<Chain<P1, SendPayload<Complete, A>>, A> {
                        let payload1 = self.payload.payload1;
                        let payload2 = self.payload.payload2.complete();
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                }
                impl<
                    P1: PayloadAction,
                    A,
                > PayloadBuilder<Chain<P1, SendPayload<Complete, A>>, A> {
                    pub fn push<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new().push(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_line<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new().push_line(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_line_crlf<T: AsRef<[u8]>>(
                        self,
                        data: T,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_line_crlf(data);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn fill_byte(
                        self,
                        byte: u8,
                        count: usize,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .fill_byte(byte, count);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn fill<T: AsRef<[u8]>>(
                        self,
                        data: T,
                        count: usize,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .fill(data, count);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_ansi_command<T: Command>(
                        self,
                        command: T,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_ansi_command(command);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u8_le(
                        self,
                        value: u8,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u8_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u8_be(
                        self,
                        value: u8,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u8_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u16_le(
                        self,
                        value: u16,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u16_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u16_be(
                        self,
                        value: u16,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u16_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u32_le(
                        self,
                        value: u32,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u32_be(
                        self,
                        value: u32,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u64_le(
                        self,
                        value: u64,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u64_be(
                        self,
                        value: u64,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u128_le(
                        self,
                        value: u128,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u128_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_u128_be(
                        self,
                        value: u128,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_u128_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_usize_le(
                        self,
                        value: usize,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_usize_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_usize_be(
                        self,
                        value: usize,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_usize_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i8_le(
                        self,
                        value: i8,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i8_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i8_be(
                        self,
                        value: i8,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i8_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i16_le(
                        self,
                        value: i16,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i16_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i16_be(
                        self,
                        value: i16,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i16_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i32_le(
                        self,
                        value: i32,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i32_be(
                        self,
                        value: i32,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i64_le(
                        self,
                        value: i64,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i64_be(
                        self,
                        value: i64,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i128_le(
                        self,
                        value: i128,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i128_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_i128_be(
                        self,
                        value: i128,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_i128_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_isize_le(
                        self,
                        value: isize,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_isize_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_isize_be(
                        self,
                        value: isize,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_isize_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f32_le(
                        self,
                        value: f32,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f32_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f32_be(
                        self,
                        value: f32,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f32_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f64_le(
                        self,
                        value: f64,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f64_le(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                    pub fn push_f64_be(
                        self,
                        value: f64,
                    ) -> PayloadBuilder<
                        Chain<
                            Chain<P1, SendPayload<Complete, A>>,
                            SendPayload<Building, A>,
                        >,
                        A,
                    > {
                        let payload1: Chain<P1, SendPayload<Complete, A>> = self.payload;
                        let payload2 = SendPayload::<Building, A>::new()
                            .push_f64_be(value);
                        PayloadBuilder::from(Chain::new(payload1, payload2))
                    }
                }
            }
            mod read {
                use crate::io::payload::payloads::{
                    Bytes, Ascii, Utf8, Chain, Initial, ReadPayload, ReadPayloadType,
                    DynamicPayload, Building, SendPayload, Complete,
                };
                use crate::io::{PayloadAction, PayloadBuilder};
                use paste::paste;
                impl<A> PayloadBuilder<Initial, A> {
                    pub fn recv_until<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Bytes>>, A> {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_ascii<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Ascii>>, A> {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_utf8<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Utf8>>, A> {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Bytes>>, A> {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex_ascii(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Ascii>>, A> {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex_utf8(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Utf8>>, A> {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Bytes>>, A> {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex_ascii(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Ascii>>, A> {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex_utf8(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Utf8>>, A> {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line(
                        self,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Bytes>>, A> {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_ascii(
                        self,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Ascii>>, A> {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_utf8(
                        self,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Utf8>>, A> {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf(
                        self,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Bytes>>, A> {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf_ascii(
                        self,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Ascii>>, A> {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf_utf8(
                        self,
                    ) -> PayloadBuilder<Chain<Initial, ReadPayload<Utf8>>, A> {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                }
                impl<
                    P1: PayloadAction,
                    RP,
                    A,
                > PayloadBuilder<Chain<P1, ReadPayload<RP>>, A>
                where
                    ReadPayload<RP>: PayloadAction,
                {
                    pub fn recv_until<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_ascii<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_utf8<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex_ascii(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex_utf8(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex_ascii(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex_utf8(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_ascii(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_utf8(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf_ascii(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf_utf8(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, ReadPayload<RP>>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                }
                impl<P, E, R, A> PayloadBuilder<DynamicPayload<P, E, R>, A>
                where
                    P: PayloadAction<ReturnType = E>,
                    R: PayloadAction,
                {
                    pub fn recv_until<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_ascii<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_utf8<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex_ascii(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex_utf8(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex_ascii(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex_utf8(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line(
                        self,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_ascii(
                        self,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_utf8(
                        self,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf(
                        self,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf_ascii(
                        self,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf_utf8(
                        self,
                    ) -> PayloadBuilder<
                        Chain<DynamicPayload<P, E, R>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                }
                impl<
                    P1: PayloadAction,
                    A,
                > PayloadBuilder<Chain<P1, SendPayload<Complete, A>>, A> {
                    pub fn recv_until<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_ascii<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_utf8<T: AsRef<[u8]>>(
                        self,
                        delim: T,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(
                            ReadPayloadType::RecvUntil(delim.as_ref().to_vec(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex_ascii(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_until_regex_utf8(
                        self,
                        pattern: &str,
                        drop: bool,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(
                            ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop),
                        );
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex_ascii(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_regex_utf8(
                        self,
                        pattern: &str,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvRegex(pattern.to_string()));
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_ascii(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_utf8(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvLine());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Bytes>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Bytes,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf_ascii(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Ascii>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Ascii,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                    pub fn recv_line_crlf_utf8(
                        self,
                    ) -> PayloadBuilder<
                        Chain<Chain<P1, SendPayload<Complete, A>>, ReadPayload<Utf8>>,
                        A,
                    > {
                        let payload = ReadPayload::<
                            Utf8,
                        >::new(ReadPayloadType::RecvLineCrlf());
                        PayloadBuilder::from(Chain::new(self.payload, payload))
                    }
                }
            }
            mod dynamic_payload {
                use crate::io::payload::payloads::{
                    Chain, DynamicPayload, Initial, ReadPayload,
                };
                use crate::io::{PayloadAction, PayloadBuilder};
                impl<
                    P1: PayloadAction,
                    RP,
                    A,
                    E,
                > PayloadBuilder<Chain<P1, ReadPayload<RP>>, A>
                where
                    ReadPayload<RP>: PayloadAction<ReturnType = E>,
                {
                    pub fn payload<R>(
                        self,
                        action: fn(E) -> R,
                    ) -> PayloadBuilder<
                        DynamicPayload<Chain<P1, ReadPayload<RP>>, E, R>,
                        A,
                    >
                    where
                        R: PayloadAction,
                    {
                        PayloadBuilder::from(DynamicPayload::new(self.payload, action))
                    }
                }
                impl<
                    A,
                    POrig,
                    EOrig,
                    ROrig,
                    E,
                > PayloadBuilder<DynamicPayload<POrig, EOrig, ROrig>, A>
                where
                    ROrig: PayloadAction<ReturnType = E>,
                    POrig: PayloadAction<ReturnType = EOrig>,
                {
                    pub fn payload<R>(
                        self,
                        action: fn(E) -> R,
                    ) -> PayloadBuilder<
                        DynamicPayload<DynamicPayload<POrig, EOrig, ROrig>, E, R>,
                        A,
                    >
                    where
                        R: PayloadAction,
                    {
                        PayloadBuilder::from(DynamicPayload::new(self.payload, action))
                    }
                }
            }
            use std::marker::PhantomData;
            use crossterm::Command;
            use crate::io::PayloadAction;
            use crate::io::payload::payloads;
            use crate::io::payload::payloads::{
                Ascii, Building, Chain, Complete, DynamicPayload, Initial, ReadPayload,
                SendPayload,
            };
            pub struct PayloadBuilder<T, A> {
                payload: T,
                _phantom_arch: PhantomData<A>,
            }
            impl<A> PayloadBuilder<Initial, A> {
                pub(crate) fn new() -> PayloadBuilder<Initial, A> {
                    PayloadBuilder::<Initial, A> {
                        payload: Initial {},
                        _phantom_arch: PhantomData::default(),
                    }
                }
            }
            impl<T, A> PayloadBuilder<T, A> {
                pub(crate) fn from(payload: T) -> PayloadBuilder<T, A> {
                    PayloadBuilder::<T, A> {
                        payload,
                        _phantom_arch: PhantomData::default(),
                    }
                }
            }
            impl<A> PayloadBuilder<Initial, A> {}
            impl<P1: PayloadAction, E, A> PayloadBuilder<Chain<P1, ReadPayload<E>>, A> {
                pub fn build(self) -> Chain<P1, ReadPayload<E>> {
                    self.payload
                }
            }
            impl<
                P1: PayloadAction,
                A,
            > PayloadBuilder<Chain<P1, SendPayload<Complete, A>>, A> {
                pub fn build(self) -> Chain<P1, SendPayload<Complete, A>> {
                    self.payload
                }
            }
            impl<P, E, T, A> PayloadBuilder<DynamicPayload<P, E, T>, A> {
                pub fn build(self) -> DynamicPayload<P, E, T> {
                    self.payload
                }
            }
        }
        mod arch {}
        pub(crate) use builder::*;
        pub use payload::*;
        pub use write::*;
        use crate::io::{PipeError, PipeRead, PipeReadExt, PipeWrite};
        pub trait PayloadAction {
            type ReturnType;
            async fn execute<R: PipeRead + Unpin, W: PipeWrite + Unpin>(
                &self,
                reader: &mut R,
                writer: &mut W,
            ) -> Result<Self::ReturnType, PipeError>;
        }
    }
    mod pipe {
        pub mod ansi {
            use crossterm::Command;
            use std::fmt;
            pub struct Home;
            #[automatically_derived]
            impl ::core::fmt::Debug for Home {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Home")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Home {
                #[inline]
                fn clone(&self) -> Home {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Home {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Home {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Home {
                #[inline]
                fn eq(&self, other: &Home) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for Home {}
            #[automatically_derived]
            impl ::core::cmp::Eq for Home {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for Home {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\u{1b}1~")
                }
            }
            pub struct Insert;
            #[automatically_derived]
            impl ::core::fmt::Debug for Insert {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Insert")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Insert {
                #[inline]
                fn clone(&self) -> Insert {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Insert {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Insert {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Insert {
                #[inline]
                fn eq(&self, other: &Insert) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for Insert {}
            #[automatically_derived]
            impl ::core::cmp::Eq for Insert {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for Insert {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\u{1b}2~")
                }
            }
            pub struct Del;
            #[automatically_derived]
            impl ::core::fmt::Debug for Del {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Del")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Del {
                #[inline]
                fn clone(&self) -> Del {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Del {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Del {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Del {
                #[inline]
                fn eq(&self, other: &Del) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for Del {}
            #[automatically_derived]
            impl ::core::cmp::Eq for Del {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for Del {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\u{1b}3~")
                }
            }
            pub struct End;
            #[automatically_derived]
            impl ::core::fmt::Debug for End {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "End")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for End {
                #[inline]
                fn clone(&self) -> End {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for End {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for End {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for End {
                #[inline]
                fn eq(&self, other: &End) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for End {}
            #[automatically_derived]
            impl ::core::cmp::Eq for End {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for End {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\u{1b}4~")
                }
            }
            pub struct PgUp;
            #[automatically_derived]
            impl ::core::fmt::Debug for PgUp {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "PgUp")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for PgUp {
                #[inline]
                fn clone(&self) -> PgUp {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for PgUp {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for PgUp {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for PgUp {
                #[inline]
                fn eq(&self, other: &PgUp) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for PgUp {}
            #[automatically_derived]
            impl ::core::cmp::Eq for PgUp {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for PgUp {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\u{1b}5~")
                }
            }
            pub struct PgDn;
            #[automatically_derived]
            impl ::core::fmt::Debug for PgDn {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "PgDn")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for PgDn {
                #[inline]
                fn clone(&self) -> PgDn {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for PgDn {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for PgDn {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for PgDn {
                #[inline]
                fn eq(&self, other: &PgDn) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for PgDn {}
            #[automatically_derived]
            impl ::core::cmp::Eq for PgDn {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for PgDn {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\u{1b}6~")
                }
            }
            pub struct F(pub u8);
            #[automatically_derived]
            impl ::core::fmt::Debug for F {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F", &&self.0)
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for F {
                #[inline]
                fn clone(&self) -> F {
                    let _: ::core::clone::AssertParamIsClone<u8>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for F {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for F {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for F {
                #[inline]
                fn eq(&self, other: &F) -> bool {
                    self.0 == other.0
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for F {}
            #[automatically_derived]
            impl ::core::cmp::Eq for F {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<u8>;
                }
            }
            impl Command for F {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    let index = match self.0 {
                        0 => "10",
                        1 => "11",
                        2 => "12",
                        3 => "13",
                        4 => "14",
                        5 => "15",
                        6 => "17",
                        7 => "18",
                        8 => "19",
                        9 => "20",
                        10 => "21",
                        11 => "23",
                        12 => "24",
                        13 => "25",
                        14 => "26",
                        15 => "28",
                        16 => "29",
                        17 => "31",
                        18 => "32",
                        19 => "33",
                        20 => "34",
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!("Unsupported F key"),
                            );
                        }
                    };
                    f.write_fmt(format_args!("\u{1b}{0}~", index))
                }
            }
            pub struct Backspace;
            #[automatically_derived]
            impl ::core::fmt::Debug for Backspace {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Backspace")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Backspace {
                #[inline]
                fn clone(&self) -> Backspace {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Backspace {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Backspace {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Backspace {
                #[inline]
                fn eq(&self, other: &Backspace) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for Backspace {}
            #[automatically_derived]
            impl ::core::cmp::Eq for Backspace {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for Backspace {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\u{1b}D")?;
                    f.write_str("\u{1b}P")
                }
            }
            pub struct Enter;
            #[automatically_derived]
            impl ::core::fmt::Debug for Enter {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Enter")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Enter {
                #[inline]
                fn clone(&self) -> Enter {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Enter {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Enter {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Enter {
                #[inline]
                fn eq(&self, other: &Enter) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for Enter {}
            #[automatically_derived]
            impl ::core::cmp::Eq for Enter {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for Enter {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\n")
                }
            }
            pub struct Up;
            #[automatically_derived]
            impl ::core::fmt::Debug for Up {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Up")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Up {
                #[inline]
                fn clone(&self) -> Up {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Up {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Up {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Up {
                #[inline]
                fn eq(&self, other: &Up) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for Up {}
            #[automatically_derived]
            impl ::core::cmp::Eq for Up {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for Up {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\u{1b}OA")
                }
            }
            pub struct Down;
            #[automatically_derived]
            impl ::core::fmt::Debug for Down {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Down")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Down {
                #[inline]
                fn clone(&self) -> Down {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Down {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Down {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Down {
                #[inline]
                fn eq(&self, other: &Down) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for Down {}
            #[automatically_derived]
            impl ::core::cmp::Eq for Down {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for Down {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\u{1b}OB")
                }
            }
            pub struct Left;
            #[automatically_derived]
            impl ::core::fmt::Debug for Left {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Left")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Left {
                #[inline]
                fn clone(&self) -> Left {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Left {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Left {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Left {
                #[inline]
                fn eq(&self, other: &Left) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for Left {}
            #[automatically_derived]
            impl ::core::cmp::Eq for Left {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for Left {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\u{1b}0D")
                }
            }
            pub struct Right;
            #[automatically_derived]
            impl ::core::fmt::Debug for Right {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Right")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Right {
                #[inline]
                fn clone(&self) -> Right {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Right {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Right {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Right {
                #[inline]
                fn eq(&self, other: &Right) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for Right {}
            #[automatically_derived]
            impl ::core::cmp::Eq for Right {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for Right {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\u{1b}0C")
                }
            }
            pub struct Esc;
            #[automatically_derived]
            impl ::core::fmt::Debug for Esc {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Esc")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Esc {
                #[inline]
                fn clone(&self) -> Esc {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Esc {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Esc {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Esc {
                #[inline]
                fn eq(&self, other: &Esc) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for Esc {}
            #[automatically_derived]
            impl ::core::cmp::Eq for Esc {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl Command for Esc {
                fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
                    f.write_str("\x1b")
                }
            }
        }
        mod convert {
            mod conv_macro {}
            mod process {
                use crate::io::merge::MergedAsyncReader;
                use crate::io::Pipe;
                use std::{ffi::OsStr, process::Stdio, result};
                use thiserror::*;
                use tokio::process::*;
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
                #[allow(unused_qualifications)]
                impl std::error::Error for ProcessPipeError {
                    fn source(
                        &self,
                    ) -> ::core::option::Option<&(dyn std::error::Error + 'static)> {
                        use thiserror::__private::AsDynError as _;
                        #[allow(deprecated)]
                        match self {
                            ProcessPipeError::StdinNotSet { .. } => {
                                ::core::option::Option::None
                            }
                            ProcessPipeError::StdoutNotSet { .. } => {
                                ::core::option::Option::None
                            }
                            ProcessPipeError::StdErrNotSet { .. } => {
                                ::core::option::Option::None
                            }
                            ProcessPipeError::IoError { 0: source, .. } => {
                                ::core::option::Option::Some(source.as_dyn_error())
                            }
                        }
                    }
                }
                #[allow(unused_qualifications)]
                impl ::core::fmt::Display for ProcessPipeError {
                    fn fmt(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        #[allow(
                            unused_variables,
                            deprecated,
                            clippy::used_underscore_binding
                        )]
                        match self {
                            ProcessPipeError::StdinNotSet {} => {
                                __formatter.write_fmt(format_args!("stdin not set"))
                            }
                            ProcessPipeError::StdoutNotSet {} => {
                                __formatter.write_fmt(format_args!("stdout not set"))
                            }
                            ProcessPipeError::StdErrNotSet {} => {
                                __formatter.write_fmt(format_args!("stderr not set"))
                            }
                            ProcessPipeError::IoError(_0) => {
                                __formatter.write_fmt(format_args!("io error"))
                            }
                        }
                    }
                }
                #[allow(unused_qualifications)]
                impl ::core::convert::From<tokio::io::Error> for ProcessPipeError {
                    #[allow(deprecated)]
                    fn from(source: tokio::io::Error) -> Self {
                        ProcessPipeError::IoError {
                            0: source,
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for ProcessPipeError {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        match self {
                            ProcessPipeError::StdinNotSet => {
                                ::core::fmt::Formatter::write_str(f, "StdinNotSet")
                            }
                            ProcessPipeError::StdoutNotSet => {
                                ::core::fmt::Formatter::write_str(f, "StdoutNotSet")
                            }
                            ProcessPipeError::StdErrNotSet => {
                                ::core::fmt::Formatter::write_str(f, "StdErrNotSet")
                            }
                            ProcessPipeError::IoError(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "IoError",
                                    &__self_0,
                                )
                            }
                        }
                    }
                }
                pub type ProcessPipeResult<T> = result::Result<T, ProcessPipeError>;
                pub type ProcessPipe = Pipe<
                    MergedAsyncReader<ChildStdout, ChildStderr>,
                    ChildStdin,
                >;
                pub type StdoutPipe = Pipe<ChildStdout, ChildStdin>;
                pub type StderrPipe = Pipe<ChildStderr, ChildStdin>;
                impl ProcessPipe {
                    pub async fn from_app<S: AsRef<OsStr>>(
                        program: S,
                    ) -> ProcessPipeResult<Self> {
                        let command = Command::new(program);
                        Self::spawn_command(command)
                    }
                    pub async fn from_app_args<
                        S: AsRef<OsStr>,
                        I: IntoIterator<Item = S>,
                    >(program: S, args: I) -> ProcessPipeResult<Self> {
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
                    pub async fn from_app<S: AsRef<OsStr>>(
                        program: S,
                    ) -> ProcessPipeResult<Self> {
                        let command = Command::new(program);
                        Self::spawn_command(command)
                    }
                    pub async fn from_app_args<
                        S: AsRef<OsStr>,
                        I: IntoIterator<Item = S>,
                    >(program: S, args: I) -> ProcessPipeResult<Self> {
                        let mut command = Command::new(program);
                        let _ = command.args(args);
                        Self::spawn_command(command)
                    }
                    pub fn spawn_command(mut value: Command) -> ProcessPipeResult<Self> {
                        let process = value
                            .stdout(Stdio::piped())
                            .stdin(Stdio::piped())
                            .spawn()?;
                        let stdin = process.stdin.ok_or(ProcessPipeError::StdinNotSet)?;
                        let stdout = process
                            .stdout
                            .ok_or(ProcessPipeError::StdoutNotSet)?;
                        Ok((stdin, stdout).into())
                    }
                }
                impl StderrPipe {
                    pub async fn from_app<S: AsRef<OsStr>>(
                        program: S,
                    ) -> ProcessPipeResult<Self> {
                        let command = Command::new(program);
                        Self::spawn_command(command)
                    }
                    pub async fn from_app_args<
                        S: AsRef<OsStr>,
                        I: IntoIterator<Item = S>,
                    >(program: S, args: I) -> ProcessPipeResult<Self> {
                        let mut command = Command::new(program);
                        let _ = command.args(args);
                        Self::spawn_command(command)
                    }
                    pub fn spawn_command(mut value: Command) -> ProcessPipeResult<Self> {
                        let process = value
                            .stderr(Stdio::piped())
                            .stdin(Stdio::piped())
                            .spawn()?;
                        let stdin = process.stdin.ok_or(ProcessPipeError::StdinNotSet)?;
                        let stderr = process
                            .stderr
                            .ok_or(ProcessPipeError::StdErrNotSet)?;
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
            }
            mod stream {
                use crate::io::Pipe;
                use tokio::io::{split, ReadHalf, WriteHalf};
                use tokio::io::{AsyncRead, AsyncWrite};
                impl<T: AsyncRead + AsyncWrite> From<T>
                for Pipe<ReadHalf<T>, WriteHalf<T>> {
                    fn from(value: T) -> Self {
                        let (read_stream, write_stream) = split(value);
                        Pipe::new(read_stream, write_stream)
                    }
                }
            }
            mod tcp {
                use crate::io::Pipe;
                use tokio::{io::Result, net::{tcp::*, *}};
                pub type TcpPipe = Pipe<OwnedReadHalf, OwnedWriteHalf>;
                impl TcpPipe {
                    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpPipe> {
                        let stream = TcpStream::connect(addr).await?;
                        Ok(stream.into())
                    }
                }
                impl From<TcpStream> for TcpPipe {
                    fn from(value: TcpStream) -> Self {
                        let (read_stream, write_stream) = value.into_split();
                        Pipe::new(read_stream, write_stream)
                    }
                }
            }
            pub use conv_macro::*;
            pub use process::*;
            pub use stream::*;
            pub use tcp::*;
        }
        mod error {
            use ascii::FromAsciiError;
            use thiserror::Error;
            pub enum PipeError {
                #[error("io error {0}")]
                IOError(#[from] std::io::Error),
                #[error("ascii parse error {0}")]
                AsciiParseError(String),
                #[error("utf8 parse error {0}")]
                Utf8ParseError(#[from] std::string::FromUtf8Error),
                #[error("format error {0}")]
                FmtError(#[from] std::fmt::Error),
                #[error("regex error {0}")]
                RegexError(#[from] regex::Error),
                #[error("recv timeout")]
                Timeout(),
                #[error("unknown error")]
                Unknown,
            }
            #[allow(unused_qualifications)]
            impl std::error::Error for PipeError {
                fn source(
                    &self,
                ) -> ::core::option::Option<&(dyn std::error::Error + 'static)> {
                    use thiserror::__private::AsDynError as _;
                    #[allow(deprecated)]
                    match self {
                        PipeError::IOError { 0: source, .. } => {
                            ::core::option::Option::Some(source.as_dyn_error())
                        }
                        PipeError::AsciiParseError { .. } => ::core::option::Option::None,
                        PipeError::Utf8ParseError { 0: source, .. } => {
                            ::core::option::Option::Some(source.as_dyn_error())
                        }
                        PipeError::FmtError { 0: source, .. } => {
                            ::core::option::Option::Some(source.as_dyn_error())
                        }
                        PipeError::RegexError { 0: source, .. } => {
                            ::core::option::Option::Some(source.as_dyn_error())
                        }
                        PipeError::Timeout { .. } => ::core::option::Option::None,
                        PipeError::Unknown { .. } => ::core::option::Option::None,
                    }
                }
            }
            #[allow(unused_qualifications)]
            impl ::core::fmt::Display for PipeError {
                fn fmt(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    use thiserror::__private::AsDisplay as _;
                    #[allow(
                        unused_variables,
                        deprecated,
                        clippy::used_underscore_binding
                    )]
                    match self {
                        PipeError::IOError(_0) => {
                            __formatter
                                .write_fmt(format_args!("io error {0}", _0.as_display()))
                        }
                        PipeError::AsciiParseError(_0) => {
                            __formatter
                                .write_fmt(
                                    format_args!("ascii parse error {0}", _0.as_display()),
                                )
                        }
                        PipeError::Utf8ParseError(_0) => {
                            __formatter
                                .write_fmt(
                                    format_args!("utf8 parse error {0}", _0.as_display()),
                                )
                        }
                        PipeError::FmtError(_0) => {
                            __formatter
                                .write_fmt(
                                    format_args!("format error {0}", _0.as_display()),
                                )
                        }
                        PipeError::RegexError(_0) => {
                            __formatter
                                .write_fmt(format_args!("regex error {0}", _0.as_display()))
                        }
                        PipeError::Timeout {} => {
                            __formatter.write_fmt(format_args!("recv timeout"))
                        }
                        PipeError::Unknown {} => {
                            __formatter.write_fmt(format_args!("unknown error"))
                        }
                    }
                }
            }
            #[allow(unused_qualifications)]
            impl ::core::convert::From<std::io::Error> for PipeError {
                #[allow(deprecated)]
                fn from(source: std::io::Error) -> Self {
                    PipeError::IOError { 0: source }
                }
            }
            #[allow(unused_qualifications)]
            impl ::core::convert::From<std::string::FromUtf8Error> for PipeError {
                #[allow(deprecated)]
                fn from(source: std::string::FromUtf8Error) -> Self {
                    PipeError::Utf8ParseError {
                        0: source,
                    }
                }
            }
            #[allow(unused_qualifications)]
            impl ::core::convert::From<std::fmt::Error> for PipeError {
                #[allow(deprecated)]
                fn from(source: std::fmt::Error) -> Self {
                    PipeError::FmtError { 0: source }
                }
            }
            #[allow(unused_qualifications)]
            impl ::core::convert::From<regex::Error> for PipeError {
                #[allow(deprecated)]
                fn from(source: regex::Error) -> Self {
                    PipeError::RegexError { 0: source }
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for PipeError {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        PipeError::IOError(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "IOError",
                                &__self_0,
                            )
                        }
                        PipeError::AsciiParseError(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "AsciiParseError",
                                &__self_0,
                            )
                        }
                        PipeError::Utf8ParseError(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Utf8ParseError",
                                &__self_0,
                            )
                        }
                        PipeError::FmtError(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "FmtError",
                                &__self_0,
                            )
                        }
                        PipeError::RegexError(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "RegexError",
                                &__self_0,
                            )
                        }
                        PipeError::Timeout() => {
                            ::core::fmt::Formatter::write_str(f, "Timeout")
                        }
                        PipeError::Unknown => {
                            ::core::fmt::Formatter::write_str(f, "Unknown")
                        }
                    }
                }
            }
            impl<T> From<FromAsciiError<T>> for PipeError {
                fn from(value: FromAsciiError<T>) -> Self {
                    PipeError::AsciiParseError({
                        let res = ::alloc::fmt::format(format_args!("{0}", value));
                        res
                    })
                }
            }
        }
        mod interactive {
            use crate::io::{
                NcursesTerminalBridge, PipeError, ShellTerminalBridge, TerminalBridge,
            };
            use tokio::io::{split, AsyncRead, AsyncWrite};
            impl<T: AsyncRead + AsyncWrite + Unpin + Send + ?Sized> PipeInteractiveExt
            for T {}
            pub trait PipeInteractiveExt: AsyncRead + AsyncWrite + Unpin + Send {
                async fn interactive_shell(&mut self) -> Result<(), PipeError> {
                    let (mut reader, mut writer) = split(self);
                    ShellTerminalBridge::bridge(&mut reader, &mut writer).await;
                    Ok(())
                }
                async fn interactive_ansi(&mut self) -> Result<(), PipeError> {
                    let (mut reader, mut writer) = split(self);
                    NcursesTerminalBridge::bridge(&mut reader, &mut writer).await;
                    Ok(())
                }
            }
        }
        mod read {
            use std::time::Duration;
            use crate::io::{
                AsyncCacheRead, AsyncReadCacheTimeoutExt, AsyncReadTimeoutExt, PipeError,
            };
            use ascii::AsciiString;
            use tokio::io::AsyncRead;
            pub trait PipeRead: AsyncRead + AsyncCacheRead {
                fn get_timeout(&self) -> Duration;
                fn set_timeout(&mut self, timeout: Duration);
                fn get_block_size(&self) -> usize;
                fn set_block_size(&mut self, block_size: usize);
            }
            impl<R: PipeRead> PipeReadExt for R {}
            pub trait PipeReadExt: PipeRead {
                async fn recv(&mut self) -> Result<Vec<u8>, PipeError>
                where
                    Self: Unpin,
                {
                    let mut data = ::alloc::vec::from_elem(0u8, self.get_block_size());
                    let _ = self
                        .read_exact_timeout(&mut data, self.get_timeout())
                        .await?;
                    Ok(data)
                }
                async fn recv_until<T: AsRef<[u8]>>(
                    &mut self,
                    delim: T,
                    drop: bool,
                ) -> Result<Vec<u8>, PipeError>
                where
                    Self: Unpin,
                {
                    let mut buf = Vec::new();
                    let delim_len = delim.as_ref().len();
                    self.read_until_timeout(delim, &mut buf, self.get_timeout()).await?;
                    if drop {
                        buf.drain(buf.len() - delim_len..);
                    }
                    Ok(buf)
                }
                async fn recv_until_regex(
                    &mut self,
                    pattern: &str,
                    drop: bool,
                ) -> Result<Vec<u8>, PipeError>
                where
                    Self: Unpin,
                {
                    let mut buf = Vec::new();
                    let (_, match_len) = self
                        .read_until_regex_timeout(pattern, &mut buf, self.get_timeout())?
                        .await?;
                    if drop {
                        buf.drain(buf.len() - match_len..);
                    }
                    Ok(buf)
                }
                async fn recv_regex(
                    &mut self,
                    pattern: &str,
                ) -> Result<Vec<u8>, PipeError>
                where
                    Self: Unpin,
                {
                    let mut buf = Vec::new();
                    let (_, match_len) = self
                        .read_until_regex_timeout(pattern, &mut buf, self.get_timeout())?
                        .await?;
                    buf.drain(..buf.len() - match_len);
                    Ok(buf)
                }
                async fn recv_until_regex_split(
                    &mut self,
                    pattern: &str,
                ) -> Result<(Vec<u8>, Vec<u8>), PipeError>
                where
                    Self: Unpin,
                {
                    let mut buf = Vec::new();
                    let (_, match_len) = self
                        .read_until_regex_timeout(pattern, &mut buf, self.get_timeout())?
                        .await?;
                    let (data, mch) = buf.split_at(buf.len() - match_len);
                    Ok((data.to_vec(), mch.to_vec()))
                }
                async fn recv_line(&mut self) -> Result<Vec<u8>, PipeError>
                where
                    Self: Unpin,
                {
                    self.recv_until(b"\n", true).await
                }
                async fn recv_line_crlf(&mut self) -> Result<Vec<u8>, PipeError>
                where
                    Self: Unpin,
                {
                    self.recv_until(b"\r\n", true).await
                }
                async fn recv_utf8(&mut self) -> Result<String, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv().await?;
                    Ok(String::from_utf8(data)?)
                }
                async fn recv_until_utf8<T: AsRef<[u8]>>(
                    &mut self,
                    delim: T,
                    drop: bool,
                ) -> Result<String, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv_until(delim, drop).await?;
                    Ok(String::from_utf8(data)?)
                }
                async fn recv_until_regex_utf8(
                    &mut self,
                    pattern: &str,
                    drop: bool,
                ) -> Result<String, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv_until_regex(pattern, drop).await?;
                    Ok(String::from_utf8(data)?)
                }
                async fn recv_regex_utf8(
                    &mut self,
                    pattern: &str,
                ) -> Result<String, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv_regex(pattern).await?;
                    Ok(String::from_utf8(data)?)
                }
                async fn recv_line_utf8(&mut self) -> Result<String, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv_line().await?;
                    Ok(String::from_utf8(data)?)
                }
                async fn recv_line_crlf_utf8(&mut self) -> Result<String, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv_line_crlf().await?;
                    Ok(String::from_utf8(data)?)
                }
                async fn recv_ascii(&mut self) -> Result<AsciiString, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv().await?;
                    Ok(AsciiString::from_ascii(data)?)
                }
                async fn recv_until_ascii<T: AsRef<[u8]>>(
                    &mut self,
                    delim: T,
                    drop: bool,
                ) -> Result<AsciiString, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv_until(delim, drop).await?;
                    Ok(AsciiString::from_ascii(data)?)
                }
                async fn recv_until_regex_ascii(
                    &mut self,
                    pattern: &str,
                    drop: bool,
                ) -> Result<AsciiString, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv_until_regex(pattern, drop).await?;
                    Ok(AsciiString::from_ascii(data)?)
                }
                async fn recv_regex_ascii(
                    &mut self,
                    pattern: &str,
                ) -> Result<AsciiString, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv_regex(pattern).await?;
                    Ok(AsciiString::from_ascii(data)?)
                }
                async fn recv_line_ascii(&mut self) -> Result<AsciiString, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv_line().await?;
                    Ok(AsciiString::from_ascii(data)?)
                }
                async fn recv_line_crlf_ascii(
                    &mut self,
                ) -> Result<AsciiString, PipeError>
                where
                    Self: Unpin,
                {
                    let data = self.recv_line_crlf().await?;
                    Ok(AsciiString::from_ascii(data)?)
                }
            }
        }
        mod readwrite {}
        mod write {
            use std::time::Duration;
            use crate::io::{AsyncCacheRead, PipeError, PipeRead, PipeReadExt};
            use crossterm::Command;
            use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
            pub trait PipeWrite: AsyncWrite {}
            impl<W: AsyncWrite> PipeWriteExt for W {}
            pub trait PipeWriteExt: AsyncWrite {
                async fn write_line<T: AsRef<[u8]>>(
                    &mut self,
                    text: T,
                ) -> Result<usize, PipeError>
                where
                    Self: Unpin,
                {
                    let mut res = text.as_ref().to_vec();
                    res.push(b'\n');
                    let size = self.write(&res).await?;
                    self.flush().await?;
                    Ok(size)
                }
                async fn write_line_crlf<T: AsRef<[u8]>>(
                    &mut self,
                    text: T,
                ) -> Result<usize, PipeError>
                where
                    Self: Unpin,
                {
                    let mut res = text.as_ref().to_vec();
                    res.push(b'\r');
                    res.push(b'\n');
                    let size = self.write(&res).await?;
                    self.flush().await?;
                    Ok(size)
                }
                async fn write_flush<T: AsRef<[u8]>>(
                    &mut self,
                    data: T,
                ) -> Result<usize, PipeError>
                where
                    Self: Unpin,
                {
                    let size = self.write(data.as_ref()).await?;
                    self.flush().await?;
                    Ok(size)
                }
                async fn write_all_flush<T: AsRef<[u8]>>(
                    &mut self,
                    data: T,
                ) -> Result<(), PipeError>
                where
                    Self: Unpin,
                {
                    self.write_all(data.as_ref()).await?;
                    self.flush().await?;
                    Ok(())
                }
                async fn write_ansi_command<T: Command>(
                    &mut self,
                    command: T,
                ) -> Result<usize, PipeError>
                where
                    Self: Unpin,
                {
                    let mut ansi_command = String::new();
                    command.write_ansi(&mut ansi_command)?;
                    let size = self.write(ansi_command.as_bytes()).await?;
                    self.flush().await?;
                    Ok(size)
                }
            }
        }
        pub use convert::*;
        pub use error::*;
        pub use interactive::*;
        pub use read::*;
        pub use readwrite::*;
        use std::io::Error;
        use std::pin::Pin;
        use std::task::{Context, Poll};
        pub use write::*;
        use pin_project_lite::pin_project;
        use std::time::Duration;
        use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
        use super::cache::*;
        /// An `AsyncRead`er which applies a timeout to read operations.
        pub struct Pipe<R, W> {
            reader: CacheReader<R>,
            writer: W,
            timeout: Duration,
            block_size: usize,
        }
        #[automatically_derived]
        impl<R: ::core::fmt::Debug, W: ::core::fmt::Debug> ::core::fmt::Debug
        for Pipe<R, W> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Pipe",
                    "reader",
                    &self.reader,
                    "writer",
                    &self.writer,
                    "timeout",
                    &self.timeout,
                    "block_size",
                    &&self.block_size,
                )
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[doc(hidden)]
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R, W>
            where
                Pipe<R, W>: '__pin,
            {
                reader: ::pin_project_lite::__private::Pin<&'__pin mut (CacheReader<R>)>,
                writer: ::pin_project_lite::__private::Pin<&'__pin mut (W)>,
                timeout: &'__pin mut (Duration),
                block_size: &'__pin mut (usize),
            }
            #[doc(hidden)]
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<'__pin, R, W>
            where
                Pipe<R, W>: '__pin,
            {
                reader: ::pin_project_lite::__private::Pin<&'__pin (CacheReader<R>)>,
                writer: ::pin_project_lite::__private::Pin<&'__pin (W)>,
                timeout: &'__pin (Duration),
                block_size: &'__pin (usize),
            }
            impl<R, W> Pipe<R, W> {
                #[doc(hidden)]
                #[inline]
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R, W> {
                    unsafe {
                        let Self { reader, writer, timeout, block_size } = self
                            .get_unchecked_mut();
                        Projection {
                            reader: ::pin_project_lite::__private::Pin::new_unchecked(
                                reader,
                            ),
                            writer: ::pin_project_lite::__private::Pin::new_unchecked(
                                writer,
                            ),
                            timeout: timeout,
                            block_size: block_size,
                        }
                    }
                }
                #[doc(hidden)]
                #[inline]
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R, W> {
                    unsafe {
                        let Self { reader, writer, timeout, block_size } = self
                            .get_ref();
                        ProjectionRef {
                            reader: ::pin_project_lite::__private::Pin::new_unchecked(
                                reader,
                            ),
                            writer: ::pin_project_lite::__private::Pin::new_unchecked(
                                writer,
                            ),
                            timeout: timeout,
                            block_size: block_size,
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R, W> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                reader: CacheReader<R>,
                writer: W,
                timeout: ::pin_project_lite::__private::AlwaysUnpin<Duration>,
                block_size: ::pin_project_lite::__private::AlwaysUnpin<usize>,
            }
            impl<'__pin, R, W> ::pin_project_lite::__private::Unpin for Pipe<R, W>
            where
                __Origin<'__pin, R, W>: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R, W> MustNotImplDrop for Pipe<R, W> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R, W>(this: &Pipe<R, W>) {
                let _ = &this.reader;
                let _ = &this.writer;
                let _ = &this.timeout;
                let _ = &this.block_size;
            }
        };
        impl<R: AsyncRead, W> PipeRead for Pipe<R, W> {
            fn get_timeout(&self) -> Duration {
                self.timeout
            }
            fn set_timeout(&mut self, timeout: Duration) {
                self.timeout = timeout;
            }
            fn get_block_size(&self) -> usize {
                self.block_size
            }
            fn set_block_size(&mut self, block_size: usize) {
                self.block_size = block_size;
            }
        }
        impl<R: AsyncRead, W> AsyncCacheRead for Pipe<R, W> {
            fn poll_reader(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                buf: &mut ReadBuf<'_>,
            ) -> Poll<std::io::Result<()>> {
                self.project().reader.poll_reader(cx, buf)
            }
            fn consume(self: Pin<&mut Self>, amt: usize) {
                self.project().reader.consume(amt)
            }
            fn restore(self: Pin<&mut Self>, data: &[u8]) {
                self.project().reader.restore(data)
            }
        }
        impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> Pipe<R, W> {
            const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);
            const DEFAULT_BLOCK_SIZE: usize = 4096;
            pub fn new(reader: R, writer: W) -> Pipe<R, W> {
                Pipe {
                    reader: CacheReader::new(reader),
                    writer: writer,
                    block_size: Self::DEFAULT_BLOCK_SIZE,
                    timeout: Self::DEFAULT_TIMEOUT,
                }
            }
        }
        impl<R: AsyncRead, W> AsyncRead for Pipe<R, W> {
            fn poll_read(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                buf: &mut ReadBuf<'_>,
            ) -> Poll<std::io::Result<()>> {
                let this = self.project();
                this.reader.poll_read(cx, buf)
            }
        }
        impl<R, W: AsyncWrite> AsyncWrite for Pipe<R, W> {
            fn poll_write(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                buf: &[u8],
            ) -> Poll<std::result::Result<usize, Error>> {
                let this = self.project();
                this.writer.poll_write(cx, buf)
            }
            fn poll_flush(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<std::result::Result<(), Error>> {
                let this = self.project();
                this.writer.poll_flush(cx)
            }
            fn poll_shutdown(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<std::result::Result<(), Error>> {
                let this = self.project();
                this.writer.poll_shutdown(cx)
            }
        }
    }
    pub mod cache {
        mod read {
            use crate::io::cache::CacheReader;
            use crate::io::AsyncCacheRead;
            use std::borrow::BorrowMut;
            use std::pin::Pin;
            use std::task::{Context, Poll};
            use tokio::io;
            use tokio::io::{AsyncRead, ReadBuf};
            impl<R: AsyncRead> AsyncRead for CacheReader<R> {
                fn poll_read(
                    self: Pin<&mut Self>,
                    cx: &mut Context<'_>,
                    buf: &mut ReadBuf<'_>,
                ) -> Poll<Result<(), io::Error>> {
                    let mut this = self.project();
                    if !this.cache.is_empty() {
                        let remaining = usize::min(buf.remaining(), this.cache.len());
                        buf.put_slice(&this.cache[..remaining]);
                        this.cache.drain(..remaining);
                        return Poll::Ready(Ok(()));
                    }
                    this.reader.poll_read(cx, buf)
                }
            }
            impl<R: AsyncRead> AsyncCacheRead for CacheReader<R> {
                fn poll_reader(
                    self: Pin<&mut Self>,
                    cx: &mut Context<'_>,
                    buf: &mut ReadBuf<'_>,
                ) -> Poll<std::io::Result<()>> {
                    self.project().reader.poll_read(cx, buf)
                }
                fn consume(self: Pin<&mut Self>, amt: usize) {
                    self.project().borrow_mut().cache.drain(..amt);
                }
                fn restore(self: Pin<&mut Self>, data: &[u8]) {
                    self.project().borrow_mut().cache.extend_from_slice(data)
                }
            }
        }
        mod traits {
            use std::io;
            use std::pin::Pin;
            use std::task::{Context, Poll};
            pub use tokio::io::AsyncBufRead;
            use tokio::io::{AsyncRead, ReadBuf};
            pub trait AsyncCacheRead: AsyncRead {
                fn poll_reader(
                    self: Pin<&mut Self>,
                    cx: &mut Context<'_>,
                    buf: &mut ReadBuf<'_>,
                ) -> Poll<io::Result<()>>;
                fn consume(self: Pin<&mut Self>, amt: usize);
                fn restore(self: Pin<&mut Self>, data: &[u8]);
            }
        }
        pub use traits::*;
        use pin_project_lite::pin_project;
        /// An `AsyncRead`er which applies a timeout to read operations.
        pub struct CacheReader<R> {
            pub(crate) reader: R,
            pub(crate) cache: Vec<u8>,
        }
        #[automatically_derived]
        impl<R: ::core::fmt::Debug> ::core::fmt::Debug for CacheReader<R> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CacheReader",
                    "reader",
                    &self.reader,
                    "cache",
                    &&self.cache,
                )
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[doc(hidden)]
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R>
            where
                CacheReader<R>: '__pin,
            {
                pub(crate) reader: ::pin_project_lite::__private::Pin<&'__pin mut (R)>,
                pub(crate) cache: ::pin_project_lite::__private::Pin<
                    &'__pin mut (Vec<u8>),
                >,
            }
            #[doc(hidden)]
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<'__pin, R>
            where
                CacheReader<R>: '__pin,
            {
                pub(crate) reader: ::pin_project_lite::__private::Pin<&'__pin (R)>,
                pub(crate) cache: ::pin_project_lite::__private::Pin<&'__pin (Vec<u8>)>,
            }
            impl<R> CacheReader<R> {
                #[doc(hidden)]
                #[inline]
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R> {
                    unsafe {
                        let Self { reader, cache } = self.get_unchecked_mut();
                        Projection {
                            reader: ::pin_project_lite::__private::Pin::new_unchecked(
                                reader,
                            ),
                            cache: ::pin_project_lite::__private::Pin::new_unchecked(
                                cache,
                            ),
                        }
                    }
                }
                #[doc(hidden)]
                #[inline]
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R> {
                    unsafe {
                        let Self { reader, cache } = self.get_ref();
                        ProjectionRef {
                            reader: ::pin_project_lite::__private::Pin::new_unchecked(
                                reader,
                            ),
                            cache: ::pin_project_lite::__private::Pin::new_unchecked(
                                cache,
                            ),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                reader: R,
                cache: Vec<u8>,
            }
            impl<'__pin, R> ::pin_project_lite::__private::Unpin for CacheReader<R>
            where
                __Origin<'__pin, R>: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R> MustNotImplDrop for CacheReader<R> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R>(this: &CacheReader<R>) {
                let _ = &this.reader;
                let _ = &this.cache;
            }
        };
        impl<R> CacheReader<R> {
            pub fn new(reader: R) -> CacheReader<R> {
                CacheReader {
                    reader,
                    cache: Vec::new(),
                }
            }
        }
    }
    mod merge {
        use pin_project_lite::pin_project;
        use std::task::Poll;
        use tokio::io::*;
        pub struct MergedAsyncReader<R1, R2> {
            pub stream0: R1,
            pub stream1: R2,
        }
        #[automatically_derived]
        impl<R1: ::core::fmt::Debug, R2: ::core::fmt::Debug> ::core::fmt::Debug
        for MergedAsyncReader<R1, R2> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "MergedAsyncReader",
                    "stream0",
                    &self.stream0,
                    "stream1",
                    &&self.stream1,
                )
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[doc(hidden)]
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R1, R2>
            where
                MergedAsyncReader<R1, R2>: '__pin,
            {
                pub stream0: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                pub stream1: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
            }
            #[doc(hidden)]
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<'__pin, R1, R2>
            where
                MergedAsyncReader<R1, R2>: '__pin,
            {
                pub stream0: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                pub stream1: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
            }
            impl<R1, R2> MergedAsyncReader<R1, R2> {
                #[doc(hidden)]
                #[inline]
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R1, R2> {
                    unsafe {
                        let Self { stream0, stream1 } = self.get_unchecked_mut();
                        Projection {
                            stream0: ::pin_project_lite::__private::Pin::new_unchecked(
                                stream0,
                            ),
                            stream1: ::pin_project_lite::__private::Pin::new_unchecked(
                                stream1,
                            ),
                        }
                    }
                }
                #[doc(hidden)]
                #[inline]
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R1, R2> {
                    unsafe {
                        let Self { stream0, stream1 } = self.get_ref();
                        ProjectionRef {
                            stream0: ::pin_project_lite::__private::Pin::new_unchecked(
                                stream0,
                            ),
                            stream1: ::pin_project_lite::__private::Pin::new_unchecked(
                                stream1,
                            ),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R1, R2> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                stream0: R1,
                stream1: R2,
            }
            impl<'__pin, R1, R2> ::pin_project_lite::__private::Unpin
            for MergedAsyncReader<R1, R2>
            where
                __Origin<'__pin, R1, R2>: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R1, R2> MustNotImplDrop for MergedAsyncReader<R1, R2> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R1, R2>(this: &MergedAsyncReader<R1, R2>) {
                let _ = &this.stream0;
                let _ = &this.stream1;
            }
        };
        impl<R1, R2> MergedAsyncReader<R1, R2> {
            pub fn new(stream0: R1, stream1: R2) -> MergedAsyncReader<R1, R2> {
                MergedAsyncReader {
                    stream0,
                    stream1,
                }
            }
        }
        impl<R1: AsyncRead + Unpin, R2: AsyncRead + Unpin> AsyncRead
        for MergedAsyncReader<R1, R2> {
            fn poll_read(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &mut ReadBuf<'_>,
            ) -> Poll<Result<()>> {
                let me = self.project();
                match me.stream0.poll_read(cx, buf) {
                    Poll::Ready(r) => Poll::Ready(r),
                    Poll::Pending => me.stream1.poll_read(cx, buf),
                }
            }
        }
    }
    pub mod stdio {
        mod ncurses_bridge {
            use crate::io::stdio::{
                is_stop_terminal, is_terminate_process, TerminalBridge, TerminalResult,
            };
            use crate::io::{ansi, TerminalError};
            use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
            use crossterm::*;
            use std::io::ErrorKind::TimedOut;
            use std::sync::atomic::{AtomicBool, Ordering};
            use std::sync::Arc;
            use std::time::Duration;
            use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
            use tokio::join;
            pub struct NcursesTerminalBridge {}
            async fn read_task<R>(
                reader: &mut R,
                stop_signal: Arc<AtomicBool>,
            ) -> TerminalResult<()>
            where
                R: AsyncRead + Send + Unpin,
            {
                let mut buffer = [0; 1024];
                loop {
                    if stop_signal.load(Ordering::SeqCst) {
                        break;
                    }
                    let n = match reader.read(&mut buffer).await {
                        Ok(n) if n == 0 => break,
                        Ok(n) => n,
                        Err(e) if e.kind() == TimedOut => continue,
                        Err(e) => return Err(e.into()),
                    };
                    let mut stdout = tokio::io::stdout();
                    terminal::disable_raw_mode().unwrap();
                    stdout.write_all(&buffer[..n]).await?;
                    stdout.flush().await?;
                    terminal::enable_raw_mode().unwrap();
                }
                Ok(())
            }
            async fn write_ansi_command<W: AsyncWrite + Send + Unpin, T: Command>(
                writer: &mut W,
                command: T,
            ) -> Result<usize, TerminalError> {
                let mut ansi_command = String::new();
                command.write_ansi(&mut ansi_command)?;
                let size = writer.write(ansi_command.as_bytes()).await?;
                Ok(size)
            }
            async fn send_key<W>(
                writer: &mut W,
                stop_signal: Arc<AtomicBool>,
                key_event: KeyEvent,
            ) -> TerminalResult<()>
            where
                W: AsyncWrite + Send + Unpin,
            {
                if is_terminate_process(key_event) {
                    std::process::exit(130);
                }
                if is_stop_terminal(key_event) {
                    stop_signal.store(true, Ordering::SeqCst);
                    return Ok(());
                }
                if key_event.kind != KeyEventKind::Press
                    && key_event.kind != KeyEventKind::Repeat
                {
                    return Ok(());
                }
                match key_event.code {
                    KeyCode::Char(c) => {
                        let mut buf = [0; 4];
                        let bytes = c.encode_utf8(&mut buf);
                        writer.write_all(bytes.as_bytes()).await?;
                    }
                    KeyCode::Left => {
                        write_ansi_command(writer, ansi::Left).await?;
                    }
                    KeyCode::Right => {
                        write_ansi_command(writer, ansi::Right).await?;
                    }
                    KeyCode::Up => {
                        write_ansi_command(writer, ansi::Up).await?;
                    }
                    KeyCode::Down => {
                        write_ansi_command(writer, ansi::Down).await?;
                    }
                    KeyCode::Delete => {
                        write_ansi_command(writer, ansi::Del).await?;
                    }
                    KeyCode::Backspace => {
                        write_ansi_command(writer, ansi::Backspace).await?;
                    }
                    KeyCode::Esc => {
                        write_ansi_command(writer, ansi::Esc).await?;
                    }
                    KeyCode::Enter => {
                        write_ansi_command(writer, ansi::Enter).await?;
                    }
                    KeyCode::Home => {
                        write_ansi_command(writer, ansi::Home).await?;
                    }
                    KeyCode::End => {
                        write_ansi_command(writer, ansi::End).await?;
                    }
                    KeyCode::PageUp => {
                        write_ansi_command(writer, ansi::PgUp).await?;
                    }
                    KeyCode::PageDown => {
                        write_ansi_command(writer, ansi::PgDn).await?;
                    }
                    KeyCode::F(no) => {
                        write_ansi_command(writer, ansi::F(no)).await?;
                    }
                    _ => {}
                };
                writer.flush().await?;
                Ok(())
            }
            async fn write_task<W>(
                writer: &mut W,
                stop_signal: Arc<AtomicBool>,
            ) -> TerminalResult<()>
            where
                W: AsyncWrite + Send + Unpin,
            {
                loop {
                    if stop_signal.load(Ordering::SeqCst) {
                        return Ok(());
                    }
                    if let Ok(true) = event::poll(Duration::from_millis(0)) {
                        match event::read() {
                            Ok(event::Event::Key(key_event)) => {
                                send_key(writer, stop_signal.clone(), key_event).await?;
                            }
                            _ => {}
                        }
                    }
                }
            }
            impl TerminalBridge for NcursesTerminalBridge {
                async fn bridge<
                    R: AsyncRead + Send + Unpin,
                    W: AsyncWrite + Send + Unpin,
                >(reader: &mut R, writer: &mut W) {
                    let reader_ptr = reader as *mut R as usize;
                    let writer_ptr = writer as *mut W as usize;
                    terminal::enable_raw_mode().unwrap();
                    let stop_signal = Arc::new(AtomicBool::new(false));
                    let read_stop_signal = stop_signal.clone();
                    let reader_task = tokio::spawn(async move {
                        let reader_ptr = reader_ptr as *mut R;
                        let reader = unsafe { &mut *reader_ptr };
                        let _ = read_task(reader, read_stop_signal.clone()).await;
                        read_stop_signal.store(true, Ordering::SeqCst);
                    });
                    let writer_task = tokio::spawn(async move {
                        let writer_ptr = writer_ptr as *mut W;
                        let writer = unsafe { &mut *writer_ptr };
                        let _ = write_task(writer, stop_signal.clone()).await;
                        stop_signal.store(true, Ordering::SeqCst);
                    });
                    let _ = {
                        use ::tokio::macros::support::{maybe_done, poll_fn, Future, Pin};
                        use ::tokio::macros::support::Poll::{Ready, Pending};
                        let mut futures = (
                            maybe_done(reader_task),
                            maybe_done(writer_task),
                        );
                        let mut futures = &mut futures;
                        let mut skip_next_time: u32 = 0;
                        poll_fn(move |cx| {
                                const COUNT: u32 = 0 + 1 + 1;
                                let mut is_pending = false;
                                let mut to_run = COUNT;
                                let mut skip = skip_next_time;
                                skip_next_time = if skip + 1 == COUNT {
                                    0
                                } else {
                                    skip + 1
                                };
                                loop {
                                    if skip == 0 {
                                        if to_run == 0 {
                                            break;
                                        }
                                        to_run -= 1;
                                        let (fut, ..) = &mut *futures;
                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                        if fut.poll(cx).is_pending() {
                                            is_pending = true;
                                        }
                                    } else {
                                        skip -= 1;
                                    }
                                    if skip == 0 {
                                        if to_run == 0 {
                                            break;
                                        }
                                        to_run -= 1;
                                        let (_, fut, ..) = &mut *futures;
                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                        if fut.poll(cx).is_pending() {
                                            is_pending = true;
                                        }
                                    } else {
                                        skip -= 1;
                                    }
                                }
                                if is_pending {
                                    Pending
                                } else {
                                    Ready((
                                        {
                                            let (fut, ..) = &mut futures;
                                            let mut fut = unsafe { Pin::new_unchecked(fut) };
                                            fut.take_output().expect("expected completed future")
                                        },
                                        {
                                            let (_, fut, ..) = &mut futures;
                                            let mut fut = unsafe { Pin::new_unchecked(fut) };
                                            fut.take_output().expect("expected completed future")
                                        },
                                    ))
                                }
                            })
                            .await
                    };
                    terminal::disable_raw_mode().unwrap();
                }
            }
        }
        mod shell_bridge {
            use crate::io::stdio::{
                is_stop_terminal, is_terminate_process, TerminalBridge, TerminalResult,
            };
            use std::io::stdout;
            use std::io::ErrorKind::TimedOut;
            use std::io::Write;
            use std::sync::atomic::{AtomicBool, Ordering};
            use std::sync::Arc;
            use std::time::Duration;
            use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
            use crossterm::cursor::{DisableBlinking, EnableBlinking, MoveTo};
            use crossterm::event::{
                DisableBracketedPaste, EnableBracketedPaste, KeyCode, KeyEvent,
                KeyEventKind,
            };
            use crossterm::style::Print;
            use crossterm::terminal::{Clear, ClearType};
            use crossterm::*;
            use tokio::join;
            use tokio::sync::mpsc::error::TryRecvError;
            use tokio::sync::mpsc::{channel, Receiver, Sender};
            pub struct ShellTerminalBridge {}
            struct StdoutState<'a, W: AsyncWrite + Unpin> {
                text: String,
                start_position: (u16, u16),
                cursor_position: (u16, u16),
                current_dimensions: (u16, u16),
                writer: &'a mut W,
                stop_signal: Arc<AtomicBool>,
            }
            impl<'a, W: AsyncWrite + Unpin> StdoutState<'a, W> {
                pub fn new(
                    writer: &mut W,
                    stop_signal: Arc<AtomicBool>,
                ) -> TerminalResult<StdoutState<W>> {
                    let cursor_position = cursor::position()?;
                    let current_dimensions = terminal::size()?;
                    Ok(StdoutState {
                        text: String::new(),
                        start_position: cursor_position,
                        cursor_position,
                        current_dimensions,
                        writer,
                        stop_signal,
                    })
                }
                pub async fn insert(
                    &mut self,
                    key_event: KeyEvent,
                ) -> TerminalResult<()> {
                    if is_terminate_process(key_event) {
                        std::process::exit(130);
                    }
                    if is_stop_terminal(key_event) {
                        self.stop_signal.store(true, Ordering::SeqCst);
                        return Ok(());
                    }
                    if key_event.kind != KeyEventKind::Press
                        && key_event.kind != KeyEventKind::Repeat
                    {
                        return Ok(());
                    }
                    match key_event.code {
                        KeyCode::Char(c) => self.insert_char(c)?,
                        KeyCode::Left => self.decrement_cursor()?,
                        KeyCode::Right => self.increment_cursor()?,
                        KeyCode::Backspace => self.backspace()?,
                        KeyCode::Delete => self.del()?,
                        KeyCode::Enter => self.send_data().await?,
                        KeyCode::Home => self.home()?,
                        KeyCode::End => self.end()?,
                        _ => {}
                    };
                    Ok(())
                }
                fn get_cursor_relative_index(&self) -> usize {
                    let (start_x, start_y) = self.start_position;
                    let (end_x, end_y) = self.cursor_position;
                    let (w, _h) = self.current_dimensions;
                    let full_lines = if end_y > start_y {
                        end_y - start_y - 1
                    } else {
                        0
                    };
                    let last_line_chars = if end_y > start_y {
                        end_x
                    } else {
                        end_x - start_x
                    };
                    full_lines as usize * w as usize + last_line_chars as usize
                }
                fn set_cursor_relative_index(
                    &mut self,
                    index: usize,
                ) -> TerminalResult<()> {
                    if index > self.text.len() {
                        return Ok(());
                    }
                    let (start_x, start_y) = self.start_position;
                    let (w, _) = self.current_dimensions;
                    let lines_down = index / w as usize;
                    let new_y = start_y + lines_down as u16;
                    let new_x = (start_x as usize + index % w as usize) as u16;
                    let (final_x, final_y) = if new_x >= w {
                        (new_x - w, new_y + 1)
                    } else {
                        (new_x, new_y)
                    };
                    self.set_cursor_position(final_x, final_y)
                }
                fn increment_cursor(&mut self) -> TerminalResult<()> {
                    let index = self.get_cursor_relative_index();
                    if index >= self.text.len() {
                        return Ok(());
                    }
                    self.set_cursor_relative_index(index + 1)
                }
                fn decrement_cursor(&mut self) -> TerminalResult<()> {
                    let index = self.get_cursor_relative_index();
                    if index <= 0 {
                        return Ok(());
                    }
                    self.set_cursor_relative_index(index - 1)
                }
                pub fn set_cursor_position(
                    &mut self,
                    x: u16,
                    y: u16,
                ) -> TerminalResult<()> {
                    self.cursor_position = (x, y);
                    {
                        use ::std::io::Write;
                        {
                            use ::std::io::Write;
                            Ok(stdout().by_ref())
                                .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                    writer,
                                    MoveTo(x, y),
                                ))
                                .map(|_| ())
                        }
                            .and_then(|()| {
                                ::std::io::Write::flush(stdout().by_ref())
                            })
                    }?;
                    Ok(())
                }
                pub fn print(&mut self, data: &[u8]) -> TerminalResult<()> {
                    self.clear()?;
                    terminal::disable_raw_mode().unwrap();
                    stdout().write_all(data)?;
                    stdout().flush()?;
                    terminal::enable_raw_mode().unwrap();
                    self.start_position = cursor::position()?;
                    self.cursor_position = self.start_position;
                    self.redraw()?;
                    Ok(())
                }
                pub fn insert_str(&mut self, text: &str) -> TerminalResult<()> {
                    let index = self.get_cursor_relative_index();
                    self.text.insert_str(index, text);
                    self.redraw()?;
                    self.set_cursor_relative_index(index + text.len())?;
                    Ok(())
                }
                pub fn insert_char(&mut self, c: char) -> TerminalResult<()> {
                    let index = self.get_cursor_relative_index();
                    self.text.insert(index, c);
                    self.redraw()?;
                    self.set_cursor_relative_index(index + 1)?;
                    Ok(())
                }
                fn del(&mut self) -> TerminalResult<()> {
                    let index = self.get_cursor_relative_index();
                    if index >= self.text.len() {
                        return Ok(());
                    }
                    let _ = self.text.remove(index);
                    self.redraw()?;
                    self.set_cursor_relative_index(index)?;
                    Ok(())
                }
                fn backspace(&mut self) -> TerminalResult<()> {
                    let index = self.get_cursor_relative_index();
                    if index <= 0 {
                        return Ok(());
                    }
                    let _ = self.text.remove(index - 1);
                    self.redraw()?;
                    self.set_cursor_relative_index(index - 1)?;
                    Ok(())
                }
                pub fn home(&mut self) -> TerminalResult<()> {
                    self.set_cursor_relative_index(0)
                }
                pub fn end(&mut self) -> TerminalResult<()> {
                    self.set_cursor_relative_index(self.text.len())
                }
                pub async fn send_data(&mut self) -> TerminalResult<()> {
                    self.end()?;
                    self.redraw()?;
                    let (_, y) = cursor::position()?;
                    {
                        ::std::io::_print(format_args!("\n"));
                    };
                    self.set_cursor_position(0, y + 1)?;
                    self.start_position = self.cursor_position;
                    let mut text = self.text.clone();
                    self.text.clear();
                    text.push('\n');
                    self.writer.write_all(text.as_bytes()).await?;
                    self.writer.flush().await?;
                    Ok(())
                }
                fn clear(&mut self) -> TerminalResult<()> {
                    let (start_x, start_y) = self.start_position;
                    self.set_cursor_position(start_x, start_y)?;
                    {
                        use ::std::io::Write;
                        {
                            use ::std::io::Write;
                            Ok(stdout().by_ref())
                                .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                    writer,
                                    MoveTo(start_x, start_y),
                                ))
                                .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                    writer,
                                    Clear(ClearType::FromCursorDown),
                                ))
                                .map(|_| ())
                        }
                            .and_then(|()| {
                                ::std::io::Write::flush(stdout().by_ref())
                            })
                    }?;
                    Ok(())
                }
                pub fn redraw(&mut self) -> TerminalResult<()> {
                    self.clear()?;
                    let (start_x, start_y) = self.start_position;
                    {
                        use ::std::io::Write;
                        {
                            use ::std::io::Write;
                            Ok(stdout().by_ref())
                                .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                    writer,
                                    MoveTo(start_x, start_y),
                                ))
                                .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                    writer,
                                    Print(&self.text),
                                ))
                                .map(|_| ())
                        }
                            .and_then(|()| {
                                ::std::io::Write::flush(stdout().by_ref())
                            })
                    }?;
                    Ok(())
                }
            }
            async fn read_task<R>(
                reader: &mut R,
                stop_signal: Arc<AtomicBool>,
                sender: Sender<Vec<u8>>,
            ) -> TerminalResult<()>
            where
                R: AsyncRead + Send + Unpin,
            {
                let mut buffer = [0; 1024];
                loop {
                    if stop_signal.load(Ordering::SeqCst) {
                        break;
                    }
                    let n = match reader.read(&mut buffer).await {
                        Ok(n) if n == 0 => break,
                        Ok(n) => n,
                        Err(e) if e.kind() == TimedOut => continue,
                        Err(e) => return Err(e.into()),
                    };
                    sender.send(buffer[..n].to_vec()).await?
                }
                Ok(())
            }
            async fn write_task<W>(
                writer: &mut W,
                stop_signal: Arc<AtomicBool>,
                mut receiver: Receiver<Vec<u8>>,
            ) -> TerminalResult<()>
            where
                W: AsyncWrite + Send + Unpin,
            {
                let mut stdout = StdoutState::new(writer, stop_signal.clone())?;
                loop {
                    if stop_signal.load(Ordering::SeqCst) {
                        return Ok(());
                    }
                    match receiver.try_recv() {
                        Ok(data) => {
                            stdout.print(&data)?;
                        }
                        Err(TryRecvError::Empty) => {}
                        Err(TryRecvError::Disconnected) => {
                            return Err(TryRecvError::Disconnected.into());
                        }
                    }
                    if let Ok(true) = event::poll(Duration::from_millis(0)) {
                        match event::read() {
                            Ok(event::Event::Key(key_event)) => {
                                stdout.insert(key_event).await?;
                            }
                            Ok(event::Event::Resize(_width, _height)) => {}
                            Ok(event::Event::Paste(text)) => {
                                stdout.insert_str(&text)?;
                            }
                            _ => {}
                        }
                    }
                }
            }
            impl TerminalBridge for ShellTerminalBridge {
                async fn bridge<
                    R: AsyncRead + Send + Unpin,
                    W: AsyncWrite + Send + Unpin,
                >(reader: &mut R, writer: &mut W) {
                    let reader_ptr = reader as *mut R as usize;
                    let writer_ptr = writer as *mut W as usize;
                    let (rx, tx) = channel(100);
                    terminal::enable_raw_mode().unwrap();
                    let _ = {
                        use ::std::io::Write;
                        {
                            use ::std::io::Write;
                            Ok(stdout().by_ref())
                                .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                    writer,
                                    EnableBlinking,
                                ))
                                .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                    writer,
                                    EnableBracketedPaste,
                                ))
                                .map(|_| ())
                        }
                            .and_then(|()| {
                                ::std::io::Write::flush(stdout().by_ref())
                            })
                    };
                    let stop_signal = Arc::new(AtomicBool::new(false));
                    let read_stop_signal = stop_signal.clone();
                    let reader_task = tokio::spawn(async move {
                        let reader_ptr = reader_ptr as *mut R;
                        let reader = unsafe { &mut *reader_ptr };
                        let _ = read_task(reader, read_stop_signal.clone(), rx).await;
                        read_stop_signal.store(true, Ordering::SeqCst);
                    });
                    let writer_task = tokio::spawn(async move {
                        let writer_ptr = writer_ptr as *mut W;
                        let writer = unsafe { &mut *writer_ptr };
                        let _ = write_task(writer, stop_signal.clone(), tx).await;
                        stop_signal.store(true, Ordering::SeqCst);
                    });
                    let _ = {
                        use ::tokio::macros::support::{maybe_done, poll_fn, Future, Pin};
                        use ::tokio::macros::support::Poll::{Ready, Pending};
                        let mut futures = (
                            maybe_done(reader_task),
                            maybe_done(writer_task),
                        );
                        let mut futures = &mut futures;
                        let mut skip_next_time: u32 = 0;
                        poll_fn(move |cx| {
                                const COUNT: u32 = 0 + 1 + 1;
                                let mut is_pending = false;
                                let mut to_run = COUNT;
                                let mut skip = skip_next_time;
                                skip_next_time = if skip + 1 == COUNT {
                                    0
                                } else {
                                    skip + 1
                                };
                                loop {
                                    if skip == 0 {
                                        if to_run == 0 {
                                            break;
                                        }
                                        to_run -= 1;
                                        let (fut, ..) = &mut *futures;
                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                        if fut.poll(cx).is_pending() {
                                            is_pending = true;
                                        }
                                    } else {
                                        skip -= 1;
                                    }
                                    if skip == 0 {
                                        if to_run == 0 {
                                            break;
                                        }
                                        to_run -= 1;
                                        let (_, fut, ..) = &mut *futures;
                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                        if fut.poll(cx).is_pending() {
                                            is_pending = true;
                                        }
                                    } else {
                                        skip -= 1;
                                    }
                                }
                                if is_pending {
                                    Pending
                                } else {
                                    Ready((
                                        {
                                            let (fut, ..) = &mut futures;
                                            let mut fut = unsafe { Pin::new_unchecked(fut) };
                                            fut.take_output().expect("expected completed future")
                                        },
                                        {
                                            let (_, fut, ..) = &mut futures;
                                            let mut fut = unsafe { Pin::new_unchecked(fut) };
                                            fut.take_output().expect("expected completed future")
                                        },
                                    ))
                                }
                            })
                            .await
                    };
                    let _ = {
                        use ::std::io::Write;
                        {
                            use ::std::io::Write;
                            Ok(stdout().by_ref())
                                .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                    writer,
                                    DisableBlinking,
                                ))
                                .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                    writer,
                                    DisableBracketedPaste,
                                ))
                                .map(|_| ())
                        }
                            .and_then(|()| {
                                ::std::io::Write::flush(stdout().by_ref())
                            })
                    };
                    terminal::disable_raw_mode().unwrap();
                }
            }
        }
        use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
        pub use ncurses_bridge::*;
        pub use shell_bridge::*;
        use std::io;
        use thiserror::Error;
        use crate::io::pipe;
        use tokio::io::{AsyncRead, AsyncWrite};
        use tokio::sync::mpsc;
        pub trait TerminalBridge {
            async fn bridge<R: AsyncRead + Send + Unpin, W: AsyncWrite + Send + Unpin>(
                reader: &mut R,
                writer: &mut W,
            );
        }
        pub enum TerminalError {
            #[error("IO error: {0}")]
            Io(#[from] io::Error),
            #[error("String Send error: {0}")]
            StringSend(#[from] mpsc::error::SendError<String>),
            #[error("Bytes Send error: {0}")]
            BytesSend(#[from] mpsc::error::SendError<Vec<u8>>),
            #[error("Recv error: {0}")]
            Recv(#[from] mpsc::error::TryRecvError),
            #[error("Pipe error: {0}")]
            Pipe(#[from] pipe::PipeError),
            #[error("format error {0}")]
            FmtError(#[from] std::fmt::Error),
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for TerminalError {
            fn source(
                &self,
            ) -> ::core::option::Option<&(dyn std::error::Error + 'static)> {
                use thiserror::__private::AsDynError as _;
                #[allow(deprecated)]
                match self {
                    TerminalError::Io { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    TerminalError::StringSend { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    TerminalError::BytesSend { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    TerminalError::Recv { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    TerminalError::Pipe { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                    TerminalError::FmtError { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::fmt::Display for TerminalError {
            fn fmt(
                &self,
                __formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                use thiserror::__private::AsDisplay as _;
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    TerminalError::Io(_0) => {
                        __formatter
                            .write_fmt(format_args!("IO error: {0}", _0.as_display()))
                    }
                    TerminalError::StringSend(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("String Send error: {0}", _0.as_display()),
                            )
                    }
                    TerminalError::BytesSend(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("Bytes Send error: {0}", _0.as_display()),
                            )
                    }
                    TerminalError::Recv(_0) => {
                        __formatter
                            .write_fmt(format_args!("Recv error: {0}", _0.as_display()))
                    }
                    TerminalError::Pipe(_0) => {
                        __formatter
                            .write_fmt(format_args!("Pipe error: {0}", _0.as_display()))
                    }
                    TerminalError::FmtError(_0) => {
                        __formatter
                            .write_fmt(format_args!("format error {0}", _0.as_display()))
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<io::Error> for TerminalError {
            #[allow(deprecated)]
            fn from(source: io::Error) -> Self {
                TerminalError::Io { 0: source }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<mpsc::error::SendError<String>> for TerminalError {
            #[allow(deprecated)]
            fn from(source: mpsc::error::SendError<String>) -> Self {
                TerminalError::StringSend {
                    0: source,
                }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<mpsc::error::SendError<Vec<u8>>> for TerminalError {
            #[allow(deprecated)]
            fn from(source: mpsc::error::SendError<Vec<u8>>) -> Self {
                TerminalError::BytesSend {
                    0: source,
                }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<mpsc::error::TryRecvError> for TerminalError {
            #[allow(deprecated)]
            fn from(source: mpsc::error::TryRecvError) -> Self {
                TerminalError::Recv { 0: source }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<pipe::PipeError> for TerminalError {
            #[allow(deprecated)]
            fn from(source: pipe::PipeError) -> Self {
                TerminalError::Pipe { 0: source }
            }
        }
        #[allow(unused_qualifications)]
        impl ::core::convert::From<std::fmt::Error> for TerminalError {
            #[allow(deprecated)]
            fn from(source: std::fmt::Error) -> Self {
                TerminalError::FmtError {
                    0: source,
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TerminalError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    TerminalError::Io(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Io",
                            &__self_0,
                        )
                    }
                    TerminalError::StringSend(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "StringSend",
                            &__self_0,
                        )
                    }
                    TerminalError::BytesSend(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "BytesSend",
                            &__self_0,
                        )
                    }
                    TerminalError::Recv(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Recv",
                            &__self_0,
                        )
                    }
                    TerminalError::Pipe(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Pipe",
                            &__self_0,
                        )
                    }
                    TerminalError::FmtError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "FmtError",
                            &__self_0,
                        )
                    }
                }
            }
        }
        type TerminalResult<T> = Result<T, TerminalError>;
        fn is_ctr_key(key_event: KeyEvent) -> bool {
            key_event.kind == KeyEventKind::Press
                && key_event.modifiers == KeyModifiers::CONTROL
        }
        pub(crate) fn is_terminate_process(key_event: KeyEvent) -> bool {
            return is_ctr_key(key_event) && key_event.code == KeyCode::Char('c');
        }
        pub(crate) fn is_stop_terminal(key_event: KeyEvent) -> bool {
            return is_ctr_key(key_event) && key_event.code == KeyCode::Char('d');
        }
    }
    mod util {
        mod cache {
            mod async_read_cache_ext {
                use crate::io::util::cache::read_until::{read_until, ReadUntil};
                use crate::io::{read_until_regex, AsyncCacheRead, ReadUntilRegex};
                pub trait AsyncCacheReadExt: AsyncCacheRead {
                    fn read_until<'a, T: AsRef<[u8]>>(
                        &'a mut self,
                        delim: T,
                        buf: &'a mut Vec<u8>,
                    ) -> ReadUntil<'a, Self, T>
                    where
                        Self: Unpin,
                    {
                        read_until(self, delim, buf)
                    }
                    fn read_line<'a>(
                        &'a mut self,
                        buf: &'a mut Vec<u8>,
                    ) -> ReadUntil<'a, Self, &'static [u8]>
                    where
                        Self: Unpin,
                    {
                        read_until(self, b"\n", buf)
                    }
                    fn read_line_crlf<'a>(
                        &'a mut self,
                        buf: &'a mut Vec<u8>,
                    ) -> ReadUntil<'a, Self, &'static [u8]>
                    where
                        Self: Unpin,
                    {
                        read_until(self, b"\r\n", buf)
                    }
                    async fn read_until_regex<'a>(
                        &'a mut self,
                        pattern: &str,
                        buf: &'a mut Vec<u8>,
                    ) -> Result<ReadUntilRegex<'a, Self>, regex::Error>
                    where
                        Self: Unpin,
                    {
                        read_until_regex(self, pattern, buf)
                    }
                }
                impl<R: AsyncCacheRead + ?Sized> AsyncCacheReadExt for R {}
            }
            mod read_until {
                use crate::io::AsyncCacheRead;
                use pin_project_lite::pin_project;
                use std::future::Future;
                use std::io;
                use std::io::ErrorKind;
                use std::marker::PhantomPinned;
                use std::pin::Pin;
                use std::task::{ready, Context, Poll};
                use tokio::io::ReadBuf;
                /// The delimiter is included in the resulting vector.
                #[must_use = "futures do nothing unless you `.await` or poll them"]
                pub struct ReadUntil<'a, R: ?Sized, D: AsRef<[u8]>> {
                    reader: &'a mut R,
                    delimiter: D,
                    buf: &'a mut Vec<u8>,
                    internal_buf: Vec<u8>,
                    read: usize,
                    _pin: PhantomPinned,
                }
                #[automatically_derived]
                impl<
                    'a,
                    R: ::core::fmt::Debug + ?Sized,
                    D: ::core::fmt::Debug + AsRef<[u8]>,
                > ::core::fmt::Debug for ReadUntil<'a, R, D> {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "reader",
                            "delimiter",
                            "buf",
                            "internal_buf",
                            "read",
                            "_pin",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &self.reader,
                            &self.delimiter,
                            &self.buf,
                            &self.internal_buf,
                            &self.read,
                            &&self._pin,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "ReadUntil",
                            names,
                            values,
                        )
                    }
                }
                #[allow(explicit_outlives_requirements)]
                #[allow(single_use_lifetimes)]
                #[allow(clippy::unknown_clippy_lints)]
                #[allow(clippy::redundant_pub_crate)]
                #[allow(clippy::used_underscore_binding)]
                const _: () = {
                    #[doc(hidden)]
                    #[allow(dead_code)]
                    #[allow(single_use_lifetimes)]
                    #[allow(clippy::unknown_clippy_lints)]
                    #[allow(clippy::mut_mut)]
                    #[allow(clippy::redundant_pub_crate)]
                    #[allow(clippy::ref_option_ref)]
                    #[allow(clippy::type_repetition_in_bounds)]
                    pub(crate) struct Projection<'__pin, 'a, R: ?Sized, D: AsRef<[u8]>>
                    where
                        ReadUntil<'a, R, D>: '__pin,
                    {
                        reader: &'__pin mut (&'a mut R),
                        delimiter: &'__pin mut (D),
                        buf: &'__pin mut (&'a mut Vec<u8>),
                        internal_buf: &'__pin mut (Vec<u8>),
                        read: &'__pin mut (usize),
                        _pin: ::pin_project_lite::__private::Pin<
                            &'__pin mut (PhantomPinned),
                        >,
                    }
                    #[doc(hidden)]
                    #[allow(dead_code)]
                    #[allow(single_use_lifetimes)]
                    #[allow(clippy::unknown_clippy_lints)]
                    #[allow(clippy::mut_mut)]
                    #[allow(clippy::redundant_pub_crate)]
                    #[allow(clippy::ref_option_ref)]
                    #[allow(clippy::type_repetition_in_bounds)]
                    pub(crate) struct ProjectionRef<
                        '__pin,
                        'a,
                        R: ?Sized,
                        D: AsRef<[u8]>,
                    >
                    where
                        ReadUntil<'a, R, D>: '__pin,
                    {
                        reader: &'__pin (&'a mut R),
                        delimiter: &'__pin (D),
                        buf: &'__pin (&'a mut Vec<u8>),
                        internal_buf: &'__pin (Vec<u8>),
                        read: &'__pin (usize),
                        _pin: ::pin_project_lite::__private::Pin<
                            &'__pin (PhantomPinned),
                        >,
                    }
                    impl<'a, R: ?Sized, D: AsRef<[u8]>> ReadUntil<'a, R, D> {
                        #[doc(hidden)]
                        #[inline]
                        pub(crate) fn project<'__pin>(
                            self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                        ) -> Projection<'__pin, 'a, R, D> {
                            unsafe {
                                let Self {
                                    reader,
                                    delimiter,
                                    buf,
                                    internal_buf,
                                    read,
                                    _pin,
                                } = self.get_unchecked_mut();
                                Projection {
                                    reader: reader,
                                    delimiter: delimiter,
                                    buf: buf,
                                    internal_buf: internal_buf,
                                    read: read,
                                    _pin: ::pin_project_lite::__private::Pin::new_unchecked(
                                        _pin,
                                    ),
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[inline]
                        pub(crate) fn project_ref<'__pin>(
                            self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                        ) -> ProjectionRef<'__pin, 'a, R, D> {
                            unsafe {
                                let Self {
                                    reader,
                                    delimiter,
                                    buf,
                                    internal_buf,
                                    read,
                                    _pin,
                                } = self.get_ref();
                                ProjectionRef {
                                    reader: reader,
                                    delimiter: delimiter,
                                    buf: buf,
                                    internal_buf: internal_buf,
                                    read: read,
                                    _pin: ::pin_project_lite::__private::Pin::new_unchecked(
                                        _pin,
                                    ),
                                }
                            }
                        }
                    }
                    #[allow(non_snake_case)]
                    pub struct __Origin<'__pin, 'a, R: ?Sized, D: AsRef<[u8]>> {
                        __dummy_lifetime: ::pin_project_lite::__private::PhantomData<
                            &'__pin (),
                        >,
                        reader: ::pin_project_lite::__private::AlwaysUnpin<&'a mut R>,
                        delimiter: ::pin_project_lite::__private::AlwaysUnpin<D>,
                        buf: ::pin_project_lite::__private::AlwaysUnpin<&'a mut Vec<u8>>,
                        internal_buf: ::pin_project_lite::__private::AlwaysUnpin<
                            Vec<u8>,
                        >,
                        read: ::pin_project_lite::__private::AlwaysUnpin<usize>,
                        _pin: PhantomPinned,
                    }
                    impl<
                        '__pin,
                        'a,
                        R: ?Sized,
                        D: AsRef<[u8]>,
                    > ::pin_project_lite::__private::Unpin for ReadUntil<'a, R, D>
                    where
                        __Origin<'__pin, 'a, R, D>: ::pin_project_lite::__private::Unpin,
                    {}
                    trait MustNotImplDrop {}
                    #[allow(clippy::drop_bounds, drop_bounds)]
                    impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
                    impl<'a, R: ?Sized, D: AsRef<[u8]>> MustNotImplDrop
                    for ReadUntil<'a, R, D> {}
                    #[forbid(unaligned_references, safe_packed_borrows)]
                    fn __assert_not_repr_packed<'a, R: ?Sized, D: AsRef<[u8]>>(
                        this: &ReadUntil<'a, R, D>,
                    ) {
                        let _ = &this.reader;
                        let _ = &this.delimiter;
                        let _ = &this.buf;
                        let _ = &this.internal_buf;
                        let _ = &this.read;
                        let _ = &this._pin;
                    }
                };
                pub(crate) fn read_until<'a, R, D: AsRef<[u8]>>(
                    reader: &'a mut R,
                    delimiter: D,
                    buf: &'a mut Vec<u8>,
                ) -> ReadUntil<'a, R, D>
                where
                    R: AsyncCacheRead + ?Sized + Unpin,
                {
                    ReadUntil {
                        reader,
                        delimiter,
                        buf,
                        internal_buf: Vec::new(),
                        read: 0,
                        _pin: PhantomPinned,
                    }
                }
                fn eof() -> io::Error {
                    io::Error::new(ErrorKind::UnexpectedEof, "early eof")
                }
                pub(super) fn read_until_internal<
                    R: AsyncCacheRead + ?Sized,
                    D: AsRef<[u8]>,
                >(
                    mut reader: Pin<&mut R>,
                    cx: &mut Context<'_>,
                    delimiter: D,
                    buf: &mut Vec<u8>,
                    internal_buf: &mut Vec<u8>,
                    read: &mut usize,
                ) -> Poll<io::Result<usize>> {
                    let delim_len = delimiter.as_ref().len();
                    if delim_len == 0 {
                        return Poll::Ready(Ok(0));
                    }
                    let mut read_buf = [0u8; 4096];
                    let mut data = ReadBuf::new(&mut read_buf);
                    loop {
                        data.clear();
                        match reader.as_mut().poll_reader(cx, &mut data) {
                            ::core::task::Poll::Ready(t) => t,
                            ::core::task::Poll::Pending => {
                                return ::core::task::Poll::Pending;
                            }
                        }?;
                        let read_len = data.filled().len();
                        if read_len == 0 {
                            return Err(eof()).into();
                        }
                        *read += read_len;
                        internal_buf.extend_from_slice(data.filled());
                        match kmp::kmp_find(delimiter.as_ref(), &internal_buf) {
                            Some(offset) => {
                                let drain_index = offset + delim_len;
                                buf.extend_from_slice(&internal_buf[..drain_index]);
                                let restore_data = &internal_buf[drain_index..];
                                reader.restore(restore_data);
                                *read -= restore_data.len();
                                return Poll::Ready(Ok(buf.len()));
                            }
                            None => {
                                if internal_buf.len() >= delim_len {
                                    let drain_range = 0..internal_buf.len() - delim_len;
                                    buf.extend_from_slice(&internal_buf[drain_range.clone()]);
                                    internal_buf.drain(drain_range);
                                }
                            }
                        }
                    }
                }
                impl<R: AsyncCacheRead + ?Sized + Unpin, D: AsRef<[u8]>> Future
                for ReadUntil<'_, R, D> {
                    type Output = io::Result<usize>;
                    fn poll(
                        self: Pin<&mut Self>,
                        cx: &mut Context<'_>,
                    ) -> Poll<Self::Output> {
                        let me = self.project();
                        read_until_internal(
                            Pin::new(*me.reader),
                            cx,
                            me.delimiter,
                            me.buf,
                            me.internal_buf,
                            me.read,
                        )
                    }
                }
            }
            mod read_until_regex {
                use crate::io::AsyncCacheRead;
                use pin_project_lite::pin_project;
                use regex::bytes::*;
                use std::future::Future;
                use std::io;
                use std::io::ErrorKind;
                use std::marker::PhantomPinned;
                use std::pin::Pin;
                use std::task::{ready, Context, Poll};
                use tokio::io::ReadBuf;
                /// The delimiter is included in the resulting vector.
                #[must_use = "futures do nothing unless you `.await` or poll them"]
                pub struct ReadUntilRegex<'a, R: ?Sized> {
                    reader: &'a mut R,
                    regex: Regex,
                    buf: &'a mut Vec<u8>,
                    internal_buf: Vec<u8>,
                    read: usize,
                    _pin: PhantomPinned,
                }
                #[automatically_derived]
                impl<'a, R: ::core::fmt::Debug + ?Sized> ::core::fmt::Debug
                for ReadUntilRegex<'a, R> {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "reader",
                            "regex",
                            "buf",
                            "internal_buf",
                            "read",
                            "_pin",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &self.reader,
                            &self.regex,
                            &self.buf,
                            &self.internal_buf,
                            &self.read,
                            &&self._pin,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "ReadUntilRegex",
                            names,
                            values,
                        )
                    }
                }
                #[allow(explicit_outlives_requirements)]
                #[allow(single_use_lifetimes)]
                #[allow(clippy::unknown_clippy_lints)]
                #[allow(clippy::redundant_pub_crate)]
                #[allow(clippy::used_underscore_binding)]
                const _: () = {
                    #[doc(hidden)]
                    #[allow(dead_code)]
                    #[allow(single_use_lifetimes)]
                    #[allow(clippy::unknown_clippy_lints)]
                    #[allow(clippy::mut_mut)]
                    #[allow(clippy::redundant_pub_crate)]
                    #[allow(clippy::ref_option_ref)]
                    #[allow(clippy::type_repetition_in_bounds)]
                    pub(crate) struct Projection<'__pin, 'a, R: ?Sized>
                    where
                        ReadUntilRegex<'a, R>: '__pin,
                    {
                        reader: &'__pin mut (&'a mut R),
                        regex: &'__pin mut (Regex),
                        buf: &'__pin mut (&'a mut Vec<u8>),
                        internal_buf: &'__pin mut (Vec<u8>),
                        read: &'__pin mut (usize),
                        _pin: ::pin_project_lite::__private::Pin<
                            &'__pin mut (PhantomPinned),
                        >,
                    }
                    #[doc(hidden)]
                    #[allow(dead_code)]
                    #[allow(single_use_lifetimes)]
                    #[allow(clippy::unknown_clippy_lints)]
                    #[allow(clippy::mut_mut)]
                    #[allow(clippy::redundant_pub_crate)]
                    #[allow(clippy::ref_option_ref)]
                    #[allow(clippy::type_repetition_in_bounds)]
                    pub(crate) struct ProjectionRef<'__pin, 'a, R: ?Sized>
                    where
                        ReadUntilRegex<'a, R>: '__pin,
                    {
                        reader: &'__pin (&'a mut R),
                        regex: &'__pin (Regex),
                        buf: &'__pin (&'a mut Vec<u8>),
                        internal_buf: &'__pin (Vec<u8>),
                        read: &'__pin (usize),
                        _pin: ::pin_project_lite::__private::Pin<
                            &'__pin (PhantomPinned),
                        >,
                    }
                    impl<'a, R: ?Sized> ReadUntilRegex<'a, R> {
                        #[doc(hidden)]
                        #[inline]
                        pub(crate) fn project<'__pin>(
                            self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                        ) -> Projection<'__pin, 'a, R> {
                            unsafe {
                                let Self { reader, regex, buf, internal_buf, read, _pin } = self
                                    .get_unchecked_mut();
                                Projection {
                                    reader: reader,
                                    regex: regex,
                                    buf: buf,
                                    internal_buf: internal_buf,
                                    read: read,
                                    _pin: ::pin_project_lite::__private::Pin::new_unchecked(
                                        _pin,
                                    ),
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[inline]
                        pub(crate) fn project_ref<'__pin>(
                            self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                        ) -> ProjectionRef<'__pin, 'a, R> {
                            unsafe {
                                let Self { reader, regex, buf, internal_buf, read, _pin } = self
                                    .get_ref();
                                ProjectionRef {
                                    reader: reader,
                                    regex: regex,
                                    buf: buf,
                                    internal_buf: internal_buf,
                                    read: read,
                                    _pin: ::pin_project_lite::__private::Pin::new_unchecked(
                                        _pin,
                                    ),
                                }
                            }
                        }
                    }
                    #[allow(non_snake_case)]
                    pub struct __Origin<'__pin, 'a, R: ?Sized> {
                        __dummy_lifetime: ::pin_project_lite::__private::PhantomData<
                            &'__pin (),
                        >,
                        reader: ::pin_project_lite::__private::AlwaysUnpin<&'a mut R>,
                        regex: ::pin_project_lite::__private::AlwaysUnpin<Regex>,
                        buf: ::pin_project_lite::__private::AlwaysUnpin<&'a mut Vec<u8>>,
                        internal_buf: ::pin_project_lite::__private::AlwaysUnpin<
                            Vec<u8>,
                        >,
                        read: ::pin_project_lite::__private::AlwaysUnpin<usize>,
                        _pin: PhantomPinned,
                    }
                    impl<'__pin, 'a, R: ?Sized> ::pin_project_lite::__private::Unpin
                    for ReadUntilRegex<'a, R>
                    where
                        __Origin<'__pin, 'a, R>: ::pin_project_lite::__private::Unpin,
                    {}
                    trait MustNotImplDrop {}
                    #[allow(clippy::drop_bounds, drop_bounds)]
                    impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
                    impl<'a, R: ?Sized> MustNotImplDrop for ReadUntilRegex<'a, R> {}
                    #[forbid(unaligned_references, safe_packed_borrows)]
                    fn __assert_not_repr_packed<'a, R: ?Sized>(
                        this: &ReadUntilRegex<'a, R>,
                    ) {
                        let _ = &this.reader;
                        let _ = &this.regex;
                        let _ = &this.buf;
                        let _ = &this.internal_buf;
                        let _ = &this.read;
                        let _ = &this._pin;
                    }
                };
                pub(crate) fn read_until_regex<'a, R>(
                    reader: &'a mut R,
                    pattern: &str,
                    buf: &'a mut Vec<u8>,
                ) -> Result<ReadUntilRegex<'a, R>, regex::Error>
                where
                    R: AsyncCacheRead + ?Sized + Unpin,
                {
                    let regex = Regex::new(pattern)?;
                    Ok(ReadUntilRegex {
                        reader,
                        regex,
                        buf,
                        internal_buf: Vec::new(),
                        read: 0,
                        _pin: PhantomPinned,
                    })
                }
                fn eof() -> io::Error {
                    io::Error::new(ErrorKind::UnexpectedEof, "early eof")
                }
                pub(super) fn read_until_regex_internal<R: AsyncCacheRead + ?Sized>(
                    mut reader: Pin<&mut R>,
                    cx: &mut Context<'_>,
                    regex: &mut Regex,
                    buf: &mut Vec<u8>,
                    internal_buf: &mut Vec<u8>,
                    read: &mut usize,
                ) -> Poll<io::Result<(usize, usize)>> {
                    let mut read_buf = [0u8; 4096];
                    let mut data = ReadBuf::new(&mut read_buf);
                    loop {
                        data.clear();
                        match reader.as_mut().poll_reader(cx, &mut data) {
                            ::core::task::Poll::Ready(t) => t,
                            ::core::task::Poll::Pending => {
                                return ::core::task::Poll::Pending;
                            }
                        }?;
                        let read_len = data.filled().len();
                        if read_len == 0 {
                            return Err(eof()).into();
                        }
                        *read += read_len;
                        internal_buf.extend_from_slice(data.filled());
                        match regex.find(&internal_buf) {
                            Some(m) => {
                                let drain_index = m.end();
                                buf.extend_from_slice(&internal_buf[..drain_index]);
                                let restore_data = &internal_buf[drain_index..];
                                reader.restore(restore_data);
                                *read -= restore_data.len();
                                return Poll::Ready(Ok((buf.len(), m.len())));
                            }
                            None => {}
                        }
                    }
                }
                impl<R: AsyncCacheRead + ?Sized + Unpin> Future
                for ReadUntilRegex<'_, R> {
                    type Output = io::Result<(usize, usize)>;
                    fn poll(
                        self: Pin<&mut Self>,
                        cx: &mut Context<'_>,
                    ) -> Poll<Self::Output> {
                        let me = self.project();
                        read_until_regex_internal(
                            Pin::new(*me.reader),
                            cx,
                            me.regex,
                            me.buf,
                            me.internal_buf,
                            me.read,
                        )
                    }
                }
            }
            pub use async_read_cache_ext::*;
            pub use read_until::*;
            pub use read_until_regex::*;
        }
        mod timeout {
            mod async_read_cache_timeout_ext {
                use crate::io::util::timeout::read_until_timeout::{
                    read_until_timeout, ReadUntilTimeout,
                };
                use crate::io::{
                    read_until_regex_timeout, AsyncCacheRead, ReadUntilRegexTimeout,
                };
                use std::time::Duration;
                pub trait AsyncReadCacheTimeoutExt: AsyncCacheRead {
                    fn read_until_timeout<'a, T: AsRef<[u8]>>(
                        &'a mut self,
                        delim: T,
                        buf: &'a mut Vec<u8>,
                        timeout: Duration,
                    ) -> ReadUntilTimeout<'a, Self, T>
                    where
                        Self: Unpin,
                    {
                        read_until_timeout(self, delim, buf, timeout)
                    }
                    fn read_until_regex_timeout<'a>(
                        &'a mut self,
                        pattern: &str,
                        buf: &'a mut Vec<u8>,
                        timeout: Duration,
                    ) -> Result<ReadUntilRegexTimeout<'a, Self>, regex::Error>
                    where
                        Self: Unpin,
                    {
                        read_until_regex_timeout(self, pattern, buf, timeout)
                    }
                    fn read_line_timeout<'a>(
                        &'a mut self,
                        buf: &'a mut Vec<u8>,
                        timeout: Duration,
                    ) -> ReadUntilTimeout<'a, Self, &'static [u8]>
                    where
                        Self: Unpin,
                    {
                        read_until_timeout(self, b"\n", buf, timeout)
                    }
                    fn read_line_crlf_timeout<'a>(
                        &'a mut self,
                        buf: &'a mut Vec<u8>,
                        timeout: Duration,
                    ) -> ReadUntilTimeout<'a, Self, &'static [u8]>
                    where
                        Self: Unpin,
                    {
                        read_until_timeout(self, b"\r\n", buf, timeout)
                    }
                }
                impl<R: AsyncCacheRead + ?Sized> AsyncReadCacheTimeoutExt for R {}
            }
            mod async_read_timeout_ext {
                use crate::io::util::timeout::read_exact_timeout::{
                    read_exact_timeout, ReadExactTimeout,
                };
                use std::time::Duration;
                use tokio::io::AsyncRead;
                pub trait AsyncReadTimeoutExt: AsyncRead {
                    fn read_exact_timeout<'a>(
                        &'a mut self,
                        buf: &'a mut [u8],
                        timeout: Duration,
                    ) -> ReadExactTimeout<'a, Self>
                    where
                        Self: Unpin,
                    {
                        read_exact_timeout(self, buf, timeout)
                    }
                    fn read_to_end_timeout<'a>(
                        &'a mut self,
                        _buf: &'a mut Vec<u8>,
                        _timeout: Duration,
                    )
                    where
                        Self: Unpin,
                    {
                        ::core::panicking::panic("not yet implemented")
                    }
                    fn read_to_string_timeout<'a>(
                        &'a mut self,
                        _dst: &'a mut String,
                        _timeout: Duration,
                    )
                    where
                        Self: Unpin,
                    {
                        ::core::panicking::panic("not yet implemented")
                    }
                }
                impl<R: AsyncRead + ?Sized> AsyncReadTimeoutExt for R {}
            }
            mod read_exact_timeout {
                use crate::io::util::timeout::get_deadline;
                use pin_project_lite::pin_project;
                use std::future::Future;
                use std::io;
                use std::io::ErrorKind;
                use std::marker::PhantomPinned;
                use std::marker::Unpin;
                use std::pin::Pin;
                use std::task::{ready, Context, Poll};
                use std::time::Duration;
                use tokio::io::{AsyncRead, ReadBuf};
                use tokio::time::Instant;
                /// A future which can be used to easily read bytes until timeout or buf is fully filled
                pub(crate) fn read_exact_timeout<'a, A>(
                    reader: &'a mut A,
                    buf: &'a mut [u8],
                    timeout: Duration,
                ) -> ReadExactTimeout<'a, A>
                where
                    A: AsyncRead + Unpin + ?Sized,
                {
                    let deadline = get_deadline(timeout);
                    ReadExactTimeout {
                        reader,
                        buf: ReadBuf::new(buf),
                        deadline,
                        _pin: PhantomPinned,
                    }
                }
                /// Creates a future which will read exactly enough bytes to fill `buf`,
                /// stops if Timeout,
                /// returning an error if EOF is hit sooner.
                ///
                /// On success the number of bytes is returned
                #[must_use = "futures do nothing unless you `.await` or poll them"]
                pub struct ReadExactTimeout<'a, A: ?Sized> {
                    reader: &'a mut A,
                    buf: ReadBuf<'a>,
                    deadline: Instant,
                    _pin: PhantomPinned,
                }
                #[automatically_derived]
                impl<'a, A: ::core::fmt::Debug + ?Sized> ::core::fmt::Debug
                for ReadExactTimeout<'a, A> {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "ReadExactTimeout",
                            "reader",
                            &self.reader,
                            "buf",
                            &self.buf,
                            "deadline",
                            &self.deadline,
                            "_pin",
                            &&self._pin,
                        )
                    }
                }
                #[allow(explicit_outlives_requirements)]
                #[allow(single_use_lifetimes)]
                #[allow(clippy::unknown_clippy_lints)]
                #[allow(clippy::redundant_pub_crate)]
                #[allow(clippy::used_underscore_binding)]
                const _: () = {
                    #[doc(hidden)]
                    #[allow(dead_code)]
                    #[allow(single_use_lifetimes)]
                    #[allow(clippy::unknown_clippy_lints)]
                    #[allow(clippy::mut_mut)]
                    #[allow(clippy::redundant_pub_crate)]
                    #[allow(clippy::ref_option_ref)]
                    #[allow(clippy::type_repetition_in_bounds)]
                    pub(crate) struct Projection<'__pin, 'a, A: ?Sized>
                    where
                        ReadExactTimeout<'a, A>: '__pin,
                    {
                        reader: &'__pin mut (&'a mut A),
                        buf: &'__pin mut (ReadBuf<'a>),
                        deadline: &'__pin mut (Instant),
                        _pin: ::pin_project_lite::__private::Pin<
                            &'__pin mut (PhantomPinned),
                        >,
                    }
                    #[doc(hidden)]
                    #[allow(dead_code)]
                    #[allow(single_use_lifetimes)]
                    #[allow(clippy::unknown_clippy_lints)]
                    #[allow(clippy::mut_mut)]
                    #[allow(clippy::redundant_pub_crate)]
                    #[allow(clippy::ref_option_ref)]
                    #[allow(clippy::type_repetition_in_bounds)]
                    pub(crate) struct ProjectionRef<'__pin, 'a, A: ?Sized>
                    where
                        ReadExactTimeout<'a, A>: '__pin,
                    {
                        reader: &'__pin (&'a mut A),
                        buf: &'__pin (ReadBuf<'a>),
                        deadline: &'__pin (Instant),
                        _pin: ::pin_project_lite::__private::Pin<
                            &'__pin (PhantomPinned),
                        >,
                    }
                    impl<'a, A: ?Sized> ReadExactTimeout<'a, A> {
                        #[doc(hidden)]
                        #[inline]
                        pub(crate) fn project<'__pin>(
                            self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                        ) -> Projection<'__pin, 'a, A> {
                            unsafe {
                                let Self { reader, buf, deadline, _pin } = self
                                    .get_unchecked_mut();
                                Projection {
                                    reader: reader,
                                    buf: buf,
                                    deadline: deadline,
                                    _pin: ::pin_project_lite::__private::Pin::new_unchecked(
                                        _pin,
                                    ),
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[inline]
                        pub(crate) fn project_ref<'__pin>(
                            self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                        ) -> ProjectionRef<'__pin, 'a, A> {
                            unsafe {
                                let Self { reader, buf, deadline, _pin } = self.get_ref();
                                ProjectionRef {
                                    reader: reader,
                                    buf: buf,
                                    deadline: deadline,
                                    _pin: ::pin_project_lite::__private::Pin::new_unchecked(
                                        _pin,
                                    ),
                                }
                            }
                        }
                    }
                    #[allow(non_snake_case)]
                    pub struct __Origin<'__pin, 'a, A: ?Sized> {
                        __dummy_lifetime: ::pin_project_lite::__private::PhantomData<
                            &'__pin (),
                        >,
                        reader: ::pin_project_lite::__private::AlwaysUnpin<&'a mut A>,
                        buf: ::pin_project_lite::__private::AlwaysUnpin<ReadBuf<'a>>,
                        deadline: ::pin_project_lite::__private::AlwaysUnpin<Instant>,
                        _pin: PhantomPinned,
                    }
                    impl<'__pin, 'a, A: ?Sized> ::pin_project_lite::__private::Unpin
                    for ReadExactTimeout<'a, A>
                    where
                        __Origin<'__pin, 'a, A>: ::pin_project_lite::__private::Unpin,
                    {}
                    trait MustNotImplDrop {}
                    #[allow(clippy::drop_bounds, drop_bounds)]
                    impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
                    impl<'a, A: ?Sized> MustNotImplDrop for ReadExactTimeout<'a, A> {}
                    #[forbid(unaligned_references, safe_packed_borrows)]
                    fn __assert_not_repr_packed<'a, A: ?Sized>(
                        this: &ReadExactTimeout<'a, A>,
                    ) {
                        let _ = &this.reader;
                        let _ = &this.buf;
                        let _ = &this.deadline;
                        let _ = &this._pin;
                    }
                };
                fn eof() -> io::Error {
                    io::Error::new(ErrorKind::UnexpectedEof, "early eof")
                }
                impl<A> Future for ReadExactTimeout<'_, A>
                where
                    A: AsyncRead + Unpin + ?Sized,
                {
                    type Output = io::Result<usize>;
                    fn poll(
                        self: Pin<&mut Self>,
                        cx: &mut Context<'_>,
                    ) -> Poll<io::Result<usize>> {
                        let me = self.project();
                        loop {
                            if *me.deadline < Instant::now() {
                                return Poll::Ready(Ok(me.buf.capacity()));
                            }
                            let rem = me.buf.remaining();
                            if rem != 0 {
                                match match Pin::new(&mut *me.reader).poll_read(cx, me.buf)
                                {
                                    ::core::task::Poll::Ready(t) => t,
                                    ::core::task::Poll::Pending => {
                                        return ::core::task::Poll::Pending;
                                    }
                                } {
                                    Ok(_) => {}
                                    Err(e) if e.kind() == ErrorKind::TimedOut => {
                                        return Poll::Ready(Ok(me.buf.capacity()));
                                    }
                                    Err(e) => {
                                        return Poll::Ready(Err(e.into()));
                                    }
                                };
                                if me.buf.remaining() == rem {
                                    return Err(eof()).into();
                                }
                            } else {
                                return Poll::Ready(Ok(me.buf.capacity()));
                            }
                        }
                    }
                }
            }
            mod read_until_regex_timeout {
                use crate::io::{get_deadline, AsyncCacheRead};
                use crate::io::{read_until_regex, ReadUntilRegex};
                use pin_project_lite::pin_project;
                use std::future::Future;
                use std::io;
                use std::io::ErrorKind;
                use std::marker::PhantomPinned;
                use std::pin::Pin;
                use std::task::{Context, Poll};
                use std::time::Duration;
                use tokio::time::Instant;
                /// The delimiter is included in the resulting vector.
                #[must_use = "futures do nothing unless you `.await` or poll them"]
                pub struct ReadUntilRegexTimeout<'a, R: ?Sized> {
                    read_until_regex: ReadUntilRegex<'a, R>,
                    deadline: Instant,
                    _pin: PhantomPinned,
                }
                #[automatically_derived]
                impl<'a, R: ::core::fmt::Debug + ?Sized> ::core::fmt::Debug
                for ReadUntilRegexTimeout<'a, R> {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "ReadUntilRegexTimeout",
                            "read_until_regex",
                            &self.read_until_regex,
                            "deadline",
                            &self.deadline,
                            "_pin",
                            &&self._pin,
                        )
                    }
                }
                #[allow(explicit_outlives_requirements)]
                #[allow(single_use_lifetimes)]
                #[allow(clippy::unknown_clippy_lints)]
                #[allow(clippy::redundant_pub_crate)]
                #[allow(clippy::used_underscore_binding)]
                const _: () = {
                    #[doc(hidden)]
                    #[allow(dead_code)]
                    #[allow(single_use_lifetimes)]
                    #[allow(clippy::unknown_clippy_lints)]
                    #[allow(clippy::mut_mut)]
                    #[allow(clippy::redundant_pub_crate)]
                    #[allow(clippy::ref_option_ref)]
                    #[allow(clippy::type_repetition_in_bounds)]
                    pub(crate) struct Projection<'__pin, 'a, R: ?Sized>
                    where
                        ReadUntilRegexTimeout<'a, R>: '__pin,
                    {
                        read_until_regex: ::pin_project_lite::__private::Pin<
                            &'__pin mut (ReadUntilRegex<'a, R>),
                        >,
                        deadline: &'__pin mut (Instant),
                        _pin: ::pin_project_lite::__private::Pin<
                            &'__pin mut (PhantomPinned),
                        >,
                    }
                    #[doc(hidden)]
                    #[allow(dead_code)]
                    #[allow(single_use_lifetimes)]
                    #[allow(clippy::unknown_clippy_lints)]
                    #[allow(clippy::mut_mut)]
                    #[allow(clippy::redundant_pub_crate)]
                    #[allow(clippy::ref_option_ref)]
                    #[allow(clippy::type_repetition_in_bounds)]
                    pub(crate) struct ProjectionRef<'__pin, 'a, R: ?Sized>
                    where
                        ReadUntilRegexTimeout<'a, R>: '__pin,
                    {
                        read_until_regex: ::pin_project_lite::__private::Pin<
                            &'__pin (ReadUntilRegex<'a, R>),
                        >,
                        deadline: &'__pin (Instant),
                        _pin: ::pin_project_lite::__private::Pin<
                            &'__pin (PhantomPinned),
                        >,
                    }
                    impl<'a, R: ?Sized> ReadUntilRegexTimeout<'a, R> {
                        #[doc(hidden)]
                        #[inline]
                        pub(crate) fn project<'__pin>(
                            self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                        ) -> Projection<'__pin, 'a, R> {
                            unsafe {
                                let Self { read_until_regex, deadline, _pin } = self
                                    .get_unchecked_mut();
                                Projection {
                                    read_until_regex: ::pin_project_lite::__private::Pin::new_unchecked(
                                        read_until_regex,
                                    ),
                                    deadline: deadline,
                                    _pin: ::pin_project_lite::__private::Pin::new_unchecked(
                                        _pin,
                                    ),
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[inline]
                        pub(crate) fn project_ref<'__pin>(
                            self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                        ) -> ProjectionRef<'__pin, 'a, R> {
                            unsafe {
                                let Self { read_until_regex, deadline, _pin } = self
                                    .get_ref();
                                ProjectionRef {
                                    read_until_regex: ::pin_project_lite::__private::Pin::new_unchecked(
                                        read_until_regex,
                                    ),
                                    deadline: deadline,
                                    _pin: ::pin_project_lite::__private::Pin::new_unchecked(
                                        _pin,
                                    ),
                                }
                            }
                        }
                    }
                    #[allow(non_snake_case)]
                    pub struct __Origin<'__pin, 'a, R: ?Sized> {
                        __dummy_lifetime: ::pin_project_lite::__private::PhantomData<
                            &'__pin (),
                        >,
                        read_until_regex: ReadUntilRegex<'a, R>,
                        deadline: ::pin_project_lite::__private::AlwaysUnpin<Instant>,
                        _pin: PhantomPinned,
                    }
                    impl<'__pin, 'a, R: ?Sized> ::pin_project_lite::__private::Unpin
                    for ReadUntilRegexTimeout<'a, R>
                    where
                        __Origin<'__pin, 'a, R>: ::pin_project_lite::__private::Unpin,
                    {}
                    trait MustNotImplDrop {}
                    #[allow(clippy::drop_bounds, drop_bounds)]
                    impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
                    impl<'a, R: ?Sized> MustNotImplDrop
                    for ReadUntilRegexTimeout<'a, R> {}
                    #[forbid(unaligned_references, safe_packed_borrows)]
                    fn __assert_not_repr_packed<'a, R: ?Sized>(
                        this: &ReadUntilRegexTimeout<'a, R>,
                    ) {
                        let _ = &this.read_until_regex;
                        let _ = &this.deadline;
                        let _ = &this._pin;
                    }
                };
                pub(crate) fn read_until_regex_timeout<'a, R>(
                    reader: &'a mut R,
                    pattern: &str,
                    buf: &'a mut Vec<u8>,
                    timeout: Duration,
                ) -> Result<ReadUntilRegexTimeout<'a, R>, regex::Error>
                where
                    R: AsyncCacheRead + ?Sized + Unpin,
                {
                    let read_until_regex = read_until_regex(reader, pattern, buf)?;
                    let deadline = get_deadline(timeout);
                    Ok(ReadUntilRegexTimeout {
                        read_until_regex,
                        deadline,
                        _pin: PhantomPinned,
                    })
                }
                fn timeout() -> io::Error {
                    io::Error::new(ErrorKind::TimedOut, "early timeout")
                }
                impl<R: AsyncCacheRead + ?Sized + Unpin> Future
                for ReadUntilRegexTimeout<'_, R> {
                    type Output = io::Result<(usize, usize)>;
                    fn poll(
                        self: Pin<&mut Self>,
                        cx: &mut Context<'_>,
                    ) -> Poll<Self::Output> {
                        let me = self.project();
                        if *me.deadline < Instant::now() {
                            return Err(timeout()).into();
                        }
                        me.read_until_regex.poll(cx)
                    }
                }
            }
            mod read_until_timeout {
                use crate::io::ReadUntil;
                use crate::io::{get_deadline, read_until, AsyncCacheRead};
                use pin_project_lite::pin_project;
                use std::future::Future;
                use std::io;
                use std::io::ErrorKind;
                use std::marker::PhantomPinned;
                use std::pin::Pin;
                use std::task::{Context, Poll};
                use std::time::Duration;
                use tokio::time::Instant;
                /// The delimiter is included in the resulting vector.
                #[must_use = "futures do nothing unless you `.await` or poll them"]
                pub struct ReadUntilTimeout<'a, R: ?Sized, D: AsRef<[u8]>> {
                    read_until: ReadUntil<'a, R, D>,
                    deadline: Instant,
                    _pin: PhantomPinned,
                }
                #[automatically_derived]
                impl<
                    'a,
                    R: ::core::fmt::Debug + ?Sized,
                    D: ::core::fmt::Debug + AsRef<[u8]>,
                > ::core::fmt::Debug for ReadUntilTimeout<'a, R, D> {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "ReadUntilTimeout",
                            "read_until",
                            &self.read_until,
                            "deadline",
                            &self.deadline,
                            "_pin",
                            &&self._pin,
                        )
                    }
                }
                #[allow(explicit_outlives_requirements)]
                #[allow(single_use_lifetimes)]
                #[allow(clippy::unknown_clippy_lints)]
                #[allow(clippy::redundant_pub_crate)]
                #[allow(clippy::used_underscore_binding)]
                const _: () = {
                    #[doc(hidden)]
                    #[allow(dead_code)]
                    #[allow(single_use_lifetimes)]
                    #[allow(clippy::unknown_clippy_lints)]
                    #[allow(clippy::mut_mut)]
                    #[allow(clippy::redundant_pub_crate)]
                    #[allow(clippy::ref_option_ref)]
                    #[allow(clippy::type_repetition_in_bounds)]
                    pub(crate) struct Projection<'__pin, 'a, R: ?Sized, D: AsRef<[u8]>>
                    where
                        ReadUntilTimeout<'a, R, D>: '__pin,
                    {
                        read_until: ::pin_project_lite::__private::Pin<
                            &'__pin mut (ReadUntil<'a, R, D>),
                        >,
                        deadline: &'__pin mut (Instant),
                        _pin: ::pin_project_lite::__private::Pin<
                            &'__pin mut (PhantomPinned),
                        >,
                    }
                    #[doc(hidden)]
                    #[allow(dead_code)]
                    #[allow(single_use_lifetimes)]
                    #[allow(clippy::unknown_clippy_lints)]
                    #[allow(clippy::mut_mut)]
                    #[allow(clippy::redundant_pub_crate)]
                    #[allow(clippy::ref_option_ref)]
                    #[allow(clippy::type_repetition_in_bounds)]
                    pub(crate) struct ProjectionRef<
                        '__pin,
                        'a,
                        R: ?Sized,
                        D: AsRef<[u8]>,
                    >
                    where
                        ReadUntilTimeout<'a, R, D>: '__pin,
                    {
                        read_until: ::pin_project_lite::__private::Pin<
                            &'__pin (ReadUntil<'a, R, D>),
                        >,
                        deadline: &'__pin (Instant),
                        _pin: ::pin_project_lite::__private::Pin<
                            &'__pin (PhantomPinned),
                        >,
                    }
                    impl<'a, R: ?Sized, D: AsRef<[u8]>> ReadUntilTimeout<'a, R, D> {
                        #[doc(hidden)]
                        #[inline]
                        pub(crate) fn project<'__pin>(
                            self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                        ) -> Projection<'__pin, 'a, R, D> {
                            unsafe {
                                let Self { read_until, deadline, _pin } = self
                                    .get_unchecked_mut();
                                Projection {
                                    read_until: ::pin_project_lite::__private::Pin::new_unchecked(
                                        read_until,
                                    ),
                                    deadline: deadline,
                                    _pin: ::pin_project_lite::__private::Pin::new_unchecked(
                                        _pin,
                                    ),
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[inline]
                        pub(crate) fn project_ref<'__pin>(
                            self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                        ) -> ProjectionRef<'__pin, 'a, R, D> {
                            unsafe {
                                let Self { read_until, deadline, _pin } = self.get_ref();
                                ProjectionRef {
                                    read_until: ::pin_project_lite::__private::Pin::new_unchecked(
                                        read_until,
                                    ),
                                    deadline: deadline,
                                    _pin: ::pin_project_lite::__private::Pin::new_unchecked(
                                        _pin,
                                    ),
                                }
                            }
                        }
                    }
                    #[allow(non_snake_case)]
                    pub struct __Origin<'__pin, 'a, R: ?Sized, D: AsRef<[u8]>> {
                        __dummy_lifetime: ::pin_project_lite::__private::PhantomData<
                            &'__pin (),
                        >,
                        read_until: ReadUntil<'a, R, D>,
                        deadline: ::pin_project_lite::__private::AlwaysUnpin<Instant>,
                        _pin: PhantomPinned,
                    }
                    impl<
                        '__pin,
                        'a,
                        R: ?Sized,
                        D: AsRef<[u8]>,
                    > ::pin_project_lite::__private::Unpin for ReadUntilTimeout<'a, R, D>
                    where
                        __Origin<'__pin, 'a, R, D>: ::pin_project_lite::__private::Unpin,
                    {}
                    trait MustNotImplDrop {}
                    #[allow(clippy::drop_bounds, drop_bounds)]
                    impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
                    impl<'a, R: ?Sized, D: AsRef<[u8]>> MustNotImplDrop
                    for ReadUntilTimeout<'a, R, D> {}
                    #[forbid(unaligned_references, safe_packed_borrows)]
                    fn __assert_not_repr_packed<'a, R: ?Sized, D: AsRef<[u8]>>(
                        this: &ReadUntilTimeout<'a, R, D>,
                    ) {
                        let _ = &this.read_until;
                        let _ = &this.deadline;
                        let _ = &this._pin;
                    }
                };
                pub(crate) fn read_until_timeout<'a, R, D: AsRef<[u8]>>(
                    reader: &'a mut R,
                    delimiter: D,
                    buf: &'a mut Vec<u8>,
                    timeout: Duration,
                ) -> ReadUntilTimeout<'a, R, D>
                where
                    R: AsyncCacheRead + ?Sized + Unpin,
                {
                    let read_until = read_until(reader, delimiter, buf);
                    let deadline = get_deadline(timeout);
                    ReadUntilTimeout {
                        read_until,
                        deadline,
                        _pin: PhantomPinned,
                    }
                }
                fn timeout() -> io::Error {
                    io::Error::new(ErrorKind::TimedOut, "early timeout")
                }
                impl<R: AsyncCacheRead + ?Sized + Unpin, D: AsRef<[u8]>> Future
                for ReadUntilTimeout<'_, R, D> {
                    type Output = io::Result<usize>;
                    fn poll(
                        self: Pin<&mut Self>,
                        cx: &mut Context<'_>,
                    ) -> Poll<Self::Output> {
                        let me = self.project();
                        if *me.deadline < Instant::now() {
                            return Err(timeout()).into();
                        }
                        me.read_until.poll(cx)
                    }
                }
            }
            pub use async_read_cache_timeout_ext::*;
            pub use async_read_timeout_ext::*;
            pub use read_exact_timeout::*;
            pub use read_until_regex_timeout::*;
            pub use read_until_timeout::*;
            use std::time::Duration;
            use tokio::time::Instant;
            pub(crate) fn get_deadline(timeout: Duration) -> Instant {
                Instant::now().checked_add(timeout).unwrap_or_else(|| far_future())
            }
            pub(crate) fn far_future() -> Instant {
                Instant::now() + Duration::from_secs(86400 * 365 * 30)
            }
        }
        pub use cache::*;
        pub use timeout::*;
    }
    pub(crate) use cache::*;
    pub use payload::*;
    pub use pipe::*;
    pub(crate) use stdio::*;
    pub use util::*;
}
