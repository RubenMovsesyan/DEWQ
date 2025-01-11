use crate::bit_utils::{bit::*, bit_string::*, bitmap::*};
use crate::galios::*;
use non_std::Vec;
use crate::alloc::vec;
use crate::test_utils::{test_println, test_print};

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

pub trait QRCode {
    fn version(&self) -> usize;
    fn error_correction_level(&self) -> &ErrorCorrectionLevel;
    fn data(&self) -> &Vec<u8>;
    fn data_len(&self) -> usize;
}

#[derive(PartialEq, Eq, Debug)]
pub struct AlphaNumericQrCode {
    data: Vec<u8>,
    version: usize,
    error_correction_level: ErrorCorrectionLevel,
}

impl QRCode for AlphaNumericQrCode {
    fn version(&self) -> usize {
        self.version
    }

    fn error_correction_level(&self) -> &ErrorCorrectionLevel {
        &self.error_correction_level
    }

    fn data(&self) -> &Vec<u8> {
        &self.data
    }

    fn data_len(&self) -> usize {
        self.data.len()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ErrorCorrectionLevel {
    L,
    M,
    Q,
    H,
}

impl ErrorCorrectionLevel {
    pub fn get_format_bits(&self) -> u32 {
        match self {
            ErrorCorrectionLevel::L => 0b01,
            ErrorCorrectionLevel::M => 0b00,
            ErrorCorrectionLevel::Q => 0b11,
            ErrorCorrectionLevel::H => 0b10,
        }
    }

    // TODO: add functionality for multiple mask patterns
    pub fn get_format_mask_bits(&self) -> u32 {
        match self {
            ErrorCorrectionLevel::L => 0x77C4,
            ErrorCorrectionLevel::M => 0x5412,
            ErrorCorrectionLevel::Q => 0x355F,
            ErrorCorrectionLevel::H => 0x1689,
        }
    }

    
    pub fn get_alpha_numeric_version_size(&self, version: usize) -> usize {
        match self {
            ErrorCorrectionLevel::L => ALPHA_NUMERIC_L_MAX_CAPACITY[version],
            ErrorCorrectionLevel::M => ALPHA_NUMERIC_M_MAX_CAPACITY[version],
            ErrorCorrectionLevel::Q => ALPHA_NUMERIC_Q_MAX_CAPACITY[version],
            ErrorCorrectionLevel::H => ALPHA_NUMERIC_H_MAX_CAPACITY[version],
        }
    }

    pub fn get_num_codewords(&self, version: usize) -> usize {
        match self {
            ErrorCorrectionLevel::L => L_NUM_CODEWORDS[version],
            ErrorCorrectionLevel::M => M_NUM_CODEWORDS[version],
            ErrorCorrectionLevel::Q => Q_NUM_CODEWORDS[version],
            ErrorCorrectionLevel::H => H_NUM_CODEWORDS[version],
        }
    }

    pub fn get_block_data(&self, version: usize) -> (usize, usize, usize, usize) {
        match self {
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
    }

    pub fn get_num_error_correction_codewords(&self, version: usize) -> usize {
        match self {
            ErrorCorrectionLevel::L => L_ERROR_CORRECTION_CODE_WORDS[version],
            ErrorCorrectionLevel::M => M_ERROR_CORRECTION_CODE_WORDS[version],
            ErrorCorrectionLevel::Q => Q_ERROR_CORRECTION_CODE_WORDS[version],
            ErrorCorrectionLevel::H => H_ERROR_CORRECTION_CODE_WORDS[version],
        }
    }
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
            // Get the alphanumeric conversion of the data
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

            // Get the version of QR code needed
            let version = {
                let mut out: usize = 0;
                for version_index in (0..MAX_VERSION).rev() {
                    if data.len() > error_correction_level.get_alpha_numeric_version_size(version_index) {
                        out = version_index + 1;
                        break;
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

    // Private getters for easy abstraction
    fn version(&self) -> usize {
        match self {
            QRMode::Numeric(_numbers) => todo!(),
            QRMode::AlphaNumeric(anqr) => anqr.version,
            QRMode::Byte(_bytes) => todo!(),
        }
    }

    fn error_correction_level(&self) -> &ErrorCorrectionLevel {
        match self {
            QRMode::Numeric(_numbers) => todo!(),
            QRMode::AlphaNumeric(anqr) => anqr.error_correction_level(),
            QRMode::Byte(_bytes) => todo!(),
        }
    }
    

    pub fn encode(&mut self) -> BitString {
        let mut bit_string: BitString = BitString::new();
        let size_of_character_length_bits: usize;


        // Perform the mode dependent encoding
        match self {
            QRMode::Numeric(_numbers) => {
                todo!()
            },
            QRMode::AlphaNumeric(anqr) => {
                // Adding the mode indicator
                bit_string.push_bit(0);
                bit_string.push_bit(0);
                bit_string.push_bit(1);
                bit_string.push_bit(0);
                
                size_of_character_length_bits = {
                    let out: usize;
                    if (anqr.version() + 1) < 10 {
                        out = 9;
                    } else if (anqr.version() + 1) > 9 && (anqr.version() + 1) < 27 {
                        out = 11;
                    } else {
                        out = 13
                    }

                    out
                };

                // Encode the character count
                for i in (0..size_of_character_length_bits).rev() {
                    bit_string.push_bit((anqr.data_len() & (1 << i)) as i32);
                }

                // Encode the values in alphanumeric encoding
                for value_index in (0..anqr.data_len()).step_by(2) {
                    // Since stepping by 2 we know that this is always going to be Some()
                    let first_value = unsafe { anqr.data().get(value_index).unwrap_unchecked() };
                    
                    match anqr.data().get(value_index + 1) {
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

            },
            QRMode::Byte(_bytes) => {
                todo!()
            }
        }


        // The rest is the mode independed encoding
        let required_number_of_bits = 
            self
            .error_correction_level()
            .get_num_codewords(self.version()) * BYTE_SIZE;


        // Add terminator 0s if necessary
        {
            let total_bits = bit_string.len() 
                - 4 
                - (size_of_character_length_bits);

            let bit_difference = required_number_of_bits - total_bits;

            for _ in 0..bit_difference.min(4) {
                bit_string.push_bit(0);
            }
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


        test_println!("{}", bit_string.as_hex());
        return bit_string;
    }

    pub fn generate_error_correction(&self, bits: BitString) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
        let mut data: Vec<Vec<u8>> = Vec::new();
        let mut error_correction_data: Vec<Vec<u8>> = Vec::new();
        
        let (
            num_blocks_group_1,
            num_code_words_group_1,
            num_blocks_group_2,
            num_code_words_group_2
        ) = self.error_correction_level().get_block_data(self.version());

        // Create all the message polynomials from the data codewords
        let mut message_polynomials: Vec<Polynomial> = Vec::new();
        let mut index = 0;

        for _ in 0..num_blocks_group_1 {
            let mut block: Vec<u8> = Vec::new();
            for _ in 0..num_code_words_group_1 {
                block.push(bits.get_byte(index));
                index += 1;
            }


            data.push(Vec::from(block.clone()));
            message_polynomials.push(Polynomial::from_integer_notation(block));
        }

        for _ in 0..num_blocks_group_2 {
            let mut block = Vec::new();
            for _ in 0..num_code_words_group_2 {
                block.push(bits.get_byte(index));
                index += 1;
            }

            data.push(Vec::from(block.clone()));
            message_polynomials.push(Polynomial::from_integer_notation(block));
        }

        for message_polynomial in message_polynomials {
            // Create a generator polynomial based on the number of blocks needed
            let mut generator_polynomial = {
                let mut poly = Polynomial::from_exponent_notation(vec![0, 0]);
                
                for i in 1..self.error_correction_level().get_num_error_correction_codewords(self.version()) {
                    poly = poly.multiply(&mut Polynomial::from_exponent_notation(vec![0, i as i32]));
                }

                poly
            };


            // Perform the long division on the message polynomial with the generator polynomial
            let mut current_message = message_polynomial.clone();
            let mut inter_poly;
            for _ in 0..message_polynomial.len() {
                // Multiply the generator by the first coefficient of the message polynomial
                inter_poly = generator_polynomial.multiply_by_exponent(current_message.get_as_exponent_vec()[0]);
                // xor the resulting multiplaction with the current message polynomial
                inter_poly = inter_poly.xor(&mut current_message);
                // Drop the leading zeros of the resulting xor operation
                inter_poly.drop_leading_zeros();
                // Set the new current message to the resulting computation
                current_message = inter_poly;
            }

            // Push the data to the error correction data as u8
            error_correction_data.push({
                let mut output: Vec<u8> = Vec::new();

                for elem in current_message.get_as_integer_vec() {
                    output.push(*elem as u8);
                }

                output
            });
        }

        (data, error_correction_data)
    }

    pub fn structure_codewords(&self, data: (Vec<Vec<u8>>, Vec<Vec<u8>>)) -> BitString {
        let mut new_data: Vec<u8> = Vec::new();

        let max_index = data.0.iter().max_by_key(|block| block.len()).unwrap().len();

        // Interleave the data codewords
        for i in 0..max_index {
            for block in data.0.iter() {
                if let Some(value) = block.get(i) {
                    new_data.push(*value);
                }
            }
        }

        let max_index = data.1.iter().max_by_key(|block| block.len()).unwrap().len();

        // Interleave the error codewords
        for i in 0..max_index {
            for block in data.1.iter() {
                if let Some(value) = block.get(i) {
                    new_data.push(*value);
                }
            }
        }
        
        // Create a new bitstring from the data and push the required remainder bits to it
        let mut new_bitstring = BitString::from_vec(new_data);
        new_bitstring.push_bit_times(0, REQUIRED_REMAINDER_BITS[self.version()]);

        new_bitstring
    }

    
    pub fn create_bit_map(&self, bits: BitString) {
        let size = 21 + (4 * (self.version()));

        let mut bit_map = BitMap::new(size);
        let mut reservations = BitMap::new(size);

        // High level overview of the steps to create the QR code
        create_finder_patterns(&mut bit_map, &mut reservations);
        create_alignment_patterns(&mut bit_map, &mut reservations);
        create_timing_patterns(&mut bit_map, &mut reservations);
        create_dark_module(&mut bit_map, &mut reservations);
        reserve_format_information_areas(&mut reservations);
        place_data_bits(&mut bit_map, &reservations, &bits);
        mask_data(&mut bit_map, &reservations);
        add_format_information(&mut bit_map, self.error_correction_level(), self.version());
        test_println!("{}", bit_map);
    }
}

fn add_format_information(bit_map: &mut BitMap, error_correction_level: &ErrorCorrectionLevel, version: usize) {
    // HACK: temparory mask variable while still testing with only mask 0
    let mask = 0;
    
    // Put the bits into the bitmap
    let mut index = 0;


    if version >= 7 {
        // Get the format information bits and error correction for those bits
        let bits: u32 = {
            let data = version as u32 + 1;
            // Generator polynomial: x^12 + x^11 + x^10 + x^9 + x^8 + x^5 + x^2 + 1
            let generator_polynomial = 0x1F25;

            let mut rem: u32 = data;
            // This is the same as doing a division by the generator polynomial until the remainder is
            // less than 4096
            for _ in 0..12 {
                rem = (rem << 1) ^ ((rem >> 11) * generator_polynomial);
            }

            (data << 12) | rem
        };

        test_println!("bits: {:018b}", bits);

        for j in 0..6 {
            for i in 0..3 {
                bit_map.set(bit_map.size() - 11 + i, j, bits & (1 << index));
                bit_map.set(j, bit_map.size() - 11 + i, bits & (1 << index));


                index += 1;
            }
        }
    }
    

    index = 0;
    // Get the format information bits and error correction for those bits
    let bits: u32 = {
        let data = error_correction_level.get_format_bits() << 3 | mask;

        // Generator polynomial: x^10 + x^8 + x^5 + x^4 + x^2 + x + 1
        let generator_polynomial = 0x537;

        let mut rem: u32 = data;
        // This is the same as doing a division by the generator polynomial until the remainder is
        // less than 1024
        for _ in 0..10 {
            rem = (rem << 1) ^ ((rem >> 9) * generator_polynomial);
        }

        // ((data << 10) | rem) ^ error_correction_level.get_format_mask_bits()
        ((data << 10) | rem) ^ 0x5412
    };
    test_println!("other bits: {:b}", bits);

    for i in 0..=5 {
        // Come up with a better solution than this
        let bit = bits & (0x4000 >> index);
        bit_map.set(8, i, bit);
        bit_map.set(bit_map.size() - 1 - i, 8, bit);
        index += 1;
    }

    // Add bits 6, 7, and 8
    let (bit_6, bit_7, bit_8) = (
        bits & (0x4000 >> index),
        bits & (0x4000 >> (index + 1)),
        bits & (0x4000 >> (index + 2))
    );
    index += 3;

    bit_map.set(bit_map.size() - 6, 8, bit_6);
    bit_map.set(8, 7, bit_6);

    bit_map.set(8, 8, bit_7);
    bit_map.set(8, bit_map.size() - 7, bit_7);

    bit_map.set(7, 8, bit_8);
    bit_map.set(8, bit_map.size() - 6, bit_8);

    for i in 9..=14 {
        let bit = bits & (0x4000 >> index);
        index += 1;
        bit_map.set(14 - i, 8, bit);
        bit_map.set(8, bit_map.size() - (15 - i), bit);
    }
}

fn mask_data(bit_map: &mut BitMap, reservations: &BitMap) {
    // Doing data masking number 0
    for row in 0..bit_map.size() {
            for column in 0..bit_map.size() {
                if (row + column) % 2 == 0 {
                    if reservations.get(row, column) == Bit::Zero {
                        bit_map.invert_bit(row, column);
                    }
                }
            }
    }
}


fn place_data_bits(bit_map: &mut BitMap, reservations: &BitMap, bits: &BitString) {
    // Place bits up in a zig zag pattern
    fn place_bits_up(
        bit_map: &mut BitMap, 
        reservations: &BitMap, 
        bits: &BitString, 
        index: &mut usize, 
        x_pos: usize
    ) {
        let mut y_pos = bit_map.size() - 1;
        
        loop {
            if reservations.get(y_pos, x_pos) == Bit::Zero {
                bit_map.set(y_pos, x_pos, bits.get_bit(*index).unwrap());
                // bit_map.set(y_pos, x_pos, 1);
                *index += 1;
            }

            if reservations.get(y_pos, x_pos - 1) == Bit::Zero {
                bit_map.set(y_pos, x_pos - 1, bits.get_bit(*index).unwrap());
                // bit_map.set(y_pos, x_pos - 1, 1);
                *index += 1;
            }

            if y_pos == 0 {
                break;
            }

            y_pos -= 1;
        }
    }


    // Place bits down in a zig zag pattern
    fn place_bits_down(
        bit_map: &mut BitMap, 
        reservations: &BitMap, 
        bits: &BitString, 
        index: &mut usize, 
        x_pos: usize
    ) {
        let mut y_pos = 0;
        while y_pos < bit_map.size() {
            if reservations.get(y_pos, x_pos) == Bit::Zero {
                bit_map.set(y_pos, x_pos, bits.get_bit(*index).unwrap_or_else(|_| {
                    test_println!("{}", bit_map);
                    panic!()
                }));
                // bit_map.set(y_pos, x_pos, 1);
                *index += 1;
            }

            if reservations.get(y_pos, x_pos - 1) == Bit::Zero {
                bit_map.set(y_pos, x_pos - 1, bits.get_bit(*index).unwrap());
                // bit_map.set(y_pos, x_pos - 1, 1);
                *index += 1;
            }

            y_pos += 1;
        }
    }
    
    let mut index = 0;
    let mut x_pos = bit_map.size() - 1;
    
    let mut going_up = true;
    while x_pos > 7 {
        place_bits_up(bit_map, reservations, bits, &mut index, x_pos);
        x_pos -= 2;
        going_up = false;
        if x_pos < 7 {
            break;
        }

        place_bits_down(bit_map, reservations, bits, &mut index, x_pos);
        x_pos -= 2;
        going_up = true;
    }

    x_pos -= 1;

    loop {
        if going_up {
            place_bits_up(bit_map, reservations, bits, &mut index, x_pos);
            if x_pos < 2 {
                break;
            }
            x_pos -= 2;
        } else {
            going_up = true;
        }

        place_bits_down(bit_map, reservations, bits, &mut index, x_pos);
        if x_pos < 2 {
            break;
        }
        x_pos -= 2;
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
    } 

    for i in 0..=8 {
        reservations.set(i, 8, 1);
        reservations.set(8, i, 1);
        reservations.set(size - i, 8, 1);
        reservations.set(8, size - i, 1);
    }
}

fn create_dark_module(bit_map: &mut BitMap, reservations: &mut BitMap) {
    let size = bit_map.size();
    bit_map.set(size - 8, 8, 1);
    reservations.set(size - 8, 8, 1);
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

    // #[test]
    // fn test_qr_error_codes() {
    //     let mut qr_mode = QRMode::analyze_data("HELLO WORLD", ErrorCorrectionLevel::M);
    //     let bits = qr_mode.encode();
    //     println!("Bits pre {}", bits);
    //     let data = qr_mode.generate_error_correction(bits);
    //     println!("data post {:?}", data);
    // }
}
