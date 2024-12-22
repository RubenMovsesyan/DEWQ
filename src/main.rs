use qr_code::{ErrorCorrectionLevel, QRMode};

mod qr_code;
mod bit_string;
mod galios;

fn main() {
    let mut my_qr = QRMode::analyze_data("HELLO WORLD", ErrorCorrectionLevel::L);
    
    let bits = my_qr.encode();
    println!("{}", bits);
    println!("length: {}", bits.len());

    my_qr.generate_error_correction(bits);
}
