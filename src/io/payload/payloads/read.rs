use std::marker::PhantomData;
use ascii::AsciiString;
use crate::io::{PayloadAction, PipeError, PipeRead, PipeReadExt, PipeWrite};

pub (crate) struct Bytes;
pub (crate) struct Utf8;
pub (crate) struct Ascii;

pub enum ReadPayloadType {
    RecvUntil(Vec<u8>, bool),
    RecvUntilRegex(String, bool),
    RecvRegex(String),
    RecvLine(),
    RecvLineCrlf(),
}

pub struct ReadPayload<E>
{
    read_type: ReadPayloadType,
    _phantom: PhantomData<E>,
}

impl<E> ReadPayload<E>
{
    pub fn new(read_type: ReadPayloadType) ->  ReadPayload<E>
    {
        ReadPayload { read_type, _phantom: PhantomData::default() }
    }

    pub fn new_utf8(read_type: ReadPayloadType) ->  ReadPayload<Utf8>
    {
        ReadPayload { read_type, _phantom: PhantomData::default() }
    }

    pub fn new_ascii(read_type: ReadPayloadType) ->  ReadPayload<Ascii>
    {
        ReadPayload { read_type, _phantom: PhantomData::default() }
    }
}

async fn execute_internal<T, T1: PipeRead + PipeWrite + Unpin>(this: &ReadPayload<T>, pipe: &mut T1) -> Result<Vec<u8>, PipeError>
{
    let result = match &this.read_type {
        ReadPayloadType::RecvUntil(delim, drop) => pipe.recv_until(delim, *drop).await?,
        ReadPayloadType::RecvUntilRegex(pattern, drop) => pipe.recv_until_regex(pattern, *drop).await?,
        ReadPayloadType::RecvRegex(pattern) => pipe.recv_regex(pattern).await?,
        ReadPayloadType::RecvLine() => pipe.recv_line().await?,
        ReadPayloadType::RecvLineCrlf() => pipe.recv_line_crlf().await?,
    };

    Ok(result)
}

impl PayloadAction for ReadPayload<Bytes>
{
    type ReturnType = Vec<u8>;

    async fn execute<T: PipeRead + PipeWrite + Unpin>(&self, pipe: &mut T) -> Result<Self::ReturnType, PipeError> {
        execute_internal(self, pipe).await
    }
}

impl PayloadAction for ReadPayload<Utf8>
{
    type ReturnType = String;

    async fn execute<T: PipeRead + PipeWrite + Unpin>(&self, pipe: &mut T) -> Result<Self::ReturnType, PipeError> {
        let result = execute_internal(self, pipe).await?;
        Ok(String::from_utf8(result)?)
    }
}

impl PayloadAction for ReadPayload<Ascii>
{
    type ReturnType = AsciiString;

    async fn execute<T: PipeRead + PipeWrite + Unpin >(&self, pipe: &mut T) -> Result<Self::ReturnType, PipeError> {
        let result = execute_internal(self, pipe).await?;
        Ok(AsciiString::from_ascii(result)?)
    }
}