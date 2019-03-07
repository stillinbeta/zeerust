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
            .set_flag(&ops::StatusFlag::Sign, (sum & 0b10000000) != 0);
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
            .set_flag(&ops::StatusFlag::Carry, (v1 & v2 & 0b01000000) != 0);
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
            .set_flag(&ops::StatusFlag::Sign, (sum & 0b10000000) != 0);
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
    fn test_get_loc8() {
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
    fn test_get_loc8_segfault() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::H, 0xFF);
        z80.registers.set_reg8(&Reg8::L, 0xFF);
        z80.get_loc8(&Location8::RegIndirect(Reg16::HL));
    }

    #[test]
    #[should_panic]
    fn test_set_loc8_immediate_panic() {
        let mut z80 = Z80::default();
        z80.set_loc8(&Location8::Immediate(0x00), 0x00);
    }

    #[test]
    fn test_set_loc8_reg() {
        let mut z80 = Z80::default();
        z80.set_loc8(&Location8::Reg(Reg8::A), 0xDD);
        assert_hex!(0xDD, z80.registers.get_reg8(&Reg8::A));

        z80.registers.set_reg8(&Reg8::H, 0x11);
        z80.registers.set_reg8(&Reg8::L, 0x0A);

        z80.set_loc8(&Location8::RegIndirect(Reg16::HL), 0xEE);
        assert_hex!(0xEE, z80.memory.memory[0x0A11]);
    }

    #[test]
    fn test_ld8() {
        let mut z80 = Z80::default();
        z80.exec(Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate(0xF5)));
        assert_hex!(0xF5, z80.registers.get_reg8(&Reg8::A))
    }

    #[test]
    fn test_add8() {
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
    fn test_adc8() {
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
    fn test_sub8() {
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
    fn test_sbc8() {
        let mut z80 = Z80::default();
        z80.registers.set_flag(&StatusFlag::Carry, true);
        z80.registers.set_reg8(&Reg8::A, 1);
        z80.exec(Op::SBC(
            Location8::Reg(Reg8::A),
            Location8::Immediate(0),
        ));
        assert_bin!(0, z80.registers.get_reg8(&Reg8::A));
        assert!(!z80.registers.get_flag(&StatusFlag::Sign));
        assert!(z80.registers.get_flag(&StatusFlag::Zero));
        assert!(!z80.registers.get_flag(&StatusFlag::HalfCarry));
        assert!(!z80.registers.get_flag(&StatusFlag::ParityOverflow));
        assert!(z80.registers.get_flag(&StatusFlag::AddSubtract));
        assert!(!z80.registers.get_flag(&StatusFlag::Carry));
    }
}
