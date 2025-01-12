use non_std::Vec;
use crate::alloc::vec;
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
use std::println;


#[cfg(any(
        test,
        feature = "test_feature"
))]
#[macro_use]
use std::print;


// Helper functions
fn get_byte_location(j: usize) -> (usize, usize) {
    (j / 8, j % 8)
}


// bitmaps can only be square in size
pub struct BitMap {
    map: Vec<Vec<u8>>,
    size: usize,
}

impl BitMap {
    pub fn new(size: usize) -> Self {
        Self {
            map: vec![vec![0u8; (size / 8) + 1]; size],
            size
        }
    }

    pub fn set<B>(&mut self, i: usize, j: usize, bit: B) 
        where B: Into<Bit>
    {
        if i >= self.size || j >= self.size { return; }


        let row = &mut self.map[i];

        let (byte, byte_offset) = get_byte_location(j);

        match bit.into() {
            Bit::One => { row[byte] |= 1 << byte_offset },
            Bit::Zero => { row[byte] &= !(1 << byte_offset) },
        }
    }

    pub fn invert(&mut self) {
        self.map.iter_mut().for_each(|inner_vec| {
            inner_vec.iter_mut().for_each(|byte| {
                *byte ^= 0xFF;
            });
        });
    }

    pub fn invert_bit(&mut self, i: usize, j: usize) {
        let row = &mut self.map[i];

        let (byte, byte_offset) = get_byte_location(j);

        row[byte] ^= 1 << byte_offset;
    }

    pub fn get(&self, i: usize, j: usize) -> Bit {
        let row = &self.map[i];

        let (byte, byte_offset) = get_byte_location(j);

        (row[byte] & (1 << byte_offset)).into()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(any(
        test,
        feature = "test_feature"
))]
impl Display for BitMap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for _ in 0..=self.size + 2 {
            write!(f, "██")?;
        }
        writeln!(f, "██")?;
        for _ in 0..=self.size + 2 {
            write!(f, "██")?;
        }
        writeln!(f, "██")?;

        for i in 0..self.size {
            write!(f, "██")?;
            write!(f, "██")?;
            for j in 0..self.size {
                match self.get(i, j) {
                    Bit::Zero => { write!(f, "██")?; }
                    Bit::One => { write!(f, "  ")?; }
                }
            }
            write!(f, "██")?;
            writeln!(f, "██")?;
        }

        for _ in 0..=self.size + 2{
            write!(f, "██")?;
        }
        writeln!(f, "██")?;
        for _ in 0..=self.size + 2{
            write!(f, "██")?;
        }
        writeln!(f, "██")?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitmap_basics() {
        let mut bit_map = BitMap::new(10);

        bit_map.set(5, 7, 1);

        assert_eq!(bit_map.get(5, 7), Bit::One);
    }

    #[test]
    fn test_bitmap_sizing() {
        let bit_map = BitMap::new(10);

        assert_eq!(bit_map.map.len(), 11);
        assert_eq!(bit_map.map[0].len(), 2);
    }
}
