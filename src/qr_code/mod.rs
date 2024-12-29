use crate::bit_utils::{bit::*, bit_string::*, bitmap::*};
use crate::galios::*;
use non_std::Vec;
use crate::alloc::vec;
use crate::test_utils::test_println;

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


// Constants
use crate::qr_code::constants::*;

mod constants;

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

            test_println!("{}", data.len());

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

            test_println!("{}", version);

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


                test_println!("{}", bit_string);
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
                            test_println!("version: {}", alpha_numeric_qr_code.version);
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
                
                test_println!("num_e_codewords: {}", num_error_codewords);

                for i in 1..num_error_codewords {
                    poly = poly.multiply_as_exponents(&GeneratorPolynomial::from(vec![0, i as i32]));
                }


                poly
            };
            
            test_println!("gen_size: {}", generator_polynomial.get().len());
            
            // Perform the long division on the message polynomial with the generator polynomial
            let size = message_polynomial.get().len();
            test_println!("poly_size: {}", size);
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
            test_println!("e_len_pre: {}", error_correction_nums.get().len());

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


        {
            let mut d_len = 0;
            for d in data.iter() {
                d_len += d.len();
            }
            let mut e_len = 0;
            for e in error_correction_data.iter() {
                e_len += e.len();
            }

            test_println!("d_len: {}, e_len: {}", d_len, e_len);
        }

        (data, error_correction_data)
    }

    pub fn structure_codewords(&self, data: (Vec<Vec<u8>>, Vec<Vec<u8>>)) -> BitString {
        let mut new_data: Vec<u8> = Vec::new();
        let max_index = {
            let mut max = data.0[0].len();

            for val in data.0.iter() {
                max = max.max(val.len());
            }

            max
        };

        test_println!("max_index_d: {}", max_index);
        // Interleave the data codewords
        for i in 0..max_index {
            for value in data.0.iter() {
                if value.len() > i {
                    new_data.push(value[i]);
                }
            }
        }

        let max_index = {
            let mut max = data.1[0].len();

            for val in data.1.iter() {
                max = max.max(val.len());
            }

            max
        };        
        test_println!("max_index_e: {}", max_index);
        // Interleave the error codewords
        for i in 0..max_index {
            for value in data.1.iter() {
                if value.len() > i {
                    new_data.push(value[i]);
                }
            }
        }
        
        // Turn into a final bitstring
        let mut new_bitstring = BitString::new();
        for byte in new_data {
            new_bitstring.push_byte(byte);
        }
        
        let version = {
            match self {
                QRMode::Numeric(_data) => { todo!() },
                QRMode::AlphaNumeric(alpha_numeric_qr_code) => { alpha_numeric_qr_code.version },
                QRMode::Byte(_data) => { todo!() },
            }
        };

        for _ in 0..REQUIRED_REMAINDER_BITS[version] {
            new_bitstring.push_bit(0);
        }

        test_println!("len: {}", new_bitstring);
        new_bitstring
    }

    
    pub fn create_bit_map(&self, bits: BitString) {
        let version = {
            match self {
                QRMode::Numeric(_data) => { todo!() },
                QRMode::AlphaNumeric(alpha_numeric_qr_code) => { alpha_numeric_qr_code.version },
                QRMode::Byte(_data) => { todo!() },
            }
        };

        let size = 21 + (4 * (version ));

        let mut bit_map = BitMap::new(size);
        let mut reservations = BitMap::new(size);
        create_finder_patterns(&mut bit_map, &mut reservations);
        create_alignment_patterns(&mut bit_map, &mut reservations);
        create_timing_patterns(&mut bit_map, &mut reservations);
        create_dark_module(&mut bit_map, &mut reservations);
        reserve_format_information_areas(&mut reservations);
        place_data_bits(&mut bit_map, &reservations, &bits);
        // test_println!("{}", bit_map);
        mask_data(&mut bit_map, &reservations);
        add_format_information(&mut bit_map, {
            match self {
                QRMode::Numeric(_data) => { todo!() },
                QRMode::AlphaNumeric(alpha_numeric_qr_code) => { &alpha_numeric_qr_code.error_correction_level },
                QRMode::Byte(_data) => { todo!() },
            }
        }, &version);
        // test_println!("{}", bits);
        test_println!("{}", reservations);
        test_println!("{}", bit_map);
    }
}

