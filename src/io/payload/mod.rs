pub struct Payload
{
    data: Vec<u8>
}

impl AsRef<[u8]> for Payload
{
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Payload
{
    pub fn push(&mut self, data: &dyn AsRef<[u8]>)
    {
        self.data.extend_from_slice(data.as_ref());
    }

    pub fn push_line(&mut self, line: &str)
    {
        self.data.extend_from_slice(line.as_bytes());
        self.data.push(b'\n');
    }

    pub fn push_line_crlf(&mut self, line: &str)
    {
        self.data.extend_from_slice(line.as_bytes());
        self.data.push(b'\r');
        self.data.push(b'\n');
    }

    pub fn push_p64_le(&mut self, ptr: u64)
    {
        self.data.extend_from_slice(&ptr.to_le_bytes())
    }

    pub fn push_p32_le(&mut self, ptr: u32)
    {
        self.data.extend_from_slice(&ptr.to_le_bytes())
    }

    pub fn push_p64_be(&mut self, ptr: u64)
    {
        self.data.extend_from_slice(&ptr.to_be_bytes())
    }

    pub fn push_p32_be(&mut self, ptr: u32)
    {
        self.data.extend_from_slice(&ptr.to_be_bytes())
    }
}