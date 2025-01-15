pub use bit_utils::bitmap::BitMap;
use qr_code::{ErrorCorrectionLevel, QRMode};

mod bit_utils;
mod galios;
mod qr_code;

pub fn create_qr_code(data: &str, error_correction_level: u8) -> BitMap {
    let mut qr_code = QRMode::analyze_data(
        data,
        match error_correction_level {
            0 => ErrorCorrectionLevel::L,
            1 => ErrorCorrectionLevel::M,
            2 => ErrorCorrectionLevel::Q,
            _ => ErrorCorrectionLevel::H,
        },
    );

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
        println!("{}", create_qr_code("Hello, World!", 2));
    }
}
