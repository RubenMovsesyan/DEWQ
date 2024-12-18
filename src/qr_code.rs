use crate::bit_string::*;

pub enum QRCode {
    Model2(QRCodeModel2),
}

pub trait Encodable {
    fn encode(&self) -> BitString;
}

pub struct QRCodeModel2 {
    mode: QRMode,
}

impl QRCodeModel2 {
    pub fn from(mode: QRMode) -> QRCodeModel2 {
        QRCodeModel2 {
            mode
        }
    }

    pub fn get_encoding(&self) -> BitString {
        self.mode.encode()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum QRMode {
    Numeric(Vec<u8>),
    AlphaNumeric(Vec<u8>),
    Byte(Vec<u8>),
    // Kanji(Vec<u16>), // Double byte mode
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
                    digit_buffer.push(converted_input[i..(i + 1)].parse().unwrap_unchecked());
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
}


impl Encodable for QRMode {
    fn encode(&self) -> BitString {
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

                // Currently only versions 1 through 9 supported
                // Encode the character count
                for i in (0..9).rev() {
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
