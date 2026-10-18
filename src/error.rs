use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuatError {
    InvalidChar(char),
    InvalidRawValue(u8),
    IndexOutOfBounds(usize),
}

impl fmt::Display for QuatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuatError::InvalidChar(c) => write!(f, "Invalid quaternary character: '{}'. Expected '0'-'3'.", c),
            QuatError::InvalidRawValue(val) => write!(f, "Invalid raw quat value: {}.", val),
            QuatError::IndexOutOfBounds(idx) => write!(f, "Index out of bounds: {}.", idx),
        }
    }
}

impl std::error::Error for QuatError {}