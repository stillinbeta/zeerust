use crate::cpu;
use crate::ops;

#[derive(Default)]
#[allow(dead_code)]
pub struct Z80 {
    registers: cpu::reg::Registers,
    memory: cpu::mem::Memory,
}

impl Z80 {
    const ONE_IMM: ops::Location8 = ops::Location8::Immediate(1);
    const ACC: ops::Location8 = ops::Location8::Reg(ops::Reg8::A);
    const HL_INDIRECT: ops::Location8 = ops::Location8::RegIndirect(ops::Reg16::HL);

    pub fn exec(&mut self, op: ops::Op) {
        match op {
            ops::Op::LD8(dst, src) => self.set_loc8(&dst, self.get_loc8(&src)),

            ops::Op::ADD8(dst, src) => self.add(&dst, &src, false),
            ops::Op::ADC(dst, src) => self.add(&dst, &src, true),
            ops::Op::INC(dst) => self.add(&dst, &Self::ONE_IMM, false),

            ops::Op::SUB8(dst, src) => self.subtract(&dst, &src, false, true),
            ops::Op::SBC(dst, src) => self.subtract(&dst, &src, true, true),
            ops::Op::DEC(dst) => self.subtract(&dst, &Self::ONE_IMM, false, true),
            ops::Op::CP(src) => self.subtract(&Self::ACC, &src, false, false),

            ops::Op::AND(dst, src) => self.bool_op(&dst, &src, |d, s| d & s),
            ops::Op::OR(dst, src) => self.bool_op(&dst, &src, |d, s| d | s),
            ops::Op::XOR(dst, src) => self.bool_op(&dst, &src, |d, s| d ^ s),

            ops::Op::DAA => unimplemented!(),
            ops::Op::CPL => self.compliment(),
            ops::Op::NEG => self.negate(),
            ops::Op::CCF => self.toggle_carry(),
            ops::Op::SCF => self.set_carry(),

            ops::Op::NOP => (),

            ops::Op::RLCA => self.rotate_left(&Self::ACC, false),
            ops::Op::RLA => self.rotate_left_thru_acc(&Self::ACC, false),
            ops::Op::RRCA => self.rotate_right(&Self::ACC, false),
            ops::Op::RRA => self.rotate_right_thru_acc(&Self::ACC, false),
            ops::Op::RLC(reg) => self.rotate_left(&reg, true),
            ops::Op::RL(reg) => self.rotate_left_thru_acc(&reg, true),
            ops::Op::RRC(reg) => self.rotate_right(&reg, true),
            ops::Op::RR(reg) => self.rotate_right_thru_acc(&reg, true),

            ops::Op::SRL(loc) => self.shift_right(&loc, false),
            ops::Op::SLA(loc) => self.shift_left(&loc),
            ops::Op::SRA(loc) => self.shift_right(&loc, true),

            ops::Op::RLD => self.rotate_nibble_left(),
            ops::Op::RRD => self.rotate_nibble_right(),
        }
    }

    fn is_borrow(min: u8, sub: u8, bit: u8) -> bool {
        let mask = (1 << (bit + 1)) - 1;
        (min & mask) < (sub & mask)
    }

    fn subtract(
        &mut self,
        dst: &ops::Location8,
        src: &ops::Location8,
        include_carry: bool,
        store_result: bool,
    ) {
        let v1 = self.get_loc8(dst);
        let mut v2 = self.get_loc8(src);

        if include_carry && self.registers.get_flag(&ops::StatusFlag::Carry) {
            // FIXME: what if _this_ overflows? Behaviour seems undefined
            v2 += 1
        }

        let (sum, ov) = v1.overflowing_sub(v2);
        if store_result {
            self.set_loc8(&dst, sum);
        }

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

        // Seven bit carry is reset
        self.registers.set_flag(&ops::StatusFlag::Carry, false);
        // Adding
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);
        // Third bit carry
        self.registers.set_flag(&ops::StatusFlag::HalfCarry, false);

