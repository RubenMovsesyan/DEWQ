use non_std::Vec;

#[cfg(test)]
extern crate std;

#[cfg(test)]
use std::fmt::Display;

#[cfg(test)]
#[macro_use]
use std::format;

pub struct BitString {
    bits: Vec<u8>,
    bits_len: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Bit {
    Zero,
    One,
}

impl<I> From<I> for Bit
    where I: Into<i32>
{
    fn from(value: I) -> Bit {
        match value.into() {
            0 => Bit::Zero,
            _ => Bit::One,
        }
    }
}

impl BitString {
    pub fn new() -> BitString {
        BitString {
            bits: Vec::new(),
            bits_len: 0,
        }
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
        if address > self.bits_len {
            return Err(BitAddressOutOfBoundsError);
        }

        let bit_address = address / 8;
        let bit_offset = address % 8;
        
        match self.bits[bit_address] & (1 << (7 - bit_offset)) {
            0 => Ok(Bit::Zero),
            _ => Ok(Bit::One),
        }
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
}

#[cfg(test)]
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
}
