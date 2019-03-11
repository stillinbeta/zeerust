use crate::ops::{Location8, Op, Reg16, Reg8};

pub fn opcode(code: [u8; 4]) -> (Op, u8) {
    match code {
        [0x00, _, _, _] => (Op::NOP, 1),
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

        [o1, o2, o3, o4] => panic!(
            "Unimplemented opcode [{:02x}, {:02x}, {:02x}, {:02x}]",
            o1, o2, o3, o4
        ),
    }
}

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

#[cfg(test)]
mod test {

    #[allow(unused_imports)]
    use crate::ops::{Location8::*, Op::*, Reg16::*, Reg8::*};

    macro_rules! op4 {
        ($o1 : expr) => {
            [$o1, 0x00, 0x00, 0x00]
        };
        ($o1 : expr, $o2 : expr) => {
            [$o1, $o2, 0x00, 0x00]
        };
        ($o1 : expr, $o2 : expr, $o3 : expr) => {
            [$o1, $o2, $o3, 0x00]
        };
    }

    macro_rules! assert_opcode {
        ($opc : expr, $bytes : expr, $o1 : expr) => {
            let (opc, bytes) = $crate::z80::opcode::opcode(op4!($o1));
            assert_eq!($bytes, bytes, "Opcode {:?} ({:02x})", $opc, $o1);
            assert_eq!($opc, opc, "Opcode {:?} ({:02x})", $opc, $o1);
        };
        ($opc : expr, $bytes : expr, $o1 : expr, $o2 : expr) => {
            let (opc, bytes) = $crate::z80::opcode::opcode(op4!($o1, $o2));
            assert_eq!($bytes, bytes, "Opcode {:?} ({:02x} {:02x})", $opc, $o1, $o2);
            assert_eq!($opc, opc, "Opcode {:?} ({:02x} {:02x})", $opc, $o1, $o2);
        };
        ($opc : expr, $bytes : expr, $o1 : expr, $o2 : expr, $o3 : expr) => {
            let (opc, bytes) = $crate::z80::opcode::opcode(op4!($o1, $o2, $o3));
            assert_eq!(
                $bytes, bytes,
                "Opcode {:?} ({:02x} {:02x} {:02x})",
                $opc, $o1, $o2, $o3
            );
            assert_eq!(
                $opc, opc,
                "Opcode {:?} ({:02x} {:02x} {:02x})",
                $opc, $o1, $o2, $o3
            );
        };
    }

    #[test]
    fn nop() {
        assert_opcode!(NOP, 1, 0x00);
    }

    #[test]
    fn inc() {
        assert_opcode!(INC(Reg(A)), 1, 0x3C);
        assert_opcode!(INC(Reg(B)), 1, 0x04);
        assert_opcode!(INC(Reg(C)), 1, 0x0C);
        assert_opcode!(INC(Reg(D)), 1, 0x14);
        assert_opcode!(INC(Reg(E)), 1, 0x1C);
        assert_opcode!(INC(Reg(H)), 1, 0x24);
        assert_opcode!(INC(Reg(L)), 1, 0x2C);

        assert_opcode!(INC(RegIndirect(HL)), 1, 0x34);
    }

    #[test]
    fn dec() {
        assert_opcode!(DEC(Reg(A)), 1, 0x3D);
        assert_opcode!(DEC(Reg(B)), 1, 0x05);
        assert_opcode!(DEC(Reg(C)), 1, 0x0D);
        assert_opcode!(DEC(Reg(D)), 1, 0x15);
        assert_opcode!(DEC(Reg(E)), 1, 0x1D);
        assert_opcode!(DEC(Reg(H)), 1, 0x25);
        assert_opcode!(DEC(Reg(L)), 1, 0x2D);

        assert_opcode!(DEC(RegIndirect(HL)), 1, 0x35);
    }

    #[test]
    fn add() {
        assert_opcode!(ADD8(Reg(A), Reg(A)), 1, 0x87);
        assert_opcode!(ADD8(Reg(A), Reg(B)), 1, 0x80);
        assert_opcode!(ADD8(Reg(A), Reg(C)), 1, 0x81);
        assert_opcode!(ADD8(Reg(A), Reg(D)), 1, 0x82);
        assert_opcode!(ADD8(Reg(A), Reg(E)), 1, 0x83);
        assert_opcode!(ADD8(Reg(A), Reg(H)), 1, 0x84);
        assert_opcode!(ADD8(Reg(A), Reg(L)), 1, 0x85);

        assert_opcode!(ADD8(Reg(A), RegIndirect(HL)), 1, 0x86);
        assert_opcode!(ADD8(Reg(A), Immediate(0x75)), 2, 0xC6, 0x75);
    }

