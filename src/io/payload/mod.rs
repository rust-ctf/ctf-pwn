use std::fmt::Write;
use crossterm::Command;

pub struct Payload {
    data: Vec<u8>,
}

impl AsRef<[u8]> for Payload {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Write for Payload
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.data.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

impl Payload {
    pub fn new() -> Payload
    {
        Payload { data: Vec::new() }
    }

    pub fn push<T: AsRef<[u8]>>(&mut self, data: T) {
        self.data.extend_from_slice(data.as_ref());
    }

    pub fn push_line(&mut self, line: &str) {
        self.data.extend_from_slice(line.as_bytes());
        self.push_newline();
    }

    pub fn fill(&mut self, byte: u8, count: usize)
    {
        for _ in 0..count
        {
            self.data.push(byte);
        }
    }

    pub fn push_line_crlf(&mut self, line: &str) {
        self.data.extend_from_slice(line.as_bytes());
        self.push_newline_crlf();
    }

    pub fn push_newline(&mut self)
    {
        self.data.push(b'\n');
    }

    pub fn push_newline_crlf(&mut self)
    {
        self.data.push(b'\r');
        self.data.push(b'\n');
    }

    pub fn push_ansi_command<T: Command>(&mut self, command:T)
    {
        command.write_ansi(self).unwrap();
    }

    pub fn push_p64_le(&mut self, ptr: u64) {
        self.data.extend_from_slice(&ptr.to_le_bytes())
    }

    pub fn push_p32_le(&mut self, ptr: u32) {
        self.data.extend_from_slice(&ptr.to_le_bytes())
    }

    pub fn push_p64_be(&mut self, ptr: u64) {
        self.data.extend_from_slice(&ptr.to_be_bytes())
    }

    pub fn push_p32_be(&mut self, ptr: u32) {
        self.data.extend_from_slice(&ptr.to_be_bytes())
    }
}
