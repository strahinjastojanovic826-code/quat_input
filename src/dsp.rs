use crate::quat::Quat;

/// Represents a QPSK signal phase in degrees for telecommunication layers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QpskSymbol {
    pub phase_degrees: f32,
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