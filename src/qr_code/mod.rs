use crate::bit_string::*;
use crate::galios::*;
use non_std::Vec;
use crate::alloc::vec;

#[cfg(test)]
extern crate std;

#[cfg(test)]
#[macro_use]
use std::println;


#[cfg(test)]
#[macro_use]
use std::print;

// Constants
use crate::qr_code::constants::*;

pub mod constants;

#[derive(PartialEq, Eq, Debug)]
pub enum QRMode {
    Numeric(Vec<u8>),
    AlphaNumeric(AlphaNumericQrCode),
    Byte(Vec<u8>),
    // Kanji(Vec<u16>), // Double byte mode
}

#[derive(PartialEq, Eq, Debug)]
pub struct AlphaNumericQrCode {
    data: Vec<u8>,
    version: usize,
    error_correction_level: ErrorCorrectionLevel,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ErrorCorrectionLevel {
    L,
    M,
    Q,
    H,
}

// ------------------ Helper functions ----------------------

fn is_numeric(input: &str) -> bool {
    for character in input.chars() {
        if !character.is_ascii_digit() {
            return false;
        }
    }

    true
}

fn is_alphanumeric(input: &str) -> bool {
    for character in input.bytes() {
        if !(
            character == 32                     // space
        || (character > 35 && character < 38)   // $ and %
        || (character > 41 && character < 44)   // * and +
        || (character > 44 && character < 59)   // -, ., /, numerals, and :
        || (character > 64 && character < 90)   // Capital Letters
        ) {
            return false;
        }
    }

    true
}

// ----------------------------------------------------------

impl QRMode {
    pub fn analyze_data<'a, S>(input: S, error_correction_level: ErrorCorrectionLevel) -> QRMode 
        where S: Into<&'a str>
    {
        let converted_input: &str = input.into();
        
        if is_numeric(&converted_input) {
            let mut digit_buffer: Vec<u8> = Vec::with_capacity(converted_input.len());
            for i in 0..converted_input.len() {
                // This is to bypass any unnecessary checking
                unsafe {
                    digit_buffer.push(converted_input[i..=i].parse().unwrap_unchecked());
                }
            }
            return QRMode::Numeric(digit_buffer);
        } else if is_alphanumeric(&converted_input) {
            let mut data: Vec<u8> = Vec::with_capacity(converted_input.len());
            for character in converted_input.bytes() {
                if character > 47 && character < 58 {
                    data.push(character - 48);
                } else if character > 64 && character < 91 {
                    data.push(character - 55);
                } else if character == 32 {
                    data.push(36);
                } else if character > 35 && character < 38 {
                    data.push(character + 1);
                } else if character > 41 && character < 44 {
                    data.push(character - 2);
                } else if character > 44 && character < 48 {
                    data.push(character - 4);
                } else {
                    data.push(44);
                }
            }

            let version = {
                let mut out: usize = 0;
                for version_index in (0..MAX_VERSION).rev() {
                    match error_correction_level {
                        ErrorCorrectionLevel::L => {
                            if data.len() > ALPHA_NUMERIC_L_MAX_CAPACITY[version_index] {
                                out = version_index + 1;
                                break;
                            }
                        }
                        ErrorCorrectionLevel::M => {
                            if data.len() > ALPHA_NUMERIC_M_MAX_CAPACITY[version_index] {
                                out = version_index + 1;
                                break;
                            }
                        },
                        ErrorCorrectionLevel::Q => {
                            if data.len() > ALPHA_NUMERIC_Q_MAX_CAPACITY[version_index] {
                                out = version_index + 1;
                                break;
                            }
                        },
                        ErrorCorrectionLevel::H => {
                            if data.len() > ALPHA_NUMERIC_H_MAX_CAPACITY[version_index] {
                                out = version_index + 1;
                                break;
                            }
                        },
                    }
                }
                out
            };

            return QRMode::AlphaNumeric(AlphaNumericQrCode {
                data,
                version,
                error_correction_level
            });
        } else if converted_input.is_ascii() {
            return QRMode::Byte(converted_input.bytes().collect());
        }

        todo!()
    }
    