    #[test]
    fn adc() {
        assert_opcode!(ADC(Reg(A), Reg(A)), 1, 0x8F);
        assert_opcode!(ADC(Reg(A), Reg(B)), 1, 0x88);
        assert_opcode!(ADC(Reg(A), Reg(C)), 1, 0x89);
        assert_opcode!(ADC(Reg(A), Reg(D)), 1, 0x8A);
        assert_opcode!(ADC(Reg(A), Reg(E)), 1, 0x8B);
        assert_opcode!(ADC(Reg(A), Reg(H)), 1, 0x8C);
        assert_opcode!(ADC(Reg(A), Reg(L)), 1, 0x8D);

        assert_opcode!(ADC(Reg(A), RegIndirect(HL)), 1, 0x8E);
        assert_opcode!(ADC(Reg(A), Immediate(0xF5)), 2, 0xCE, 0xF5);
    }

    #[test]
    fn sub() {
        assert_opcode!(SUB8(Reg(A), Reg(A)), 1, 0x97);
        assert_opcode!(SUB8(Reg(A), Reg(B)), 1, 0x90);
        assert_opcode!(SUB8(Reg(A), Reg(C)), 1, 0x91);
        assert_opcode!(SUB8(Reg(A), Reg(D)), 1, 0x92);
        assert_opcode!(SUB8(Reg(A), Reg(E)), 1, 0x93);
        assert_opcode!(SUB8(Reg(A), Reg(H)), 1, 0x94);
        assert_opcode!(SUB8(Reg(A), Reg(L)), 1, 0x95);

        assert_opcode!(SUB8(Reg(A), RegIndirect(HL)), 1, 0x96);
        assert_opcode!(SUB8(Reg(A), Immediate(0x75)), 2, 0xD6, 0x75);
    }

    #[test]
    fn sbc() {
        assert_opcode!(SBC(Reg(A), Reg(A)), 1, 0x9F);
        assert_opcode!(SBC(Reg(A), Reg(B)), 1, 0x98);
        assert_opcode!(SBC(Reg(A), Reg(C)), 1, 0x99);
        assert_opcode!(SBC(Reg(A), Reg(D)), 1, 0x9A);
        assert_opcode!(SBC(Reg(A), Reg(E)), 1, 0x9B);
        assert_opcode!(SBC(Reg(A), Reg(H)), 1, 0x9C);
        assert_opcode!(SBC(Reg(A), Reg(L)), 1, 0x9D);

        assert_opcode!(SBC(Reg(A), RegIndirect(HL)), 1, 0x9E);
        assert_opcode!(SBC(Reg(A), Immediate(0xF5)), 2, 0xDE, 0xF5);
    }

