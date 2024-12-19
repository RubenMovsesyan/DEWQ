use crate::bit_string::*;

// Constants

const MAX_VERSION: usize = 40;
// the total number of codewords for each error correction level
// index 0 = Version 1
// index 39 = Version 40
const L_NUM_CODEWORDS: [usize; MAX_VERSION] = [
    19,
    34,
    55,
    80,
    108,
    136,
    156,
    194,
    232,
    274,
    324,
    370,
    428,
    461,
    523,
    589,
    647,
    721,
    795,
    861,
    932,
    1006,
    1094,
    1174,
    1276,
    1370,
    1468,
    1531,
    1631,
    1735,
    1843,
    1955,
    2071,
    2191,
    2306,
    2434,
    2566,
    2702,
    2812,
    2956,
];

// TODO:
// const M_NUM_CODEWORDS
// const Q_NUM_CODEWORDS
// const H_NUM_CODEWORDS


const ALPHA_NUMERIC_L_MAX_CAPACITY: [usize; MAX_VERSION] = [
    25,
    47,
    77,
    114,
    154,
    195,
    224,
    279,
    335,
    395,
    468,
    535,
    619,
    667,
    758,
    854,
    938,
    1046,
    1153,
    1249,
    1352,
    1460,
    1588,
    1704,
    1853,
    1990,
    2132,
    2223,
    2369,
    2520,
    2677,
    2840,
    3009,
    3183,
    3351,
    3537,
    3729,
    3927,
    4087,
    4296,
];

// TODO:
// const ALPHA_NUMERIC_M_MAX_CAPACITY
// const ALPHA_NUMERIC_Q_MAX_CAPACITY
// const ALPHA_NUMERIC_H_MAX_CAPACITY

// TODO:
// NUMERIC
// BYTE
// KANJI

#[derive(PartialEq, Eq, Debug)]
pub enum QRMode {
    Numeric(Vec<u8>),
    AlphaNumeric(Vec<u8>),
    Byte(Vec<u8>),
    // Kanji(Vec<u16>), // Double byte mode
}

pub enum ErrorCorrectionLevel {
    L,
    M,
    Q,
    H,
}

// ------------------ Helper functions ----------------------

fn is_numeric(input: &String) -> bool {
    for character in input.chars() {
        if !character.is_ascii_digit() {
            return false;
        }
    }

    true
}

fn is_alphanumeric(input: &String) -> bool {
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
    pub fn analyze_data<S>(input: S) -> QRMode 
        where S: Into<String>
    {
        let converted_input: String = input.into();
        
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
            let mut buffer: Vec<u8> = Vec::with_capacity(converted_input.len());
            for character in converted_input.bytes() {
                if character > 47 && character < 58 {
                    buffer.push(character - 48);
                } else if character > 64 && character < 91 {
                    buffer.push(character - 55);
                } else if character == 32 {
                    buffer.push(36);
                } else if character > 35 && character < 38 {
                    buffer.push(character + 1);
                } else if character > 41 && character < 44 {
                    buffer.push(character - 2);
                } else if character > 44 && character < 48 {
                    buffer.push(character - 4);
                } else {
                    buffer.push(44);
                }
            }

            return QRMode::AlphaNumeric(buffer);
        } else if converted_input.is_ascii() {
            return QRMode::Byte(converted_input.bytes().collect());
        }

        todo!()
    }
    

    pub fn encode(&self, error_correction_level: ErrorCorrectionLevel) -> BitString {
        let mut bit_string: BitString = BitString::new();
        match self {
            QRMode::Numeric(numbers) => {
                todo!()
            },
            QRMode::AlphaNumeric(alpha_numeric_values) => {
                // Adding the mode indicator
                bit_string.push_bit(0);
                bit_string.push_bit(0);
                bit_string.push_bit(1);
                bit_string.push_bit(0);
                
                let version = {
                    match error_correction_level {
                        ErrorCorrectionLevel::L => {
                            let mut out: usize = 0;
                            for version_index in (0..MAX_VERSION).rev() {
                                if alpha_numeric_values.len() > ALPHA_NUMERIC_L_MAX_CAPACITY[version_index] {
                                    out = version_index + 1;
                                    break;
                                }
                            }

                            out
                        },
                        ErrorCorrectionLevel::M => {
                            todo!()
                        },
                        ErrorCorrectionLevel::Q => {
                            todo!()
                        },
                        ErrorCorrectionLevel::H => {
                            todo!()
                        },
                    }
                };

                let size_of_character_length_bits = {
                    let out: usize;
                    if (version + 1) < 10 {
                        out = 9;
                    } else if (version + 1) > 9 && (version + 1) < 27 {
                        out = 11;
                    } else {
                        out = 13
                    }

                    out
                };

                // Currently only versions 1 through 9 supported
                // Encode the character count
                for i in (0..size_of_character_length_bits).rev() {
                    bit_string.push_bit((alpha_numeric_values.len() & (1 << i)) as i32);
                }

                // Encode the values
                for value_index in (0..alpha_numeric_values.len()).step_by(2) {
                    // Since stepping by 2 we know that this is always going to be Some()
                    let first_value = unsafe { alpha_numeric_values.get(value_index).unwrap_unchecked() };
                    
                    match alpha_numeric_values.get(value_index + 1) {
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
                    match error_correction_level {
                        ErrorCorrectionLevel::L => {
                            L_NUM_CODEWORDS[version] * 8
                        },
                        ErrorCorrectionLevel::M => {
                            todo!()
                        },
                        ErrorCorrectionLevel::Q => {
                            todo!()
                        },
                        ErrorCorrectionLevel::H => {
                            todo!()
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
            QRMode::Byte(bytes) => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_modes() {
        let qr_mode = QRMode::analyze_data("123");
        assert_eq!(qr_mode, QRMode::Numeric(vec![1, 2, 3]));

        let qr_mode = QRMode::analyze_data("A113");
        assert_eq!(qr_mode, QRMode::AlphaNumeric(vec![10, 1, 1, 3]));

        let qr_mode = QRMode::analyze_data("a113");
        assert_eq!(qr_mode, QRMode::Byte(vec![97, 49, 49, 51]));
    }
}
