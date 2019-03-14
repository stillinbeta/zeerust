use crate::cpu::opcodes::util::*;
use crate::ops::{Location16, Op, Reg16};

pub fn parse(reg: Reg16, op: u8, n1: u8, n2: u8) -> (Op, usize) {
    match op {
        0x21 => (Op::LD16(Location16::Reg(reg), le_immediate(n1, n2)), 4),
        0x2A => (Op::LD16(Location16::Reg(reg), le_imm_indir(n1, n2)), 4),
        0x22 => (Op::LD16(le_imm_indir(n1, n2), Location16::Reg(reg)), 4),
        0xF9 => (
            Op::LD16(Location16::Reg(Reg16::SP), Location16::Reg(reg)),
            2,
        ),
        0xE1 => (Op::POP(Location16::Reg(reg)), 2),
        0xE5 => (Op::PUSH(Location16::Reg(reg)), 2),
        _op => unimplemented!("{:?} {:02x}", reg, op),
    }
}
