use crate::cpu::opcodes::util::*;
use crate::ops::{Location16, Op, Reg16};

pub fn parse(reg: Reg16, op: u8, n1: u8, n2: u8) -> (Op, usize) {
    match op {
        0x21 => (Op::LD16(Location16::Reg(reg), le_immediate(n1, n2)), 4),
        _op => unimplemented!(),
    }
}
