#[cfg(any(
        test,
        feature = "test_feature"
))]
extern crate std;

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


#[macro_export]
macro_rules! test_println {
    ($($t:tt)*) => {{
        #[cfg(any(
                test,
                feature = "test_feature"
        ))]
        println!($($t)*);
    }};
}

#[macro_export]
macro_rules! test_print {
    ($($t:tt)*) => {{
        #[cfg(any(
                test,
                feature = "test_feature"
        ))]
        print!($($t)*);
    }};
}

pub(crate) use test_println;
pub(crate) use test_print;
