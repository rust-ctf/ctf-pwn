mod builder;
mod payloads;

pub(crate) use builder::*;
pub(crate) use payloads::*;

use crate::io::payload::builder::PayloadBuilder;
use crate::io::payload::payloads::Initial;
use crate::io::{AsyncCacheRead, PipeError, PipeRead, PipeReadExt, PipeWrite, PipeWriteExt};

pub trait PayloadAction: Clone {
    type ReturnType: Clone;

    async fn execute<T: PipeRead + PipeWrite + Unpin>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError>;
}

#[derive(Clone)]
pub struct Payload {}

#[derive(Clone)]
pub struct UnknownArch;
#[derive(Clone)]
pub struct X86;
#[derive(Clone)]
pub struct X64;

impl Payload {
    pub fn builder() -> PayloadBuilder<Initial, UnknownArch> {
        PayloadBuilder::<Initial, UnknownArch>::new()
    }
}
