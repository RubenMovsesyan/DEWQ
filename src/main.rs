#![no_std]

extern crate alloc;

#[cfg(any(
        test,
        feature = "test_feature"
))]
extern crate std;

#[cfg(any(
        test,
        feature = "test_feature"
))]
#[macro_use]
use std::println;

#[cfg(any(
        test,
        feature = "test_feature"
))]
#[macro_use]
use std::print;



use bit_utils::bitmap::*;
use qr_code::{ErrorCorrectionLevel, QRMode};

mod test_utils;
mod qr_code;
mod bit_utils;
mod galios;

fn main() {
    let mut my_qr = QRMode::analyze_data("HELLO WORLD HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORL HELLO WORLD", ErrorCorrectionLevel::Q);
    // let mut my_qr = QRMode::analyze_data("HELLO WORLD", ErrorCorrectionLevel::Q);
    let mut bits = my_qr.encode();
    let qr_data = my_qr.generate_error_correction(bits);
    bits = my_qr.structure_codewords(qr_data);

    my_qr.create_bit_map(bits);
}
