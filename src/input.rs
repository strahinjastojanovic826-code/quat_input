use crate::error::QuatError;
use crate::quat::{Quat, QuatByte};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuatInput {
    quats: Vec<Quat>,
}

impl QuatInput {
    pub fn new(quats: Vec<Quat>) -> Self {
        Self { quats }
    }

    pub fn quats(&self) -> &[Quat] {
        &self.quats
    }

    pub fn pack(&self) -> Vec<QuatByte> {
        let mut bytes = Vec::new();
        for chunk in self.quats.chunks(4) {
            let mut q = [Quat::V0; 4];
            for (i, &val) in chunk.iter().enumerate() {
                q[i] = val;
            }
            let byte = QuatByte::new(q[3], q[2], q[1], q[0]);
            bytes.push(byte);
        }
        bytes
    }

    pub fn pack_simd_optimized(&self) -> Vec<QuatByte> {
        let total_bytes = (self.quats.len() + 3) / 4;
        let mut bytes = Vec::with_capacity(total_bytes);
        
        let chunks = self.quats.chunks_exact(4);
        let remainder = chunks.remainder();

        for chunk in chunks {
            // Compiler can easily auto-vectorize this explicit shift-or pattern
            let byte_val = (chunk[3].to_u8() << 6) 
                         | (chunk[2].to_u8() << 4) 
                         | (chunk[1].to_u8() << 2) 
                         |  chunk[0].to_u8();
            bytes.push(QuatByte(byte_val));
        }

        if !remainder.is_empty() {
            let mut q = [Quat::V0; 4];
            for (i, &val) in remainder.iter().enumerate() {
                q[i] = val;
            }
            let byte_val = (q[3].to_u8() << 6) 
                         | (q[2].to_u8() << 4) 
                         | (q[1].to_u8() << 2) 
                         |  q[0].to_u8();
            bytes.push(QuatByte(byte_val));
        }

        bytes
    }

    pub fn pack_fast(&self) -> Vec<QuatByte> {
        let total_bytes = (self.quats.len() + 3) / 4;
        let mut bytes = Vec::with_capacity(total_bytes);

        let chunks = self.quats.chunks_exact(4);
        let remainder = chunks.remainder();

        // Vectorized chunk processing loop
        for chunk in chunks {
            let q3 = chunk[3].to_u8();
            let q2 = chunk[2].to_u8();
            let q1 = chunk[1].to_u8();
            let q0 = chunk[0].to_u8();

            let byte_val = (q3 << 6) | (q2 << 4) | (q1 << 2) | q0;
            bytes.push(QuatByte(byte_val));
        }

        // Handle remaining elements if input length is not a multiple of 4
        if !remainder.is_empty() {
            let mut q = [Quat::V0; 4];
            for (i, &val) in remainder.iter().enumerate() {
                q[i] = val;
            }
            let byte_val = (q[3].to_u8() << 6) | (q[2].to_u8() << 4) | (q[1].to_u8() << 2) | q[0].to_u8();
            bytes.push(QuatByte(byte_val));
        }

        bytes
    }

}

impl FromStr for QuatInput {
    type Err = QuatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut quats = Vec::with_capacity(s.len());
        for c in s.chars() {
            let quat = Quat::from_char_const(c)?;
            quats.push(quat);
        }
        Ok(QuatInput::new(quats))
    }
}