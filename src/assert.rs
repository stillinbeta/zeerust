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
