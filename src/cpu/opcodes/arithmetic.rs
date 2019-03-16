use crate::ops::{Location8, Op, Reg8};

use super::util::reg_bits;

pub fn add_subtract(op: u8, o1: u8) -> (Op, usize) {
    let opr = match op & 0b0001_1000 {
        0b0000_0000 => Op::ADD8,
        0b0000_1000 => Op::ADC,
        0b0001_0000 => Op::SUB8,
        0b0001_1000 => Op::SBC,
        _ => unreachable!(),
    };
    let (loc, b) = if op & 0b0100_0000 == 0b0100_0000 {
        // if 6th bit set, immediate
        (Location8::Immediate(o1), 2)
    } else {
        // Otherwise Just a regular bit register
        (reg_bits(op), 1)
    };
    (opr(Location8::Reg(Reg8::A), loc), b)
}

pub fn boolean(op: u8, o1: u8) -> (Op, usize) {
    let opr = match op & 0b0001_1000 {
        0b0000_0000 => Op::AND,
        0b0001_0000 => Op::OR,
        0b0000_1000 => Op::XOR,
        0b0001_1000 => Op::CP,
        _ => unreachable!(),
    };
    let (loc, b) = if op & 0b0100_0000 == 0b0100_0000 {
        // if 6th bit set, immediate
        (Location8::Immediate(o1), 2)
    } else {
        // Otherwise Just a regular bit register
        (reg_bits(op), 1)
    };
    (opr(loc), b)
}
