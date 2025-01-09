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


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Bit {
    Zero,
    One,
}

macro_rules! impl_from_for_bit {
    ($($t:ty), *) => {
        $(
            impl From<$t> for Bit {
                fn from(value: $t) -> Self {
                    match value {
                        0 => Bit::Zero,
                        _ => Bit::One,
                    }
                }
            }
        )*
    };
}

impl_from_for_bit!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[cfg(any(
        test,
        feature = "test_feature"
))]
impl Display for Bit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Bit::Zero => { write!(f, "{}", 0)?; },
            Bit::One => { write!(f, "{}", 1)?; },
        }

        Ok(())
    }
}
