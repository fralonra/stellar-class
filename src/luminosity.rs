use crate::error::Error;

/// Luminosity classes in the MK system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LuminosityClass {
    /// 0 or Ia+, hypergiants or extremely luminous supergiants
    Zero,
    /// Luminous supergiants
    Ia,
    /// Intermediate-size luminous supergiants
    Iab,
    /// Less luminous supergiants
    Ib,
    /// Bright giants
    II,
    /// Normal giants
    III,
    /// Subgiants
    IV,
    /// Main-sequence stars (dwarfs)
    V,
    /// Subdwarfs
    VI,
    /// White dwarfs
    VII,
}

impl TryFrom<&str> for LuminosityClass {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let luminosity_class = match value {
            "0" | "Ia+" => LuminosityClass::Zero,
            "Ia" => LuminosityClass::Ia,
            "Iab" => LuminosityClass::Iab,
            "Ib" => LuminosityClass::Ib,
            "II" => LuminosityClass::II,
            "III" => LuminosityClass::III,
            "IV" => LuminosityClass::IV,
            "V" => LuminosityClass::V,
            "VI" => LuminosityClass::VI,
            "VII" => LuminosityClass::VII,
            _ => return Err(Error::InvalidLuminosityClass),
        };

        Ok(luminosity_class)
    }
}

impl Into<&'static str> for LuminosityClass {
    fn into(self) -> &'static str {
        match self {
            LuminosityClass::Zero => "0",
            LuminosityClass::Ia => "Ia",
            LuminosityClass::Iab => "Iab",
            LuminosityClass::Ib => "Ib",
            LuminosityClass::II => "II",
            LuminosityClass::III => "III",
            LuminosityClass::IV => "IV",
            LuminosityClass::V => "V",
            LuminosityClass::VI => "VI",
            LuminosityClass::VII => "VII",
        }
    }
}
