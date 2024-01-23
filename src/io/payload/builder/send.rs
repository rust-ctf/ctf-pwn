use crate::io::payload::builder::PayloadBuilder;
use crate::io::payload::payloads::{
    Building, Chain, Complete, DynamicPayload, Initial, ReadPayload, SendPayload,
};
use crate::io::PayloadAction;
use crossterm::Command;

//
//
// macro_rules! create_payload_fn {
//     // For methods with a specific trait bound on T
//     ($method:ident, $trait_bound:path, T $(, $arg:ident : $arg_type:ty)*) => {
//         pub fn $method<T: $trait_bound>(self $(, $arg: $arg_type)*) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>,A> {
//             let payload1 = self.payload.payload1;
//             let payload2 = self.payload.payload2.$method($($arg),*);
//             PayloadBuilder::from(Chain::new(payload1, payload2))
//         }
//     };
//     // For methods not requiring a generic type T
//     ($method:ident $(, $arg:ident : $arg_type:ty)*) => {
//         pub fn $method(self $(, $arg: $arg_type)*) -> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
//             let payload1 = self.payload.payload1;
//             let payload2 = self.payload.payload2.$method($($arg),*);
//             PayloadBuilder::from(Chain::new(payload1, payload2))
//         }
//     };
// }
//
// macro_rules! create_new_payload_fn {
//     // For methods with a specific trait bound on T and custom payload handling
//     ($method:ident, $trait_bound:path, $payload_type:ty, T $(, $arg:ident : $arg_type:ty)*) => {
//         pub fn $method<T: $trait_bound>(self $(, $arg: $arg_type)*) -> PayloadBuilder<Chain<$payload_type, SendPayload<Building, A>>,A> {
//             let payload1: $payload_type = self.payload;
//             let payload2 = SendPayload::<Building,A>::new().$method($($arg),*);
//             PayloadBuilder::from(Chain::new(payload1, payload2))
//         }
//     };
//     // For methods not requiring a generic type T but with custom payload handling
//     ($method:ident, $payload_type:ty $(, $arg:ident : $arg_type:ty)*) => {
//         pub fn $method(self $(, $arg: $arg_type)*) -> PayloadBuilder<Chain<$payload_type, SendPayload<Building, A>>, A> {
//             let payload1: $payload_type = self.payload;
//             let payload2 = SendPayload::<Building,A>::new().$method($($arg),*);
//             PayloadBuilder::from(Chain::new(payload1, payload2))
//         }
//     };
// }
//
// macro_rules! create_payload_for_numeric {
//     ($base_name:ident, $type:ty) => {
//         paste::item! {
//             create_payload_fn!([< $base_name _le >], value: $type);
//             create_payload_fn!([< $base_name _be >], value: $type);
//         }
//     };
// }
//
// macro_rules! create_new_payload_for_numeric {
//     ($base_name:ident, $type:ty, $payload_type:ty) => {
//         paste::item! {
//             create_new_payload_fn!([< $base_name _le >], $payload_type, value: $type);
//             create_new_payload_fn!([< $base_name _be >], $payload_type, value: $type);
//         }
//     };
// }
//
// macro_rules! impl_payload_fns_core {
//     ($payload_type:ty) => {
//         create_new_payload_fn!(push, AsRef<[u8]>, $payload_type, T, data: T);
//         create_new_payload_fn!(push_line, AsRef<[u8]>, $payload_type, T, data: T);
//         create_new_payload_fn!(push_line_crlf, AsRef<[u8]>, $payload_type, T, data: T);
//         create_new_payload_fn!(fill_byte, $payload_type, byte: u8, count: usize);
//         create_new_payload_fn!(fill, AsRef<[u8]>, $payload_type, T, data: T, count: usize);
//         create_new_payload_fn!(push_ansi_command, Command, $payload_type, T, command: T);
//
//         create_new_payload_for_numeric!(push_u8, u8, $payload_type);
//         create_new_payload_for_numeric!(push_u16, u16, $payload_type);
//         create_new_payload_for_numeric!(push_u32, u32, $payload_type);
//         create_new_payload_for_numeric!(push_u64, u64, $payload_type);
//         create_new_payload_for_numeric!(push_u128, u128, $payload_type);
//         create_new_payload_for_numeric!(push_usize, usize, $payload_type);
//         create_new_payload_for_numeric!(push_i8, i8, $payload_type);
//         create_new_payload_for_numeric!(push_i16, i16, $payload_type);
//         create_new_payload_for_numeric!(push_i32, i32, $payload_type);
//         create_new_payload_for_numeric!(push_i64, i64, $payload_type);
//         create_new_payload_for_numeric!(push_i128, i128, $payload_type);
//         create_new_payload_for_numeric!(push_isize, isize, $payload_type);
//         create_new_payload_for_numeric!(push_f32, f32, $payload_type);
//         create_new_payload_for_numeric!(push_f64, f64, $payload_type);
//     };
//     () => {
//         create_payload_fn!(push, AsRef<[u8]>, T, data: T);
//         create_payload_fn!(push_line, AsRef<[u8]>, T, data: T);
//         create_payload_fn!(push_line_crlf, AsRef<[u8]>, T, data: T);
//         create_payload_fn!(fill_byte, byte: u8, count: usize);
//         create_payload_fn!(fill, AsRef<[u8]>, T, data: T, count: usize);
//         create_payload_fn!(push_ansi_command, Command, T, command: T);
//
//         create_payload_for_numeric!(push_u8, u8);
//         create_payload_for_numeric!(push_u16, u16);
//         create_payload_for_numeric!(push_u32, u32);
//         create_payload_for_numeric!(push_u64, u64);
//         create_payload_for_numeric!(push_u128, u128);
//         create_payload_for_numeric!(push_usize, usize);
//         create_payload_for_numeric!(push_i8, i8);
//         create_payload_for_numeric!(push_i16, i16);
//         create_payload_for_numeric!(push_i32, i32);
//         create_payload_for_numeric!(push_i64, i64);
//         create_payload_for_numeric!(push_i128, i128);
//         create_payload_for_numeric!(push_isize, isize);
//         create_payload_for_numeric!(push_f32, f32);
//         create_payload_for_numeric!(push_f64, f64);
//     };
// }
//
// // macro_rules! impl_payload_fns_ptr {
// //     (P32, $payload_type:ty) => {
// //         create_new_payload_fn!(push_p, AsRef<[u8]>, $payload_type, T, value: u32);
// //     };
// //     (P64, $payload_type:ty) => {
// //         create_new_payload_fn!(push_p, AsRef<[u8]>, $payload_type, T, value: u64);
// //     };
// //     (P32) => {
// //        create_payload_fn!(push_p, AsRef<[u8]>, T, value: u32);
// //     };
// //     (P64) => {
// //         create_payload_fn!(push_p, AsRef<[u8]>, T, value: u64);
// //     };
// // }
//
// macro_rules! impl_payload_fns {
//     ($payload_type:ty) => {
//         impl_payload_fns_core!($payload_type);
//     };
//     (P32, $payload_type:ty) => {
//         impl_payload_fns_core!($payload_type);
//         //impl_payload_fns_ptr!(P32, $payload_type);
//     };
//     (P64, $payload_type:ty) => {
//         impl_payload_fns_core!($payload_type);
//         //impl_payload_fns_ptr!(P64, $payload_type);
//     };
//     () => {
//         impl_payload_fns_core!();
//         //impl_payload_fns_ptr!(P32);
//     };
//     (P32) => {
//         impl_payload_fns_core!();
//         //impl_payload_fns_ptr!(P64);
//     };
//     (P64) => {
//         impl_payload_fns_core!();
//         //impl_payload_fns_core!();
//     };
// }
//
// impl<A> PayloadBuilder<Initial, A> {
//     impl_payload_fns!(Initial);
// }
//
// impl<P1: PayloadAction, RP, A> PayloadBuilder<Chain<P1, ReadPayload<RP>>, A>
// where
//     ReadPayload<RP>: PayloadAction,
// {
//     impl_payload_fns!(Chain<P1,ReadPayload<RP>>);
// }
//
// impl<P, E, R, A> PayloadBuilder<DynamicPayload<P, E, R>, A>
// where
//     P: PayloadAction<ReturnType = E>,
//     R: PayloadAction<ReturnType = E>,
// {
//     impl_payload_fns!(DynamicPayload<P, E, R>);
// }
//
// impl<P1: PayloadAction, A> PayloadBuilder<Chain<P1, SendPayload<Building, A>>, A> {
//     impl_payload_fns!();
//
//     pub fn send(self) -> PayloadBuilder<Chain<P1, SendPayload<Complete, A>>, A> {
//         let payload1 = self.payload.payload1;
//         let payload2 = self.payload.payload2.complete();
//         PayloadBuilder::from(Chain::new(payload1, payload2))
//     }
// }
//
// impl<P1: PayloadAction, A> PayloadBuilder<Chain<P1, SendPayload<Complete, A>>, A> {
//     impl_payload_fns!(Chain<P1,SendPayload<Complete, A>>);
// }

// create_new_payload_for_numeric!(push_u32, u32, $payload_type);
// create_new_payload_for_numeric!(push_u64, u64, $payload_type);
