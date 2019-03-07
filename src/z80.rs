use crate::cpu;
use crate::ops;

#[derive(Default)]
#[allow(dead_code)]
pub struct Z80 {
    registers: cpu::reg::Registers,
    memory: cpu::mem::Memory,
}

impl Z80 {
    pub fn exec(&mut self, op: ops::Op) {
        match op {
            ops::Op::LD8(dst, src) => self.set_loc8(&dst, self.get_loc8(&src)),

            ops::Op::ADD8(dst, src) => self.add(&dst, &src, false),
            ops::Op::ADC(dst, src) => self.add(&dst, &src, true),
            ops::Op::SUB8(dst, src) => self.subtract(&dst, &src, false),
            ops::Op::SBC(dst, src) => self.subtract(&dst, &src, true),

            ops::Op::AND(dst, src) => self.bool_op(&dst, &src, |d, s| d & s),
            ops::Op::OR(dst, src) => self.bool_op(&dst, &src, |d, s| d | s),
            ops::Op::XOR(dst, src) => self.bool_op(&dst, &src, |d, s| d ^ s),
        }
    }

    fn is_borrow(min: u8, sub: u8, bit: u8) -> bool {
        let min_is_zero = (min & (1 << bit)) == 0;
        let sub_is_one = (sub & (1 << bit)) != 0;
        min_is_zero && sub_is_one
    }

    fn subtract(&mut self, dst: &ops::Location8, src: &ops::Location8, include_carry: bool) {
        let v1 = self.get_loc8(dst);
        let mut v2 = self.get_loc8(src);

        if include_carry && self.registers.get_flag(&ops::StatusFlag::Carry) {
            // FIXME: what if _this_ overflows? Behaviour seems undefined
            v2 += 1
        }

        let (sum, ov) = v1.overflowing_sub(v2);
        self.set_loc8(&dst, sum);

        // Seven bit carry
        self.registers
            .set_flag(&ops::StatusFlag::Carry, Self::is_borrow(v1, v2, 6));
        // Subtracting
        self.registers.set_flag(&ops::StatusFlag::AddSubtract, true);
        // Eight bit carry
        self.registers
            .set_flag(&ops::StatusFlag::ParityOverflow, ov);
        // Third bit carry
        self.registers
            .set_flag(&ops::StatusFlag::HalfCarry, Self::is_borrow(v1, v2, 2));
        // Result is zero
        self.registers.set_flag(&ops::StatusFlag::Zero, sum == 0);
        // 8th bit is 1
        self.registers
            .set_flag(&ops::StatusFlag::Sign, (sum & 0b1000_0000) != 0);
    }

