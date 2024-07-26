use std::string::FromUtf8Error;

use ascii::{AsciiString, FromAsciiError};

pub struct RecvResult {
    data: Vec<u8>,
}

impl From<&[u8]> for RecvResult {
    fn from(value: &[u8]) -> Self {
        RecvResult {
            data: value.to_vec(),
        }
    }
}

impl RecvResult {
    pub fn as_hex(&self) -> String {
        hex::encode(&self.data)
    }

    pub fn as_ascii(&self) -> Result<AsciiString, FromAsciiError<Vec<u8>>> {
        AsciiString::from_ascii(self.data.clone())
    }

    pub fn as_utf8(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl AsRef<[u8]> for RecvResult {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
