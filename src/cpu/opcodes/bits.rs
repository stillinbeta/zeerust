use super::util::reg_bits;
use crate::ops::Op;

pub fn parse(op: u8) -> (Op, usize) {
    let loc = reg_bits(op);
    let opr = match op >> 6 {
        0b00 => {
            let opr = match op >> 3 {
                0b000 => Op::RLC,
                0b001 => Op::RRC,
                0b010 => Op::RL,
                0b011 => Op::RR,
                0b100 => Op::SLA,
                0b101 => Op::SRA,
                // http://z80-heaven.wikidot.com/instructions-set:sll
                0b110 => panic!("Use of undocumented instruction SLL"),
                0b111 => Op::SRL,
                _ => unreachable!(),
            };
            return (opr(loc), 2);
        }
        0b01 => Op::BIT,
        0b10 => Op::RES,
        0b11 => Op::SET,
        _ => unreachable!(),
    };
    let reg = (op >> 3) & 0b111;
    (opr(reg, loc), 2)
}
