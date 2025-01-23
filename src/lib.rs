//! # QR Code Generator Library
//!
//! This library provides functionality to create QR codes with different error correction levels.
//!
//! ## Modules
//!
//! - `bit_utils`: Utility functions for bit manipulation.
//! - `galios`: Functions for Galois field arithmetic.
//! - `qr_code`: Core QR code generation logic.
//!
//! ## Functions
//!
//! ### `create_qr_code`
//!
//! ```rust
//! pub fn create_qr_code(data: &str, error_correction_level: ErrorCorrectionLevel) -> BitMap
//! ```
//!
//! Creates a QR code from the given data and error correction level.
//!
//! #### Parameters
//!
//! - `data`: A string slice that holds the data to be encoded in the QR code.
//! - `error_correction_level`: The desired level of error correction for the QR code.
//!
//! #### Returns
//!
//! A `BitMap` representing the generated QR code.
//!
//! ## Examples
//!
//! ```rust
//! use qr_code::*;
//!
//! let qr_code = create_qr_code("HELLO, WORLD!", ErrorCorrectionLevel::Q);
//! println!("{}", qr_code);
//! ```

#![allow(non_snake_case)]

pub use bit_utils::bitmap::BitMap;
pub use qr_code::ErrorCorrectionLevel;
use qr_code::QRMode;

mod bit_utils;
mod galios;
mod qr_code;

/// Creates a QR code bitmap from input data with specified error correction
///
/// This function generates a complete QR code bitmap by performing the following steps:
/// 1. Analyze the input data to determine the appropriate encoding mode
/// 2. Encode the data into a bitstring
/// 3. Generate error correction codes
/// 4. Structure the data and error correction codewords
/// 5. Create the final QR code bitmap
///
/// # Arguments
///
/// * `data` - The string data to be encoded in the QR code
/// * `error_correction_level` - The error correction level for redundancy and recovery
///
/// # Returns
///
/// A `BitMap` representing the fully generated QR code
///
/// # Errors
///
/// Panics if the data cannot be encoded or exceeds QR code capacity
///
/// # Examples
///
/// ```
/// use qr_code::{create_qr_code, ErrorCorrectionLevel};
///
/// let qr_code = create_qr_code("HELLO, WORLD!", ErrorCorrectionLevel::Q);
/// ```
///
/// # Performance
///
/// The function's complexity depends on the data length and chosen error correction level
pub fn create_qr_code(data: &str, error_correction_level: ErrorCorrectionLevel) -> BitMap {
    let mut qr_code = QRMode::analyze_data(data, error_correction_level);
    let mut bits = qr_code.encode();
    let qr_data = qr_code.generate_error_correction(bits);
    bits = qr_code.structure_codewords(qr_data);

    qr_code.create_bit_map(bits)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_qr_code() {
        println!(
            "{}",
            create_qr_code("HELLO, WORLD!", ErrorCorrectionLevel::Q)
        );
    }
}
