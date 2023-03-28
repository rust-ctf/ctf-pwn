use std::io::{Read, Result};

use super::Pipe;

const BUFFER_SIZE: usize = 256;

impl<T> Pipe<T>
where
    T: Read,
{
    pub fn read_buffer(&mut self) -> Result<([u8; BUFFER_SIZE], usize)> {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let len = match self.read_cache.len() {
            0 => self.stream.read(&mut buffer)?,
            len => {
                if len >= 256 {
                    self.read_cache.read(&mut buffer)?
                } else {
                    self.read_cache.read(&mut buffer[..len])?;
                    len + self.stream.read(&mut buffer[len..])?
                }
            }
        };
        Ok((buffer, len))
    }

    pub fn read_all(&mut self) -> Vec<u8> {
        let mut result = Vec::new();
        loop {
            match self.read_buffer() {
                Err(_) => break,
                Ok((_, 0)) => break,
                Ok((buffer, len)) => result.extend_from_slice(&buffer[..len]),
            }
        }
        result
    }

    pub fn read_until(&mut self, end: &[u8]) -> Option<Vec<u8>> {
        self.read_until_wait_internal(end, false)
    }

    pub fn read_until_wait(&mut self, end: &[u8]) -> Option<Vec<u8>> {
        self.read_until_wait_internal(end, true)
    }

    fn read_until_wait_internal(&mut self, end: &[u8], wait: bool) -> Option<Vec<u8>> {
        if end.len() == 0 {
            return None;
        }
        let mut result = Vec::new();
        let mut position = 0usize;
        loop {
            match self.read_buffer() {
                Err(_) => {
                    self.read_cache.extend(result.iter());
                    return None;
                }
                Ok((_, 0)) => {
                    if !wait {
                        return None;
                    }
                }
                Ok((buffer, len)) => result.extend_from_slice(&buffer[..len]),
            }

            for w in result[position..].windows(end.len()) {
                if w == end {
                    self.read_cache
                        .extend(result[(position + end.len())..].iter());
                    result.truncate(position);
                    return Some(result);
                }
                position += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::pipe::Pipe;

    fn stream_from_bytes(data: &[u8]) -> Pipe<VecDeque<u8>> {
        let mut queue = VecDeque::new();
        queue.extend(data.iter().copied());
        Pipe::from(queue)
    }

    #[test]
    fn read_all() {
        let mut stream = stream_from_bytes(b"Test stream");
        let result = stream.read_all();
        assert_eq!(result, b"Test stream");
    }

    #[test]
    fn read_all_empty() {
        let mut stream = stream_from_bytes(b"");
        let result = stream.read_all();
        assert_eq!(result, b"");
    }

    #[test]
    fn read_all_large() {
        let mut stream = stream_from_bytes(&[1u8; 1000]);
        let result = stream.read_all();
        assert_eq!(result, [1u8; 1000]);
    }

    #[test]
    fn read_until_fail() {
        let mut stream = stream_from_bytes(b"Test stream");
        let result = stream.read_until(b"match");
        assert_eq!(result, None);
    }

    #[test]
    fn read_until_fail_empty() {
        let mut stream = stream_from_bytes(b"");
        let result = stream.read_until(b"match");
        assert_eq!(result, None);
    }

    #[test]
    fn read_until_fail_large() {
        let mut stream = stream_from_bytes(&[1u8; 1000]);
        let result = stream.read_until(b"match");
        assert_eq!(result, None);
    }

    #[test]
    fn read_until_empty_end() {
        let mut stream = stream_from_bytes(b"Test stream match");
        let result = stream.read_until(b"");
        assert_eq!(result, None);
    }

    #[test]
    fn read_until() {
        let mut stream = stream_from_bytes(b"Test stream match");
        let result = stream.read_until(b"match");
        assert_eq!(result.unwrap(), b"Test stream ");
    }

    #[test]
    fn read_until_left() {
        let mut stream = stream_from_bytes(b"Test stream match This is after");
        let result = stream.read_until(b"match");
        assert_eq!(result.unwrap(), b"Test stream ");
        let result = stream.read_all();
        assert_eq!(result, b" This is after");
    }

    #[test]
    fn read_until_twice() {
        let mut stream = stream_from_bytes(b"Test stream match Test stream 2 match");
        let result = stream.read_until(b"match");
        assert_eq!(result.unwrap(), b"Test stream ");
        let result = stream.read_until(b"match");
        assert_eq!(result.unwrap(), b" Test stream 2 ");
    }

    #[test]
    fn read_until_large() {
        let mut data = [1u8; 1000];
        data[5..10].copy_from_slice(b"match");
        let mut stream = stream_from_bytes(&data);
        let result = stream.read_until(b"match");
        assert_eq!(result.unwrap(), [1u8; 5]);
        let result = stream.read_all();
        assert_eq!(result, [1u8; 990]);
    }

    #[test]
    fn read_until_large2() {
        let mut data = [1u8; 1000];
        data[900..905].copy_from_slice(b"match");
        let mut stream = stream_from_bytes(&data);
        let result = stream.read_until(b"match");
        assert_eq!(result.unwrap(), [1u8; 900]);
        let result = stream.read_all();
        assert_eq!(result, [1u8; 95]);
    }

    #[test]
    fn read_until_large_twice() {
        let mut data = [1u8; 1000];
        data[5..10].copy_from_slice(b"match");
        data[900..905].copy_from_slice(b"match");
        let mut stream = stream_from_bytes(&data);
        let result = stream.read_until(b"match");
        assert_eq!(result.unwrap(), [1u8; 5]);
        let result = stream.read_until(b"match");
        assert_eq!(result.unwrap(), [1u8; 890]);
        let result = stream.read_all();
        assert_eq!(result, [1u8; 95]);
    }

    #[test]
    fn read_until_mid_buffer() {
        let mut data = [1u8; 1000];
        data[(super::BUFFER_SIZE - 3)..(super::BUFFER_SIZE + 2)].copy_from_slice(b"match");
        let mut stream = stream_from_bytes(&data);
        let result = stream.read_until(b"match");
        assert_eq!(result.unwrap(), [1u8; (super::BUFFER_SIZE - 3)]);
        let result = stream.read_all();
        assert_eq!(result, [1u8; (1000 - super::BUFFER_SIZE - 2)]);
    }

    #[test]
    fn read_until_mid_buffer_twice() {
        let mut data = [1u8; 1000];
        data[5..10].copy_from_slice(b"match");
        data[(super::BUFFER_SIZE - 3)..(super::BUFFER_SIZE + 2)].copy_from_slice(b"match");
        let mut stream = stream_from_bytes(&data);
        let result = stream.read_until(b"match");
        assert_eq!(result.unwrap(), [1u8; 5]);
        let result = stream.read_until(b"match");
        assert_eq!(result.unwrap(), [1u8; (super::BUFFER_SIZE - 3 - 10)]);
        let result = stream.read_all();
        assert_eq!(result, [1u8; 1000 - super::BUFFER_SIZE - 2]);
    }

    #[test]
    fn read_until_wait() {
        let mut stream = stream_from_bytes(b"Test stream match");
        let result = stream.read_until_wait(b"match");
        assert_eq!(result.unwrap(), b"Test stream ");
    }

    #[test]
    fn read_until_left_wait() {
        let mut stream = stream_from_bytes(b"Test stream match This is after");
        let result = stream.read_until_wait(b"match");
        assert_eq!(result.unwrap(), b"Test stream ");
        let result = stream.read_all();
        assert_eq!(result, b" This is after");
    }

    #[test]
    fn read_until_twice_wait() {
        let mut stream = stream_from_bytes(b"Test stream match Test stream 2 match");
        let result = stream.read_until_wait(b"match");
        assert_eq!(result.unwrap(), b"Test stream ");
        let result = stream.read_until_wait(b"match");
        assert_eq!(result.unwrap(), b" Test stream 2 ");
    }

    #[test]
    fn read_until_large_wait() {
        let mut data = [1u8; 1000];
        data[5..10].copy_from_slice(b"match");
        let mut stream = stream_from_bytes(&data);
        let result = stream.read_until_wait(b"match");
        assert_eq!(result.unwrap(), [1u8; 5]);
        let result = stream.read_all();
        assert_eq!(result, [1u8; 990]);
    }

    #[test]
    fn read_until_large2_wait() {
        let mut data = [1u8; 1000];
        data[900..905].copy_from_slice(b"match");
        let mut stream = stream_from_bytes(&data);
        let result = stream.read_until_wait(b"match");
        assert_eq!(result.unwrap(), [1u8; 900]);
        let result = stream.read_all();
        assert_eq!(result, [1u8; 95]);
    }

    #[test]
    fn read_until_large_twice_wait() {
        let mut data = [1u8; 1000];
        data[5..10].copy_from_slice(b"match");
        data[900..905].copy_from_slice(b"match");
        let mut stream = stream_from_bytes(&data);
        let result = stream.read_until_wait(b"match");
        assert_eq!(result.unwrap(), [1u8; 5]);
        let result = stream.read_until_wait(b"match");
        assert_eq!(result.unwrap(), [1u8; 890]);
        let result = stream.read_all();
        assert_eq!(result, [1u8; 95]);
    }

    #[test]
    fn read_until_mid_buffer_wait() {
        let mut data = [1u8; 1000];
        data[(super::BUFFER_SIZE - 3)..(super::BUFFER_SIZE + 2)].copy_from_slice(b"match");
        let mut stream = stream_from_bytes(&data);
        let result = stream.read_until_wait(b"match");
        assert_eq!(result.unwrap(), [1u8; (super::BUFFER_SIZE - 3)]);
        let result = stream.read_all();
        assert_eq!(result, [1u8; (1000 - super::BUFFER_SIZE - 2)]);
    }

    #[test]
    fn read_until_mid_buffer_twice_wait() {
        let mut data = [1u8; 1000];
        data[5..10].copy_from_slice(b"match");
        data[(super::BUFFER_SIZE - 3)..(super::BUFFER_SIZE + 2)].copy_from_slice(b"match");
        let mut stream = stream_from_bytes(&data);
        let result = stream.read_until_wait(b"match");
        assert_eq!(result.unwrap(), [1u8; 5]);
        let result = stream.read_until_wait(b"match");
        assert_eq!(result.unwrap(), [1u8; (super::BUFFER_SIZE - 3 - 10)]);
        let result = stream.read_all();
        assert_eq!(result, [1u8; 1000 - super::BUFFER_SIZE - 2]);
    }
}
