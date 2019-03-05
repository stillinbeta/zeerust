use crate::ops::{Reg8, Reg16};

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
}

impl Registers {
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

    pub fn get_reg16(&self, r: Reg16) -> u16 {
        let (r0, r1) = match r {
            Reg16::AF => (self.a, self.f),
            Reg16::BC => (self.b, self.c),
            Reg16::DE => (self.d, self.e),
            Reg16::HL => (self.h, self.l),
            Reg16::AFP => (self.ap, self.fp),
            Reg16::BCP => (self.bp, self.cp),
            Reg16::DEP => (self.dp, self.ep),
            Reg16::HLP => (self.hp, self.lp),
        };
        ((r1 as u16) << 8) + (r0 as u16)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const REGS: Registers = Registers{
        a: 0x1,
        b: 0x2,
        c: 0x3,
        d: 0x4,
        e: 0x5,
        f: 0x6,
        h: 0x7,
        l: 0x8,

        ap: 0x21,
        bp: 0x22,
        cp: 0x23,
        dp: 0x24,
        ep: 0x25,
        fp: 0x26,
        hp: 0x27,
        lp: 0x28,
    };

    #[test]
    fn test_reg8() {
        assert_eq!(0x1, REGS.get_reg8(Reg8::A));
        assert_eq!(0x2, REGS.get_reg8(Reg8::B));
        assert_eq!(0x3, REGS.get_reg8(Reg8::C));
        assert_eq!(0x4, REGS.get_reg8(Reg8::D));
        assert_eq!(0x5, REGS.get_reg8(Reg8::E));
        assert_eq!(0x6, REGS.get_reg8(Reg8::F));
        assert_eq!(0x7, REGS.get_reg8(Reg8::H));
        assert_eq!(0x8, REGS.get_reg8(Reg8::L));

        assert_eq!(0x21, REGS.get_reg8(Reg8::AP));
        assert_eq!(0x22, REGS.get_reg8(Reg8::BP));
        assert_eq!(0x23, REGS.get_reg8(Reg8::CP));
        assert_eq!(0x24, REGS.get_reg8(Reg8::DP));
        assert_eq!(0x25, REGS.get_reg8(Reg8::EP));
        assert_eq!(0x26, REGS.get_reg8(Reg8::FP));
        assert_eq!(0x27, REGS.get_reg8(Reg8::HP));
        assert_eq!(0x28, REGS.get_reg8(Reg8::LP));
    }

    #[test]
    fn test_reg16() {
        assert_eq!(0x0601, REGS.get_reg16(Reg16::AF));
        assert_eq!(0x0302, REGS.get_reg16(Reg16::BC));
        assert_eq!(0x0504, REGS.get_reg16(Reg16::DE));
        assert_eq!(0x0807, REGS.get_reg16(Reg16::HL));

        assert_eq!(0x2621, REGS.get_reg16(Reg16::AFP));
        assert_eq!(0x2322, REGS.get_reg16(Reg16::BCP));
        assert_eq!(0x2524, REGS.get_reg16(Reg16::DEP));
        assert_eq!(0x2827, REGS.get_reg16(Reg16::HLP));

    }
}
