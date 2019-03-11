use crate::ops::{Location8, Op, Reg8, Reg16};

pub fn opcode(code: [u8; 4]) -> (Op, u8) {
    match code {
        [0x00, _, _, _] => (Op::NOP, 1),
        // INC
        [a, _, _, _] if a & 0b1100_0100 == 0b0000_0100 => {
            let loc = match (a & 0b0011_1000) >> 3 {
                0b111 => Location8::Reg(Reg8::A),
                0b000 => Location8::Reg(Reg8::B),
                0b001 => Location8::Reg(Reg8::C),
                0b010 => Location8::Reg(Reg8::D),
                0b011 => Location8::Reg(Reg8::E),
                0b100 => Location8::Reg(Reg8::H),
                0b101 => Location8::Reg(Reg8::L),
                0b110 => Location8::RegIndirect(Reg16::HL),
                _ => unreachable!(),
            };
            (Op::INC(loc), 1)
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! op4 {
        ($o1 : expr) => {
            [$o1, 0x00, 0x00, 0x00]
        };
        ($o1 : expr, $o2 : expr) => {
            [$o1, $o2, 0x00, 0x00]
        }; // TODO 3 and 4
    }

    #[test]
    fn nop() {
        assert_eq!((Op::NOP, 1), opcode([0x00; 4]))
    }

    #[test]
    fn inc_r() {
        let expected = [
            (Reg8::A, 0b0011_1100),
            (Reg8::B, 0b0000_0100),
            (Reg8::C, 0b0000_1100),
            (Reg8::D, 0b0001_0100),
            (Reg8::E, 0b0001_1100),
            (Reg8::H, 0b0010_0100),
            (Reg8::L, 0b0010_1100),
        ];

        for (reg, op) in expected.into_iter() {
            assert_eq!((Op::INC(Location8::Reg(*reg)), 1), opcode(op4!(*op)));
        }
    }

    #[test]
    fn inc_hl() {
        assert_eq!((Op::INC(Location8::RegIndirect(Reg16::HL)), 1), opcode(op4!(0b0011_0100)));
    }
}
