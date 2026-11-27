use crate::quat::Quat;

/// Represents a QPSK signal phase in degrees for telecommunication layers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QpskSymbol {
    pub phase_degrees: f32,
}

/// Baseband In-Phase / Quadrature (I/Q) constellation point normalized to +-1.0.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IqSample {
    pub i: f32,
    pub q: f32,
}

impl From<Quat> for QpskSymbol {
    fn from(quat: Quat) -> Self {
        let phase = match quat {
            Quat::V0 => 0.0,
            Quat::V1 => 90.0,
            Quat::V2 => 180.0,
            Quat::V3 => 270.0,
        };
        Self { phase_degrees: phase }
    }
}

/// Converts a quaternary stream into telecommunication phase symbols.
pub fn modulate_stream(stream: &[Quat]) -> Vec<QpskSymbol> {
    stream.iter().map(|&q| QpskSymbol::from(q)).collect()
}

impl Quat {
    /// Maps quaternary symbol to Gray-coded 2-bit constellation index:
    /// V0 (00) -> 00, V1 (01) -> 01, V2 (10) -> 11, V3 (11) -> 10.
    #[inline]
    pub const fn to_gray(self) -> Self {
        match self {
            Quat::V0 => Quat::V0,
            Quat::V1 => Quat::V1,
            Quat::V2 => Quat::V3,
            Quat::V3 => Quat::V2,
        }
    }

    /// Inverse Gray mapping back to natural quaternary symbol.
    #[inline]
    pub const fn from_gray(gray: Self) -> Self {
        match gray {
            Quat::V0 => Quat::V0,
            Quat::V1 => Quat::V1,
            Quat::V3 => Quat::V2,
            Quat::V2 => Quat::V3,
        }
    }
}

impl From<Quat> for IqSample {
    /// Gray-mapped QPSK I/Q constellation mapping:
    /// V0 -> (+1, +1), V1 -> (-1, +1), V2 -> (-1, -1), V3 -> (+1, -1)
    fn from(quat: Quat) -> Self {
        match quat.to_gray() {
            Quat::V0 => Self { i: 1.0, q: 1.0 },
            Quat::V1 => Self { i: -1.0, q: 1.0 },
            Quat::V3 => Self { i: -1.0, q: -1.0 },
            Quat::V2 => Self { i: 1.0, q: -1.0 },
        }
    }
}

impl IqSample {
    /// Hard-decision QPSK demodulator: decides the nearest quadrant and returns natural `Quat`.
    #[inline]
    pub fn demodulate_hard(self) -> Quat {
        let gray = match (self.i >= 0.0, self.q >= 0.0) {
            (true, true) => Quat::V0,
            (false, true) => Quat::V1,
            (false, false) => Quat::V3,
            (true, false) => Quat::V2,
        };
        Quat::from_gray(gray)
    }
}

/// Modulates a quaternary stream into Gray-coded I/Q baseband samples.
pub fn modulate_iq_stream(stream: &[Quat]) -> Vec<IqSample> {
    stream.iter().map(|&q| IqSample::from(q)).collect()
}

/// Demodulates noisy I/Q baseband samples back into quaternary symbols using hard decision.
pub fn demodulate_iq_stream(samples: &[IqSample]) -> Vec<Quat> {
    samples.iter().map(|s| s.demodulate_hard()).collect()
}