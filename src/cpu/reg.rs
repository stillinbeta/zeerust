//!  The internal representation of all the registers of the z80.
use crate::ops::{Reg16, Reg8, StatusFlag};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,

    ap: u8,
    bp: u8,
    cp: u8,
    dp: u8,
    ep: u8,
    fp: u8,
    hp: u8,
    lp: u8,

    pc: u16,
    ix: u16,
    iy: u16,
    sp: u16,
}

impl Registers {
    fn flag_mask(f: &StatusFlag) -> u8 {
        match f {
            StatusFlag::Carry => 1,
            StatusFlag::AddSubtract => 1 << 1,
            StatusFlag::ParityOverflow => 1 << 2,
            // bit 3 is unused
            StatusFlag::HalfCarry => 1 << 4,
            // bit 5 is unused
            StatusFlag::Zero => 1 << 6,
            StatusFlag::Sign => 1 << 7,
        }
    }

    /// Retrieve a given flag, bitmasked out of register F.
    pub fn get_flag(&self, f: &StatusFlag) -> bool {
        (self.f & Self::flag_mask(f)) != 0
    }

    /// Retrieve a given flag, bitmasked into of register F.
    pub fn set_flag(&mut self, f: &StatusFlag, set: bool) {
        if set {
            self.f |= Self::flag_mask(f)
        } else {
            self.f &= !Self::flag_mask(f)
        }
    }

    pub fn get_reg8(&self, r: Reg8) -> u8 {
        match r {
            Reg8::A => self.a,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::F => self.f,
            Reg8::H => self.h,
            Reg8::L => self.l,

            Reg8::AP => self.ap,
            Reg8::BP => self.bp,
            Reg8::CP => self.cp,
            Reg8::DP => self.dp,
            Reg8::EP => self.ep,
            Reg8::FP => self.fp,
            Reg8::HP => self.hp,
            Reg8::LP => self.lp,
        }
    }

    /// Set an 8-bit register
    pub fn set_reg8(&mut self, r: Reg8, v: u8) {
        match r {
            Reg8::A => self.a = v,
            Reg8::B => self.b = v,
            Reg8::C => self.c = v,
            Reg8::D => self.d = v,
            Reg8::E => self.e = v,
            Reg8::F => self.f = v,
            Reg8::H => self.h = v,
            Reg8::L => self.l = v,

            Reg8::AP => self.ap = v,
            Reg8::BP => self.bp = v,
            Reg8::CP => self.cp = v,
            Reg8::DP => self.dp = v,
            Reg8::EP => self.ep = v,
            Reg8::FP => self.fp = v,
            Reg8::HP => self.hp = v,
            Reg8::LP => self.lp = v,
        }
    }

    /// Set a 16-bit registers. These are made of two 8-bit registers, little-endian.
    pub fn set_reg16(&mut self, r: &Reg16, v: u16) {
        let [r0, r1] = v.to_le_bytes();
        match r {
            Reg16::AF => {
                self.a = r0;
                self.f = r1;
            }
            Reg16::BC => {
                self.b = r0;
                self.c = r1;
            }
            Reg16::DE => {
                self.d = r0;
                self.e = r1;
            }
            Reg16::HL => {
                self.h = r0;
                self.l = r1;
            }
            Reg16::AFP => {
                self.ap = r0;
                self.fp = r1;
            }
            Reg16::BCP => {
                self.bp = r0;
                self.cp = r1;
            }
            Reg16::DEP => {
                self.dp = r0;
                self.ep = r1;
            }
            Reg16::HLP => {
                self.hp = r0;
                self.lp = r1;
            }
            Reg16::IX => self.ix = v,
            Reg16::IY => self.iy = v,
            Reg16::SP => self.sp = v,
        }
    }

    /// Get a 16-bit register. These are a little-endian combination of two 8-bit registers.
    pub fn get_reg16(&self, r: &Reg16) -> u16 {
        let (r0, r1) = match r {
            Reg16::AF => (self.a, self.f),
            Reg16::BC => (self.b, self.c),
            Reg16::DE => (self.d, self.e),
            Reg16::HL => (self.h, self.l),
            Reg16::AFP => (self.ap, self.fp),
            Reg16::BCP => (self.bp, self.cp),
            Reg16::DEP => (self.dp, self.ep),
            Reg16::HLP => (self.hp, self.lp),
            Reg16::IX => return self.ix,
            Reg16::IY => return self.iy,
            Reg16::SP => return self.sp,
        };
        (u16::from(r1) << 8) + u16::from(r0)
    }

    /// Get the current program counter
    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    /// Set the program counter
    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_flag() {
        let mut regs = Registers {
            f: 0b1010_1010,
            ..Default::default()
        };

        assert!(!regs.get_flag(&StatusFlag::Carry));
        assert!(regs.get_flag(&StatusFlag::AddSubtract));
        assert!(!regs.get_flag(&StatusFlag::ParityOverflow));
        assert!(!regs.get_flag(&StatusFlag::HalfCarry));
        assert!(!regs.get_flag(&StatusFlag::Zero));
        assert!(regs.get_flag(&StatusFlag::Sign));

        regs.f = 0b0101_0101;

        assert!(regs.get_flag(&StatusFlag::Carry));
        assert!(!regs.get_flag(&StatusFlag::AddSubtract));
        assert!(regs.get_flag(&StatusFlag::ParityOverflow));
        assert!(regs.get_flag(&StatusFlag::HalfCarry));
        assert!(regs.get_flag(&StatusFlag::Zero));
        assert!(!regs.get_flag(&StatusFlag::Sign));
    }

