/// Calculates fizzbuzz (assembly)
pub const FIZZBUZZ_ASM: &str = include_str!("fizzbuzz.asm");
/// Calculates fizzbuzz (machine code)
pub const FIZZBUZZ_BIN: &[u8] = include_bytes!("fizzbuzz.bin");

/// Print ZEERUST using a hand-unrolled loop (assembly)
pub const HELLO_ZEERUST_ASM: &str = include_str!("zeerust.asm");
/// Print ZEERUST using a hand-unrolled loop (machine code)
pub const HELLO_ZEERUST_BIN: &[u8] = include_bytes!("zeerust.bin");

/// Print Hello World using a loop (assembly)
pub const HELLO_WORLD_ASM: &str = include_str!("hello_world.asm");
/// Print Hello World using a loop (machine code)
pub const HELLO_WORLD_BIN: &[u8] = include_bytes!("hello_world.bin");

/// Count down from 9 to 1 (assembly)
pub const COUNTDOWN_ASM: &str = include_str!("countdown.asm");
/// Count down from 9 to 1 (machine code)
pub const COUNTDOWN_BIN: &[u8] = include_bytes!("countdown.bin");

/// Example is a named z80 program, along with its associated assembly.
pub struct Example {
    pub name: &'static str,
    pub assembly: &'static str,
    pub binary: &'static [u8],
}

pub const EXAMPLES: &[Example] = &[
    Example {
        name: "fizzbuzz",
        assembly: FIZZBUZZ_ASM,
        binary: FIZZBUZZ_BIN,
    },
    Example {
        name: "hello zeerust",
        assembly: HELLO_ZEERUST_ASM,
        binary: HELLO_ZEERUST_BIN,
    },
    Example {
        name: "hello world",
        assembly: HELLO_WORLD_ASM,
        binary: HELLO_WORLD_BIN,
    },
    Example {
        name: "countdown",
        assembly: COUNTDOWN_ASM,
        binary: COUNTDOWN_BIN,
    },
];
