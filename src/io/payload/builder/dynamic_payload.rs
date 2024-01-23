// use crate::io::payload::payloads::{Chain, DynamicPayload, Initial, ReadPayload};
// use crate::io::{PayloadAction, PayloadBuilder};
//
// impl<P1: PayloadAction, RP, A, E> PayloadBuilder<Chain<P1, ReadPayload<RP>>, A>
// where
//     ReadPayload<RP>: PayloadAction<ReturnType = E>,
// {
//     pub fn payload<R>(
//         self,
//         action: fn(E) -> R,
//     ) -> PayloadBuilder<DynamicPayload<Chain<P1, ReadPayload<RP>>, E, R>, A>
//     where
//         R: PayloadAction,
//     {
//         PayloadBuilder::from(DynamicPayload::new(self.payload, action))
//     }
// }
//
// impl<A, POrig, EOrig, ROrig, E> PayloadBuilder<DynamicPayload<POrig, EOrig, ROrig>, A>
// where
//     ROrig: PayloadAction<ReturnType = E>,
//     POrig: PayloadAction<ReturnType = EOrig>,
// {
//     pub fn payload<R>(
//         self,
//         action: fn(E) -> R,
//     ) -> PayloadBuilder<DynamicPayload<DynamicPayload<POrig, EOrig, ROrig>, E, R>, A>
//     where
//         R: PayloadAction,
//     {
//         PayloadBuilder::from(DynamicPayload::new(self.payload, action))
//     }
// }
