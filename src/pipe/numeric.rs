use super::*;

use std::{
    io::{Result, Write},
    mem::size_of,
};

use num_traits::{AsPrimitive, PrimInt};

impl<T> Pipe<T>
where
    T: Write,
{
    pub fn write_be<D: PrimInt + AsPrimitive<usize>>(&mut self, data: D) -> Result<usize> {
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

    pub fn write_le<D: PrimInt + AsPrimitive<usize>>(&mut self, data: D) -> Result<usize> {
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
