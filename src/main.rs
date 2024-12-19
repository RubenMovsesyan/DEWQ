use qr_code::{ErrorCorrectionLevel, QRMode};

mod qr_code;
mod bit_string;

fn main() {
    let my_qr = QRMode::analyze_data("HELLO WORLD");
    // my_qr.encode(ErrorCorrectionLevel::L);
    
    let bits = my_qr.encode(ErrorCorrectionLevel::L);
    println!("{}", bits);
    println!("length: {}", bits.len());
}
