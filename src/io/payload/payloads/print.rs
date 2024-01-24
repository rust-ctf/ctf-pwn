use crate::io::*;
use std::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct FmtDefault;
#[derive(Clone)]
pub struct FmtBytes;
#[derive(Clone)]
pub struct FmtDebug;
#[derive(Clone)]
pub struct FmtLowerHex;
#[derive(Clone)]
pub struct FmtUpperHex;
#[derive(Clone)]
pub struct FmtOctal;
#[derive(Clone)]
pub struct FmtBinary;
#[derive(Clone)]
pub struct FmtPointer;
#[derive(Clone)]
pub struct FmtLowerExp;
#[derive(Clone)]
pub struct FmtUpperExp;

#[derive(Clone)]
pub struct Print<P, M> {
    payload: P,
    _phantom: PhantomData<M>,
}

impl<P, M> Buildable for Print<P, M> where Self: PayloadAction {}
impl<P, M> Sendable for Print<P, M> where Self: PayloadAction {}
impl<P, M> Readable for Print<P, M> where Self: PayloadAction {}
impl<P, M> ReturnsValue for Print<P, M> where Self: PayloadAction {}

impl<P> Print<P, FmtDefault>
where
    P: PayloadAction,
    P::ReturnType: Display,
{
    pub fn from(payload: P) -> Self {
        Print {
            payload,
            _phantom: PhantomData::default(),
        }
    }
}

impl<P> Print<P, FmtBytes>
where
    P: PayloadAction,
    P::ReturnType: AsRef<[u8]>,
{
    pub fn from(payload: P) -> Self {
        Print {
            payload,
            _phantom: PhantomData::default(),
        }
    }
}

impl<P> Print<P, FmtDebug>
where
    P: PayloadAction,
    P::ReturnType: Debug,
{
    pub fn from(payload: P) -> Self {
        Print {
            payload,
            _phantom: PhantomData::default(),
        }
    }
}

impl<P> Print<P, FmtLowerHex>
where
    P: PayloadAction,
    P::ReturnType: LowerHex,
{
    pub fn from(payload: P) -> Self {
        Print {
            payload,
            _phantom: PhantomData,
        }
    }
}

impl<P> Print<P, FmtUpperHex>
where
    P: PayloadAction,
    P::ReturnType: UpperHex,
{
    pub fn from(payload: P) -> Self {
        Print {
            payload,
            _phantom: PhantomData,
        }
    }
}

impl<P> Print<P, FmtOctal>
where
    P: PayloadAction,
    P::ReturnType: Octal,
{
    pub fn from(payload: P) -> Self {
        Print {
            payload,
            _phantom: PhantomData,
        }
    }
}

impl<P> Print<P, FmtBinary>
where
    P: PayloadAction,
    P::ReturnType: Binary,
{
    pub fn from(payload: P) -> Self {
        Print {
            payload,
            _phantom: PhantomData,
        }
    }
}

impl<P> Print<P, FmtPointer>
where
    P: PayloadAction,
    P::ReturnType: Pointer,
{
    pub fn from(payload: P) -> Self {
        Print {
            payload,
            _phantom: PhantomData,
        }
    }
}

impl<P> Print<P, FmtLowerExp>
where
    P: PayloadAction,
    P::ReturnType: LowerExp,
{
    pub fn from(payload: P) -> Self {
        Print {
            payload,
            _phantom: PhantomData,
        }
    }
}

impl<P> Print<P, FmtUpperExp>
where
    P: PayloadAction,
    P::ReturnType: UpperExp,
{
    pub fn from(payload: P) -> Self {
        Print {
            payload,
            _phantom: PhantomData,
        }
    }
}

impl<P> PayloadAction for Print<P, FmtDefault>
where
    P: PayloadAction,
    P::ReturnType: Display,
{
    type ReturnType = P::ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        let data = self.payload.execute(pipe).await?;
        println!("{}", data);
        Ok(data)
    }
}

impl<P> PayloadAction for Print<P, FmtBytes>
where
    P: PayloadAction,
    P::ReturnType: AsRef<[u8]>,
{
    type ReturnType = P::ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        let data = self.payload.execute(pipe).await?;
        println!("{}", hex::encode(&data));
        Ok(data)
    }
}

impl<P> PayloadAction for Print<P, FmtDebug>
where
    P: PayloadAction,
    P::ReturnType: Debug,
{
    type ReturnType = P::ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        let data = self.payload.execute(pipe).await?;
        println!("{:?}", data);
        Ok(data)
    }
}

impl<P> PayloadAction for Print<P, FmtLowerHex>
where
    P: PayloadAction,
    P::ReturnType: LowerHex,
{
    type ReturnType = P::ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        let data = self.payload.execute(pipe).await?;
        println!("{:x}", data);
        Ok(data)
    }
}

impl<P> PayloadAction for Print<P, FmtUpperHex>
where
    P: PayloadAction,
    P::ReturnType: UpperHex,
{
    type ReturnType = P::ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        let data = self.payload.execute(pipe).await?;
        println!("{:X}", data);
        Ok(data)
    }
}

impl<P> PayloadAction for Print<P, FmtOctal>
where
    P: PayloadAction,
    P::ReturnType: Octal,
{
    type ReturnType = P::ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        let data = self.payload.execute(pipe).await?;
        println!("{:o}", data);
        Ok(data)
    }
}

impl<P> PayloadAction for Print<P, FmtBinary>
where
    P: PayloadAction,
    P::ReturnType: Binary,
{
    type ReturnType = P::ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        let data = self.payload.execute(pipe).await?;
        println!("{:b}", data);
        Ok(data)
    }
}

impl<P> PayloadAction for Print<P, FmtPointer>
where
    P: PayloadAction,
    P::ReturnType: Pointer,
{
    type ReturnType = P::ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        let data = self.payload.execute(pipe).await?;
        println!("{:p}", data);
        Ok(data)
    }
}

impl<P> PayloadAction for Print<P, FmtLowerExp>
where
    P: PayloadAction,
    P::ReturnType: LowerExp,
{
    type ReturnType = P::ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        let data = self.payload.execute(pipe).await?;
        println!("{:e}", data);
        Ok(data)
    }
}

impl<P> PayloadAction for Print<P, FmtUpperExp>
where
    P: PayloadAction,
    P::ReturnType: UpperExp,
{
    type ReturnType = P::ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin + Send>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        let data = self.payload.execute(pipe).await?;
        println!("{:E}", data);
        Ok(data)
    }
}
