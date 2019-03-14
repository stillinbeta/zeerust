use crate::ops::{Location16, Location8, Reg16, Reg8};

// Many instructions use a common bit pattern to designate single registers.
pub fn reg_bits(bits: u8) -> Location8 {
    match bits & 0b111 {
        0b111 => Location8::Reg(Reg8::A),
        0b000 => Location8::Reg(Reg8::B),
        0b001 => Location8::Reg(Reg8::C),
        0b010 => Location8::Reg(Reg8::D),
        0b011 => Location8::Reg(Reg8::E),
        0b100 => Location8::Reg(Reg8::H),
        0b101 => Location8::Reg(Reg8::L),
        0b110 => Location8::RegIndirect(Reg16::HL),
        _ => unreachable!(),
    }
}

pub fn reg16_bits(bits: u8) -> Location16 {
    match bits & 0b11 {
        0b00 => Location16::Reg(Reg16::BC),
        0b01 => Location16::Reg(Reg16::DE),
        0b10 => Location16::Reg(Reg16::HL),
        0b11 => Location16::Reg(Reg16::SP),
        _ => unreachable!(),
    }
}

// PUSH and POP use a slightly different bit pattern
pub fn reg16_bits_af(bits: u8) -> Location16 {
    match bits & 0b11 {
        0b00 => Location16::Reg(Reg16::BC),
        0b01 => Location16::Reg(Reg16::DE),
        0b10 => Location16::Reg(Reg16::HL),
        0b11 => Location16::Reg(Reg16::AF),
        _ => unreachable!(),
    }
}

pub fn le_immediate(n0: u8, n1: u8) -> Location16 {
    Location16::Immediate(u16::from_le_bytes([n0, n1]))
}

pub fn le_imm_indir(n0: u8, n1: u8) -> Location16 {
    Location16::ImmediateIndirect(u16::from_le_bytes([n0, n1]))
}
