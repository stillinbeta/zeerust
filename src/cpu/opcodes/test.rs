use crate::cpu::opcodes::opcode;
#[allow(unused_imports)]
use crate::ops::Op;
use crate::ops::{
    JumpConditional::*,
    Location16::{Immediate as I16, Reg as R16},
    Location8::*,
    Op::*,
    Reg16::*,
    Reg8::*,
};

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
        let (opc, bytes) = $crate::cpu::opcodes::opcode(op4!($o1));
        assert_eq!($opc, opc, "Opcode {:?} ({:02x})", $opc, $o1);
        assert_eq!($bytes, bytes, "Opcode {:?} ({:02x})", $opc, $o1);
    };
    ($opc : expr, $bytes : expr, $o1 : expr, $o2 : expr) => {
        let (opc, bytes) = $crate::cpu::opcodes::opcode(op4!($o1, $o2));
        assert_eq!($opc, opc, "Opcode {:?} ({:02x} {:02x})", $opc, $o1, $o2);
        assert_eq!($bytes, bytes, "Opcode {:?} ({:02x} {:02x})", $opc, $o1, $o2);
    };
    ($opc : expr, $bytes : expr, $o1 : expr, $o2 : expr, $o3 : expr) => {
        let (opc, bytes) = $crate::cpu::opcodes::opcode(op4!($o1, $o2, $o3));
        assert_eq!(
            $opc, opc,
            "Opcode {:?} ({:02x} {:02x} {:02x})",
            $opc, $o1, $o2, $o3
        );
        assert_eq!(
            $bytes, bytes,
            "Opcode {:?} ({:02x} {:02x} {:02x})",
            $opc, $o1, $o2, $o3
        );
    };
    ($opc : expr, $bytes : expr, $o1 : expr, $o2 : expr, $o3 : expr, $o4 : expr) => {
        let (opc, bytes) = $crate::cpu::opcodes::opcode([$o1, $o2, $o3, $o4]);
        assert_eq!(
            $opc, opc,
            "Opcode {:?} ({:02x} {:02x} {:02x} {:02x})",
            $opc, $o1, $o2, $o3, $o4,
        );
        assert_eq!(
            $bytes, bytes,
            "Opcode {:?} ({:02x} {:02x} {:02x} {:02x})",
            $opc, $o1, $o2, $o3, $o4,
        );
    };
}

#[test]
fn nop() {
    assert_opcode!(NOP, 1, 0x00);
}

