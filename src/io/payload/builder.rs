use crate::io::payload::write::PayloadWrite;
use crate::io::payload::write::PayloadWriteExt;
use crate::io::Payload;
use crossterm::Command;
use std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct Initial;
#[derive(Default, Clone)]
pub struct Stateless;
#[derive(Default, Clone)]
pub struct Sending;
#[derive(Default, Clone)]
pub struct Reading;

pub struct PayloadBuilder<T> {
    payload: Payload,
    _phantom: PhantomData<T>,
}

impl<T> PayloadBuilder<T> {
    pub(crate) fn new(payload: Payload) -> PayloadBuilder<T> {
        PayloadBuilder::<T> {
            payload,
            _phantom: PhantomData::default(),
        }
    }
}

macro_rules! reading_fns {
    () => {
        pub fn recv_until<T: AsRef<[u8]>>(self, data: T) -> PayloadBuilder<Reading> {
            let mut payload = self.payload;
            payload.recv_until(data);
            PayloadBuilder::new(payload)
        }

        pub fn recv_until_regex(self, pattern: &str) -> PayloadBuilder<Reading> {
            let mut payload = self.payload;
            payload.recv_until_regex(pattern);
            PayloadBuilder::new(payload)
        }

        pub fn recv_regex(self, pattern: &str) -> PayloadBuilder<Reading> {
            let mut payload = self.payload;
            payload.recv_regex(pattern);
            PayloadBuilder::new(payload)
        }

        pub fn recv_line(self) -> PayloadBuilder<Reading> {
            let mut payload = self.payload;
            payload.recv_line();
            PayloadBuilder::new(payload)
        }

        pub fn recv_line_crlf(self) -> PayloadBuilder<Reading> {
            let mut payload = self.payload;
            payload.recv_line_crlf();
            PayloadBuilder::new(payload)
        }
    };
}

macro_rules! numeric_push_methods {
    ($name:ident, $type:ty) => {
        paste::item! {
            pub fn [<$name _le>](self, value: $type) -> PayloadBuilder<Sending> {
                let mut payload = self.payload;
                payload.[<$name _le>](value);
                PayloadBuilder::new(payload)
            }

            pub fn [<$name _be>](self, value: $type) -> PayloadBuilder<Sending>  {
                let mut payload = self.payload;
                payload.[<$name _be>](value);
                PayloadBuilder::new(payload)
            }
        }
    };
}

macro_rules! build_fn {
    () => {
        pub fn build(self) -> Payload {
            self.payload
        }
    }
}

macro_rules! sending_fns {
    () => {
        pub fn push<T: AsRef<[u8]>>(self, data: T) -> PayloadBuilder<Sending> {
            let mut payload = self.payload;
            payload.push(data);
            PayloadBuilder::new(payload)
        }

        pub fn fill_byte(self, byte: u8, count: usize) -> PayloadBuilder<Sending> {
            let mut payload = self.payload;
            payload.fill_byte(byte, count);
            PayloadBuilder::new(payload)
        }

        pub fn fill<T: AsRef<[u8]>>(self, data: T, count: usize) -> PayloadBuilder<Sending> {
            let mut payload = self.payload;
            payload.fill(data, count);
            PayloadBuilder::new(payload)
        }

        pub fn push_ansi_command<T: Command>(self, command: T) -> PayloadBuilder<Sending> {
            let mut payload = self.payload;
            payload.push_ansi_command(command);
            PayloadBuilder::new(payload)
        }

        pub fn push_p64_le(self, ptr: u64) -> PayloadBuilder<Sending> {
            let mut payload = self.payload;
            payload.push_p64_le(ptr);
            PayloadBuilder::new(payload)
        }

        pub fn push_p32_le(self, ptr: u32) -> PayloadBuilder<Sending> {
            let mut payload = self.payload;
            payload.push_p32_le(ptr);
            PayloadBuilder::new(payload)
        }

        pub fn push_p64_be(self, ptr: u64) -> PayloadBuilder<Sending> {
            let mut payload = self.payload;
            payload.push_p64_be(ptr);
            PayloadBuilder::new(payload)
        }

        pub fn push_p32_be(self, ptr: u32) -> PayloadBuilder<Sending> {
            let mut payload = self.payload;
            payload.push_p32_be(ptr);
            PayloadBuilder::new(payload)
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
    };
}

impl PayloadBuilder<Initial> {
    reading_fns! {}
    sending_fns! {}
}


impl PayloadBuilder<Stateless> {
    reading_fns! {}
    sending_fns! {}
    build_fn! {}
}

impl PayloadBuilder<Reading> {
    reading_fns! {}
    sending_fns! {}
    build_fn! {}

    pub fn print(self) -> PayloadBuilder<Stateless> {
        let mut payload = self.payload;
        payload.print();
        PayloadBuilder::new(payload)
    }
}

impl PayloadBuilder<Sending> {
    sending_fns! {}

    pub fn send(self) -> PayloadBuilder<Stateless> {
        let mut payload = self.payload;
        payload.send();
        PayloadBuilder::new(payload)
    }
}
