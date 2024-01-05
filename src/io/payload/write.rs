use crossterm::Command;

pub trait PayloadWrite
{
    fn push<T: AsRef<[u8]>>(&mut self, data: T);
}

impl<T> PayloadWriteExt for T where T: PayloadWrite {}


macro_rules! numeric_push_methods {
    ($name:ident, $type:ty) => {
        paste::item! {
            fn [<$name _le>](&mut self, value: $type) {
                self.push(&value.to_le_bytes());
            }

            fn [<$name _be>](&mut self, value: $type) {
                self.push(&value.to_be_bytes());
            }
        }
    };
}

pub trait PayloadWriteExt : PayloadWrite
{
    fn push_line(&mut self, line: &str) {
        self.push(line.as_bytes());
        self.push_newline();
    }

    fn fill(&mut self, byte: u8, count: usize)
    {
        for _ in 0..count
        {
            self.push([byte]);
        }
    }

    fn push_line_crlf(&mut self, line: &str) {
        self.push(line.as_bytes());
        self.push_newline_crlf();
    }

    fn push_newline(&mut self)
    {
        self.push(b"\n");
    }

    fn push_newline_crlf(&mut self)
    {
        self.push(b"\r\n");
    }

    fn push_ansi_command<T: Command>(&mut self, command:T)
    {
        let mut ansi = String::new();
        command.write_ansi(&mut ansi).unwrap();
        self.push(ansi);
    }

    fn push_p64_le(&mut self, ptr: u64) {
        self.push(&ptr.to_le_bytes())
    }

    fn push_p32_le(&mut self, ptr: u32) {
        self.push(&ptr.to_le_bytes())
    }

    fn push_p64_be(&mut self, ptr: u64) {
        self.push(&ptr.to_be_bytes())
    }

    fn push_p32_be(&mut self, ptr: u32) {
        self.push(&ptr.to_be_bytes())
    }

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