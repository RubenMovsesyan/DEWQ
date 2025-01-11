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
