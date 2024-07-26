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
    pub fn hex(&self) -> String {
        hex::encode(&self.data)
    }
}
