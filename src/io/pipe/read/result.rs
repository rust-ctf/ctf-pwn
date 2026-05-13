use std::string::FromUtf8Error;

use ascii::{AsciiString, FromAsciiError};

/// Result of a receive operation containing raw bytes.
#[derive(Debug, Clone)]
pub struct RecvResult {
    data: Vec<u8>,
}

/// Result of a regex receive operation with full match and capture groups.
#[derive(Debug, Clone)]
pub struct RecvRegexResult {
    full_match: RecvResult,
    groups: Vec<RecvResult>,
}

impl From<&[u8]> for RecvResult {
    fn from(value: &[u8]) -> Self {
        Self::new(value.to_vec())
    }
}

impl RecvResult {
    /// Create a new `RecvResult` from raw bytes.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Returns the data as a hex-encoded string.
    #[must_use]
    pub fn as_hex(&self) -> String {
        hex::encode(&self.data)
    }

    /// Returns the data as an ASCII string.
    ///
    /// # Errors
    ///
    /// Returns `FromAsciiError` if the data contains non-ASCII bytes.
    pub fn as_ascii(&self) -> Result<AsciiString, FromAsciiError<Vec<u8>>> {
        AsciiString::from_ascii(self.data.clone())
    }

    /// Returns the data as a UTF-8 string.
    ///
    /// # Errors
    ///
    /// Returns `FromUtf8Error` if the data is not valid UTF-8.
    pub fn as_utf8(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    /// Returns a reference to the raw bytes.
    #[must_use]
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
    /// Returns the full regex match.
    pub fn full_match(&self) -> &RecvResult {
        &self.full_match
    }

    /// Returns the capture groups.
    pub fn groups(&self) -> &[RecvResult] {
        &self.groups
    }
}

impl<'a> From<regex::bytes::Captures<'a>> for RecvRegexResult {
    fn from(value: regex::bytes::Captures<'a>) -> Self {
        let groups = value
            .iter()
            .map(|m| {
                m.map_or_else(|| RecvResult::new(Vec::new()), |m| m.as_bytes().into())
            })
            .collect::<Vec<RecvResult>>();

        Self {
            full_match: groups[0].clone(),
            groups,
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use crate::io::{
        pipe::{PipeRead, PipeReadExt, PipeReader},
        test::{AsyncTestReader, TestAction},
    };

    fn test_pipe(actions: &[TestAction]) -> PipeReader<AsyncTestReader> {
        let mut pipe = PipeReader::new(AsyncTestReader::new(actions));
        pipe.set_read_timeout(Some(Duration::from_millis(200)));
        pipe
    }

    #[tokio::test]
    async fn recv_result_as_hex() {
        let mut pipe = test_pipe(&[TestAction::Data(vec![0xDE, 0xAD, 0xBE, 0xEF])]);
        let result = pipe.recv().await.expect("recv should succeed");
        assert_eq!(result.as_hex(), "deadbeef");
    }

    #[tokio::test]
    async fn recv_result_as_utf8() {
        let mut pipe = test_pipe(&[TestAction::Data(b"hello".to_vec())]);
        let result = pipe.recv().await.expect("recv should succeed");
        assert_eq!(result.as_utf8().expect("valid utf8"), "hello");
    }

    #[tokio::test]
    async fn recv_result_as_utf8_invalid() {
        let mut pipe = test_pipe(&[TestAction::Data(vec![0xFF, 0xFE])]);
        let result = pipe.recv().await.expect("recv should succeed");
        assert!(result.as_utf8().is_err());
    }

    #[tokio::test]
    async fn recv_result_as_ascii() {
        let mut pipe = test_pipe(&[TestAction::Data(b"ascii".to_vec())]);
        let result = pipe.recv().await.expect("recv should succeed");
        assert_eq!(
            result.as_ascii().expect("valid ascii").as_str(),
            "ascii"
        );
    }

    #[tokio::test]
    async fn recv_result_as_ascii_invalid() {
        let mut pipe = test_pipe(&[TestAction::Data(vec![0x80])]);
        let result = pipe.recv().await.expect("recv should succeed");
        assert!(result.as_ascii().is_err());
    }
}
