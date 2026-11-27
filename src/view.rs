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

    /// Unpacks up to `out.len().min(self.total_quats)` quaternary symbols into a caller-provided buffer
    /// without heap allocation. Returns the number of symbols written.
    pub fn unpack_into(&self, out: &mut [Quat]) -> Result<usize, QuatError> {
        let count = out.len().min(self.total_quats);
        let full_bytes = count / 4;

        if full_bytes > self.bytes.len() {
            return Err(QuatError::IndexOutOfBounds(full_bytes * 4));
        }

        for (b_idx, &raw) in self.bytes[..full_bytes].iter().enumerate() {
            let base = b_idx * 4;
            out[base] = Quat::from_u8_const(raw & 0b11)?;
            out[base + 1] = Quat::from_u8_const((raw >> 2) & 0b11)?;
            out[base + 2] = Quat::from_u8_const((raw >> 4) & 0b11)?;
            out[base + 3] = Quat::from_u8_const((raw >> 6) & 0b11)?;
        }

        for idx in (full_bytes * 4)..count {
            out[idx] = self.get(idx)?;
        }

        Ok(count)
    }

    /// Iterates over packed `QuatByte` chunks directly from the underlying slice without copying.
    #[inline]
    pub fn packed_bytes(&self) -> impl Iterator<Item = QuatByte> + 'a {
        let valid_bytes = (self.total_quats + 3) / 4;
        let bound = valid_bytes.min(self.bytes.len());
        self.bytes[..bound].iter().copied().map(QuatByte)
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