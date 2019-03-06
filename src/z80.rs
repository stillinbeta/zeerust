use crate::cpu;
use crate::ops;

#[derive(Default)]
#[allow(dead_code)]
pub struct Z80 {
    registers: cpu::reg::Registers,
    memory: cpu::mem::Memory,
}

impl Z80 {
    pub fn exec (&mut self, op: ops::Op) {
        match op {
            ops::Op::LD8(dst, src) => self.set_loc8(&dst, self.get_loc8(&src)),
            ops::Op::ADD8(dst, src) => {
                let v1 = self.get_loc8(&dst);
                let v2 = self.get_loc8(&src);
                self.set_loc8(&dst, v1 + v2)
            }
        }
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
    use crate::ops::{Location8, Op, Reg8, Reg16};

    #[test]
    fn test_get_loc8() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0xC5);
        z80.registers.set_reg8(&Reg8::H, 0xAA);
        z80.registers.set_reg8(&Reg8::L, 0x0F);
        z80.memory.memory[0x0FAA] = 0xD1;

        assert_eq!(0xC5, z80.get_loc8(&Location8::Reg(Reg8::A)));
        assert_eq!(0xD1, z80.get_loc8(&Location8::RegIndirect(Reg16::HL)));
        assert_eq!(0xCC, z80.get_loc8(&Location8::Immediate(0xCC)));
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
        assert_eq!(0xDD, z80.registers.get_reg8(&Reg8::A));

        z80.registers.set_reg8(&Reg8::H, 0x11);
        z80.registers.set_reg8(&Reg8::L, 0x0A);

        z80.set_loc8(&Location8::RegIndirect(Reg16::HL), 0xEE);
        assert_eq!(0xEE, z80.memory.memory[0x0A11]);
    }

    #[test]
    fn test_ld8() {
        let mut z80 = Z80::default();
        z80.exec(Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate(0xF5)));
        assert_eq!(0xF5, z80.registers.get_reg8(&Reg8::A))
    }

    #[test]
    fn test_add8() {
        let mut z80 = Z80::default();
        z80.registers.set_reg8(&Reg8::A, 0x11);
        z80.exec(Op::ADD8(Location8::Reg(Reg8::A), Location8::Immediate(0xC5)));
        assert_eq!(0xD6, z80.registers.get_reg8(&Reg8::A))
    }
}

