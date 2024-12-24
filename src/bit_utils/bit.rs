#[cfg(test)]
extern crate std;

#[cfg(test)]
use std::fmt::Display;


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

#[cfg(test)]
impl Display for Bit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Bit::Zero => { write!(f, "{}", 0)?; },
            Bit::One => { write!(f, "{}", 1)?; },
        }

        Ok(())
    }
}
