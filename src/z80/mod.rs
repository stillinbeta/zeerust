use crate::cpu;
use crate::ops;

#[cfg(test)]
mod tests;

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

