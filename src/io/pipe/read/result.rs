use std::string::FromUtf8Error;

use ascii::{AsciiString, FromAsciiError};

#[derive(Debug, Clone)]
pub struct RecvResult {
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct RecvRegexResult {
    full_match: RecvResult,
    groups: Vec<RecvResult>,
}

impl From<&[u8]> for RecvResult {
    fn from(value: &[u8]) -> Self {
        RecvResult::new(value.to_vec())
    }
}

impl RecvResult {
    pub fn new(data: Vec<u8>) -> RecvResult {
        RecvResult { data }
    }

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

impl RecvRegexResult {
    pub fn full_match(&self) -> &RecvResult {
        &self.full_match
    }

    pub fn groups(&self) -> &[RecvResult] {
        &self.groups
    }
}

impl<'a> From<regex::bytes::Captures<'a>> for RecvRegexResult {
    fn from(value: regex::bytes::Captures<'a>) -> Self {
        let groups = value
            .iter()
            .map(|m| {
                m.map(|m| m.as_bytes().into())
                    .unwrap_or(RecvResult::new(Vec::new()))
            })
            .collect::<Vec<RecvResult>>();

        RecvRegexResult {
            full_match: groups[0].clone(),
            groups,
        }
    }
}
