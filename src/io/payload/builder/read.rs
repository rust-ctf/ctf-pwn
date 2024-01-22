use crate::io::payload::payloads::{
    Ascii, Building, Bytes, Chain, Complete, DynamicPayload, Initial, ReadPayload, ReadPayloadType,
    SendPayload, Utf8,
};
use crate::io::{PayloadAction, PayloadBuilder};
use paste::paste;

macro_rules! create_payload_builder_fn {
    ($initial_type:ty, $encoding_type:ty, $enum_name:ident, $method:ident $(, $arg:ident : $arg_type:ty : $arg_call:expr)*) => {
        pub fn $method(self $(, $arg: $arg_type)*) -> PayloadBuilder<Chain<$initial_type, ReadPayload<$encoding_type>>, A> {
            let payload = ReadPayload::<$encoding_type>::new(ReadPayloadType::$enum_name($($arg_call),*));
            PayloadBuilder::from(Chain::new(self.payload, payload))
        }
    };
    ($initial_type:ty, $encoding_type:ty, $enum_name:ident, $method:ident, $trait_bound:path, T $(, $arg:ident : $arg_type:ty : $arg_call:expr)*) => {
        pub fn $method<T:$trait_bound>(self $(, $arg: $arg_type)*) -> PayloadBuilder<Chain<$initial_type, ReadPayload<$encoding_type>>, A> {
            let payload = ReadPayload::<$encoding_type>::new(ReadPayloadType::$enum_name($($arg_call),*));
            PayloadBuilder::from(Chain::new(self.payload, payload))
        }
    };
}

macro_rules! create_payload_builder_fns {
    ($initial_type:ty, $enum_name:ident, $method:ident, $trait_bound:path, T $(, $arg:ident : $arg_type:ty : $arg_call:expr)*) => {
        create_payload_builder_fn!($initial_type, Bytes, $enum_name, $method, $trait_bound, T $(, $arg : $arg_type : $arg_call)*);

        paste! {
            create_payload_builder_fn!($initial_type, Ascii, $enum_name, [< $method _ascii >], $trait_bound, T $(, $arg : $arg_type : $arg_call)*);
            create_payload_builder_fn!($initial_type, Utf8, $enum_name, [<$method _utf8>], $trait_bound, T $(, $arg : $arg_type : $arg_call)*);
        }
    };
     ($initial_type:ty, $enum_name:ident, $method:ident $(, $arg:ident : $arg_type:ty : $arg_call:expr)*) => {
        create_payload_builder_fn!($initial_type, Bytes, $enum_name, $method $(, $arg : $arg_type : $arg_call)*);

        paste! {
            create_payload_builder_fn!($initial_type, Ascii, $enum_name, [<$method _ascii>] $(, $arg : $arg_type : $arg_call)*);
            create_payload_builder_fn!($initial_type, Utf8, $enum_name, [<$method _utf8>] $(, $arg : $arg_type : $arg_call)*);
        }
    };
}

macro_rules! generate_payload_fns {
    ($initial_type:ty) => {
        create_payload_builder_fns!($initial_type, RecvUntil, recv_until, AsRef<[u8]>, T,  delim:T:delim.as_ref().to_vec(), drop:bool:drop);
        create_payload_builder_fns!($initial_type, RecvUntilRegex, recv_until_regex, pattern:&str:pattern.to_string(), drop:bool:drop);
        create_payload_builder_fns!($initial_type, RecvRegex, recv_regex, pattern:&str:pattern.to_string());
        create_payload_builder_fns!($initial_type, RecvLine, recv_line);
        create_payload_builder_fns!($initial_type, RecvLineCrlf, recv_line_crlf);
    };
}

impl<A> PayloadBuilder<Initial, A> {
    generate_payload_fns!(Initial);
}

impl<P1: PayloadAction, RP, A> PayloadBuilder<Chain<P1, ReadPayload<RP>>, A>
where
    ReadPayload<RP>: PayloadAction,
{
    generate_payload_fns!(Chain<P1,ReadPayload<RP>>);
}

impl<P, E, R, A> PayloadBuilder<DynamicPayload<P, E, R>, A>
where
    P: PayloadAction<ReturnType = E>,
    R: PayloadAction,
{
    generate_payload_fns!(DynamicPayload<P, E, R>);
}

impl<P1: PayloadAction, A> PayloadBuilder<Chain<P1, SendPayload<Complete, A>>, A> {
    generate_payload_fns!(Chain<P1,SendPayload<Complete, A>>);
}
