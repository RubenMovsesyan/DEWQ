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

pub(crate) use test_println;
