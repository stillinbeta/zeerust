extern crate zeerust;

use zeerust::cpu::opcodes::parse_stream;
use zeerust::ops::*;

const HELLO_ZEERUST: &'static [u8] = include_bytes!("zeerust.bin");

#[test]
fn parse_bin() {
    let expected: Vec<Op> = vec![
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate('Z' as u8)),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate('E' as u8)),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate('R' as u8)),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate('U' as u8)),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate('S' as u8)),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate('T' as u8)),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::HALT,
    ];
    assert_eq!(expected, parse_stream(HELLO_ZEERUST.into()));
}
