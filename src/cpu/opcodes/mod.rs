use crate::ops::{Location8, Op, Reg16, Reg8};

#[cfg(test)]
mod test;

pub fn opcode(code: [u8; 4]) -> (Op, u8) {
    match code {
        [0x00, _, _, _] => (Op::NOP, 1),

        // Rotates without operands
        [0x07, _, _, _] => (Op::RLCA, 1),
        [0x0F, _, _, _] => (Op::RRCA, 1),
        [0x17, _, _, _] => (Op::RLA, 1),
        [0x1F, _, _, _] => (Op::RRA, 1),
        [0xED, 0x67, _, _] => (Op::RRD, 2),
        [0xED, 0x6F, _, _] => (Op::RLD, 2),
        // Bits are all 0xCB
        [0xCB, op, _, _] => {
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

        // Input/Output
        [0xDB, n, _, _] => (Op::IN(Location8::Reg(Reg8::A), Location8::Immediate(n)), 2),
        [0xD3, n, _, _] => (Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(n)), 2),
        [0xED, op, _, _] if op & 0b1100_0110 == 0b0100_0000 => {
            let opr = if op & 0b1 == 0b1 {
                Op::OUT
            } else {
                Op::IN
            };
            if let reg@Location8::Reg(_) = reg_bits(op >> 3) {
                (opr(reg, Location8::Reg(Reg8::C)), 2)
            } else {
                // {IN,OUT}((HL), (C)) is not valid
                panic!("Unknown ExtendeD operation {:02x}", op)
            }
        }
        [0xED, op, _, _] if op & 0b1100_0111 == 0b0100_0000 => {
            if let reg@Location8::Reg(_) = reg_bits(op >> 3) {
                (Op::IN(reg, Location8::Reg(Reg8::C)), 2)
            } else {
                // IN((HL), (C)) is not valid
                panic!("Unknown IN operation")
            }
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
        // 8-bit Load
        [op, _, _, _] if op & 0b1100_0000 == 0b0100_0000 => {
            (Op::LD8(reg_bits(op >> 3), reg_bits(op)), 1)
        }
        [op, i, _, _] if op & 0b1100_0111 == 0b0000_0110 => {
            (Op::LD8(reg_bits(op >> 3), Location8::Immediate(i)), 2)
        }

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

        // [op, _, _, ]
        [o1, o2, o3, o4] => panic!(
            "Unimplemented opcode [{:02x}, {:02x}, {:02x}, {:02x}]",
            o1, o2, o3, o4
        ),
    }
}

// Many instructions use a common bit pattern to designate single registers.
fn reg_bits(bits: u8) -> Location8 {
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
