use qr_code::{ErrorCorrectionLevel, QRMode};
use std::env;

mod bit_utils;
mod galios;
mod qr_code;

fn main() {
    let args: Vec<String> = env::args().collect();

    let data = &args[1];
    let error_correction_level = &args[2];
    let save_path = &args[3];

    let mut qr_code = QRMode::analyze_data(
        data.as_str(),
        match error_correction_level.as_str() {
            "L" | "l" => ErrorCorrectionLevel::L,
            "M" | "m" => ErrorCorrectionLevel::M,
            "Q" | "q" => ErrorCorrectionLevel::Q,
            "H" | "h" => ErrorCorrectionLevel::H,
            _ => panic!(),
        },
    );
    let mut bits = qr_code.encode();
    let qr_data = qr_code.generate_error_correction(bits);
    bits = qr_code.structure_codewords(qr_data);

    let bitmap = qr_code.create_bit_map(bits);
    bitmap.save_to_file(save_path.as_str());
}