    pub fn encode(&mut self) -> BitString {
        let mut bit_string: BitString = BitString::new();
        match self {
            QRMode::Numeric(_numbers) => {
                todo!()
            },
            QRMode::AlphaNumeric(alpha_numeric_qr_code) => {
                // Adding the mode indicator
                bit_string.push_bit(0);
                bit_string.push_bit(0);
                bit_string.push_bit(1);
                bit_string.push_bit(0);
                
                let size_of_character_length_bits = {
                    let out: usize;
                    if (alpha_numeric_qr_code.version + 1) < 10 {
                        out = 9;
                    } else if (alpha_numeric_qr_code.version + 1) > 9 && (alpha_numeric_qr_code.version + 1) < 27 {
                        out = 11;
                    } else {
                        out = 13
                    }

                    out
                };

                // Encode the character count
                for i in (0..size_of_character_length_bits).rev() {
                    bit_string.push_bit((alpha_numeric_qr_code.data.len() & (1 << i)) as i32);
                }

                // Encode the values
                for value_index in (0..alpha_numeric_qr_code.data.len()).step_by(2) {
                    // Since stepping by 2 we know that this is always going to be Some()
                    let first_value = unsafe { alpha_numeric_qr_code.data.get(value_index).unwrap_unchecked() };
                    
                    match alpha_numeric_qr_code.data.get(value_index + 1) {
                        Some(second_value) => {
                            let encoded = (*first_value as u16 * 45) + *second_value as u16;
                            
                            // 11 bits for double characters
                            for i in (0..11).rev() {
                                bit_string.push_bit(encoded & (1 << i));
                            }
                        },
                        None => {
                            let encoded = *first_value;

                            // 6 bits for single characters
                            for i in (0..6).rev() {
                                bit_string.push_bit(encoded & (1 << i));
                            }
                        }
                    }
                }

                // Add terminator 0s if necessary
                let required_number_of_bits = {
                    match alpha_numeric_qr_code.error_correction_level {
                        ErrorCorrectionLevel::L => {
                            L_NUM_CODEWORDS[alpha_numeric_qr_code.version] * 8
                        },
                        ErrorCorrectionLevel::M => {
                            M_NUM_CODEWORDS[alpha_numeric_qr_code.version] * 8
                        },
                        ErrorCorrectionLevel::Q => {
                            Q_NUM_CODEWORDS[alpha_numeric_qr_code.version] * 8
                        },
                        ErrorCorrectionLevel::H => {
                            H_NUM_CODEWORDS[alpha_numeric_qr_code.version] * 8
                        },
                    }
                };

                let total_bits = bit_string.len() - 4 - (size_of_character_length_bits);
                let bit_difference = required_number_of_bits - total_bits;

                for _ in 0..bit_difference.min(4) {
                    bit_string.push_bit(0);
                }

                // Make sure the bitstring is a multiple of 8
                while bit_string.len() % 8 != 0 {
                    bit_string.push_bit(0);
                }
                
                // Add the necessary pad bytes
                while bit_string.len() < required_number_of_bits {
                    // append binary 236
                    bit_string.push_bit(1);
                    bit_string.push_bit(1);
                    bit_string.push_bit(1);
                    bit_string.push_bit(0);
                    bit_string.push_bit(1);
                    bit_string.push_bit(1);
                    bit_string.push_bit(0);
                    bit_string.push_bit(0);

                    // Check if the bitsting is long enough
                    if bit_string.len() >= required_number_of_bits {
                        break;
                    }

                    // append binary 17
                    bit_string.push_bit(0);
                    bit_string.push_bit(0);
                    bit_string.push_bit(0);
                    bit_string.push_bit(1);
                    bit_string.push_bit(0);
                    bit_string.push_bit(0);
                    bit_string.push_bit(0);
                    bit_string.push_bit(1);
                }


                return bit_string;
            },
            QRMode::Byte(_bytes) => {
                todo!()
            }
        }
    }

