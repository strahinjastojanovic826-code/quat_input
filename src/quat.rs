use crate::error::QuatError;
use std::fmt;
use std::ops::{Add, Sub, Mul, Not, BitAnd, BitOr, BitXor};

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

    /// Post negation: 3 - x (matches existing `not` semantics)
    pub const fn post_neg(self) -> Self {
        self.not()
    }

    /// Upward / cyclic shift by +1 modulo 4: (x + 1) mod 4
    pub const fn inc_mod4(self) -> Self {
        match Self::from_u8_const((self.to_u8() + 1) % 4) {
            Ok(q) => q,
            Err(_) => Self::V0,
        }
    }

    /// Downward / cyclic shift by -1 modulo 4: (x + 3) mod 4
    pub const fn dec_mod4(self) -> Self {
        match Self::from_u8_const((self.to_u8() + 3) % 4) {
            Ok(q) => q,
            Err(_) => Self::V0,
        }
    }

    /// Double cycle shift (+2 mod 4)
    pub const fn rot2(self) -> Self {
        match Self::from_u8_const((self.to_u8() + 2) % 4) {
            Ok(q) => q,
            Err(_) => Self::V0,
        }
    }

    /// Threshold literal: returns `high` if `self >= threshold`, else `low`
    pub const fn threshold(self, threshold: Self, low: Self, high: Self) -> Self {
        if self.to_u8() >= threshold.to_u8() {
            high
        } else {
            low
        }
    }

    /// Unary literal / window selector: returns `inside` if `low <= self <= high`, else `outside`
    pub const fn window_literal(self, low: Self, high: Self, inside: Self, outside: Self) -> Self {
        let v = self.to_u8();
        if v >= low.to_u8() && v <= high.to_u8() {
            inside
        } else {
            outside
        }
    }

    // --- Binary operators ---

    /// Multiplication modulo 4: (a * b) mod 4
    pub const fn mul_mod4(self, other: Self) -> Self {
        let val = (self.to_u8() * other.to_u8()) % 4;
        match Self::from_u8_const(val) {
            Ok(q) => q,
            Err(_) => Self::V0,
        }
    }

    /// Bitwise 2-bit AND over underlying representation (00..=11)
    pub const fn bit_and(self, other: Self) -> Self {
        match Self::from_u8_const(self.to_u8() & other.to_u8()) {
            Ok(q) => q,
            Err(_) => Self::V0,
        }
    }

    /// Bitwise 2-bit OR over underlying representation (00..=11)
    pub const fn bit_or(self, other: Self) -> Self {
        match Self::from_u8_const(self.to_u8() | other.to_u8()) {
            Ok(q) => q,
            Err(_) => Self::V0,
        }
    }

    /// Bitwise 2-bit XOR over underlying representation (00..=11)
    pub const fn bit_xor(self, other: Self) -> Self {
        match Self::from_u8_const(self.to_u8() ^ other.to_u8()) {
            Ok(q) => q,
            Err(_) => Self::V0,
        }
    }

    /// Quaternary implication (Zadeh / Kleene-style max over ~a and b): max(3 - a, b)
    pub const fn implies(self, other: Self) -> Self {
        self.not().max(other)
    }

    /// Quaternary equivalence / XNOR on 2-bit values: ~(a ^ b) & 0b11
    pub const fn xnor_bits(self, other: Self) -> Self {
        match Self::from_u8_const((!(self.to_u8() ^ other.to_u8())) & 0b11) {
            Ok(q) => q,
            Err(_) => Self::V0,
        }
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

impl Add for Quat {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Quat::add(self, rhs)
    }
}

impl Sub for Quat {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Quat::sub(self, rhs)
    }
}

impl Mul for Quat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_mod4(rhs)
    }
}

impl Not for Quat {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Quat::not(self)
    }
}

impl BitAnd for Quat {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.bit_and(rhs)
    }
}

impl BitOr for Quat {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.bit_or(rhs)
    }
}

impl BitXor for Quat {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.bit_xor(rhs)
    }
}