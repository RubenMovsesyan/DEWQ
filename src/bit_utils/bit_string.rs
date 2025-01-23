use super::bit::Bit;
use std::fmt::Display;

pub struct BitString {
    bits: Vec<u8>,
    bits_len: usize,
}

impl BitString {
    pub fn new() -> Self {
        Self {
            bits: Vec::new(),
            bits_len: 0,
        }
    }

    pub fn from_vec(vector: Vec<u8>) -> Self {
        Self {
            bits_len: vector.len() * 8,
            bits: vector,
        }
    }

    pub fn push_bit<B>(&mut self, bit: B)
    where
        B: Into<Bit>,
    {
        let bit_offset = self.bits_len % 8;

        if bit_offset == 0 {
            self.bits.push(0);
        }

        match bit.into() {
            Bit::Zero => {}
            Bit::One => {
                if let Some(byte) = self.bits.last_mut() {
                    *byte |= 1 << (7 - bit_offset);
                }
            }
        }
        self.bits_len += 1;
    }

    pub fn push_bit_times<B, USIZE>(&mut self, bit: B, times: USIZE)
    where
        B: Into<Bit>,
        USIZE: Into<usize>,
    {
        let b = bit.into();
        for _ in 0..times.into() {
            self.push_bit(b.clone());
        }
    }

    pub fn push_byte<U8>(&mut self, byte: U8)
    where
        U8: Into<u8>,
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

    pub fn get_byte(&self, index: usize) -> u8 {
        self.bits[index]
    }

    pub fn len(&self) -> usize {
        self.bits_len
    }

    #[allow(unused)]
    pub fn as_hex(&self) -> HexStr {
        let mut chars: HexStr = HexStr(Vec::new());

        let mut index = 0;

        while index < self.len() {
            let mut val: u8 = 0;
            for _ in 0..4 {
                val <<= 1;
                if let Ok(bit) = self.get_bit(index) {
                    match bit {
                        Bit::One => val |= 1,
                        _ => {}
                    }
                }
                index += 1;
            }

            chars.0.push(match val {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'A',
                11 => 'B',
                12 => 'C',
                13 => 'D',
                14 => 'E',
                15 => 'F',
                _ => '@',
            });
        }

        chars
    }
}

pub struct HexStr(Vec<char>);

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
impl Display for BitString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.bits_len {
            write!(
                f,
                "{}",
                if self.get_bit(i).unwrap() == Bit::One {
                    1
                } else {
                    0
                }
            )?;
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
                }
                Err(_) => {
                    assert!(false);
                }
            }
        }

        match bit_string.get_bit(5) {
            Ok(_) => {
                assert!(false);
            }
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
            }
            Err(_) => {
                assert!(false);
            }
        }

        match bit_string.get_bit(50) {
            Ok(bit) => {
                assert_eq!(bit, Bit::Zero);
            }
            Err(_) => {
                assert!(false);
            }
        }

        match bit_string.get_bit(99) {
            Ok(bit) => {
                assert_eq!(bit, Bit::One);
            }
            Err(_) => {
                assert!(false);
            }
        }

        match bit_string.get_bit(110) {
            Ok(_) => {
                assert!(false);
            }
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
