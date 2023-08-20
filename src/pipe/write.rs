use std::io::{Result, Write};

use crossterm::event::KeyCode;

use super::Pipe;

const DEFAULT_DELIMITER: [u8; 1] = [b'\n'];

impl<T: Write> Pipe<T>
where
    T: Write,
{
    pub fn write_line<D: AsRef<[u8]>>(&mut self, data: D) -> Result<usize> {
        let size = self.stream.write(data.as_ref())?;
        let size2 = self.stream.write(&DEFAULT_DELIMITER)?;
        Ok(size + size2)
    }

    pub fn write_line_delim<D1: AsRef<[u8]>, D2: AsRef<[u8]>>(
        &mut self,
        data: D1,
        delim: D2,
    ) -> Result<usize> {
        let size = self.stream.write(data.as_ref())?;
        let size2 = self.stream.write(delim.as_ref())?;
        Ok(size + size2)
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        let size = self.stream.write(data.as_ref())?;
        self.stream.flush()?;
        Ok(size)
    }

    pub fn write_key(&mut self, key: KeyCode) -> Result<usize> {
        match key {
            KeyCode::Left => self.write(b"\x1bOD"),
            KeyCode::Right => self.write(b"\x1bOC"),
            KeyCode::Up => self.write(b"\x1bOA"),
            KeyCode::Down => self.write(b"\x1bOB"),
            KeyCode::Backspace => self.write(b"\x08"),
            KeyCode::Enter => self.write(b"\n"),
            KeyCode::Home => self.write(b"\x1bOH"),
            KeyCode::End => self.write(b"\x1bOF"),
            KeyCode::PageUp => self.write(b"\x1bO5~"),
            KeyCode::PageDown => self.write(b"\x1bO6~"),
            KeyCode::Tab => self.write(b"\t"),
            KeyCode::BackTab => self.write(b"\x1bOZ"),
            KeyCode::Delete => self.write(b"\x1bO3~"),
            KeyCode::Insert => self.write(b"\x1bO2~"),
            KeyCode::F(1) => self.write(b"\x1bOP"),
            KeyCode::F(2) => self.write(b"\x1bOQ"),
            KeyCode::F(3) => self.write(b"\x1bOR"),
            KeyCode::F(4) => self.write(b"\x1bOS"),
            KeyCode::F(5) => self.write(b"\x1bO15~"),
            KeyCode::F(6) => self.write(b"\x1bO17~"),
            KeyCode::F(7) => self.write(b"\x1bO18~"),
            KeyCode::F(8) => self.write(b"\x1bO19~"),
            KeyCode::F(9) => self.write(b"\x1bO20~"),
            KeyCode::F(10) => self.write(b"\x1bO21~"),
            KeyCode::F(11) => self.write(b"\x1bO23~"),
            KeyCode::F(12) => self.write(b"\x1bO24~"),
            KeyCode::Char(c) => self.write(&[c as u8]),
            KeyCode::Null => self.write(b"\x00"),
            KeyCode::Esc => self.write(b"\x1b"),
            KeyCode::CapsLock => self.write(b"\x1bO?25h"),
            KeyCode::ScrollLock => self.write(b"\x1bO?5h"),
            KeyCode::NumLock => self.write(b"\x1bO?47l\x1bO?25h"),
            KeyCode::PrintScreen => self.write(b"\x1bO2i"),
            KeyCode::Pause => self.write(b"\x1bO?24l"),
            KeyCode::Menu => self.write(b"\x1bO?68h"),
            KeyCode::KeypadBegin => self.write(b"\x1bO?69h"),
            _ => panic!("Unknown key"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::pipe::{write::DEFAULT_DELIMITER, Pipe};

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