    #[test]
    #[allow(clippy::cyclomatic_complexity)]
    fn ld_rr() {
        assert_opcode!(LD8(Reg(A), Reg(A)), 1, 0x7F);
        assert_opcode!(LD8(Reg(A), Reg(B)), 1, 0x78);
        assert_opcode!(LD8(Reg(A), Reg(C)), 1, 0x79);
        assert_opcode!(LD8(Reg(A), Reg(D)), 1, 0x7A);
        assert_opcode!(LD8(Reg(A), Reg(E)), 1, 0x7B);
        assert_opcode!(LD8(Reg(A), Reg(H)), 1, 0x7C);
        assert_opcode!(LD8(Reg(A), Reg(L)), 1, 0x7D);
        assert_opcode!(LD8(Reg(A), RegIndirect(HL)), 1, 0x7E);

        assert_opcode!(LD8(Reg(B), Reg(A)), 1, 0x47);
        assert_opcode!(LD8(Reg(B), Reg(B)), 1, 0x40);
        assert_opcode!(LD8(Reg(B), Reg(C)), 1, 0x41);
        assert_opcode!(LD8(Reg(B), Reg(D)), 1, 0x42);
        assert_opcode!(LD8(Reg(B), Reg(E)), 1, 0x43);
        assert_opcode!(LD8(Reg(B), Reg(H)), 1, 0x44);
        assert_opcode!(LD8(Reg(B), Reg(L)), 1, 0x45);
        assert_opcode!(LD8(Reg(B), RegIndirect(HL)), 1, 0x46);

        assert_opcode!(LD8(Reg(C), Reg(A)), 1, 0x4F);
        assert_opcode!(LD8(Reg(C), Reg(B)), 1, 0x48);
        assert_opcode!(LD8(Reg(C), Reg(C)), 1, 0x49);
        assert_opcode!(LD8(Reg(C), Reg(D)), 1, 0x4A);
        assert_opcode!(LD8(Reg(C), Reg(E)), 1, 0x4B);
        assert_opcode!(LD8(Reg(C), Reg(H)), 1, 0x4C);
        assert_opcode!(LD8(Reg(C), Reg(L)), 1, 0x4D);
        assert_opcode!(LD8(Reg(C), RegIndirect(HL)), 1, 0x4E);

        assert_opcode!(LD8(Reg(D), Reg(A)), 1, 0x57);
        assert_opcode!(LD8(Reg(D), Reg(B)), 1, 0x50);
        assert_opcode!(LD8(Reg(D), Reg(C)), 1, 0x51);
        assert_opcode!(LD8(Reg(D), Reg(D)), 1, 0x52);
        assert_opcode!(LD8(Reg(D), Reg(E)), 1, 0x53);
        assert_opcode!(LD8(Reg(D), Reg(H)), 1, 0x54);
        assert_opcode!(LD8(Reg(D), Reg(L)), 1, 0x55);
        assert_opcode!(LD8(Reg(D), RegIndirect(HL)), 1, 0x56);

        assert_opcode!(LD8(Reg(E), Reg(A)), 1, 0x5F);
        assert_opcode!(LD8(Reg(E), Reg(B)), 1, 0x58);
        assert_opcode!(LD8(Reg(E), Reg(C)), 1, 0x59);
        assert_opcode!(LD8(Reg(E), Reg(D)), 1, 0x5A);
        assert_opcode!(LD8(Reg(E), Reg(E)), 1, 0x5B);
        assert_opcode!(LD8(Reg(E), Reg(H)), 1, 0x5C);
        assert_opcode!(LD8(Reg(E), Reg(L)), 1, 0x5D);
        assert_opcode!(LD8(Reg(E), RegIndirect(HL)), 1, 0x5E);

        assert_opcode!(LD8(Reg(H), Reg(A)), 1, 0x67);
        assert_opcode!(LD8(Reg(H), Reg(B)), 1, 0x60);
        assert_opcode!(LD8(Reg(H), Reg(C)), 1, 0x61);
        assert_opcode!(LD8(Reg(H), Reg(D)), 1, 0x62);
        assert_opcode!(LD8(Reg(H), Reg(E)), 1, 0x63);
        assert_opcode!(LD8(Reg(H), Reg(H)), 1, 0x64);
        assert_opcode!(LD8(Reg(H), Reg(L)), 1, 0x65);
        assert_opcode!(LD8(Reg(H), RegIndirect(HL)), 1, 0x66);

        assert_opcode!(LD8(Reg(L), Reg(A)), 1, 0x6F);
        assert_opcode!(LD8(Reg(L), Reg(B)), 1, 0x68);
        assert_opcode!(LD8(Reg(L), Reg(C)), 1, 0x69);
        assert_opcode!(LD8(Reg(L), Reg(D)), 1, 0x6A);
        assert_opcode!(LD8(Reg(L), Reg(E)), 1, 0x6B);
        assert_opcode!(LD8(Reg(L), Reg(H)), 1, 0x6C);
        assert_opcode!(LD8(Reg(L), Reg(L)), 1, 0x6D);
        assert_opcode!(LD8(Reg(L), RegIndirect(HL)), 1, 0x6E);

        assert_opcode!(LD8(RegIndirect(HL), Reg(A)), 1, 0x77);
        assert_opcode!(LD8(RegIndirect(HL), Reg(B)), 1, 0x70);
        assert_opcode!(LD8(RegIndirect(HL), Reg(C)), 1, 0x71);
        assert_opcode!(LD8(RegIndirect(HL), Reg(D)), 1, 0x72);
        assert_opcode!(LD8(RegIndirect(HL), Reg(E)), 1, 0x73);
        assert_opcode!(LD8(RegIndirect(HL), Reg(H)), 1, 0x74);
        assert_opcode!(LD8(RegIndirect(HL), Reg(L)), 1, 0x75);
        // ld (hl), (hl) is HALT
    }

    #[test]
    fn ld_immediate() {
        assert_opcode!(LD8(Reg(A), Immediate(0x25)), 2, 0x3E, 0x25);
        assert_opcode!(LD8(Reg(B), Immediate(0x99)), 2, 0x06, 0x99);
        assert_opcode!(LD8(Reg(C), Immediate(0xAA)), 2, 0x0E, 0xAA);
        assert_opcode!(LD8(Reg(D), Immediate(0xCD)), 2, 0x16, 0xCD);
        assert_opcode!(LD8(Reg(E), Immediate(0xDA)), 2, 0x1E, 0xDA);
        assert_opcode!(LD8(Reg(H), Immediate(0xFA)), 2, 0x26, 0xFA);
        assert_opcode!(LD8(Reg(L), Immediate(0xCA)), 2, 0x2E, 0xCA);
        assert_opcode!(LD8(RegIndirect(HL), Immediate(0xC7)), 2, 0x36, 0xC7);
    }

    #[test]
    fn ld_indirect() {
        // HL is in ld_rr above, because "register" 110 is (HL)
        assert_opcode!(LD8(Reg(A), RegIndirect(BC)), 1, 0x0A);
        assert_opcode!(LD8(Reg(A), RegIndirect(DE)), 1, 0x1A);
        assert_opcode!(LD8(Reg(A), ImmediateIndirect(0x0F32)), 3, 0x3A, 0x32, 0x0F);

        assert_opcode!(LD8(RegIndirect(BC), Reg(A)), 1, 0x02);
        assert_opcode!(LD8(RegIndirect(DE), Reg(A)), 1, 0x12);
        assert_opcode!(LD8(ImmediateIndirect(0x01AA), Reg(A)), 3, 0x32, 0xAA, 0x01);
    }
}