        self.parity_flags(result);
    }

    fn compliment(&mut self) {
        let reg_a = ops::Reg8::A;
        let a = self.registers.get_reg8(&reg_a);
        self.registers.set_reg8(&reg_a, !a);

        self.registers.set_flag(&ops::StatusFlag::HalfCarry, true);
        self.registers.set_flag(&ops::StatusFlag::AddSubtract, true);
    }

    fn negate(&mut self) {
        let reg_a = ops::Reg8::A;
        let a = self.registers.get_reg8(&reg_a);

        let compliment = (1_u16 << 8) - u16::from(a);
        let [result, _] = compliment.to_le_bytes();
        // let result = (!a) + 1
        self.registers.set_reg8(&reg_a, result);

        self.registers
            .set_flag(&ops::StatusFlag::Sign, (result & 0b1000_0000) != 0);
        self.registers.set_flag(&ops::StatusFlag::Zero, result == 0);
        self.registers
            .set_flag(&ops::StatusFlag::HalfCarry, Self::is_borrow(0, a, 3));
        self.registers
            .set_flag(&ops::StatusFlag::ParityOverflow, a == 0x80);
        self.registers.set_flag(&ops::StatusFlag::AddSubtract, true);
        self.registers.set_flag(&ops::StatusFlag::Carry, a != 0x00);
    }

    fn toggle_carry(&mut self) {
        let carry = self.registers.get_flag(&ops::StatusFlag::Carry);
        self.registers.set_flag(&ops::StatusFlag::Carry, !carry);
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);
    }

    fn set_carry(&mut self) {
        self.registers.set_flag(&ops::StatusFlag::Carry, true);
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);
        self.registers.set_flag(&ops::StatusFlag::HalfCarry, false);
    }

    fn rotate_left(&mut self, loc: &ops::Location8, set_parity: bool) {
        let val = self.get_loc8(loc);
        let result = val.rotate_left(1);
        self.set_loc8(loc, result);
        self.registers
            .set_flag(&ops::StatusFlag::Carry, result & 0x80 != 0);
        self.registers.set_flag(&ops::StatusFlag::HalfCarry, false);
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);

        if set_parity {
            self.parity_flags(result)
        }
    }

    fn rotate_left_thru_acc(&mut self, loc: &ops::Location8, set_parity: bool) {
        let val = self.get_loc8(loc);
        let carry = self.registers.get_flag(&ops::StatusFlag::Carry);
        let mut result = val << 1;
        if carry {
            result |= 0b1;
        }
        self.set_loc8(loc, result);

        self.registers
            .set_flag(&ops::StatusFlag::Carry, val & 0x80 != 0);
        self.registers.set_flag(&ops::StatusFlag::HalfCarry, false);
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);

        if set_parity {
            self.parity_flags(result)
        }
    }

    fn rotate_right(&mut self, loc: &ops::Location8, set_parity: bool) {
        let val = self.get_loc8(loc);
        let result = val.rotate_right(1);
        self.set_loc8(loc, result);
        self.registers
            .set_flag(&ops::StatusFlag::Carry, result & 0b1 != 0);
        self.registers.set_flag(&ops::StatusFlag::HalfCarry, false);
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);

        if set_parity {
            self.parity_flags(result)
        }
    }

    fn rotate_right_thru_acc(&mut self, loc: &ops::Location8, set_parity: bool) {
        let val = self.get_loc8(loc);
        let carry = self.registers.get_flag(&ops::StatusFlag::Carry);
        let mut result = val >> 1;
        if carry {
            result |= 0x80;
        }
        self.set_loc8(loc, result);
        self.registers
            .set_flag(&ops::StatusFlag::Carry, val & 0b1 != 0);
        self.registers.set_flag(&ops::StatusFlag::HalfCarry, false);
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);

        if set_parity {
            self.parity_flags(result);
        }
    }

    fn shift_right(&mut self, loc: &ops::Location8, preserve_sign: bool) {
        let val = self.get_loc8(loc);
        let carry = (val & 0x1) != 0;
        let mut result = val >> 1;

        if preserve_sign {
            result |= val & 0x80;
        }

        self.set_loc8(loc, result);

        self.registers.set_flag(&ops::StatusFlag::Carry, carry);
        self.registers.set_flag(&ops::StatusFlag::HalfCarry, false);
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);
        self.parity_flags(result);
    }

    fn shift_left(&mut self, loc: &ops::Location8) {
        let val = self.get_loc8(loc);
        let carry = (val & 0x80) != 0;
        let result = val << 1;

        self.set_loc8(loc, result);

        self.registers.set_flag(&ops::StatusFlag::Carry, carry);
        self.registers.set_flag(&ops::StatusFlag::HalfCarry, false);
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);
        self.parity_flags(result);
    }

    fn rotate_nibble_left(&mut self) {
        let acc = self.get_loc8(&Self::ACC);
        let hl = self.get_loc8(&Self::HL_INDIRECT);

        let acc2 = (acc & 0xf0) | (hl >> 4);
        let hl2 = (hl << 4) | (acc & 0x0f);

        self.set_loc8(&Self::ACC, acc2);
        self.set_loc8(&Self::HL_INDIRECT, hl2);

        self.registers.set_flag(&ops::StatusFlag::HalfCarry, false);
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);
        self.parity_flags(acc2);
    }

    fn rotate_nibble_right(&mut self) {
        let acc = self.get_loc8(&Self::ACC);
        let hl = self.get_loc8(&Self::HL_INDIRECT);

        let acc2 = (acc & 0xf0) | (hl & 0x0f);
        let hl2 = (hl >> 4) | ((acc & 0x0f) << 4);

        self.set_loc8(&Self::ACC, acc2);
        self.set_loc8(&Self::HL_INDIRECT, hl2);

        self.registers.set_flag(&ops::StatusFlag::HalfCarry, false);
        self.registers
            .set_flag(&ops::StatusFlag::AddSubtract, false);
        self.parity_flags(acc2);
    }

    fn parity_flags(&mut self, val: u8) {
        let parity = val.count_zeros() % 2 == 0;

        self.registers
            .set_flag(&ops::StatusFlag::ParityOverflow, parity);
        self.registers.set_flag(&ops::StatusFlag::Zero, val == 0);
        self.registers
            .set_flag(&ops::StatusFlag::Sign, (val & 0b1000_0000) != 0);
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
        assert_flags!(
            z80.registers,
            Sign = true,
            Zero = false,
            HalfCarry = true,
            ParityOverflow = false,
            AddSubtract = false,
            Carry = true,
        );

        z80.registers.set_reg8(&Reg8::A, 0xFF);
        z80.exec(Op::ADD8(
            Location8::Reg(Reg8::A),
            Location8::Immediate(0x01),
        ));
        assert_hex!(0x00, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = true,
            HalfCarry = false,
            ParityOverflow = true,
            AddSubtract = false,
            Carry = false,
        );
    }

    #[test]
    fn inc_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::H, 0xCC);
        z80.registers.set_reg8(&Reg8::L, 0x20);
        z80.memory.memory[0x20CC] = 0xFF;

        z80.exec(Op::INC(Location8::RegIndirect(Reg16::HL)));

        assert_hex!(0x00, z80.memory.memory[0x20CC]);
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = true,
            HalfCarry = false,
            ParityOverflow = true,
            AddSubtract = false,
            Carry = false,
        );
    }

    #[test]
    fn adc8_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0x64);
        z80.registers.set_flag(&StatusFlag::Carry, true);
        z80.exec(Op::ADC(Location8::Reg(Reg8::A), Location8::Immediate(0x44)));
        assert_hex!(0xA9, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            Sign = true,
            Zero = false,
            HalfCarry = true,
            ParityOverflow = false,
            AddSubtract = false,
            Carry = true,
        );
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
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = false,
            HalfCarry = true,
            ParityOverflow = false,
            AddSubtract = true,
            Carry = true,
        );
    }

    #[test]
    fn cp_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0b1010_0000);
        z80.exec(Op::CP(Location8::Immediate(0b0100_0100)));
        assert_bin!(0b1010_0000, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = false,
            HalfCarry = true,
            ParityOverflow = false,
            AddSubtract = true,
            Carry = true,
        );
    }

    #[test]
    fn sbc8_op() {
        let mut z80 = Z80::default();
        z80.registers.set_flag(&StatusFlag::Carry, true);
        z80.registers.set_reg8(&Reg8::A, 1);
        z80.exec(Op::SBC(Location8::Reg(Reg8::A), Location8::Immediate(0)));
        assert_bin!(0, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = true,
            HalfCarry = false,
            ParityOverflow = false,
            AddSubtract = true,
            Carry = false,
        );
    }

    #[test]
    fn dec_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0b1010_0000);
        z80.exec(Op::DEC(Location8::Reg(Reg8::A)));
        assert_bin!(0b1001_1111, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            Sign = true,
            Zero = false,
            HalfCarry = true,
            ParityOverflow = false,
            AddSubtract = true,
            Carry = false,
        );
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
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = true,
            HalfCarry = false,
            ParityOverflow = true,
            AddSubtract = false,
            Carry = false,
        );
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
        assert_flags!(
            z80.registers,
            Sign = true,
            Zero = false,
            HalfCarry = false,
            ParityOverflow = false,
            AddSubtract = false,
            Carry = false,
        );
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
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = false,
            HalfCarry = false,
            ParityOverflow = true,
            AddSubtract = false,
            Carry = false,
        );
    }

    #[test]
    #[should_panic]
    fn daa_op() {
        let mut z80 = Z80::default();
        z80.exec(Op::DAA);
    }

    #[test]
    fn cpl_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0b1011_1101);
        z80.exec(Op::CPL);
        assert_bin!(0b0100_0010, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(z80.registers, HalfCarry = true, AddSubtract = true,);
    }

    #[test]
    fn neg_op() {
        let mut z80 = Z80::default();
        // Sign is positive
        z80.registers.set_reg8(&Reg8::A, 0b1001_1000);
        z80.exec(Op::NEG);
        assert_bin!(0b0110_1000, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = false,
            HalfCarry = true,
            ParityOverflow = false,
            AddSubtract = true,
            Carry = true,
        );

        // Sign is negative
        z80.registers.set_reg8(&Reg8::A, 0b0001_1000);
        z80.exec(Op::NEG);
        assert_bin!(0b1110_1000, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            Sign = true,
            Zero = false,
            HalfCarry = true,
            ParityOverflow = false,
            AddSubtract = true,
            Carry = true,
        );

        // A was 0x80
        z80.registers.set_reg8(&Reg8::A, 0x80);
        z80.exec(Op::NEG);
        // TODO: not 100% on 2's compliment of 0x80
        assert_bin!(0b1000_0000, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            Sign = true,
            Zero = false,
            HalfCarry = false,
            ParityOverflow = true,
            AddSubtract = true,
            Carry = true,
        );

        // A was 0x00
        z80.registers.set_reg8(&Reg8::A, 0b0000_0000);
        z80.exec(Op::NEG);
        assert_bin!(0b0000_0000, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = true,
            HalfCarry = false,
            ParityOverflow = false,
            AddSubtract = true,
            Carry = false,
        );
    }

    #[test]
    fn ccf_op() {
        let mut z80 = Z80::default();
        z80.registers.set_flag(&StatusFlag::AddSubtract, true);
        z80.registers.set_flag(&StatusFlag::Carry, false);
        z80.exec(Op::CCF);
        assert_flags!(z80.registers, Carry = true, AddSubtract = false,);
        z80.exec(Op::CCF);
        assert_flags!(z80.registers, Carry = false, AddSubtract = false,);
    }

    #[test]
    fn scf_op() {
        let mut z80 = Z80::default();
        z80.registers.set_flag(&StatusFlag::AddSubtract, true);
        z80.registers.set_flag(&StatusFlag::HalfCarry, true);
        z80.registers.set_flag(&StatusFlag::Carry, false);
        z80.exec(Op::SCF);
        assert_flags!(
            z80.registers,
            Carry = true,
            AddSubtract = false,
            HalfCarry = false,
        );
    }

    #[test]
    fn rlca_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0b0101_1011);
        z80.exec(Op::RLCA);
        assert_bin!(0b1011_0110, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            HalfCarry = false,
            AddSubtract = false,
            Carry = true,
        );
    }

    #[test]
    fn rla_op() {
        let mut z80 = Z80::default();
        z80.registers.set_flag(&StatusFlag::Carry, true);
        z80.registers.set_reg8(&Reg8::A, 0b1001_1011);
        z80.exec(Op::RLA);
        assert_bin!(0b0011_0111, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            HalfCarry = false,
            AddSubtract = false,
            Carry = true,
        );

        z80.registers.set_flag(&StatusFlag::Carry, false);
        z80.registers.set_reg8(&Reg8::A, 0b0001_1001);
        z80.exec(Op::RLA);
        assert_bin!(0b0011_0010, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            HalfCarry = false,
            AddSubtract = false,
            Carry = false,
        );
    }

    #[test]
    fn rrca_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0b0110_1001);
        z80.exec(Op::RRCA);
        assert_bin!(0b1011_0100, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            HalfCarry = false,
            AddSubtract = false,
            Carry = false,
        );
    }

    #[test]
    fn rra_op() {
        let mut z80 = Z80::default();
        z80.registers.set_flag(&StatusFlag::Carry, true);
        z80.registers.set_reg8(&Reg8::A, 0b0101_1100);
        z80.exec(Op::RRA);
        assert_bin!(0b1010_1110, z80.registers.get_reg8(&Reg8::A));
        assert_flags!(
            z80.registers,
            HalfCarry = false,
            AddSubtract = false,
            Carry = false,
        );

        z80.registers.set_flag(&StatusFlag::Carry, false);
        z80.registers.set_reg8(&Reg8::A, 0b1010_1011);
        z80.exec(Op::RRA);
        assert_bin!(0b0101_0101, z80.registers.get_reg8(&Reg8::A));

        assert_flags!(
            z80.registers,
            HalfCarry = false,
            AddSubtract = false,
            Carry = true,
        );
    }

    #[test]
    fn rlc_op() {
        let mut z80 = Z80::default();
        z80.registers.set_flag(&StatusFlag::Carry, true);
        z80.registers.set_reg8(&Reg8::B, 0b1111_0000);
        z80.exec(Op::RLC(Location8::Reg(Reg8::B)));
        assert_bin!(0b1110_0001, z80.registers.get_reg8(&Reg8::B));

        assert_flags!(
            z80.registers,
            Sign = true,
            Zero = false,
            HalfCarry = false,
            ParityOverflow = true,
            AddSubtract = false,
            Carry = true,
        );
    }

    #[test]
    fn rl_op() {
        let mut z80 = Z80::default();
        z80.registers.set_flag(&StatusFlag::Carry, false);
        z80.registers.set_reg8(&Reg8::B, 0b1000_0000);
        z80.exec(Op::RL(Location8::Reg(Reg8::B)));
        assert_bin!(0b0000_0000, z80.registers.get_reg8(&Reg8::B));
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = true,
            HalfCarry = false,
            ParityOverflow = true,
            AddSubtract = false,
            Carry = true,
        );
    }

    #[test]
    fn rrc_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::B, 0b1000_1011);
        z80.exec(Op::RRC(Location8::Reg(Reg8::B)));
        assert_bin!(0b1100_0101, z80.registers.get_reg8(&Reg8::B));
        assert_flags!(
            z80.registers,
            Sign = true,
            Zero = false,
            HalfCarry = false,
            ParityOverflow = true,
            AddSubtract = false,
            Carry = true,
        );
    }

    #[test]
    fn rr_op() {
        let mut z80 = Z80::default();
        z80.registers.set_flag(&StatusFlag::Carry, true);
        z80.registers.set_reg8(&Reg8::B, 0b1110_1110);
        z80.exec(Op::RR(Location8::Reg(Reg8::B)));
        assert_bin!(0b1111_0111, z80.registers.get_reg8(&Reg8::B));

        assert_flags!(
            z80.registers,
            Sign = true,
            Zero = false,
            HalfCarry = false,
            ParityOverflow = false,
            AddSubtract = false,
            Carry = false,
        );
    }

    #[test]
    fn srl_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::C, 0b0110_0001);
        z80.exec(Op::SRL(Location8::Reg(Reg8::C)));
        assert_bin!(0b0011_0000, z80.registers.get_reg8(&Reg8::C));
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = false,
            HalfCarry = false,
            ParityOverflow = true,
            AddSubtract = false,
            Carry = true
        );
    }

    #[test]
    fn sla_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::D, 0b1000_0000);
        z80.exec(Op::SLA(Location8::Reg(Reg8::D)));
        assert_bin!(0b0000_0000, z80.registers.get_reg8(&Reg8::D));
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = true,
            HalfCarry = false,
            ParityOverflow = true,
            AddSubtract = false,
            Carry = true
        );
    }

    #[test]
    fn sra_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::C, 0b1100_1100);
        z80.exec(Op::SRA(Location8::Reg(Reg8::C)));
        assert_bin!(0b1110_0110, z80.registers.get_reg8(&Reg8::C));
        assert_flags!(
            z80.registers,
            Sign = true,
            Zero = false,
            HalfCarry = false,
            ParityOverflow = false,
            AddSubtract = false,
            Carry = false
        );
    }

    #[test]
    fn rld_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::H, 0xCC);
        z80.registers.set_reg8(&Reg8::L, 0x20);
        z80.registers.set_reg8(&Reg8::A, 0b0111_1010);
        z80.memory.memory[0x20CC] = 0b0011_0001;

        z80.exec(Op::RLD);

        assert_bin!(0b0111_0011, z80.registers.get_reg8(&Reg8::A));
        assert_bin!(0b0001_1010, z80.memory.memory[0x20CC]);
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = false,
            HalfCarry = false,
            ParityOverflow = false,
            AddSubtract = false,
        );

        // Zero accumulator
        z80.registers.set_reg8(&Reg8::H, 0xCC);
        z80.registers.set_reg8(&Reg8::L, 0x20);
        z80.registers.set_reg8(&Reg8::A, 0b0000_1010);
        z80.memory.memory[0x20CC] = 0b0000_1110;

        z80.exec(Op::RLD);

        assert_bin!(0b0000_0000, z80.registers.get_reg8(&Reg8::A));
        assert_bin!(0b1110_1010, z80.memory.memory[0x20CC]);
        assert_flags!(
            z80.registers,
            Sign = false,
            Zero = true,
            HalfCarry = false,
            ParityOverflow = true,
            AddSubtract = false,
        );
    }

    #[test]
    fn rrd_op() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::H, 0xCC);
        z80.registers.set_reg8(&Reg8::L, 0x20);
        z80.registers.set_reg8(&Reg8::A, 0b1000_0100);
        z80.memory.memory[0x20CC] = 0b0010_0000;

        z80.exec(Op::RRD);

        assert_bin!(0b1000_0000, z80.registers.get_reg8(&Reg8::A));
        assert_bin!(0b0100_0010, z80.memory.memory[0x20CC]);
        assert_flags!(
            z80.registers,
            Sign = true,
            Zero = false,
            HalfCarry = false,
            ParityOverflow = false,
            AddSubtract = false,
        );
    }
}
