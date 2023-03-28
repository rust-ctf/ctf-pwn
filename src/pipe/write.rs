use std::io::{Write, Result};

use super::Pipe;

const DEFAULT_DELIMITER: [u8; 1] = [b'\n'];

impl<T:Write> Pipe<T> where T:Write
{
    fn write_line<D:AsRef<[u8]>>(&mut self, data: D) -> Result<usize>
    {
        let size = self.stream.write(data.as_ref())?;
        let size2 = self.stream.write(&DEFAULT_DELIMITER)?;
        Ok(size + size2)
    }

    fn write_line_delim<D1:AsRef<[u8]>,D2:AsRef<[u8]>>(&mut self, data: D1, delim: D2) -> Result<usize>
    {
        let size = self.stream.write(data.as_ref())?;
        let size2 = self.stream.write(delim.as_ref())?;
        Ok(size + size2)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::pipe::{Pipe, write::DEFAULT_DELIMITER};

    #[test]
    fn write_line_empty() {
        let stream = VecDeque::<u8>::new();
        let mut pipe = Pipe::from(stream);
        let len = pipe.write_line("");

        let data = pipe.read_all();
        assert_eq!(data, b"\n");
        assert_eq!(len.unwrap(), data.len());
    }

    #[test]
    fn write_line() {
        let stream = VecDeque::<u8>::new();
        let mut pipe = Pipe::from(stream);
        let len = pipe.write_line("line");

        let data = pipe.read_all();
        assert_eq!(data, b"line\n");
        assert_eq!(len.unwrap(), data.len());
    }

    #[test]
    fn write_line2() {
        let stream = VecDeque::<u8>::new();
        let mut pipe = Pipe::from(stream);
        let len1 = pipe.write_line("line").unwrap();
        let len2 = pipe.write_line("line 2 ").unwrap();

        let data = pipe.read_all();
        assert_eq!(data, b"line\nline 2 \n");
        assert_eq!(len1 + len2, data.len());
        assert_eq!(len1, "line".len() + DEFAULT_DELIMITER.len());
        assert_eq!(len2, "line 2 ".len() + DEFAULT_DELIMITER.len());
    }

    
    #[test]
    fn write_line_delim_empty() {
        let stream = VecDeque::<u8>::new();
        let mut pipe = Pipe::from(stream);
        let len = pipe.write_line_delim("", "not empty");

        let data = pipe.read_all();
        assert_eq!(data, b"not empty");
        assert_eq!(len.unwrap(), data.len());
    }

    #[test]
    fn write_line_delim_empty_delim() {
        let stream = VecDeque::<u8>::new();
        let mut pipe = Pipe::from(stream);
        let len = pipe.write_line_delim("text", "");

        let data = pipe.read_all();
        assert_eq!(data, b"text");
        assert_eq!(len.unwrap(), data.len());
    }


    #[test]
    fn write_line_delim() {
        let stream = VecDeque::<u8>::new();
        let mut pipe = Pipe::from(stream);
        let len = pipe.write_line_delim("line", "delimiter");

        let data = pipe.read_all();
        assert_eq!(data, b"linedelimiter");
        assert_eq!(len.unwrap(), data.len());
    }

    #[test]
    fn write_line_delim_multi() {
        let stream = VecDeque::<u8>::new();
        let mut pipe = Pipe::from(stream);
        let len1 = pipe.write_line_delim("line", " a ").unwrap();
        let len2 = pipe.write_line_delim("line 2 ", [0xff]).unwrap();

        let data = pipe.read_all();
        assert_eq!(data, b"line a line 2 \xff");
        assert_eq!(len1 + len2, data.len());
        assert_eq!(len1, "line".len() + " a ".len());
        assert_eq!(len2, "line 2 ".len() + [0xff].len())
    }
}