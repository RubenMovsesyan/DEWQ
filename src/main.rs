use bit_string::{Bit, BitString};
use qr_code::{Encodable, QRMode};

mod qr_code;
mod bit_string;

fn main() {
    let my_qr = QRMode::analyze_data("HELLO WORLD");

    println!("{}", my_qr.encode());
}
