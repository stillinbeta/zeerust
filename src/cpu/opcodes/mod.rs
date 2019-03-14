use crate::ops::{JumpConditional, Location16, Location8, Op, Reg16, Reg8};

mod bits;
mod file;
mod index;
mod util;

#[cfg(test)]
mod test;

pub use file::parse_stream;
use util::*;

pub fn opcode(code: [u8; 4]) -> (Op, usize) {
    match code {
        [0x00, _, _, _] => (Op::NOP, 1),
        [0x76, _, _, _] => (Op::HALT, 1),

        // Rotates without operands
        [0x07, _, _, _] => (Op::RLCA, 1),
        [0x0F, _, _, _] => (Op::RRCA, 1),
        [0x17, _, _, _] => (Op::RLA, 1),
        [0x1F, _, _, _] => (Op::RRA, 1),
        [0xED, 0x67, _, _] => (Op::RRD, 2),
        [0xED, 0x6F, _, _] => (Op::RLD, 2),
        // Bits are all 0xCB
        [0xCB, op, _, _] => bits::parse(op),

        // Input/Output
        [0xDB, n, _, _] => (Op::IN(Location8::Reg(Reg8::A), Location8::Immediate(n)), 2),
        [0xD3, n, _, _] => (Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(n)), 2),
        [0xED, op, _, _] if op & 0b1100_0110 == 0b0100_0000 => {
            let opr = if op & 0b1 == 0b1 { Op::OUT } else { Op::IN };
            if let reg @ Location8::Reg(_) = reg_bits(op >> 3) {
                (opr(reg, Location8::Reg(Reg8::C)), 2)
            } else {
                // {IN,OUT}((HL), (C)) is not valid
                panic!("Unknown ExtendeD operation {:02x}", op)
            }
        }
        [0xED, op, _, _] if op & 0b1100_0111 == 0b0100_0000 => {
            if let reg @ Location8::Reg(_) = reg_bits(op >> 3) {
                (Op::IN(reg, Location8::Reg(Reg8::C)), 2)
            } else {
                // IN((HL), (C)) is not valid
                panic!("Unknown IN operation")
            }
        }

        // Jump
        [0xC3, n1, n2, _] => (
            Op::JP(
                JumpConditional::Unconditional,
                Location16::Immediate(u16::from_le_bytes([n1, n2])),
            ),
            3,
        ),
        [op, n1, n2, _] if op & 0b1100_0111 == 0b1100_0010 => {
            let jc = match (op >> 3) & 0b111 {
                0b000 => JumpConditional::NonZero,
                0b001 => JumpConditional::Zero,
                0b010 => JumpConditional::NoCarry,
                0b011 => JumpConditional::Carry,
                0b100 => JumpConditional::ParityOdd,
                0b101 => JumpConditional::ParityEven,
                0b110 => JumpConditional::SignPositive,
                0b111 => JumpConditional::SignNegative,
                _ => unreachable!(),
            };
            (
                Op::JP(jc, Location16::Immediate(u16::from_le_bytes([n1, n2]))),
                3,
            )
        }
        // Jump Relative
        [0x18, e, _, _] => (Op::JR(JumpConditional::Unconditional, e as i8), 2),
        [op, e, _, _] if op & 0b1110_0111 == 0b0010_0000 => {
            let jc = match op >> 3 & 0b11 {
                0b00 => JumpConditional::NonZero,
                0b01 => JumpConditional::Zero,
                0b10 => JumpConditional::NoCarry,
                0b11 => JumpConditional::Carry,
                _ => unreachable!(),
            };
            (Op::JR(jc, e as i8), 2)
        }
        [0x10, e, _, _] => (Op::DJNZ(e as i8), 2),

        // 8-bit Load
        [op, _, _, _] if op & 0b1100_0000 == 0b0100_0000 => {
            (Op::LD8(reg_bits(op >> 3), reg_bits(op)), 1)
        }
        [op, i, _, _] if op & 0b1100_0111 == 0b0000_0110 => {
            (Op::LD8(reg_bits(op >> 3), Location8::Immediate(i)), 2)
        }

        // 16 bit loads
        [op, n1, n2, _] if op & 0b1100_1111 == 0b0000_0001 => {
            (Op::LD16(reg16_bits(op >> 4), le_immediate(n1, n2)), 3)
        }

        [0x2A, n1, n2, _] => (
            Op::LD16(Location16::Reg(Reg16::HL), le_imm_indir(n1, n2)),
            3,
        ),
        [0xED, op, n1, n2] if op & 0b1100_1111 == 0b0100_1011 => {
            (Op::LD16(reg16_bits(op >> 4), le_imm_indir(n1, n2)), 4)
        }

        [0x22, n1, n2, _] => (
            Op::LD16(le_imm_indir(n1, n2), Location16::Reg(Reg16::HL)),
            3,
        ),
        [0xED, op, n1, n2] if op & 0b1100_1111 == 0b0100_0011 => {
            (Op::LD16(le_imm_indir(n1, n2), reg16_bits(op >> 4)), 4)
        }
        [0xF9, _, _, _] => (
            Op::LD16(Location16::Reg(Reg16::SP), Location16::Reg(Reg16::HL)),
            1,
        ),

        [op, _, _, _] if op & 0b1100_1111 == 0b1100_0101 => (Op::PUSH(reg16_bits_af(op >> 4)), 1),
        [op, _, _, _] if op & 0b1100_1111 == 0b1100_0001 => (Op::POP(reg16_bits_af(op >> 4)), 1),
        [0xDD, o1, n1, n2] => index::parse(Reg16::IX, o1, n1, n2),
        [0xFD, o1, n1, n2] => index::parse(Reg16::IY, o1, n1, n2),

        // Indirect Loads
        [0x0A, _, _, _] => (
            Op::LD8(Location8::Reg(Reg8::A), Location8::RegIndirect(Reg16::BC)),
            1,
        ),
        [0x1A, _, _, _] => (
            Op::LD8(Location8::Reg(Reg8::A), Location8::RegIndirect(Reg16::DE)),
            1,
        ),
        [0x3A, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (
                Op::LD8(Location8::Reg(Reg8::A), Location8::ImmediateIndirect(addr)),
                3,
            )
        }
        [0x02, _, _, _] => (
            Op::LD8(Location8::RegIndirect(Reg16::BC), Location8::Reg(Reg8::A)),
            1,
        ),
        [0x12, _, _, _] => (
            Op::LD8(Location8::RegIndirect(Reg16::DE), Location8::Reg(Reg8::A)),
            1,
        ),
        [0x32, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (
                Op::LD8(Location8::ImmediateIndirect(addr), Location8::Reg(Reg8::A)),
                3,
            )
        }

        // Misc Math
        [0x2F, _, _, _] => (Op::CPL, 1),
        [0xED, 0x44, _, _] => (Op::NEG, 2),
        [0x3F, _, _, _] => (Op::CCF, 1),
        [0x37, _, _, _] => (Op::SCF, 1),

        [op, o1, _, _] if op & 0b1010_0000 == 0b1010_0000 => {
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

        // INC
        [a, _, _, _] if a & 0b1100_0111 == 0b0000_0100 => (Op::INC(reg_bits(a >> 3)), 1),
        // DEC
        [a, _, _, _] if a & 0b1100_0111 == 0b0000_0101 => (Op::DEC(reg_bits(a >> 3)), 1),
        // Add and Subtract
        [op, o1, _, _] if op & 0b1010_0000 == 0x80 => {
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

        // [op, _, _, ]
        [o1, o2, o3, o4] => panic!(
            "Unimplemented opcode [{:02x}, {:02x}, {:02x}, {:02x}]",
            o1, o2, o3, o4
        ),
    }
}
