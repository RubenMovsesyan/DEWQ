#![no_std]

extern crate alloc;


#[cfg(test)]
extern crate std;

#[cfg(test)]
#[macro_use]
use std::println;

use qr_code::{ErrorCorrectionLevel, QRMode};

mod qr_code;
mod bit_utils;
mod galios;

fn test() {
    // println!("{}", bits);
    // println!("length: {}", bits.len());

    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_5_q() {
        let mut my_qr = QRMode::analyze_data("TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT", ErrorCorrectionLevel::Q);

        let mut bits = my_qr.encode();
        let qr_data = my_qr.generate_error_correction(bits);
        println!("{:?}", qr_data);
        bits = my_qr.structure_codewords(qr_data);
        println!("{}", bits);
    }
}
