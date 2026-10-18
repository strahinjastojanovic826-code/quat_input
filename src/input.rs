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