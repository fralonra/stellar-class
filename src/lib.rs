//! # stellar-class
//!
//! Morgan-Keenan (MK) Spectral Classification representations.
//! Noted that this is not an academic project.
//!
//! Covers the standard classes.
//! Spectral peculiarities are not supported yet.

mod classification;
mod error;
mod luminosity;
mod spectral_types;

pub use classification::Classification;
pub use error::Error;
pub use luminosity::LuminosityClass;
pub use spectral_types::SpectralType;
