use crate::error::QuatError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Quat {
    V0 = 0b00,
    V1 = 0b01,
    V2 = 0b10,
    V3 = 0b11,
}

impl Quat {
    pub const fn from_u8_const(val: u8) -> Result<Self, QuatError> {
        match val {
            0 => Ok(Quat::V0),
            1 => Ok(Quat::V1),
            2 => Ok(Quat::V2),
            3 => Ok(Quat::V3),
            _ => Err(QuatError::InvalidRawValue(val)),
        }
    }

    pub const fn from_char_const(c: char) -> Result<Self, QuatError> {
        match c {
            '0' => Ok(Quat::V0),
            '1' => Ok(Quat::V1),
            '2' => Ok(Quat::V2),
            '3' => Ok(Quat::V3),
            _ => Err(QuatError::InvalidChar(c)),
        }
    }
    pub const fn add(self, other: Self) -> Self {
        let val = (self.to_u8() + other.to_u8()) % 4;
        match Self::from_u8_const(val) {
            Ok(q) => q,
            Err(_) => Self::V0,
        }
    }

    /// Quaternary subtraction modulo 4 (V_a - V_b mod 4)
    pub const fn sub(self, other: Self) -> Self {
        let val = (self.to_u8() + 4 - other.to_u8()) % 4;
        match Self::from_u8_const(val) {
            Ok(q) => q,
            Err(_) => Self::V0,
        }
    }

    /// Quaternary Post negation (NOT operation: 3 - x)
    pub const fn not(self) -> Self {
        let val = 3 - self.to_u8();
        match Self::from_u8_const(val) {
            Ok(q) => q,
            Err(_) => Self::V0,
        }
    }

    /// Quaternary Maximum (Equivalent to OR in multi-valued logic)
    pub const fn max(self, other: Self) -> Self {
        if self.to_u8() > other.to_u8() {
            self
        } else {
            other
        }
    }

    /// Quaternary Minimum (Equivalent to AND in multi-valued logic)
    pub const fn min(self, other: Self) -> Self {
        if self.to_u8() < other.to_u8() {
            self
        } else {
            other
        }
    }

    #[inline]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Quat {
    type Error = QuatError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8_const(value)
    }
}

impl From<Quat> for u8 {
    #[inline]
    fn from(quat: Quat) -> Self {
        quat.to_u8()
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Quat::V0 => '0',
            Quat::V1 => '1',
            Quat::V2 => '2',
            Quat::V3 => '3',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuatByte(pub u8);

impl QuatByte {
    #[inline]
    pub const fn new(q3: Quat, q2: Quat, q1: Quat, q0: Quat) -> Self {
        let val = (q3.to_u8() << 6)
                | (q2.to_u8() << 4)
                | (q1.to_u8() << 2)
                | q0.to_u8();
        QuatByte(val)
    }

    #[inline]
    pub const fn get(&self, index: usize) -> Result<Quat, QuatError> {
        if index > 3 {
            return Err(QuatError::IndexOutOfBounds(index));
        }
        let shift = index * 2;
        let val = (self.0 >> shift) & 0b11;
        Quat::from_u8_const(val)
    }
}