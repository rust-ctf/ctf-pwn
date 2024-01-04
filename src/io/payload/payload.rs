use crate::io::payload::write::PayloadWrite;

pub struct Payload {
    data: Vec<u8>,
}

impl AsRef<[u8]> for Payload {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Payload
{
    pub fn new() -> Payload
    {
        Payload { data: Vec::new() }
    }
}

impl PayloadWrite for Payload
{
    fn push<T: AsRef<[u8]>>(&mut self, data: T) {
        self.data.extend_from_slice(data.as_ref());
    }
}
