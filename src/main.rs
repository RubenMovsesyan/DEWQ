#![no_std]

extern crate alloc;

#[cfg(any(test, feature = "test_feature"))]
extern crate std;

#[cfg(any(test, feature = "test_feature"))]
#[macro_use]
use std::println;

#[cfg(any(test, feature = "test_feature"))]
#[macro_use]
use std::print;

use bit_utils::bitmap::*;
use qr_code::{ErrorCorrectionLevel, QRMode};

mod bit_utils;
mod galios;
mod qr_code;
mod test_utils;

fn main() {
    let mut my_qr = QRMode::analyze_data("9523524937562398569286786238946567892648756238745623648756238465689236489620465034502364513045312457892324365620454017614016401654365023497567832568923645632465134298632459623896458976234786234987623458764529456782347862345962345786328462395634578264827845629836458628945689236478562896489756238746587236485623786458", ErrorCorrectionLevel::Q);
    let mut bits = my_qr.encode();
    let qr_data = my_qr.generate_error_correction(bits);
    bits = my_qr.structure_codewords(qr_data);

    my_qr.create_bit_map(bits);

    // let mut my_qr = QRMode::analyze_data("HELLO WORLD HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORLD", ErrorCorrectionLevel::Q);
    // // let mut my_qr = QRMode::analyze_data("HELLO WORLD", ErrorCorrectionLevel::Q);
    // let mut bits = my_qr.encode();
    // let qr_data = my_qr.generate_error_correction(bits);
    // bits = my_qr.structure_codewords(qr_data);

    // my_qr.create_bit_map(bits);

    // let mut bitmap = BitMap::new(10);
    // bitmap.invert();
    // bitmap.set(0, 9, 0);
    // bitmap.set(0, 0, 0);
    // test_println!("{}", bitmap);
}
