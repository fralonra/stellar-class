use crate::error::Error;

/// The main spectral types in the MK system.
/// Covers the standard OBAFGKM sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SpectralType {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

impl TryFrom<&str> for SpectralType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let spectral_type = match value {
            "O" => SpectralType::O,
            "B" => SpectralType::B,
            "A" => SpectralType::A,
            "F" => SpectralType::F,
            "G" => SpectralType::G,
            "K" => SpectralType::K,
            "M" => SpectralType::M,
            _ => return Err(Error::InvalidSpectralType),
        };

        Ok(spectral_type)
    }
}

impl Into<&'static str> for SpectralType {
    fn into(self) -> &'static str {
        match self {
            SpectralType::O => "O",
            SpectralType::B => "B",
            SpectralType::A => "A",
            SpectralType::F => "F",
            SpectralType::G => "G",
            SpectralType::K => "K",
            SpectralType::M => "M",
        }
    }
}
