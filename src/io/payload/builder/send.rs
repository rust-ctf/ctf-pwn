use crate::io::payload::builder::PayloadBuilder;
use crate::io::{Buildable, Readable, SendCompletable, Sendable, X64, X86};
use crossterm::Command;

impl<T: SendCompletable, A> PayloadBuilder<T, A> {
    pub fn send(self) -> PayloadBuilder<impl Buildable + Readable + Sendable, A> {
        PayloadBuilder::from(self.payload.complete())
    }
}

impl<T: Sendable> PayloadBuilder<T, X86> {
    pub fn push_ptr(self, ptr: u32) -> PayloadBuilder<impl SendCompletable, X86> {
        self.push_u32_le(ptr)
    }
}

impl<T: Sendable> PayloadBuilder<T, X64> {
    pub fn push_ptr(self, ptr: u64) -> PayloadBuilder<impl SendCompletable, X64> {
        self.push_u64_le(ptr)
    }
}

impl<T: Sendable, A> PayloadBuilder<T, A> {
    pub fn push<D: AsRef<[u8]>>(self, data: D) -> PayloadBuilder<impl SendCompletable, A> {
        PayloadBuilder::from(self.payload.push::<A, D>(data))
    }

    pub fn push_line<D: AsRef<[u8]>>(self, data: D) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data).push("\n")
    }

    pub fn push_line_crlf<D: AsRef<[u8]>>(
        self,
        data: D,
    ) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data).push("\r\n")
    }

    pub fn fill_byte(self, byte: u8, count: usize) -> PayloadBuilder<impl SendCompletable, A> {
        let mut data = Vec::new();
        for _ in 0..count {
            data.push(byte)
        }
        self.push(data)
    }

    pub fn fill<D: AsRef<[u8]>>(
        self,
        data: D,
        count: usize,
    ) -> PayloadBuilder<impl SendCompletable, A> {
        let mut full_data = Vec::new();
        for _ in 0..count {
            full_data.extend_from_slice(data.as_ref())
        }
        self.push(full_data)
    }

    pub fn push_ansi_command<D: Command>(
        self,
        command: D,
    ) -> PayloadBuilder<impl SendCompletable, A> {
        let mut ansi = String::new();
        command.write_ansi(&mut ansi).unwrap();
        self.push(ansi)
    }

    pub fn push_i8_be(self, data: i8) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_be_bytes())
    }

    pub fn push_i8_le(self, data: i8) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_le_bytes())
    }

    pub fn push_i16_be(self, data: i16) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_be_bytes())
    }

    pub fn push_i16_le(self, data: i16) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_le_bytes())
    }

    pub fn push_i32_be(self, data: i32) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_be_bytes())
    }

    pub fn push_i32_le(self, data: i32) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_le_bytes())
    }

    pub fn push_i64_be(self, data: i64) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_be_bytes())
    }

    pub fn push_i64_le(self, data: i64) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_le_bytes())
    }

    pub fn push_i128_be(self, data: i128) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_be_bytes())
    }

    pub fn push_i128_le(self, data: i128) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_le_bytes())
    }

    pub fn push_u8_be(self, data: u8) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_be_bytes())
    }

    pub fn push_u8_le(self, data: u8) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_le_bytes())
    }

    pub fn push_u16_be(self, data: u16) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_be_bytes())
    }

    pub fn push_u16_le(self, data: u16) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_le_bytes())
    }

    pub fn push_u32_be(self, data: u32) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_be_bytes())
    }

    pub fn push_u32_le(self, data: u32) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_le_bytes())
    }

    pub fn push_u64_be(self, data: u64) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_be_bytes())
    }

    pub fn push_u64_le(self, data: u64) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_le_bytes())
    }

    pub fn push_u128_be(self, data: u128) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_be_bytes())
    }

    pub fn push_u128_le(self, data: u128) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_le_bytes())
    }

    pub fn push_f32_be(self, data: f32) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_bits().to_be_bytes())
    }

    pub fn push_f32_le(self, data: f32) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_bits().to_le_bytes())
    }

    pub fn push_f64_be(self, data: f64) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_bits().to_be_bytes())
    }

    pub fn push_f64_le(self, data: f64) -> PayloadBuilder<impl SendCompletable, A> {
        self.push(data.to_bits().to_le_bytes())
    }
}
