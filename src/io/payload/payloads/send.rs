use crate::io::{
    Buildable, PayloadAction, PipeError, PipeRead, PipeWrite, Readable, ReturnsValue,
    SendCompletable, Sendable,
};
use crossterm::Command;
use std::marker::PhantomData;
use tokio::io::AsyncWriteExt;

pub struct Building;
pub struct Complete;

pub struct SendPayload<T, A> {
    data: Vec<u8>,
    _phantom: PhantomData<T>,
    _phantom_arch: PhantomData<A>,
}

impl<A> Buildable for SendPayload<Complete, A> {}
impl<A> Sendable for SendPayload<Complete, A> {}

impl<A> Sendable for SendPayload<Building, A> {
    fn push<A1, T: AsRef<[u8]>>(self, data: T) -> SendPayload<Building, A>
    where
        Self: Sized,
    {
        self.push_data(data)
    }
}

impl<A> SendCompletable for SendPayload<Building, A> {
    fn complete(self) -> SendPayload<Complete, A>
    where
        Self: Sized,
    {
        self.to_complete()
    }
}
impl<A> Readable for SendPayload<Complete, A> {}

impl<A> PayloadAction for SendPayload<Complete, A> {
    type ReturnType = ();

    async fn execute<T: PipeRead + PipeWrite + Unpin + ?Sized>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        pipe.write_all(&self.data).await?;
        Ok(())
    }
}

impl<A> PayloadAction for SendPayload<Building, A> {
    type ReturnType = ();

    async fn execute<T: PipeRead + PipeWrite + Unpin + ?Sized>(
        &self,
        _pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        unreachable!()
    }
}

impl<A> SendPayload<Building, A> {
    pub fn new() -> Self {
        SendPayload {
            data: Vec::new(),
            _phantom: PhantomData::default(),
            _phantom_arch: PhantomData::default(),
        }
    }

    pub fn to_complete(self) -> SendPayload<Complete, A> {
        SendPayload {
            data: self.data,
            _phantom: PhantomData::default(),
            _phantom_arch: PhantomData::default(),
        }
    }

    pub fn push_data<T: AsRef<[u8]>>(mut self, data: T) -> Self {
        self.data.extend_from_slice(data.as_ref());
        Self {
            data: self.data,
            _phantom: PhantomData::default(),
            _phantom_arch: PhantomData::default(),
        }
    }
}
