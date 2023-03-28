use std::{
    io::{Result, Write},
    mem::size_of,
};

use num_traits::{AsPrimitive, PrimInt};

use super::Pipe;

const DEFAULT_DELIMITER: [u8; 1] = [b'\n'];

impl<T: Write> Pipe<T>
where
    T: Write,
{
    fn write_line<D: AsRef<[u8]>>(&mut self, data: D) -> Result<usize> {
        let size = self.stream.write(data.as_ref())?;
        let size2 = self.stream.write(&DEFAULT_DELIMITER)?;
        Ok(size + size2)
    }

    fn write_line_delim<D1: AsRef<[u8]>, D2: AsRef<[u8]>>(
        &mut self,
        data: D1,
        delim: D2,
    ) -> Result<usize> {
        let size = self.stream.write(data.as_ref())?;
        let size2 = self.stream.write(delim.as_ref())?;
        Ok(size + size2)
    }

    fn write_be<D: PrimInt + AsPrimitive<usize>>(&mut self, data: D) -> Result<usize> {
        let size = size_of::<D>();
        if size == 1 {
            self.stream.write(&[data.as_() as u8])
        } else {
            let mut n = data;
            let ff = D::from(0xFF).unwrap();
            let mut bytes = vec![0u8; size];
            for i in (0..size).rev() {
                let b = (n & ff).as_() as u8;
                bytes[i] = b;
                n = n.rotate_right(8);
            }
            self.stream.write(&bytes)
        }
    }

    fn write_le<D: PrimInt + AsPrimitive<usize>>(&mut self, data: D) -> Result<usize> {
        let size = size_of::<D>();
        if size == 1 {
            self.stream.write(&[data.as_() as u8])
        } else {
            let mut n = data;
            let ff = D::from(0xFF).unwrap();
            let mut bytes = vec![0u8; size];
            for i in 0..size {
                let b = (n & ff).as_() as u8;
                bytes[i] = b;
                n = n.rotate_right(8);
            }
            self.stream.write(&bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, mem::size_of};

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

    macro_rules! test_write {
        ($($t:ty, $le_fn_name:ident, $be_fn_name:ident, $values:expr),*) => {
            $(
                #[test]
                        fn $le_fn_name() {

                            for &value in $values.iter()
                            {
                                let stream = VecDeque::<u8>::new();
                                let mut pipe = Pipe::from(stream);
                                let len = pipe.write_le(value);

                                let data = pipe.read_all();
                                assert_eq!(data, (value as $t).to_le_bytes());
                                assert_eq!(len.unwrap(), size_of::<$t>());
                            }
                        }

                        #[test]
                        fn $be_fn_name() {
                            for &value in $values.iter()
                            {
                                let stream = VecDeque::<u8>::new();
                                let mut pipe = Pipe::from(stream);
                                let len = pipe.write_be(value);

                                let data = pipe.read_all();
                                assert_eq!(data, (value as $t).to_be_bytes());
                                assert_eq!(len.unwrap(), size_of::<$t>());
                            }
                        }
            )*
        };
    }

    // Generate test functions for different primitive types and values
    test_write!(
        u8,
        write_u8_tests,
        write_u8_tests_be,
        [u8::MIN, 42, u8::MAX],
        i8,
        write_i8_tests,
        write_i8_tests_be,
        [i8::MIN, -42, i8::MAX],
        u16,
        write_u16_tests,
        write_u16_tests_be,
        [u16::MIN, 4242, u16::MAX],
        i16,
        write_i16_tests,
        write_i16_tests_be,
        [i16::MIN, -4242, i16::MAX],
        u32,
        write_u32_tests,
        write_u32_tests_be,
        [u32::MIN, 424242, u32::MAX],
        i32,
        write_i32_tests,
        write_i32_tests_be,
        [i32::MIN, -424242, i32::MAX],
        u64,
        write_u64_tests,
        write_u64_tests_be,
        [u64::MIN, 4242424242, u64::MAX],
        i64,
        write_i64_tests,
        write_i64_tests_be,
        [i64::MIN, -4242424242, i64::MAX],
        usize,
        write_usize_tests,
        write_usize_tests_be,
        [usize::MIN, 4242424242, usize::MAX],
        isize,
        write_isize_tests,
        write_isize_tests_be,
        [isize::MIN, -4242424242, isize::MAX]
    );
}