    pub fn generate_error_correction(&self, bits: BitString) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
        let mut data: Vec<Vec<u8>> = Vec::new();
        let mut error_correction_data: Vec<Vec<u8>> = Vec::new();
        
        let (
            num_blocks_group_1,
            num_code_words_group_1,
            num_blocks_group_2,
            num_code_words_group_2
        ) = {
            let (version, error_correction_level) = {
                match self {
                    QRMode::Numeric(_data) => { todo!() },
                    QRMode::AlphaNumeric(alpha_numeric_qr_code) => { (alpha_numeric_qr_code.version, &alpha_numeric_qr_code.error_correction_level) },
                    QRMode::Byte(_data) => { todo!() },
                }
            };

            match error_correction_level {
                ErrorCorrectionLevel::L => {
                    (
                        NUM_ERROR_CORRECTION_BLOCKS_GROUP_1_L[version],
                        NUM_CODE_WORDS_PER_BLOCK_GROUP_1_L[version],
                        NUM_ERROR_CORRECTION_BLOCKS_GROUP_2_L[version],
                        NUM_CODE_WORDS_PER_BLOCK_GROUP_2_L[version],
                    )
                },
                ErrorCorrectionLevel::M => {
                    (
                        NUM_ERROR_CORRECTION_BLOCKS_GROUP_1_M[version],
                        NUM_CODE_WORDS_PER_BLOCK_GROUP_1_M[version],
                        NUM_ERROR_CORRECTION_BLOCKS_GROUP_2_M[version],
                        NUM_CODE_WORDS_PER_BLOCK_GROUP_2_M[version],
                    )
                },
                ErrorCorrectionLevel::Q => {
                    (
                        NUM_ERROR_CORRECTION_BLOCKS_GROUP_1_Q[version],
                        NUM_CODE_WORDS_PER_BLOCK_GROUP_1_Q[version],
                        NUM_ERROR_CORRECTION_BLOCKS_GROUP_2_Q[version],
                        NUM_CODE_WORDS_PER_BLOCK_GROUP_2_Q[version],
                    )
                },
                ErrorCorrectionLevel::H => {
                    (
                        NUM_ERROR_CORRECTION_BLOCKS_GROUP_1_H[version],
                        NUM_CODE_WORDS_PER_BLOCK_GROUP_1_H[version],
                        NUM_ERROR_CORRECTION_BLOCKS_GROUP_2_H[version],
                        NUM_CODE_WORDS_PER_BLOCK_GROUP_2_H[version],
                    )
                },
            }
        };

        // Create all the message polynomials from the data codewords
        let mut message_polynomials: Vec<GeneratorPolynomial> = Vec::new();
        let mut index = 0;

        for _ in 0..num_blocks_group_1 {
            let mut block: Vec<u8> = Vec::new();
            for _ in 0..num_code_words_group_1 {
                block.push(bits.get_byte(index));
                index += 1;
            }


            data.push(Vec::from(block.clone()));
            message_polynomials.push(GeneratorPolynomial::from(block));
        }

        for _ in 0..num_blocks_group_2 {
            let mut block = Vec::new();
            for _ in 0..num_code_words_group_2 {
                block.push(bits.get_byte(index));
                index += 1;
            }

            data.push(Vec::from(block.clone()));
            message_polynomials.push(GeneratorPolynomial::from(block));
        }

