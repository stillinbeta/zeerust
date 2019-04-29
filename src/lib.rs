//! An emulator for an idealised z80 CPU.

#[macro_use]
extern crate enum_display_derive;

pub mod cpu;
pub mod ops;
#[macro_use]
mod assert;
pub mod z80;
