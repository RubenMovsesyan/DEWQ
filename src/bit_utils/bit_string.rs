use non_std::Vec;
use super::bit::Bit;

#[cfg(any(
        test,
        feature = "test_feature"
))]
extern crate std;

#[cfg(any(
        test,
        feature = "test_feature"
))]
use std::fmt::Display;

#[cfg(any(
        test,
        feature = "test_feature"
))]
#[macro_use]
use std::format;

pub struct BitString {
    bits: Vec<u8>,
    bits_len: usize,
}


impl BitString {
    pub fn new() -> BitString {
        Self {
            bits: Vec::new(),
            bits_len: 0,
        }
    }

    pub fn from_string<'a, S>(string: S) -> BitString
        where S: Into<&'a str>
    {
        let mut output = BitString::new();
        
        let str_ref: &str = string.into();

        for character in str_ref.chars().collect::<Vec<char>>().iter() {
            match character {
                '0' => output.push_bit(Bit::Zero),
                _ => output.push_bit(Bit::One),
            }
        }

        output
    }

    pub fn push_bit<B>(&mut self, bit: B)
        where B: Into<Bit>
    {
        let bit_offset = self.bits_len % 8;
        
        if bit_offset == 0 {
            self.bits.push(0);
        }

        match bit.into() {
            Bit::Zero => {},
            Bit::One => {
                if let Some(byte) = self.bits.last_mut() {
                    *byte |= 1 << (7 - bit_offset);
                }
            }
        }
        self.bits_len += 1;
    }

    pub fn push_byte<U8>(&mut self, byte: U8) 
        where U8: Into<u8>
    {
        let byte_to_push = byte.into();

        for i in 0..8 {
            self.push_bit(byte_to_push & (1 << (7 - i)));
        }
    }

    pub fn get_bit(&self, address: usize) -> Result<Bit, BitAddressOutOfBoundsError> {
        if address >= self.bits_len {
            return Err(BitAddressOutOfBoundsError);
        }

        let bit_address = address / 8;
        let bit_offset = address % 8;
        
        match self.bits[bit_address] & (1 << (7 - bit_offset)) {
            0 => Ok(Bit::Zero),
            _ => Ok(Bit::One),
        }
    }

    pub fn xor_with_other(&mut self, other: &BitString) -> Result<(), BitIndiciesDontMatchError> {
        if self.bits_len != other.bits_len {
            return Err(BitIndiciesDontMatchError);
        }
        
        for index in 0..self.bits.len() {
            self.bits[index] ^= other.bits[index];
        }

        Ok(())
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        self.bits[index]
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bits
    }

    pub fn len(&self) -> usize {
        self.bits_len
    }

    // TODO: make sure to check if the bit string is right shiftable
    pub fn right_shift(&mut self) {
        for index in (0..self.bits.len()).rev() {
            let byte = self.bits[index];
            
            if byte & 1 == 1 {
                if index >= self.bits.len() - 1 {
                    self.bits.push(0);
                }
                self.bits[index + 1] |= 0b10000000;
            }


            self.bits[index] >>= 1;
        }

        self.bits_len += 1;
    }

    pub fn as_hex(&self) -> HexStr {
        let mut chars: HexStr = HexStr(Vec::new());

        let mut index = 0;
        

        while index < self.len() {
            let mut val: u8 = 0;
            for _ in 0..4 {
                val <<= 1;
                if let Ok(bit) = self.get_bit(index) {
                    match bit {
                        Bit::One => {val |= 1},
                        _ => {},
                    }
                }
                index += 1;
            }

            // HACK: This is just a quick way to get this working
            chars.0.push(match val {
                0 => { '0' },
                1 => { '1' },
                2 => { '2' },
                3 => { '3' },
                4 => { '4' },
                5 => { '5' },
                6 => { '6' },
                7 => { '7' },
                8 => { '8' },
                9 => { '9' },
                10 => { 'A' },
                11 => { 'B' },
                12 => { 'C' },
                13 => { 'D' },
                14 => { 'E' },
                15 => { 'F' },
                _ => '@'
            });
        }

        chars
    }
}

