use crate::io::*;

pub struct Chain<P1, P2> {
    pub payload1: P1,
    pub payload2: P2,
}

impl<P1, P2: Buildable> Buildable for Chain<P1, P2> where Chain<P1, P2>: PayloadAction {}
impl<P1: PayloadAction, P2: Sendable> Sendable for Chain<P1, P2> {
    fn push<A, T: AsRef<[u8]>>(self, data: T) -> impl SendCompletable
    where
        Self: Sized,
    {
        Chain::new(self.payload1, self.payload2.push::<A, T>(data))
    }
}

impl<P1: PayloadAction, P2: SendCompletable> SendCompletable for Chain<P1, P2> {
    fn complete(self) -> impl Buildable + Readable + Sendable {
        let payload2 = self.payload2.complete();
        Chain::new(self.payload1, payload2)
    }
}
impl<P1, P2: Readable> Readable for Chain<P1, P2> where Chain<P1, P2>: PayloadAction {}
impl<P1, P2: ReturnsValue> ReturnsValue for Chain<P1, P2> where Chain<P1, P2>: PayloadAction {}

impl<P1: PayloadAction, P2: PayloadAction> Chain<P1, P2> {
    pub fn new(payload1: P1, payload2: P2) -> Chain<P1, P2> {
        Chain { payload1, payload2 }
    }
}

impl<P1: PayloadAction, P2: PayloadAction> PayloadAction for Chain<P1, P2> {
    type ReturnType = P2::ReturnType;

    async fn execute<T: PipeRead + PipeWrite + Unpin>(
        &self,
        pipe: &mut T,
    ) -> Result<Self::ReturnType, PipeError> {
        let _ = self.payload1.execute(pipe).await?;
        self.payload2.execute(pipe).await
    }
}
