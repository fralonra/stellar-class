use thiserror::Error;

/// Error types.
#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// The input classification string contains non-ASCII characters.
    #[error("Invalid class string: not all chars are ascii")]
    InvalidStringNonAscii,

    /// The input classification string is too short.
    /// A valid classification must have at least 3 characters (e.g., `O1V`).
    #[error("Invalid class string: must contain at least 3 chars")]
    InvalidStringTooShort,

    /// The subtype part of the classification could is invalid.
    ///
    /// Subtypes must be a valid float between 0.0 and 9.9 (e.g. `1`, `0.5`),
    /// and do not include redundant zeros or dots.
    #[error("Invalid subtype")]
    InvalidSubtype,

    /// The spectral type letter is not recognized.
    ///
    /// Valid spectral types are O, B, A, F, G, K, M (uppercase).
    #[error("Invalid spectral type")]
    InvalidSpectralType,

    /// The luminosity class is not supported or recognized.
    ///
    /// Standard classes like `Ia`, `III`, `V` are supported.
    /// Non-standard formats are not supported yet.
    #[error("Unsupported luminosity class")]
    InvalidLuminosityClass,
}