    #[test]
    fn set_flag() {
        let mut regs = Registers::default();

        regs.set_flag(&StatusFlag::Carry, true);
        assert_eq!("00000001", format!("{:08b}", regs.f));
        regs.set_flag(&StatusFlag::AddSubtract, false);
        regs.set_flag(&StatusFlag::ParityOverflow, true);
        regs.set_flag(&StatusFlag::HalfCarry, false);
        regs.set_flag(&StatusFlag::Zero, true);
        regs.set_flag(&StatusFlag::Sign, false);

        assert_eq!("01000101", format!("{:08b}", regs.f))
    }

    #[test]
    fn get_set_reg8() {
        let mut regs = Registers::default();
        regs.set_reg8(Reg8::A, 0x1);
        regs.set_reg8(Reg8::B, 0x2);
        regs.set_reg8(Reg8::C, 0x3);
        regs.set_reg8(Reg8::D, 0x4);
        regs.set_reg8(Reg8::E, 0x5);
        regs.set_reg8(Reg8::F, 0x6);
        regs.set_reg8(Reg8::H, 0x7);
        regs.set_reg8(Reg8::L, 0x8);

        regs.set_reg8(Reg8::AP, 0x21);
        regs.set_reg8(Reg8::BP, 0x22);
        regs.set_reg8(Reg8::CP, 0x23);
        regs.set_reg8(Reg8::DP, 0x24);
        regs.set_reg8(Reg8::EP, 0x25);
        regs.set_reg8(Reg8::FP, 0x26);
        regs.set_reg8(Reg8::HP, 0x27);
        regs.set_reg8(Reg8::LP, 0x28);

        assert_eq!(0x1, regs.get_reg8(Reg8::A));
        assert_eq!(0x2, regs.get_reg8(Reg8::B));
        assert_eq!(0x3, regs.get_reg8(Reg8::C));
        assert_eq!(0x4, regs.get_reg8(Reg8::D));
        assert_eq!(0x5, regs.get_reg8(Reg8::E));
        assert_eq!(0x6, regs.get_reg8(Reg8::F));
        assert_eq!(0x7, regs.get_reg8(Reg8::H));
        assert_eq!(0x8, regs.get_reg8(Reg8::L));

        assert_eq!(0x21, regs.get_reg8(Reg8::AP));
        assert_eq!(0x22, regs.get_reg8(Reg8::BP));
        assert_eq!(0x23, regs.get_reg8(Reg8::CP));
        assert_eq!(0x24, regs.get_reg8(Reg8::DP));
        assert_eq!(0x25, regs.get_reg8(Reg8::EP));
        assert_eq!(0x26, regs.get_reg8(Reg8::FP));
        assert_eq!(0x27, regs.get_reg8(Reg8::HP));
        assert_eq!(0x28, regs.get_reg8(Reg8::LP));
    }

    #[test]
    fn get_set_reg16() {
        let mut regs = Registers::default();
        regs.set_reg16(&Reg16::AF, 0x0601);
        regs.set_reg16(&Reg16::BC, 0x0302);
        regs.set_reg16(&Reg16::DE, 0x0504);
        regs.set_reg16(&Reg16::HL, 0x0807);
        regs.set_reg16(&Reg16::IX, 0xABBA);
        regs.set_reg16(&Reg16::IY, 0x0BB0);
        regs.set_reg16(&Reg16::SP, 0x0809);

        assert_eq!(0x0601, regs.get_reg16(&Reg16::AF));
        assert_eq!(0x0302, regs.get_reg16(&Reg16::BC));
        assert_eq!(0x0504, regs.get_reg16(&Reg16::DE));
        assert_eq!(0x0807, regs.get_reg16(&Reg16::HL));
        assert_eq!(0xABBA, regs.get_reg16(&Reg16::IX));
        assert_eq!(0x0BB0, regs.get_reg16(&Reg16::IY));
        assert_eq!(0x0809, regs.get_reg16(&Reg16::SP));

        regs.set_reg16(&Reg16::AFP, 0x2621);
        regs.set_reg16(&Reg16::BCP, 0x2322);
        regs.set_reg16(&Reg16::DEP, 0x2524);
        regs.set_reg16(&Reg16::HLP, 0x2827);

        assert_eq!(0x2621, regs.get_reg16(&Reg16::AFP));
        assert_eq!(0x2322, regs.get_reg16(&Reg16::BCP));
        assert_eq!(0x2524, regs.get_reg16(&Reg16::DEP));
        assert_eq!(0x2827, regs.get_reg16(&Reg16::HLP));
    }

    #[test]
    fn pc() {
        let mut regs = Registers::default();

        regs.set_pc(0x75);
        assert_eq!(0x75, regs.get_pc());

        regs.set_pc(0xF5);
        assert_eq!(0xF5, regs.get_pc());
    }
}
