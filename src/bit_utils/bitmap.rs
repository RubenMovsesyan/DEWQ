/// A bitmap representation for storing and manipulating bit-level data
///
/// # Structure
///
/// The `BitMap` stores a 2D grid of bits as a vector of bytes, optimized for space efficiency
///
/// # Methods
///
/// Provides methods to:
/// - Create a new bitmap
/// - Set and get individual bits
/// - Invert bits
/// - Get bitmap size
/// - Save bitmap to a file
///
/// # Example
///
/// ```rust
/// let mut bitmap = BitMap::new(10);
/// bitmap.set(5, 7, 1);
/// assert_eq!(bitmap.get(5, 7), Bit::One);
/// ```


use super::bit::Bit;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;

// Helper functions
fn get_byte_location(j: usize) -> (usize, usize) {
    (j / 8, j % 8)
}

// bitmaps can only be square in size
pub struct BitMap {
    /// Internal storage of bits using byte arrays
    map: Vec<Vec<u8>>,
    /// Size of the bitmap (width and height)
    size: usize,
}

impl BitMap {
    /// Creates a new bitmap with specified size
    ///
    /// # Arguments
    ///
    /// * `size` - The width and height of the bitmap (must be square)
    ///
    /// # Returns
    ///
    /// A new `BitMap` initialized with zeros
    pub fn new(size: usize) -> Self {
        Self {
            map: vec![vec![0u8; (size / 8) + 1]; size],
            size,
        }
    }

    /// Sets a specific bit in the bitmap
    ///
    /// # Arguments
    ///
    /// * `i` - Row index
    /// * `j` - Column index
    /// * `bit` - Bit value to set (can be `Bit::One` or `Bit::Zero`)
    pub fn set<B>(&mut self, i: usize, j: usize, bit: B)
    where
        B: Into<Bit>,
    {
        if i >= self.size || j >= self.size {
            return;
        }

        let row = &mut self.map[i];

        let (byte, byte_offset) = get_byte_location(j);

        match bit.into() {
            Bit::One => row[byte] |= 1 << byte_offset,
            Bit::Zero => row[byte] &= !(1 << byte_offset),
        }
    }

    /// Inverts the bit at the specified location
    ///
    /// # Arguments
    ///
    /// * `i` - Row index
    /// * `j` - Column index
    pub fn invert_bit(&mut self, i: usize, j: usize) {
        let row = &mut self.map[i];

        let (byte, byte_offset) = get_byte_location(j);

        row[byte] ^= 1 << byte_offset;
    }

    /// Retrieves the bit value at a specific location
    ///
    /// # Arguments
    ///
    /// * `i` - Row index
    /// * `j` - Column index
    ///
    /// # Returns
    ///
    /// The `Bit` value at the specified location
    pub fn get(&self, i: usize, j: usize) -> Bit {
        let row = &self.map[i];

        let (byte, byte_offset) = get_byte_location(j);

        (row[byte] & (1 << byte_offset)).into()
    }

    /// Returns the size of the bitmap
    ///
    /// # Returns
    ///
    /// The width/height of the bitmap
    pub fn size(&self) -> usize {
        self.size
    }

    /// Saves the bitmap to a file in BMP format
    ///
    /// # Arguments
    ///
    /// * `path` - File path to save the bitmap
    ///
    /// # Remarks
    ///
    /// Creates a 1bpp bitmap image with a black and white color palette
    pub fn save_to_file<P>(&self, path: P)
    where
        P: AsRef<std::path::Path>,
    {
        if let Ok(mut file) = File::create(path) {
            // Write the bmp header 14 bytes

            // header field
            _ = file.write(&[0x42, 0x4D]); // ASCII BM

            // Size of the bmp file in bytes
            let file_size = (62 + (self.size() * self.size())) as u32;
            // let file_size = (62 + 100) as u32;
            _ = file.write(&[
                file_size as u8,
                (file_size >> 8) as u8,
                (file_size >> 16) as u8,
                (file_size >> 24) as u8,
            ]);

            // reserved bytes
            _ = file.write(&[0, 0, 0, 0]);

            // Offset of the pixel array
            _ = file.write(&[62, 0, 0, 0]);

            // BITMAPINFOHEADER --------
            // Size of this header (40 bytes)
            _ = file.write(&[40, 0, 0, 0]);

            // Bitmap width in pixels
            // _ = file.write(&[10, 0, 0, 0]);
            // _ = file.write(&[10, 0, 0, 0]);
            _ = file.write(&[
                self.size() as u8,
                (self.size() >> 8) as u8,
                (self.size() >> 16) as u8,
                (self.size() >> 24) as u8,
            ]);

            // Bitmap height in pixels
            _ = file.write(&[
                self.size() as u8,
                (self.size() >> 8) as u8,
                (self.size() >> 16) as u8,
                (self.size() >> 24) as u8,
            ]);

            // Number of color planes (must be 1)
            _ = file.write(&[1, 0]);

            // Number of bits per pixel (1 in our case)
            _ = file.write(&[1, 0]);

            // Compression method being used (no compression)
            _ = file.write(&[0, 0, 0, 0]);

            // Image size (can be ignored)
            _ = file.write(&[0, 0, 0, 0]);

            // Horizontal resolution of the bitmap
            _ = file.write(&[255, 255, 255, 255]);

            // Vertical resolution of the bitmap
            _ = file.write(&[255, 255, 255, 255]);

            // Number of colors in the palette
            _ = file.write(&[2, 0, 0, 0]);

            // Number of important colors in the palette
            _ = file.write(&[0, 0, 0, 0]);

            // Color Palette
            // White
            _ = file.write(&[255, 255, 255, 0]);
            // Black
            _ = file.write(&[0, 0, 0, 0]);

            // Write the bits to the bitmap
            let mut bit_index = 0;
            let mut current_byte = 0;
            for i in (0..self.size()).rev() {
                for j in 0..self.size() {
                    match self.get(i, j) {
                        Bit::Zero => {}
                        Bit::One => {
                            current_byte |= 1 << (31 - bit_index);
                        }
                    }

                    bit_index += 1;

                    if bit_index == 32 {
                        _ = file.write(&[
                            (current_byte >> 24) as u8,
                            (current_byte >> 16) as u8,
                            (current_byte >> 8) as u8,
                            current_byte as u8,
                        ]);
                        current_byte = 0;
                        bit_index = 0;
                    }
                }

                if bit_index != 0 {
                    _ = file.write(&[
                        (current_byte >> 24) as u8,
                        (current_byte >> 16) as u8,
                        (current_byte >> 8) as u8,
                        current_byte as u8,
                    ]);
                }
                current_byte = 0;
                bit_index = 0;
            }
        }
    }
}

/// Implements a text-based display of the bitmap
///
/// Renders the bitmap using block characters, where:
/// - `██` represents a black/set bit
/// - `  ` represents a white/unset bit
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
                    Bit::Zero => {
                        write!(f, "██")?;
                    }
                    Bit::One => {
                        write!(f, "  ")?;
                    }
                }
            }
            write!(f, "██")?;
            writeln!(f, "██")?;
        }

        for _ in 0..=self.size + 2 {
            write!(f, "██")?;
        }
        writeln!(f, "██")?;
        for _ in 0..=self.size + 2 {
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

        assert_eq!(bit_map.map.len(), 10);
        assert_eq!(bit_map.map[0].len(), 2);
    }
}
