

#[doc(hidden)]
#[macro_export]
macro_rules! __str_const {
    ($e:expr) => {
        const { $crate::const_format::pmr::__AssertStr { x: $e }.x }
    };
}