        for message_polynomial in message_polynomials {
            let generator_polynomial = {
                let mut poly = GeneratorPolynomial::from(vec![0, 0]);

                let num_error_codewords = {
                    match self {
                        QRMode::Numeric(_data) => todo!(),
                        QRMode::AlphaNumeric(alpha_numeric_qr_code) => {
                            match alpha_numeric_qr_code.error_correction_level {
                                ErrorCorrectionLevel::L => L_ERROR_CORRECTION_CODE_WORDS[alpha_numeric_qr_code.version],
                                ErrorCorrectionLevel::M => M_ERROR_CORRECTION_CODE_WORDS[alpha_numeric_qr_code.version],
                                ErrorCorrectionLevel::Q => Q_ERROR_CORRECTION_CODE_WORDS[alpha_numeric_qr_code.version],
                                ErrorCorrectionLevel::H => H_ERROR_CORRECTION_CODE_WORDS[alpha_numeric_qr_code.version],
                            }
                        },
                        QRMode::Byte(_data) => todo!(),
                    }
                };

                for i in 1..num_error_codewords {
                    poly = poly.multiply_as_exponents(&GeneratorPolynomial::from(vec![0, i as i32]));
                }


                poly
            };

            
            // Perform the long division on the message polynomial with the generator polynomial
            let size = message_polynomial.get().len();
            let mut current_xor = message_polynomial.to_exponent_notation();

            for _ in 0..size {
                // Multiply the generator polynomial by the leading term of the message polynomial
                let mut inter_poly = generator_polynomial.multiply_by_exponent(current_xor.get()[0]);
                inter_poly = current_xor.to_integer_notation().xor_with_other(&inter_poly.to_integer_notation());
                inter_poly = inter_poly.drop_leading_zeros();
                current_xor = inter_poly.to_exponent_notation();
            }

            // Current xor is the error correction codewords to use in exponent notation
            let error_correction_nums = current_xor.to_integer_notation();

            // WARNING: if this doesn't give the correct result
            // then try changing the logic in push_byte to push it
            // in reverse order instead
            // for num in error_correction_nums.get() {
            //     error_correction_data.push(*num as u8);
            // }
            let u8_error_correction_nums = {
                let mut output: Vec<u8> = Vec::new();

                for elem in error_correction_nums.get() {
                    output.push(*elem as u8);
                }

                output
            };

            error_correction_data.push(u8_error_correction_nums);
        }


        (data, error_correction_data)
    }

