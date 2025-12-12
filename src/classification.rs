use crate::{error::Error, luminosity::LuminosityClass, spectral_types::SpectralType};

/// Represents a MK stellar classification.
/// Peculiarities not yet supported.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Classification {
    pub spectral_type: SpectralType,
    pub subtype: f32,
    pub luminosity_class: LuminosityClass,
}

impl Classification {
    /// Parses a string into the struct.
    pub fn from_string(class_str: &str) -> Result<Self, Error> {
        let class_str = preprocess_class_str(class_str)?;

        // Spectral type
        let (spectral_str, remaining) = class_str.split_at(1);
        let spectral_type = spectral_str.try_into()?;

        // Subtype
        let mut chars = remaining.chars();

        let mut subtype_str = String::new();

        let first_digit = chars.next().unwrap();
        if !first_digit.is_digit(10) {
            return Err(Error::InvalidSubtype);
        }
        subtype_str.push(first_digit);

        let possible_dot = chars.next().unwrap();
        if possible_dot == '.' {
            subtype_str.push(possible_dot);

            while let Some(next_digit) = chars.next() {
                if !next_digit.is_digit(10) {
                    break;
                }

                subtype_str.push(next_digit);
            }
        }

        if subtype_str == "00" {
            let _ = subtype_str.pop();
        }

        if subtype_str.ends_with('.') || subtype_str.ends_with("00") {
            return Err(Error::InvalidSubtype);
        }

        let subtype = subtype_str
            .parse::<f32>()
            .map_err(|_| Error::InvalidSubtype)?;
        if subtype == 0.0 && subtype_str.contains('.') {
            return Err(Error::InvalidSubtype);
        }

        let remaining = remaining.split_at(subtype_str.len()).1;

        // Luminosity class
        let mut luminosity_class = None;
        for lum_len in (1..remaining.len() + 1).rev() {
            if let Ok(lum_class) = remaining.get(0..lum_len).unwrap().try_into() {
                luminosity_class = Some(lum_class);
                break;
            }
        }

        let Some(luminosity_class) = luminosity_class else {
            return Err(Error::InvalidLuminosityClass);
        };

        Ok(Classification {
            spectral_type,
            subtype,
            luminosity_class,
        })
    }

    pub fn to_string(&self) -> String {
        let spectral: &str = self.spectral_type.into();

        let subtype = if self.subtype.fract() == 0.0 {
            format!("{:.0}", self.subtype)
        } else {
            format!("{:.1}", self.subtype)
        };

        let luminosity: &str = self.luminosity_class.into();

        format!("{}{}{}", spectral, subtype, luminosity)
    }
}

