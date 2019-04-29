extern crate zeerust;

use zeerust::cpu::opcodes::parse_stream;
use zeerust::examples::HELLO_ZEERUST_BIN;
use zeerust::ops::*;

#[test]
fn parse_bin() {
    let expected: Vec<Op> = vec![
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate(b'Z')),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate(b'E')),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate(b'R')),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate(b'U')),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate(b'S')),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate(b'T')),
        Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)),
        Op::HALT,
    ];
    assert_eq!(expected, parse_stream(HELLO_ZEERUST_BIN.into()));
}
