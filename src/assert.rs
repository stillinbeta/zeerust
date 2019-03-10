#[macro_export]
macro_rules! assert_bin {
    ( $ left : expr , $ right : expr ) => {
        assert_eq!($left, $right, "{:08b} does not equal {:08b}", $left, $right)
    };
}

#[macro_export]
macro_rules! assert_hex {
    ( $ left : expr , $ right : expr ) => {
        assert_eq!($left, $right, "{:02x} does not equal {:02x}", $left, $right)
    };
}

#[macro_export]
macro_rules! assert_flags {
    ( $registers : expr,  $($flag:ident = $value:expr),*  $(,)?) => {
        $(assert_eq!($registers.get_flag(&$crate::ops::StatusFlag::$flag),
                     $value,
                     "(CPU flag {})", stringify!($flag),
        );)*
    }
}
