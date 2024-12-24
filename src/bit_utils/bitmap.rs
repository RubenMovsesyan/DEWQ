use non_std::Vec;
use crate::alloc::vec;
use super::bit::Bit;

#[cfg(test)]
extern crate std;

#[cfg(test)]
use std::fmt::Display;

#[cfg(test)]
#[macro_use]
use std::println;


#[cfg(test)]
#[macro_use]
use std::print;

// bitmaps can only be square in size
pub struct BitMap {
    map: Vec<Vec<u8>>,
    size: usize,
}

impl BitMap {
    pub fn new(size: usize) -> Self {
        let mut map: Vec<Vec<u8>> = Vec::with_capacity(size);
        for _ in 0..size {
            let mut row: Vec<u8> = Vec::with_capacity((size / 8) + 1);
            for _ in 0..=(size / 8) {
                row.push(0);
            }
            map.push(row);
        }

        Self {
            map,
            size
        }
    }

    pub fn set<B>(&mut self, i: usize, j: usize, bit: B) 
        where B: Into<Bit>
    {
        let row = &mut self.map[i];

        let byte = j / 8;
        let byte_offset = j % 8;

        row[byte] |= {
            match bit.into() {
                Bit::One => { 1 },
                Bit::Zero => { 0 },
            }
        } << byte_offset;
    }

    pub fn get(&self, i: usize, j: usize) -> Bit {
        let row = &self.map[i];

        let byte = j / 8;
        let byte_offset = j % 8;

        (row[byte] & (1 << byte_offset)).into()
    }
}

#[cfg(test)]
impl Display for BitMap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                match self.get(i, j) {
                    Bit::Zero => { write!(f, "⬛")?; }
                    Bit::One => { write!(f, "⬜")?; }
                }
            }
            writeln!(f)?;
        }

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

        println!("{}", bit_map);
    }
}