    fn add(&mut self, dst: &ops::Location8, src: &ops::Location8, include_carry: bool) {
        let v1 = self.get_loc8(dst);
        let mut v2 = self.get_loc8(src);

        if include_carry && self.registers.get_flag(&ops::StatusFlag::Carry) {
            // FIXME: what if _this_ overflows? Behaviour seems undefined
            v2 += 1
        }

        let (sum, ov) = v1.overflowing_add(v2);
        self.set_loc8(&dst, sum);
        // Seven bit carry
        self.registers
            .set_flag(&ops::StatusFlag::Carry, (v1 & v2 & 0b0100_0000) != 0);
        // Adding
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);
        // Eight bit carry
        self.registers
            .set_flag(&ops::StatusFlag::ParityOverflow, ov);
        // Third bit carry
        self.registers
            .set_flag(&ops::StatusFlag::HalfCarry, (v1 & v2 & 0b00100) != 0);
        // Sum is zero
        self.registers.set_flag(&ops::StatusFlag::Zero, sum == 0);
        // 8th bit is 1
        self.registers
            .set_flag(&ops::StatusFlag::Sign, (sum & 0b1000_0000) != 0);
    }

    fn bool_op<F>(&mut self, dst: &ops::Location8, src: &ops::Location8, f: F)
    where
        F: Fn(u8, u8) -> u8,
    {
        let v1 = self.get_loc8(dst);
        let v2 = self.get_loc8(src);

        let result = f(v1, v2);
        self.set_loc8(&dst, result);
        let parity = (0..8).fold(0, |acc, b| acc + ((result & (1 << b)) == 0) as u8) % 2 == 0;

        // Seven bit carry is reset
        self.registers.set_flag(&ops::StatusFlag::Carry, false);
        // Adding
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);
        // Eight bit carry
        self.registers
            .set_flag(&ops::StatusFlag::ParityOverflow, parity);
        // Third bit carry
        self.registers.set_flag(&ops::StatusFlag::HalfCarry, false);

        self.registers.set_flag(&ops::StatusFlag::Zero, result == 0);
        self.registers
            .set_flag(&ops::StatusFlag::Sign, (result & 0b1000_0000) != 0);
    }

    fn get_loc8(&self, loc: &ops::Location8) -> u8 {
        match loc {
            ops::Location8::Immediate(v) => *v,
            ops::Location8::Reg(reg) => self.registers.get_reg8(&reg),
            ops::Location8::RegIndirect(reg) => {
                let addr = self.registers.get_reg16(&reg);
                self.memory.memory[addr as usize]
            }
        }
    }

    fn set_loc8(&mut self, loc: &ops::Location8, val: u8) {
        match loc {
            ops::Location8::Immediate(_) => panic!("Attempting to set immediate value!"),
            ops::Location8::Reg(reg) => self.registers.set_reg8(reg, val),
            ops::Location8::RegIndirect(reg) => {
                let addr = self.registers.get_reg16(reg);
                self.memory.memory[addr as usize] = val;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Z80;
    use crate::ops::{Location8, Op, Reg16, Reg8, StatusFlag};

    #[test]
    fn get_loc8() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0xC5);
        z80.registers.set_reg8(&Reg8::H, 0xAA);
        z80.registers.set_reg8(&Reg8::L, 0x0F);
        z80.memory.memory[0x0FAA] = 0xD1;

        assert_hex!(0xC5, z80.get_loc8(&Location8::Reg(Reg8::A)));
        assert_hex!(0xD1, z80.get_loc8(&Location8::RegIndirect(Reg16::HL)));
        assert_hex!(0xCC, z80.get_loc8(&Location8::Immediate(0xCC)));
    }

    #[test]
    #[should_panic]
    fn get_loc8_segfault() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::H, 0xFF);
        z80.registers.set_reg8(&Reg8::L, 0xFF);
        z80.get_loc8(&Location8::RegIndirect(Reg16::HL));
    }

    #[test]
    #[should_panic]
    fn set_loc8_immediate_panic() {
        let mut z80 = Z80::default();
        z80.set_loc8(&Location8::Immediate(0x00), 0x00);
    }

    #[test]
    fn set_loc8_register() {
        let mut z80 = Z80::default();
        z80.set_loc8(&Location8::Reg(Reg8::A), 0xDD);
        assert_hex!(0xDD, z80.registers.get_reg8(&Reg8::A));

        z80.registers.set_reg8(&Reg8::H, 0x11);
        z80.registers.set_reg8(&Reg8::L, 0x0A);

        z80.set_loc8(&Location8::RegIndirect(Reg16::HL), 0xEE);
        assert_hex!(0xEE, z80.memory.memory[0x0A11]);
    }

    #[test]
    fn ld8_op() {
        let mut z80 = Z80::default();
        z80.exec(Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate(0xF5)));
        assert_hex!(0xF5, z80.registers.get_reg8(&Reg8::A))
    }

    #[test]
    fn add8_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0x64);
        z80.exec(Op::ADD8(
            Location8::Reg(Reg8::A),
            Location8::Immediate(0x44),
        ));
        assert_hex!(0xA8, z80.registers.get_reg8(&Reg8::A));
        assert!(z80.registers.get_flag(&StatusFlag::Sign));
        assert!(!z80.registers.get_flag(&StatusFlag::Zero));
        assert!(z80.registers.get_flag(&StatusFlag::HalfCarry));
        assert!(!z80.registers.get_flag(&StatusFlag::ParityOverflow));
        assert!(!z80.registers.get_flag(&StatusFlag::AddSubtract));
        assert!(z80.registers.get_flag(&StatusFlag::Carry));

        z80.registers.set_reg8(&Reg8::A, 0xFF);
        z80.exec(Op::ADD8(
            Location8::Reg(Reg8::A),
            Location8::Immediate(0x01),
        ));
        assert_hex!(0x00, z80.registers.get_reg8(&Reg8::A));
        assert!(!z80.registers.get_flag(&StatusFlag::Sign));
        assert!(z80.registers.get_flag(&StatusFlag::Zero));
        assert!(!z80.registers.get_flag(&StatusFlag::HalfCarry));
        assert!(z80.registers.get_flag(&StatusFlag::ParityOverflow));
        assert!(!z80.registers.get_flag(&StatusFlag::AddSubtract));
        assert!(!z80.registers.get_flag(&StatusFlag::Carry));
    }

    #[test]
    fn adc8_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0x64);
        z80.registers.set_flag(&StatusFlag::Carry, true);
        z80.exec(Op::ADC(Location8::Reg(Reg8::A), Location8::Immediate(0x44)));
        assert_hex!(0xA9, z80.registers.get_reg8(&Reg8::A));
        assert!(z80.registers.get_flag(&StatusFlag::Sign));
        assert!(!z80.registers.get_flag(&StatusFlag::Zero));
        assert!(z80.registers.get_flag(&StatusFlag::HalfCarry));
        assert!(!z80.registers.get_flag(&StatusFlag::ParityOverflow));
        assert!(!z80.registers.get_flag(&StatusFlag::AddSubtract));
        assert!(z80.registers.get_flag(&StatusFlag::Carry));
    }

    #[test]
    fn sub8_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0b10100000);
        z80.exec(Op::SUB8(
            Location8::Reg(Reg8::A),
            Location8::Immediate(0b01000100),
        ));
        assert_bin!(0b01011100, z80.registers.get_reg8(&Reg8::A));
        assert!(!z80.registers.get_flag(&StatusFlag::Sign));
        assert!(!z80.registers.get_flag(&StatusFlag::Zero));
        assert!(z80.registers.get_flag(&StatusFlag::HalfCarry));
        assert!(!z80.registers.get_flag(&StatusFlag::ParityOverflow));
        assert!(z80.registers.get_flag(&StatusFlag::AddSubtract));
        assert!(z80.registers.get_flag(&StatusFlag::Carry));
    }

    #[test]
    fn sbc8_op() {
        let mut z80 = Z80::default();
        z80.registers.set_flag(&StatusFlag::Carry, true);
        z80.registers.set_reg8(&Reg8::A, 1);
        z80.exec(Op::SBC(Location8::Reg(Reg8::A), Location8::Immediate(0)));
        assert_bin!(0, z80.registers.get_reg8(&Reg8::A));
        assert!(!z80.registers.get_flag(&StatusFlag::Sign));
        assert!(z80.registers.get_flag(&StatusFlag::Zero));
        assert!(!z80.registers.get_flag(&StatusFlag::HalfCarry));
        assert!(!z80.registers.get_flag(&StatusFlag::ParityOverflow));
        assert!(z80.registers.get_flag(&StatusFlag::AddSubtract));
        assert!(!z80.registers.get_flag(&StatusFlag::Carry));
    }

    #[test]
    fn and_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0b1001_1000);
        z80.exec(Op::AND(
            Location8::Reg(Reg8::A),
            Location8::Immediate(0b0000_0000),
        ));
        assert_bin!(0b0000_0000, z80.registers.get_reg8(&Reg8::A));
        assert!(!z80.registers.get_flag(&StatusFlag::Sign));
        assert!(z80.registers.get_flag(&StatusFlag::Zero));
        assert!(!z80.registers.get_flag(&StatusFlag::HalfCarry));
        assert!(z80.registers.get_flag(&StatusFlag::ParityOverflow));
        assert!(!z80.registers.get_flag(&StatusFlag::AddSubtract));
        assert!(!z80.registers.get_flag(&StatusFlag::Carry));
    }

    #[test]
    fn or_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0b1001_1000);
        z80.exec(Op::OR(
            Location8::Reg(Reg8::A),
            Location8::Immediate(0b0001_1011),
        ));
        assert_bin!(0b1001_1011, z80.registers.get_reg8(&Reg8::A));
        assert!(z80.registers.get_flag(&StatusFlag::Sign));
        assert!(!z80.registers.get_flag(&StatusFlag::Zero));
        assert!(!z80.registers.get_flag(&StatusFlag::HalfCarry));
        assert!(!z80.registers.get_flag(&StatusFlag::ParityOverflow));
        assert!(!z80.registers.get_flag(&StatusFlag::AddSubtract));
        assert!(!z80.registers.get_flag(&StatusFlag::Carry));
    }

    #[test]
    fn xor_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0b0011_1100);
        z80.exec(Op::XOR(
            Location8::Reg(Reg8::A),
            Location8::Immediate(0b0001_1011),
        ));
        assert_bin!(0b0010_0111, z80.registers.get_reg8(&Reg8::A));
        assert!(!z80.registers.get_flag(&StatusFlag::Sign));
        assert!(!z80.registers.get_flag(&StatusFlag::Zero));
        assert!(!z80.registers.get_flag(&StatusFlag::HalfCarry));
        assert!(z80.registers.get_flag(&StatusFlag::ParityOverflow));
        assert!(!z80.registers.get_flag(&StatusFlag::AddSubtract));
        assert!(!z80.registers.get_flag(&StatusFlag::Carry));
    }
}