#[test]
fn halt() {
    assert_opcode!(HALT, 1, 0x76);
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
fn ld_immediate_16() {
    assert_opcode!(LD16(R16(BC), I16(0xABBA)), 3, 0x01, 0xBA, 0xAB);
    assert_opcode!(LD16(R16(DE), I16(0xACC0)), 3, 0x11, 0xC0, 0xAC);
    assert_opcode!(LD16(R16(HL), I16(0x1337)), 3, 0x21, 0x37, 0x13);
    assert_opcode!(LD16(R16(SP), I16(0x4004)), 3, 0x31, 0x04, 0x40);

    assert_opcode!(LD16(R16(IX), I16(0x45A2)), 4, 0xDD, 0x21, 0xA2, 0x45);
    assert_opcode!(LD16(R16(IY), I16(0x45A2)), 4, 0xFD, 0x21, 0xA2, 0x45);
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

#[test]
fn cpl() {
    assert_opcode!(CPL, 1, 0x2f);
}

#[test]
fn neg() {
    assert_opcode!(NEG, 2, 0xED, 0x44);
}

#[test]
fn ccf() {
    assert_opcode!(CCF, 1, 0x3F);
}

#[test]
fn scf() {
    assert_opcode!(SCF, 1, 0x37);
}

#[test]
fn and() {
    assert_opcode!(AND(Reg(A)), 1, 0xA7);
    assert_opcode!(AND(Reg(B)), 1, 0xA0);
    assert_opcode!(AND(Reg(C)), 1, 0xA1);
    assert_opcode!(AND(Reg(D)), 1, 0xA2);
    assert_opcode!(AND(Reg(E)), 1, 0xA3);
    assert_opcode!(AND(Reg(H)), 1, 0xA4);
    assert_opcode!(AND(Reg(L)), 1, 0xA5);
    assert_opcode!(AND(RegIndirect(HL)), 1, 0xA6);

    assert_opcode!(AND(Immediate(0xAB)), 2, 0xE6, 0xAB);
}

#[test]
fn or() {
    assert_opcode!(OR(Reg(A)), 1, 0xB7);
    assert_opcode!(OR(Reg(B)), 1, 0xB0);
    assert_opcode!(OR(Reg(C)), 1, 0xB1);
    assert_opcode!(OR(Reg(D)), 1, 0xB2);
    assert_opcode!(OR(Reg(E)), 1, 0xB3);
    assert_opcode!(OR(Reg(H)), 1, 0xB4);
    assert_opcode!(OR(Reg(L)), 1, 0xB5);
    assert_opcode!(OR(RegIndirect(HL)), 1, 0xB6);
    assert_opcode!(OR(Immediate(0xBA)), 2, 0xF6, 0xBA);
}

#[test]
fn xor() {
    assert_opcode!(XOR(Reg(A)), 1, 0xAF);
    assert_opcode!(XOR(Reg(B)), 1, 0xA8);
    assert_opcode!(XOR(Reg(C)), 1, 0xA9);
    assert_opcode!(XOR(Reg(D)), 1, 0xAA);
    assert_opcode!(XOR(Reg(E)), 1, 0xAB);
    assert_opcode!(XOR(Reg(H)), 1, 0xAC);
    assert_opcode!(XOR(Reg(L)), 1, 0xAD);
    assert_opcode!(XOR(RegIndirect(HL)), 1, 0xAE);
    assert_opcode!(XOR(Immediate(0xCA)), 2, 0xEE, 0xCA);
}

#[test]
fn cp() {
    // Name collision on bare CP
    assert_opcode!(Op::CP(Reg(A)), 1, 0xBF);
    assert_opcode!(Op::CP(Reg(B)), 1, 0xB8);
    assert_opcode!(Op::CP(Reg(C)), 1, 0xB9);
    assert_opcode!(Op::CP(Reg(D)), 1, 0xBA);
    assert_opcode!(Op::CP(Reg(E)), 1, 0xBB);
    assert_opcode!(Op::CP(Reg(H)), 1, 0xBC);
    assert_opcode!(Op::CP(Reg(L)), 1, 0xBD);
    assert_opcode!(Op::CP(RegIndirect(HL)), 1, 0xBE);
    assert_opcode!(Op::CP(Immediate(0xAC)), 2, 0xFE, 0xAC);
}

#[cfg(test)]
mod bits {
    use super::*;

    #[test]
    #[allow(clippy::cyclomatic_complexity)]
    fn get() {
        assert_opcode!(BIT(0, Reg(A)), 2, 0xCB, 0x47);
        assert_opcode!(BIT(0, Reg(B)), 2, 0xCB, 0x40);
        assert_opcode!(BIT(0, Reg(C)), 2, 0xCB, 0x41);
        assert_opcode!(BIT(0, Reg(D)), 2, 0xCB, 0x42);
        assert_opcode!(BIT(0, Reg(E)), 2, 0xCB, 0x43);
        assert_opcode!(BIT(0, Reg(H)), 2, 0xCB, 0x44);
        assert_opcode!(BIT(0, Reg(L)), 2, 0xCB, 0x45);
        assert_opcode!(BIT(0, RegIndirect(HL)), 2, 0xCB, 0x46);

        assert_opcode!(BIT(1, Reg(A)), 2, 0xCB, 0x4F);
        assert_opcode!(BIT(1, Reg(B)), 2, 0xCB, 0x48);
        assert_opcode!(BIT(1, Reg(C)), 2, 0xCB, 0x49);
        assert_opcode!(BIT(1, Reg(D)), 2, 0xCB, 0x4A);
        assert_opcode!(BIT(1, Reg(E)), 2, 0xCB, 0x4B);
        assert_opcode!(BIT(1, Reg(H)), 2, 0xCB, 0x4C);
        assert_opcode!(BIT(1, Reg(L)), 2, 0xCB, 0x4D);
        assert_opcode!(BIT(1, RegIndirect(HL)), 2, 0xCB, 0x4E);

        assert_opcode!(BIT(2, Reg(A)), 2, 0xCB, 0x57);
        assert_opcode!(BIT(2, Reg(B)), 2, 0xCB, 0x50);
        assert_opcode!(BIT(2, Reg(C)), 2, 0xCB, 0x51);
        assert_opcode!(BIT(2, Reg(D)), 2, 0xCB, 0x52);
        assert_opcode!(BIT(2, Reg(E)), 2, 0xCB, 0x53);
        assert_opcode!(BIT(2, Reg(H)), 2, 0xCB, 0x54);
        assert_opcode!(BIT(2, Reg(L)), 2, 0xCB, 0x55);
        assert_opcode!(BIT(2, RegIndirect(HL)), 2, 0xCB, 0x56);

        assert_opcode!(BIT(3, Reg(A)), 2, 0xCB, 0x5F);
        assert_opcode!(BIT(3, Reg(B)), 2, 0xCB, 0x58);
        assert_opcode!(BIT(3, Reg(C)), 2, 0xCB, 0x59);
        assert_opcode!(BIT(3, Reg(D)), 2, 0xCB, 0x5A);
        assert_opcode!(BIT(3, Reg(E)), 2, 0xCB, 0x5B);
        assert_opcode!(BIT(3, Reg(H)), 2, 0xCB, 0x5C);
        assert_opcode!(BIT(3, Reg(L)), 2, 0xCB, 0x5D);
        assert_opcode!(BIT(3, RegIndirect(HL)), 2, 0xCB, 0x5E);

        assert_opcode!(BIT(4, Reg(A)), 2, 0xCB, 0x67);
        assert_opcode!(BIT(4, Reg(B)), 2, 0xCB, 0x60);
        assert_opcode!(BIT(4, Reg(C)), 2, 0xCB, 0x61);
        assert_opcode!(BIT(4, Reg(D)), 2, 0xCB, 0x62);
        assert_opcode!(BIT(4, Reg(E)), 2, 0xCB, 0x63);
        assert_opcode!(BIT(4, Reg(H)), 2, 0xCB, 0x64);
        assert_opcode!(BIT(4, Reg(L)), 2, 0xCB, 0x65);
        assert_opcode!(BIT(4, RegIndirect(HL)), 2, 0xCB, 0x66);

        assert_opcode!(BIT(5, Reg(A)), 2, 0xCB, 0x6F);
        assert_opcode!(BIT(5, Reg(B)), 2, 0xCB, 0x68);
        assert_opcode!(BIT(5, Reg(C)), 2, 0xCB, 0x69);
        assert_opcode!(BIT(5, Reg(D)), 2, 0xCB, 0x6A);
        assert_opcode!(BIT(5, Reg(E)), 2, 0xCB, 0x6B);
        assert_opcode!(BIT(5, Reg(H)), 2, 0xCB, 0x6C);
        assert_opcode!(BIT(5, Reg(L)), 2, 0xCB, 0x6D);
        assert_opcode!(BIT(5, RegIndirect(HL)), 2, 0xCB, 0x6E);

        assert_opcode!(BIT(6, Reg(A)), 2, 0xCB, 0x77);
        assert_opcode!(BIT(6, Reg(B)), 2, 0xCB, 0x70);
        assert_opcode!(BIT(6, Reg(C)), 2, 0xCB, 0x71);
        assert_opcode!(BIT(6, Reg(D)), 2, 0xCB, 0x72);
        assert_opcode!(BIT(6, Reg(E)), 2, 0xCB, 0x73);
        assert_opcode!(BIT(6, Reg(H)), 2, 0xCB, 0x74);
        assert_opcode!(BIT(6, Reg(L)), 2, 0xCB, 0x75);
        assert_opcode!(BIT(6, RegIndirect(HL)), 2, 0xCB, 0x76);

        assert_opcode!(BIT(7, Reg(A)), 2, 0xCB, 0x7F);
        assert_opcode!(BIT(7, Reg(B)), 2, 0xCB, 0x78);
        assert_opcode!(BIT(7, Reg(C)), 2, 0xCB, 0x79);
        assert_opcode!(BIT(7, Reg(D)), 2, 0xCB, 0x7A);
        assert_opcode!(BIT(7, Reg(E)), 2, 0xCB, 0x7B);
        assert_opcode!(BIT(7, Reg(H)), 2, 0xCB, 0x7C);
        assert_opcode!(BIT(7, Reg(L)), 2, 0xCB, 0x7D);
        assert_opcode!(BIT(7, RegIndirect(HL)), 2, 0xCB, 0x7E);
    }

    #[test]
    #[allow(clippy::cyclomatic_complexity)]
    fn reset() {
        assert_opcode!(RES(0, Reg(A)), 2, 0xCB, 0x87);
        assert_opcode!(RES(0, Reg(B)), 2, 0xCB, 0x80);
        assert_opcode!(RES(0, Reg(C)), 2, 0xCB, 0x81);
        assert_opcode!(RES(0, Reg(D)), 2, 0xCB, 0x82);
        assert_opcode!(RES(0, Reg(E)), 2, 0xCB, 0x83);
        assert_opcode!(RES(0, Reg(H)), 2, 0xCB, 0x84);
        assert_opcode!(RES(0, Reg(L)), 2, 0xCB, 0x85);
        assert_opcode!(RES(0, RegIndirect(HL)), 2, 0xCB, 0x86);

        assert_opcode!(RES(1, Reg(A)), 2, 0xCB, 0x8F);
        assert_opcode!(RES(1, Reg(B)), 2, 0xCB, 0x88);
        assert_opcode!(RES(1, Reg(C)), 2, 0xCB, 0x89);
        assert_opcode!(RES(1, Reg(D)), 2, 0xCB, 0x8A);
        assert_opcode!(RES(1, Reg(E)), 2, 0xCB, 0x8B);
        assert_opcode!(RES(1, Reg(H)), 2, 0xCB, 0x8C);
        assert_opcode!(RES(1, Reg(L)), 2, 0xCB, 0x8D);
        assert_opcode!(RES(1, RegIndirect(HL)), 2, 0xCB, 0x8E);

        assert_opcode!(RES(2, Reg(A)), 2, 0xCB, 0x97);
        assert_opcode!(RES(2, Reg(B)), 2, 0xCB, 0x90);
        assert_opcode!(RES(2, Reg(C)), 2, 0xCB, 0x91);
        assert_opcode!(RES(2, Reg(D)), 2, 0xCB, 0x92);
        assert_opcode!(RES(2, Reg(E)), 2, 0xCB, 0x93);
        assert_opcode!(RES(2, Reg(H)), 2, 0xCB, 0x94);
        assert_opcode!(RES(2, Reg(L)), 2, 0xCB, 0x95);
        assert_opcode!(RES(2, RegIndirect(HL)), 2, 0xCB, 0x96);

        assert_opcode!(RES(3, Reg(A)), 2, 0xCB, 0x9F);
        assert_opcode!(RES(3, Reg(B)), 2, 0xCB, 0x98);
        assert_opcode!(RES(3, Reg(C)), 2, 0xCB, 0x99);
        assert_opcode!(RES(3, Reg(D)), 2, 0xCB, 0x9A);
        assert_opcode!(RES(3, Reg(E)), 2, 0xCB, 0x9B);
        assert_opcode!(RES(3, Reg(H)), 2, 0xCB, 0x9C);
        assert_opcode!(RES(3, Reg(L)), 2, 0xCB, 0x9D);
        assert_opcode!(RES(3, RegIndirect(HL)), 2, 0xCB, 0x9E);

        assert_opcode!(RES(4, Reg(A)), 2, 0xCB, 0xA7);
        assert_opcode!(RES(4, Reg(B)), 2, 0xCB, 0xA0);
        assert_opcode!(RES(4, Reg(C)), 2, 0xCB, 0xA1);
        assert_opcode!(RES(4, Reg(D)), 2, 0xCB, 0xA2);
        assert_opcode!(RES(4, Reg(E)), 2, 0xCB, 0xA3);
        assert_opcode!(RES(4, Reg(H)), 2, 0xCB, 0xA4);
        assert_opcode!(RES(4, Reg(L)), 2, 0xCB, 0xA5);
        assert_opcode!(RES(4, RegIndirect(HL)), 2, 0xCB, 0xA6);

        assert_opcode!(RES(5, Reg(A)), 2, 0xCB, 0xAF);
        assert_opcode!(RES(5, Reg(B)), 2, 0xCB, 0xA8);
        assert_opcode!(RES(5, Reg(C)), 2, 0xCB, 0xA9);
        assert_opcode!(RES(5, Reg(D)), 2, 0xCB, 0xAA);
        assert_opcode!(RES(5, Reg(E)), 2, 0xCB, 0xAB);
        assert_opcode!(RES(5, Reg(H)), 2, 0xCB, 0xAC);
        assert_opcode!(RES(5, Reg(L)), 2, 0xCB, 0xAD);
        assert_opcode!(RES(5, RegIndirect(HL)), 2, 0xCB, 0xAE);

        assert_opcode!(RES(6, Reg(A)), 2, 0xCB, 0xB7);
        assert_opcode!(RES(6, Reg(B)), 2, 0xCB, 0xB0);
        assert_opcode!(RES(6, Reg(C)), 2, 0xCB, 0xB1);
        assert_opcode!(RES(6, Reg(D)), 2, 0xCB, 0xB2);
        assert_opcode!(RES(6, Reg(E)), 2, 0xCB, 0xB3);
        assert_opcode!(RES(6, Reg(H)), 2, 0xCB, 0xB4);
        assert_opcode!(RES(6, Reg(L)), 2, 0xCB, 0xB5);
        assert_opcode!(RES(6, RegIndirect(HL)), 2, 0xCB, 0xB6);

        assert_opcode!(RES(7, Reg(A)), 2, 0xCB, 0xBF);
        assert_opcode!(RES(7, Reg(B)), 2, 0xCB, 0xB8);
        assert_opcode!(RES(7, Reg(C)), 2, 0xCB, 0xB9);
        assert_opcode!(RES(7, Reg(D)), 2, 0xCB, 0xBA);
        assert_opcode!(RES(7, Reg(E)), 2, 0xCB, 0xBB);
        assert_opcode!(RES(7, Reg(H)), 2, 0xCB, 0xBC);
        assert_opcode!(RES(7, Reg(L)), 2, 0xCB, 0xBD);
        assert_opcode!(RES(7, RegIndirect(HL)), 2, 0xCB, 0xBE);
    }

    #[test]
    #[allow(clippy::cyclomatic_complexity)]
    fn set() {
        assert_opcode!(SET(0, Reg(A)), 2, 0xCB, 0xC7);
        assert_opcode!(SET(0, Reg(B)), 2, 0xCB, 0xC0);
        assert_opcode!(SET(0, Reg(C)), 2, 0xCB, 0xC1);
        assert_opcode!(SET(0, Reg(D)), 2, 0xCB, 0xC2);
        assert_opcode!(SET(0, Reg(E)), 2, 0xCB, 0xC3);
        assert_opcode!(SET(0, Reg(H)), 2, 0xCB, 0xC4);
        assert_opcode!(SET(0, Reg(L)), 2, 0xCB, 0xC5);
        assert_opcode!(SET(0, RegIndirect(HL)), 2, 0xCB, 0xC6);

        assert_opcode!(SET(1, Reg(A)), 2, 0xCB, 0xCF);
        assert_opcode!(SET(1, Reg(B)), 2, 0xCB, 0xC8);
        assert_opcode!(SET(1, Reg(C)), 2, 0xCB, 0xC9);
        assert_opcode!(SET(1, Reg(D)), 2, 0xCB, 0xCA);
        assert_opcode!(SET(1, Reg(E)), 2, 0xCB, 0xCB);
        assert_opcode!(SET(1, Reg(H)), 2, 0xCB, 0xCC);
        assert_opcode!(SET(1, Reg(L)), 2, 0xCB, 0xCD);
        assert_opcode!(SET(1, RegIndirect(HL)), 2, 0xCB, 0xCE);

        assert_opcode!(SET(2, Reg(A)), 2, 0xCB, 0xD7);
        assert_opcode!(SET(2, Reg(B)), 2, 0xCB, 0xD0);
        assert_opcode!(SET(2, Reg(C)), 2, 0xCB, 0xD1);
        assert_opcode!(SET(2, Reg(D)), 2, 0xCB, 0xD2);
        assert_opcode!(SET(2, Reg(E)), 2, 0xCB, 0xD3);
        assert_opcode!(SET(2, Reg(H)), 2, 0xCB, 0xD4);
        assert_opcode!(SET(2, Reg(L)), 2, 0xCB, 0xD5);
        assert_opcode!(SET(2, RegIndirect(HL)), 2, 0xCB, 0xD6);

        assert_opcode!(SET(3, Reg(A)), 2, 0xCB, 0xDF);
        assert_opcode!(SET(3, Reg(B)), 2, 0xCB, 0xD8);
        assert_opcode!(SET(3, Reg(C)), 2, 0xCB, 0xD9);
        assert_opcode!(SET(3, Reg(D)), 2, 0xCB, 0xDA);
        assert_opcode!(SET(3, Reg(E)), 2, 0xCB, 0xDB);
        assert_opcode!(SET(3, Reg(H)), 2, 0xCB, 0xDC);
        assert_opcode!(SET(3, Reg(L)), 2, 0xCB, 0xDD);
        assert_opcode!(SET(3, RegIndirect(HL)), 2, 0xCB, 0xDE);

        assert_opcode!(SET(4, Reg(A)), 2, 0xCB, 0xE7);
        assert_opcode!(SET(4, Reg(B)), 2, 0xCB, 0xE0);
        assert_opcode!(SET(4, Reg(C)), 2, 0xCB, 0xE1);
        assert_opcode!(SET(4, Reg(D)), 2, 0xCB, 0xE2);
        assert_opcode!(SET(4, Reg(E)), 2, 0xCB, 0xE3);
        assert_opcode!(SET(4, Reg(H)), 2, 0xCB, 0xE4);
        assert_opcode!(SET(4, Reg(L)), 2, 0xCB, 0xE5);
        assert_opcode!(SET(4, RegIndirect(HL)), 2, 0xCB, 0xE6);

        assert_opcode!(SET(5, Reg(A)), 2, 0xCB, 0xEF);
        assert_opcode!(SET(5, Reg(B)), 2, 0xCB, 0xE8);
        assert_opcode!(SET(5, Reg(C)), 2, 0xCB, 0xE9);
        assert_opcode!(SET(5, Reg(D)), 2, 0xCB, 0xEA);
        assert_opcode!(SET(5, Reg(E)), 2, 0xCB, 0xEB);
        assert_opcode!(SET(5, Reg(H)), 2, 0xCB, 0xEC);
        assert_opcode!(SET(5, Reg(L)), 2, 0xCB, 0xED);
        assert_opcode!(SET(5, RegIndirect(HL)), 2, 0xCB, 0xEE);

        assert_opcode!(SET(6, Reg(A)), 2, 0xCB, 0xF7);
        assert_opcode!(SET(6, Reg(B)), 2, 0xCB, 0xF0);
        assert_opcode!(SET(6, Reg(C)), 2, 0xCB, 0xF1);
        assert_opcode!(SET(6, Reg(D)), 2, 0xCB, 0xF2);
        assert_opcode!(SET(6, Reg(E)), 2, 0xCB, 0xF3);
        assert_opcode!(SET(6, Reg(H)), 2, 0xCB, 0xF4);
        assert_opcode!(SET(6, Reg(L)), 2, 0xCB, 0xF5);
        assert_opcode!(SET(6, RegIndirect(HL)), 2, 0xCB, 0xF6);

        assert_opcode!(SET(7, Reg(A)), 2, 0xCB, 0xFF);
        assert_opcode!(SET(7, Reg(B)), 2, 0xCB, 0xF8);
        assert_opcode!(SET(7, Reg(C)), 2, 0xCB, 0xF9);
        assert_opcode!(SET(7, Reg(D)), 2, 0xCB, 0xFA);
        assert_opcode!(SET(7, Reg(E)), 2, 0xCB, 0xFB);
        assert_opcode!(SET(7, Reg(H)), 2, 0xCB, 0xFC);
        assert_opcode!(SET(7, Reg(L)), 2, 0xCB, 0xFD);
        assert_opcode!(SET(7, RegIndirect(HL)), 2, 0xCB, 0xFE);
    }

    #[test]
    fn rlc() {
        assert_opcode!(RLC(Reg(A)), 2, 0xCB, 0x07);
        assert_opcode!(RLC(Reg(B)), 2, 0xCB, 0x00);
        assert_opcode!(RLC(Reg(C)), 2, 0xCB, 0x01);
        assert_opcode!(RLC(Reg(D)), 2, 0xCB, 0x02);
        assert_opcode!(RLC(Reg(E)), 2, 0xCB, 0x03);
        assert_opcode!(RLC(Reg(H)), 2, 0xCB, 0x04);
        assert_opcode!(RLC(Reg(L)), 2, 0xCB, 0x05);
        assert_opcode!(RLC(RegIndirect(HL)), 2, 0xCB, 0x06);
    }

    #[test]
    fn rrc() {
        assert_opcode!(RRC(Reg(A)), 2, 0xCB, 0x0F);
        assert_opcode!(RRC(Reg(B)), 2, 0xCB, 0x08);
        assert_opcode!(RRC(Reg(C)), 2, 0xCB, 0x09);
        assert_opcode!(RRC(Reg(D)), 2, 0xCB, 0x0A);
        assert_opcode!(RRC(Reg(E)), 2, 0xCB, 0x0B);
        assert_opcode!(RRC(Reg(H)), 2, 0xCB, 0x0C);
        assert_opcode!(RRC(Reg(L)), 2, 0xCB, 0x0D);
        assert_opcode!(RRC(RegIndirect(HL)), 2, 0xCB, 0x0E);
    }

    #[test]
    fn rl() {
        assert_opcode!(RL(Reg(A)), 2, 0xCB, 0x17);
        assert_opcode!(RL(Reg(B)), 2, 0xCB, 0x10);
        assert_opcode!(RL(Reg(C)), 2, 0xCB, 0x11);
        assert_opcode!(RL(Reg(D)), 2, 0xCB, 0x12);
        assert_opcode!(RL(Reg(E)), 2, 0xCB, 0x13);
        assert_opcode!(RL(Reg(H)), 2, 0xCB, 0x14);
        assert_opcode!(RL(Reg(L)), 2, 0xCB, 0x15);
        assert_opcode!(RL(RegIndirect(HL)), 2, 0xCB, 0x16);
    }

    #[test]
    fn rr() {
        assert_opcode!(RR(Reg(A)), 2, 0xCB, 0x1F);
        assert_opcode!(RR(Reg(B)), 2, 0xCB, 0x18);
        assert_opcode!(RR(Reg(C)), 2, 0xCB, 0x19);
        assert_opcode!(RR(Reg(D)), 2, 0xCB, 0x1A);
        assert_opcode!(RR(Reg(E)), 2, 0xCB, 0x1B);
        assert_opcode!(RR(Reg(H)), 2, 0xCB, 0x1C);
        assert_opcode!(RR(Reg(L)), 2, 0xCB, 0x1D);
        assert_opcode!(RR(RegIndirect(HL)), 2, 0xCB, 0x1E);
    }

    #[test]
    fn sla() {
        assert_opcode!(SLA(Reg(A)), 2, 0xCB, 0x27);
        assert_opcode!(SLA(Reg(B)), 2, 0xCB, 0x20);
        assert_opcode!(SLA(Reg(C)), 2, 0xCB, 0x21);
        assert_opcode!(SLA(Reg(D)), 2, 0xCB, 0x22);
        assert_opcode!(SLA(Reg(E)), 2, 0xCB, 0x23);
        assert_opcode!(SLA(Reg(H)), 2, 0xCB, 0x24);
        assert_opcode!(SLA(Reg(L)), 2, 0xCB, 0x25);
        assert_opcode!(SLA(RegIndirect(HL)), 2, 0xCB, 0x26);
    }

    #[test]
    fn sra() {
        assert_opcode!(SRA(Reg(A)), 2, 0xCB, 0x2F);
        assert_opcode!(SRA(Reg(B)), 2, 0xCB, 0x28);
        assert_opcode!(SRA(Reg(C)), 2, 0xCB, 0x29);
        assert_opcode!(SRA(Reg(D)), 2, 0xCB, 0x2A);
        assert_opcode!(SRA(Reg(E)), 2, 0xCB, 0x2B);
        assert_opcode!(SRA(Reg(H)), 2, 0xCB, 0x2C);
        assert_opcode!(SRA(Reg(L)), 2, 0xCB, 0x2D);
        assert_opcode!(SRA(RegIndirect(HL)), 2, 0xCB, 0x2E);
    }

    // There's an SLL, but it is undocumented
    // http://z80-heaven.wikidot.com/instructions-set:sll
    #[test]
    #[should_panic(expected = "undocumented instruction SLL")]
    fn sll() {
        opcode(op4!(0xCB, 0x30));
    }

    #[test]
    fn srl() {
        assert_opcode!(SRL(Reg(A)), 2, 0xCB, 0x3F);
        assert_opcode!(SRL(Reg(B)), 2, 0xCB, 0x38);
        assert_opcode!(SRL(Reg(C)), 2, 0xCB, 0x39);
        assert_opcode!(SRL(Reg(D)), 2, 0xCB, 0x3A);
        assert_opcode!(SRL(Reg(E)), 2, 0xCB, 0x3B);
        assert_opcode!(SRL(Reg(H)), 2, 0xCB, 0x3C);
        assert_opcode!(SRL(Reg(L)), 2, 0xCB, 0x3D);
        assert_opcode!(SRL(RegIndirect(HL)), 2, 0xCB, 0x3E);
    }

    #[test]
    fn rlca() {
        assert_opcode!(RLCA, 1, 0x07);
    }

    #[test]
    fn rla() {
        assert_opcode!(RLA, 1, 0x17);
    }

    #[test]
    fn rrca() {
        assert_opcode!(RRCA, 1, 0x0F);
    }

    #[test]
    fn rra() {
        assert_opcode!(RRA, 1, 0x1F);
    }

    #[test]
    fn rld() {
        assert_opcode!(RLD, 2, 0xED, 0x6F);
    }

    #[test]
    fn rrd() {
        assert_opcode!(RRD, 2, 0xED, 0x67);
    }
}

#[test]
fn input() {
    assert_opcode!(IN(Reg(A), Immediate(0x75)), 2, 0xDB, 0x75);

    assert_opcode!(IN(Reg(A), Reg(C)), 2, 0xED, 0x78);
    assert_opcode!(IN(Reg(B), Reg(C)), 2, 0xED, 0x40);
    assert_opcode!(IN(Reg(C), Reg(C)), 2, 0xED, 0x48);
    assert_opcode!(IN(Reg(D), Reg(C)), 2, 0xED, 0x50);
    assert_opcode!(IN(Reg(E), Reg(C)), 2, 0xED, 0x58);
    assert_opcode!(IN(Reg(H), Reg(C)), 2, 0xED, 0x60);
    assert_opcode!(IN(Reg(L), Reg(C)), 2, 0xED, 0x68);
}

#[test]
#[should_panic(expected = "Unknown ExtendeD operation")]
fn input_hl() {
    opcode(op4!(0xED, 0x70));
}

#[test]
fn output() {
    assert_opcode!(OUT(Reg(A), Immediate(0xF5)), 2, 0xD3, 0xF5);

    assert_opcode!(OUT(Reg(A), Reg(C)), 2, 0xED, 0x79);
    assert_opcode!(OUT(Reg(B), Reg(C)), 2, 0xED, 0x41);
    assert_opcode!(OUT(Reg(C), Reg(C)), 2, 0xED, 0x49);
    assert_opcode!(OUT(Reg(D), Reg(C)), 2, 0xED, 0x51);
    assert_opcode!(OUT(Reg(E), Reg(C)), 2, 0xED, 0x59);
    assert_opcode!(OUT(Reg(H), Reg(C)), 2, 0xED, 0x61);
    assert_opcode!(OUT(Reg(L), Reg(C)), 2, 0xED, 0x69);
}

#[test]
#[should_panic(expected = "Unknown ExtendeD operation")]
fn output_hl() {
    opcode(op4!(0xED, 0x71));
}

#[test]
fn jp() {
    assert_opcode!(JP(Unconditional, I16(0xABBA)), 3, 0xC3, 0xBA, 0xAB);

    assert_opcode!(JP(NonZero, I16(0xBAB0)), 3, 0xC2, 0xB0, 0xBA);
    assert_opcode!(JP(Zero, I16(0xC01E)), 3, 0xCA, 0x1E, 0xC0);

    assert_opcode!(JP(NoCarry, I16(0xBAB0)), 3, 0xD2, 0xB0, 0xBA);
    assert_opcode!(JP(Carry, I16(0xC01E)), 3, 0xDA, 0x1E, 0xC0);

    assert_opcode!(JP(ParityOdd, I16(0xBAB0)), 3, 0xE2, 0xB0, 0xBA);
    assert_opcode!(JP(ParityEven, I16(0xC01E)), 3, 0xEA, 0x1E, 0xC0);

    assert_opcode!(JP(SignPositive, I16(0xC01E)), 3, 0xF2, 0x1E, 0xC0);
    assert_opcode!(JP(SignNegative, I16(0xBAB0)), 3, 0xFA, 0xB0, 0xBA);
}

#[test]
fn jr() {
    assert_opcode!(JR(Unconditional, -128), 2, 0x18, 0x80);

    assert_opcode!(JR(NonZero, 127), 2, 0x20, 0x7F);
    assert_opcode!(JR(Zero, 16), 2, 0x28, 0x10);

    assert_opcode!(JR(NoCarry, -1), 2, 0x30, 0xFF);
    assert_opcode!(JR(Carry, -127), 2, 0x38, 0x81);
}

#[test]
fn djnz() {
    assert_opcode!(DJNZ(-10), 2, 0x10, 0xF6);
}