fn preprocess_class_str(class_str: &str) -> Result<&str, Error> {
    let class_str = class_str.trim();
    if !class_str.is_ascii() {
        return Err(Error::InvalidStringNonAscii);
    }
    if class_str.len() < 3 {
        return Err(Error::InvalidStringTooShort);
    }

    Ok(class_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_o1v() {
        let class_str = "O1V";
        let result = Classification::from_string(class_str).unwrap();
        assert_eq!(result.spectral_type, SpectralType::O);
        assert_eq!(result.subtype, 1.0);
        assert_eq!(result.luminosity_class, LuminosityClass::V);
    }

    #[test]
    fn test_from_string_a0v() {
        let class_str = "A0V";
        let result = Classification::from_string(class_str).unwrap();
        assert_eq!(result.spectral_type, SpectralType::A);
        assert_eq!(result.subtype, 0.0);
        assert_eq!(result.luminosity_class, LuminosityClass::V);
    }

    #[test]
    fn test_from_string_b0_5iv() {
        let class_str = "B0.5IV";
        let result = Classification::from_string(class_str).unwrap();
        assert_eq!(result.spectral_type, SpectralType::B);
        assert_eq!(result.subtype, 0.5);
        assert_eq!(result.luminosity_class, LuminosityClass::IV);
    }

    #[test]
    fn test_from_string_g2v() {
        let class_str = "G2V";
        let result = Classification::from_string(class_str).unwrap();
        assert_eq!(result.spectral_type, SpectralType::G);
        assert_eq!(result.subtype, 2.0);
        assert_eq!(result.luminosity_class, LuminosityClass::V);
    }

    #[test]
    fn test_from_string_m1iab() {
        let class_str = "M1Iab";
        let result = Classification::from_string(class_str).unwrap();
        assert_eq!(result.spectral_type, SpectralType::M);
        assert_eq!(result.subtype, 1.0);
        assert_eq!(result.luminosity_class, LuminosityClass::Iab);
    }

    #[test]
    fn test_from_string_k0iii() {
        let class_str = "K0III";
        let result = Classification::from_string(class_str).unwrap();
        assert_eq!(result.spectral_type, SpectralType::K);
        assert_eq!(result.subtype, 0.0);
        assert_eq!(result.luminosity_class, LuminosityClass::III);
    }

    #[test]
    fn test_from_string_f5vi() {
        let class_str = "F5VI";
        let result = Classification::from_string(class_str).unwrap();
        assert_eq!(result.spectral_type, SpectralType::F);
        assert_eq!(result.subtype, 5.0);
        assert_eq!(result.luminosity_class, LuminosityClass::VI);
    }

    #[test]
    fn test_from_string_with_peculiarities() {
        let class_str = "O1Vpe";
        let result = Classification::from_string(class_str).unwrap();
        assert_eq!(result.spectral_type, SpectralType::O);
        assert_eq!(result.subtype, 1.0);
        assert_eq!(result.luminosity_class, LuminosityClass::V);
    }

    #[test]
    fn test_from_string_invalid_spectral_type() {
        let class_str = "X1V";
        let result = Classification::from_string(class_str);
        assert_eq!(result, Err(Error::InvalidSpectralType));
    }

    #[test]
    fn test_from_string_invalid_subtype() {
        let class_str = "OxV";
        let result = Classification::from_string(class_str);
        assert_eq!(result, Err(Error::InvalidSubtype));
    }

    #[test]
    fn test_from_string_subtype_out_of_range() {
        let class_str = "O11V";
        let result = Classification::from_string(class_str);
        assert_eq!(result, Err(Error::InvalidLuminosityClass));
    }

    #[test]
    fn test_from_string_subtype_out_of_range_decimal() {
        let class_str = "O11.1V";
        let result = Classification::from_string(class_str);
        assert_eq!(result, Err(Error::InvalidLuminosityClass));
    }

    #[test]
    fn test_from_string_redundant_dot() {
        let class_str = "O1.V";
        let result = Classification::from_string(class_str);
        assert_eq!(result, Err(Error::InvalidSubtype));
    }

    #[test]
    fn test_from_string_redundant_zero_and_dot() {
        let class_str = "O0.0V";
        let result = Classification::from_string(class_str);
        assert_eq!(result, Err(Error::InvalidSubtype));
    }

    #[test]
    fn test_from_string_redundant_zeros() {
        let class_str = "O1.00V";
        let result = Classification::from_string(class_str);
        assert_eq!(result, Err(Error::InvalidSubtype));
    }

    #[test]
    fn test_from_string_zero_with_dot() {
        let class_str = "O0.0V";
        let result = Classification::from_string(class_str);
        assert_eq!(result, Err(Error::InvalidSubtype));
    }

    #[test]
    fn test_to_string_o1v() {
        let class = Classification {
            spectral_type: SpectralType::O,
            subtype: 1.0,
            luminosity_class: LuminosityClass::V,
        };
        assert_eq!(class.to_string(), "O1V");
    }

    #[test]
    fn test_to_string_a0v() {
        let class = Classification {
            spectral_type: SpectralType::A,
            subtype: 0.0,
            luminosity_class: LuminosityClass::V,
        };
        assert_eq!(class.to_string(), "A0V");
    }

    #[test]
    fn test_to_string_b0_5iv() {
        let class = Classification {
            spectral_type: SpectralType::B,
            subtype: 0.5,
            luminosity_class: LuminosityClass::IV,
        };
        assert_eq!(class.to_string(), "B0.5IV");
    }

    #[test]
    fn test_to_string_g2v() {
        let class = Classification {
            spectral_type: SpectralType::G,
            subtype: 2.0,
            luminosity_class: LuminosityClass::V,
        };
        assert_eq!(class.to_string(), "G2V");
    }

    #[test]
    fn test_to_string_with_decimal() {
        let class = Classification {
            spectral_type: SpectralType::B,
            subtype: 2.5,
            luminosity_class: LuminosityClass::V,
        };
        assert_eq!(class.to_string(), "B2.5V");
    }
}
