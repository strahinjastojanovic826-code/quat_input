use crate::error::QuatError;
use crate::quat::{Quat, QuatByte};

#[derive(Debug, Clone, Copy)]
pub struct QuatStream<'a> {
    bytes: &'a [u8],
    total_quats: usize,
}

impl<'a> QuatStream<'a> {
    #[inline]
    pub const fn new(bytes: &'a [u8], total_quats: usize) -> Self {
        Self { bytes, total_quats }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.total_quats
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.total_quats == 0
    }

    pub fn get(&self, index: usize) -> Result<Quat, QuatError> {
        if index >= self.total_quats {
            return Err(QuatError::IndexOutOfBounds(index));
        }
        let byte_idx = index / 4;
        let sub_idx = index % 4;

        if byte_idx >= self.bytes.len() {
            return Err(QuatError::IndexOutOfBounds(index));
        }

        let q_byte = QuatByte(self.bytes[byte_idx]);
        q_byte.get(sub_idx)
    }

    pub fn iter(&self) -> QuatStreamIter<'a> {
        QuatStreamIter {
            stream: *self,
            current_index: 0,
        }
    }
}

pub struct QuatStreamIter<'a> {
    stream: QuatStream<'a>,
    current_index: usize,
}

impl<'a> Iterator for QuatStreamIter<'a> {
    type Item = Result<Quat, QuatError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.stream.len() {
            return None;
        }
        let val = self.stream.get(self.current_index);
        self.current_index += 1;
        Some(val)
    }
}