pub struct HexStr(Vec<char>);

#[cfg(any(
        test,
        feature = "test_feature"
))]
impl Display for HexStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut character_index = 0;

        for elem in self.0.iter() {
            write!(f, "{}", elem)?;

            character_index += 1;

            if character_index % 2 == 0 {
                write!(f, " ")?;
            }

            if character_index % 32 == 0 {
                writeln!(f)?;
            }
        }

        writeln!(f)?;

        Ok(())
    }
}

#[cfg(any(
        test,
        feature = "test_feature"
))]
impl Display for BitString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.bits_len {
            write!(f, "{}", if self.get_bit(i).unwrap() == Bit::One { 1 } else { 0 })?;
        }
        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct BitAddressOutOfBoundsError;

#[derive(Debug, Clone)]
pub struct BitIndiciesDontMatchError;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_basics() {
        let mut bit_string: BitString = BitString::new();

        bit_string.push_bit(Bit::One);
        bit_string.push_bit(Bit::One);
        bit_string.push_bit(Bit::One);
        bit_string.push_bit(Bit::One);

        for i in 0..4 {
            match bit_string.get_bit(i) {
                Ok(bit) => {
                    assert_eq!(bit, Bit::One);
                },
                Err(_) => {
                    assert!(false);
                }
            }
        }

        match  bit_string.get_bit(5) {
            Ok(_) => {
                assert!(false);
            },
            Err(_) => {
                assert!(true);
            }
        }

        bit_string = BitString::new();
        for _ in 0..50 {
            bit_string.push_bit(Bit::One);
        }
        bit_string.push_bit(Bit::Zero);
        for _ in 0..50 {
            bit_string.push_bit(Bit::One);
        }

        match bit_string.get_bit(0) {
            Ok(bit) => {
                assert_eq!(bit, Bit::One);
            },
            Err(_) => {
                assert!(false);
            }
        }

        // println!("{}", bit_string);
        match bit_string.get_bit(50) {
            Ok(bit) => {
                assert_eq!(bit, Bit::Zero);
            },
            Err(_) => {
                assert!(false);
            }
        }


        match bit_string.get_bit(99) {
            Ok(bit) => {
                assert_eq!(bit, Bit::One);
            },
            Err(_) => {
                assert!(false);
            }
        }

        
        match bit_string.get_bit(110) {
            Ok(_) => {
                assert!(false);
            },
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_bit_push_byte() {
        let mut bits = BitString::new();
        
        bits.push_bit(1);
        bits.push_bit(0);
        bits.push_bit(1);
        bits.push_bit(0);
        
        assert_eq!("1010", format!("{}", bits));

        bits.push_byte(1);
        assert_eq!("101000000001", format!("{}", bits));
    }

    #[test]
    fn test_xor_with_other() {
        let mut bits = BitString::new();
        bits.push_byte(255);

        let mut xor_bits = BitString::new();
        xor_bits.push_byte(254);
        
        let _ = bits.xor_with_other(&xor_bits);
        assert_eq!("00000001", format!("{}", bits));

        bits = BitString::from_string("110010001111010");
        xor_bits = BitString::from_string("011000001101000");

        let _ = bits.xor_with_other(&xor_bits);
        assert_eq!("101010000010010", format!("{}", bits));
    }

    #[test]
    fn test_right_shift() {
        let mut bits = BitString::new();
        bits.push_byte(255);
        assert_eq!("11111111", format!("{}", bits));
        bits.right_shift();
        assert_eq!("011111111", format!("{}", bits));
    }

    #[test]
    fn test_bitstring_from_string() {
        let bits = BitString::from_string("1110111010");
        assert_eq!("1110111010", format!("{}", bits));
    }
}