fn add_format_information(bit_map: &mut BitMap, error_correction_level: &ErrorCorrectionLevel, version: &usize) {
    let mut bits = Vec::with_capacity(15);
    match error_correction_level {
        ErrorCorrectionLevel::L => { bits.push(0); bits.push(1); },
        ErrorCorrectionLevel::M => { bits.push(0); bits.push(0); },
        ErrorCorrectionLevel::Q => { bits.push(1); bits.push(1); },
        ErrorCorrectionLevel::H => { bits.push(1); bits.push(0); },
    }

    // Add the mask pattern bits
    bits.push(0);
    bits.push(0);
    bits.push(1);

    for _ in 0..10 {
        bits.push(0);
    }

    let mut bits_poly = GeneratorPolynomial::from(bits.clone());
    bits_poly = bits_poly.drop_leading_zeros();


    while bits_poly.len() > 10 {
        let generator_poly = GeneratorPolynomial::from({
            let mut output = vec![1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1];

            for _ in output.len()..bits_poly.len() {
                output.push(0);
            }

            output
        });

        // test_println!("{}", bits_poly);
        // test_println!("{}", generator_poly);

        // Perform the division 4 times
        bits_poly = bits_poly.xor_with_other(&generator_poly);
        bits_poly = bits_poly.drop_leading_zeros();

        // test_println!("{}", bits_poly);
        // test_println!("{}", generator_poly);
    }

    test_println!("{}", bits_poly);

    let prepend = {
        let mut prepend_vec = Vec::with_capacity(10 - bits_poly.len());
        for _ in bits_poly.len()..10 {
            prepend_vec.push(0);
        }
        prepend_vec
    };

    bits_poly.prepend(prepend);

    test_println!("{}", bits_poly);

    let mut format_bits = {
        let mut output = BitString::new();
        for i in 0..=4 {
            output.push_bit(bits[i]);
        }

        for coefficient in bits_poly.get() {
            output.push_bit(*coefficient);
        }

        output
    };

    // Mask the new bitstring
    // NOTE: Currently using masking number 1
    let mask_bits = match error_correction_level {
        ErrorCorrectionLevel::L => {
            BitString::from_string("111001011110011")
        },
        ErrorCorrectionLevel::M => {
            BitString::from_string("101000100100101")
        },
        ErrorCorrectionLevel::Q => {
            BitString::from_string("011000001101000")
        },
        ErrorCorrectionLevel::H => {
            BitString::from_string("001001110111110")
        },
    };
    
    test_println!("{}", format_bits);
    let _ = format_bits.xor_with_other(&mask_bits);
    test_println!("{}", format_bits);

    // Put the bits into the bitmap
    let mut index = 0;
    if *version < 7 {
        for i in 0..=5 {
            // Come up with a better solution than this
            let bit = unsafe { format_bits.get_bit(index).unwrap_unchecked() };
            bit_map.set(8, i, bit);
            bit_map.set(bit_map.size() - i, 8, bit);
            index += 1;
        }

        // Add bits 6, 7, and 8
        let (bit_6, bit_7, bit_8) = (
            unsafe { format_bits.get_bit(index).unwrap_unchecked() },
            unsafe { format_bits.get_bit(index + 1).unwrap_unchecked() },
            unsafe { format_bits.get_bit(index + 2).unwrap_unchecked() },
        );
        index += 3;

        bit_map.set(bit_map.size() - 6, 8, bit_6);
        bit_map.set(8, 7, bit_6);

        bit_map.set(8, 8, bit_7);
        bit_map.set(8, bit_map.size() - 7, bit_7);

        bit_map.set(7, 8, bit_8);
        bit_map.set(8, bit_map.size() - 6, bit_8);

        for i in 9..=14 {
            let bit = unsafe { format_bits.get_bit(index).unwrap_unchecked() };
            index += 1;
            bit_map.set(14 - i, 8, bit);
            bit_map.set(8, bit_map.size() - (15 - i), bit);
        }
    }
}

fn mask_data(bit_map: &mut BitMap, reservations: &BitMap) {
    // Doing data masking number 1
    for row in 0..bit_map.size() {
        if row % 2 == 0 {
            for column in 0..bit_map.size() {
                if reservations.get(row, column) == Bit::Zero {
                    bit_map.invert_bit(row, column);
                }
            }
        }
    }
}


fn place_data_bits(bit_map: &mut BitMap, reservations: &BitMap, bits: &BitString) {
    let size = bit_map.size();
    let mut position = (size - 1, size - 1);
    let mut index = 0;
        
    test_println!("here {}", bits.len());
    let place_row = |bm: &mut BitMap, x: &usize, y: &usize, ind: &mut usize| {
            for i in 0..=1 {
                if reservations.get(*y, *x - i) == Bit::Zero {
                    bm.set(*y, *x - i, {
                        let mut out = 1;
                        if let Ok(bit) = bits.get_bit(*ind) {
                            match bit {
                                Bit::Zero => out = 0,
                                Bit::One => {},
                            }
                        } else {
                            test_println!("out of bounds");
                            return;
                        }

                        out
                    });
                    *ind += 1;
                }
            }
    };
    
    let mut going_up: bool = true;
    while position.0 > 6 {
        while position.1 > 0 {
            place_row(bit_map, &position.0, &position.1, &mut index);
            position.1 -= 1;
        }


        going_up = false;

        position.0 -= 2;
        
        if position.0 <= 6 {
            break;
        }

        while position.1 < size {
            place_row(bit_map, &position.0, &position.1, &mut index);
            position.1 += 1;
        }

        going_up = true;

        position.0 -= 2;
    }

    position.0 -= 1;

    while index < bits.len() {
        if going_up {
            while position.1 > 0 {
                place_row(bit_map, &position.0, &position.1, &mut index);
                position.1 -= 1;
            }

            if position.0 > 1 {
                position.0 -= 2;
            } else {
                test_println!("{}", index);
                return;
            }
        } else {
            going_up = true;
        }

        while position.1 < size {
            place_row(bit_map, &position.0, &position.1, &mut index);
            position.1 += 1;
        }

        if position.0 > 1 {
            position.0 -= 2;
        } else {
            test_println!("{}", index);
            return;
        }
    }
}