    pub fn structure_codewords(&self, data: (Vec<Vec<u8>>, Vec<Vec<u8>>)) -> BitString {
        let mut new_data: Vec<u8> = Vec::new();
        let max_index = data.0[0].len().max(data.0[1].len()).max(data.0[2].len()).max(data.0[3].len());
        // Interleave the data codewords
        for i in 0..max_index {
            if data.0[0].len() > i {
                new_data.push(data.0[0][i]);
            }

            if data.0[1].len() > i {
                new_data.push(data.0[1][i]);
            }

            if data.0[2].len() > i {
                new_data.push(data.0[2][i]);
            }

            if data.0[3].len() > i {
                new_data.push(data.0[3][i]);
            }
        }

        let max_index = data.1[0].len().max(data.1[1].len()).max(data.1[2].len()).max(data.1[3].len());
        // Interleave the error codewords
        for i in 0..max_index {
            if data.1[0].len() > i {
                new_data.push(data.1[0][i]);
            }

            if data.1[1].len() > i {
                new_data.push(data.1[1][i]);
            }

            if data.1[2].len() > i {
                new_data.push(data.1[2][i]);
            }

            if data.1[3].len() > i {
                new_data.push(data.1[3][i]);
            }
        }
        
        #[cfg(test)]
        println!("{:?}", new_data);

        // Turn into a final bitstring
        let mut new_bitstring = BitString::new();
        for byte in new_data {
            new_bitstring.push_byte(byte);
        }
        new_bitstring


        // Get the variables for the structure from the constants
        // let (
        //     num_blocks_group_1,
        //     num_code_words_group_1,
        //     num_blocks_group_2,
        //     num_code_words_group_2
        // ) = {
        //     let (version, error_correction_level) = {
        //         match self {
        //             QRMode::Numeric(_data) => { todo!() },
        //             QRMode::AlphaNumeric(alpha_numeric_qr_code) => { (alpha_numeric_qr_code.version, &alpha_numeric_qr_code.error_correction_level) },
        //             QRMode::Byte(_data) => { todo!() },
        //         }
        //     };

        //     match error_correction_level {
        //         ErrorCorrectionLevel::L => {
        //             (
        //                 NUM_ERROR_CORRECTION_BLOCKS_GROUP_1_L[version],
        //                 NUM_CODE_WORDS_PER_BLOCK_GROUP_1_L[version],
        //                 NUM_ERROR_CORRECTION_BLOCKS_GROUP_2_L[version],
        //                 NUM_CODE_WORDS_PER_BLOCK_GROUP_2_L[version],
        //             )
        //         },
        //         ErrorCorrectionLevel::M => {
        //             (
        //                 NUM_ERROR_CORRECTION_BLOCKS_GROUP_1_M[version],
        //                 NUM_CODE_WORDS_PER_BLOCK_GROUP_1_M[version],
        //                 NUM_ERROR_CORRECTION_BLOCKS_GROUP_2_M[version],
        //                 NUM_CODE_WORDS_PER_BLOCK_GROUP_2_M[version],
        //             )
        //         },
        //         ErrorCorrectionLevel::Q => {
        //             (
        //                 NUM_ERROR_CORRECTION_BLOCKS_GROUP_1_Q[version],
        //                 NUM_CODE_WORDS_PER_BLOCK_GROUP_1_Q[version],
        //                 NUM_ERROR_CORRECTION_BLOCKS_GROUP_2_Q[version],
        //                 NUM_CODE_WORDS_PER_BLOCK_GROUP_2_Q[version],
        //             )
        //         },
        //         ErrorCorrectionLevel::H => {
        //             (
        //                 NUM_ERROR_CORRECTION_BLOCKS_GROUP_1_H[version],
        //                 NUM_CODE_WORDS_PER_BLOCK_GROUP_1_H[version],
        //                 NUM_ERROR_CORRECTION_BLOCKS_GROUP_2_H[version],
        //                 NUM_CODE_WORDS_PER_BLOCK_GROUP_2_H[version],
        //             )
        //         },
        //     }
        // };
        
        // if num_blocks_group_2 != 0 {
        //     let mut blocks: Vec<Vec<u8>> = Vec::new();
        //     
        //     let mut index = 0;
        //     for _ in 0..num_blocks_group_1 {
        //         let mut block = Vec::new();
        //         for _ in 0..num_code_words_group_1 {
        //             block.push(bits.get_byte(index));
        //             index += 1;
        //         }
        //         blocks.push(block);
        //     }

        //     for _ in 0..num_blocks_group_2 {
        //         let mut block = Vec::new();
        //         for _ in 0..num_code_words_group_2 {
        //             block.push(bits.get_byte(index));
        //             index += 1;
        //         }
        //         blocks.push(block);
        //     }

        //     let mut new_bitstring = BitString::new();

        //     for block in blocks {
        //         for byte in block {
        //             new_bitstring.push_byte(byte);
        //         }
        //     }

        //     return new_bitstring;
        // }

        // bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_modes() {
        let qr_mode = QRMode::analyze_data("123", ErrorCorrectionLevel::L);
        assert_eq!(qr_mode, QRMode::Numeric(vec![1, 2, 3]));

        let qr_mode = QRMode::analyze_data("A113", ErrorCorrectionLevel::L);
        assert_eq!(qr_mode, QRMode::AlphaNumeric(AlphaNumericQrCode {
            data: vec![10, 1, 1, 3],
            version: 0,
            error_correction_level: ErrorCorrectionLevel::L
        }));

        let qr_mode = QRMode::analyze_data("a113", ErrorCorrectionLevel::L);
        assert_eq!(qr_mode, QRMode::Byte(vec![97, 49, 49, 51]));
    }

    #[test]
    fn test_qr_error_codes() {
        let mut qr_mode = QRMode::analyze_data("HELLO WORLD", ErrorCorrectionLevel::M);
        let bits = qr_mode.encode();
        println!("Bits pre {}", bits);
        let data = qr_mode.generate_error_correction(bits);
        println!("data post {:?}", data);
    }
}
