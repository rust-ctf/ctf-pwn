use std::marker::PhantomData;
use crossterm::Command;
use tokio::io::AsyncWriteExt;
use crate::io::{PayloadAction, PipeError, PipeRead, PipeWrite};

pub struct Building;
pub struct  Complete;

pub struct SendPayload<T, A>
{
    data: Vec<u8>,
    _phantom: PhantomData<T>,
    _phantom_arch: PhantomData<A>,
}

macro_rules! numeric_push_methods {
    ($name:ident, $type:ty) => {
        paste::item! {
            pub fn [<$name _le>](self, value: $type) -> Self {
                self.push(&value.to_le_bytes())
            }

            pub fn [<$name _be>](self, value: $type) -> Self  {
                self.push(&value.to_be_bytes())
            }
        }
    };
}

impl<A> SendPayload<Building, A>
{
    pub fn new() -> Self
    {
        SendPayload { data: Vec::new(), _phantom: PhantomData::default(), _phantom_arch: PhantomData::default()}
    }

    pub fn complete(self) -> SendPayload<Complete, A>
    {
        SendPayload { data: self.data, _phantom: PhantomData::default(), _phantom_arch: PhantomData::default()}
    }

    pub fn push<T: AsRef<[u8]>>(mut self, data: T) -> Self
    {
        self.data.extend_from_slice(data.as_ref());
        Self { data: self.data, _phantom: PhantomData::default(), _phantom_arch: PhantomData::default()  }
    }

    pub fn push_line<T: AsRef<[u8]>>(self, data: T) -> Self{
        self.push(data).push("\n")
    }

    pub fn push_line_crlf<T: AsRef<[u8]>>(self, data: T) -> Self{
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

    // For unsigned integers
    numeric_push_methods!(push_u8, u8);
    numeric_push_methods!(push_u16, u16);
    numeric_push_methods!(push_u32, u32);
    numeric_push_methods!(push_u64, u64);
    numeric_push_methods!(push_u128, u128);
    numeric_push_methods!(push_usize, usize);

    // For signed integers
    numeric_push_methods!(push_i8, i8);
    numeric_push_methods!(push_i16, i16);
    numeric_push_methods!(push_i32, i32);
    numeric_push_methods!(push_i64, i64);
    numeric_push_methods!(push_i128, i128);
    numeric_push_methods!(push_isize, isize);

    // For floating numbers
    numeric_push_methods!(push_f32, f32);
    numeric_push_methods!(push_f64, f64);
}

impl<A> PayloadAction for SendPayload<Complete, A>
{
    type ReturnType = ();

    async fn execute<T: PipeRead + PipeWrite + Unpin + ?Sized>(&self, pipe: &mut T) -> Result<Self::ReturnType, PipeError>
    {
        pipe.write_all(&self.data).await?;
        Ok(())
    }
}

impl<A> PayloadAction for SendPayload<Building,A>
{
    type ReturnType = ();

    async fn execute<T: PipeRead + PipeWrite + Unpin + ?Sized>(&self, pipe: &mut T) -> Result<Self::ReturnType, PipeError>
    {
        unreachable!()
    }
}