fn reserve_format_information_areas(reservations: &mut BitMap) {
    let size = reservations.size();

    let version = ((size - 21) / 4) + 1;

    if version >= 7 {
        for i in 0..=5 {
            reservations.set(i, size - 11, 1);
            reservations.set(i, size - 10, 1);
            reservations.set(i, size - 9, 1);

            reservations.set(size - 11, i, 1);
            reservations.set(size - 10, i, 1);
            reservations.set(size - 9, i, 1);
        }
    } else {
        for i in 0..=8 {
            reservations.set(i, 8, 1);
            reservations.set(8, i, 1);
            reservations.set(size - i, 8, 1);
            reservations.set(8, size - i, 1);
        }
    }
}

fn create_dark_module(bit_map: &mut BitMap, reservations: &mut BitMap) {
    let size = bit_map.size();
    bit_map.set(size - 7, 8, 1);
    reservations.set(size - 7, 8, 1);
}

fn create_timing_patterns(bit_map: &mut BitMap, reservations: &mut BitMap) {
    for i in 7..(bit_map.size() - 7) {
        if i % 2 == 0 {
            bit_map.set(i, 6, 1);
            bit_map.set(6, i, 1);
        }

        reservations.set(i, 6, 1);
        reservations.set(6, i, 1);
    }
}


fn get_alignment_pattern_coordinates_list(bit_map_size: usize) -> Vec<usize> {
    let version = ((bit_map_size - 21) / 4) + 1;
    
    test_println!("{} {}", version, bit_map_size);

    let intervals = (version / 7) + 1;
    let distance = 4 * version + 4;
    let mut step = ((distance as f64) / (intervals as f64)).round() as usize;
    step += step & 0b1; // Round step to the next even number
    let mut coordinates: Vec<usize> = vec![6]; // The first coordinate is always 6
    for i in 1..=intervals {
        coordinates.push(6 + distance - step * (intervals - i));
    }

    coordinates
}

fn create_alignment_patterns(bit_map: &mut BitMap, reservations: &mut BitMap) {
    if !(bit_map.size() > 21) {
        return;
    }

    let add_alignment_pattern = |bm: &mut BitMap, rv: &mut BitMap, i: usize, j: usize| {
        bm.set(i, j, 1);
        
        for x in -2..=2 as isize {
            bm.set(i - 2, (j as isize + x) as usize, 1);
            bm.set(i + 2, (j as isize + x) as usize, 1);
            bm.set((i as isize + x) as usize, j + 2, 1);
            bm.set((i as isize + x) as usize, j - 2, 1);
        }

        // Reserve the alignment patterns
        for rev_x in -2..=2 as isize {
            for rev_y in -2..=2 as isize {
                rv.set((rev_x + i as isize) as usize, (rev_y + j as isize) as usize, 1);
            }
        }
    };
    
    let coords = get_alignment_pattern_coordinates_list(bit_map.size());
    
    test_println!("{:?}", coords);
        
    for x in 0..coords.len() {
        for y in 0..coords.len() {
            if (x == 0 && y == 0)
            || (x == 0 && y == coords.len() - 1) 
            || (x == coords.len() - 1 && y == 0) {
                continue;
            }
            add_alignment_pattern(bit_map, reservations, coords[x] as usize, coords[y] as usize);
        }
    }
}

fn create_finder_patterns(bit_map: &mut BitMap, reservations: &mut BitMap) {
    let size = bit_map.size();

    let mut add_finder = |i: usize, j: usize| {
        for x in 0..7 {
            bit_map.set(i, j + x, 1);
            bit_map.set(i + 6, j + x, 1);

            bit_map.set(i + x, j, 1);
            bit_map.set(i + x, j + 6, 1);
        }

        for x in 2..5 {
            for y in 2..5 {
                bit_map.set(i + x, j + y, 1);
            }
        }

        for reservation_x in 0..7 {
            for reservation_y in 0..7 {
                reservations.set(reservation_x + i, reservation_y + j, 1);
            }
        }
    };
    
    add_finder(0, 0);
    add_finder(size - 7, 0);
    add_finder(0, size - 7);
    
    // Reserve the separators
    for reservation_index in 0..=7 {
        reservations.set(7, reservation_index, 1);
        reservations.set(reservation_index, 7, 1);


        reservations.set(size - 8, reservation_index, 1);
        reservations.set(size - reservation_index, 7, 1);


        reservations.set(reservation_index, size - 8, 1);
        reservations.set(7, size - reservation_index, 1);
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
