// #![no_std]

// extern crate alloc;

// #[cfg(any(test, feature = "test_feature"))]
// extern crate std;

// #[cfg(any(test, feature = "test_feature"))]
// #[macro_use]
// use std::println;

// #[cfg(any(test, feature = "test_feature"))]
// #[macro_use]
// use std::print;

use qr_code::{ErrorCorrectionLevel, QRMode};

mod bit_utils;
mod galios;
mod qr_code;

fn main() {
    let mut my_qr = QRMode::analyze_data("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", ErrorCorrectionLevel::Q);
    let mut bits = my_qr.encode();
    let qr_data = my_qr.generate_error_correction(bits);
    bits = my_qr.structure_codewords(qr_data);

    println!("{}", my_qr.create_bit_map(bits));

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
