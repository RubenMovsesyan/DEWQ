use std::{error::Error, fmt::Display};

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
                    *byte = *byte | (1 << bit_offset);
                }
            }
        }
        self.bits_len += 1;
    }

    pub fn get_bit(&self, address: usize) -> Result<Bit, BitAddressOutOfBoundsError> {
        if address > self.bits_len {
            return Err(BitAddressOutOfBoundsError);
        }

        let bit_address = address / 8;
        let bit_offset = address % 8;
        
        match self.bits[bit_address] & (1 << bit_offset) {
            0 => Ok(Bit::Zero),
            _ => Ok(Bit::One),
        }
    }
}

impl Display for BitString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.bits.iter() {
            for i in 0..8 {
                write!(f, "{}", ((*byte & (1 << i)) >> i))?;
            }
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

        println!("{}", bit_string);
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
}
