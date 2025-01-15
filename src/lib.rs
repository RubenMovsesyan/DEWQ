pub use bit_utils::bitmap::BitMap;
pub use qr_code::ErrorCorrectionLevel;
use qr_code::QRMode;

mod bit_utils;
mod galios;
mod qr_code;

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
