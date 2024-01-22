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
