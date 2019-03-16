use super::Z80;
use crate::ops::{JumpConditional, Location16, Location8, Op, Reg16, Reg8, StatusFlag};

#[test]
fn get_loc8() {
    let mut z80 = Z80::default();
    z80.registers.set_reg8(Reg8::A, 0xC5);
    z80.registers.set_reg8(Reg8::H, 0xAA);
    z80.registers.set_reg8(Reg8::L, 0x0F);
    z80.memory.memory[0x0FAA] = 0xD1;
    z80.memory.memory[0x0DCC] = 0x75;

    assert_hex!(0xC5, z80.get_loc8(&Location8::Reg(Reg8::A)));
    assert_hex!(0xD1, z80.get_loc8(&Location8::RegIndirect(Reg16::HL)));
    assert_hex!(0xCC, z80.get_loc8(&Location8::Immediate(0xCC)));
    assert_hex!(0x75, z80.get_loc8(&Location8::ImmediateIndirect(0x0DCC)));
}

#[test]
fn get_loc16() {
    let mut z80 = Z80::default();
    z80.registers.set_reg8(Reg8::H, 0xCC);
    z80.registers.set_reg8(Reg8::L, 0x0D);

    assert_hex!(0x0DCC, z80.get_loc16(&Location16::Reg(Reg16::HL)));
    assert_hex!(0xF0C5, z80.get_loc16(&Location16::Immediate(0xF0C5)));

    z80.memory.memory[0x0545] = 0x37;
    z80.memory.memory[0x0546] = 0xA1;
    assert_hex!(
        0xa137,
        z80.get_loc16(&Location16::ImmediateIndirect(0x0545))
    );
}

#[test]
#[should_panic]
fn get_loc8_segfault() {
    let mut z80 = Z80::default();
    z80.registers.set_reg8(Reg8::H, 0xFF);
    z80.registers.set_reg8(Reg8::L, 0xFF);
    z80.get_loc8(&Location8::RegIndirect(Reg16::HL));
}

#[test]
#[should_panic]
fn set_loc8_immediate_panic() {
    let mut z80 = Z80::default();
    z80.set_loc8(&Location8::Immediate(0x00), 0x00);
}

#[test]
fn set_loc8() {
    let mut z80 = Z80::default();
    z80.set_loc8(&Location8::Reg(Reg8::A), 0xDD);
    assert_hex!(0xDD, z80.registers.get_reg8(Reg8::A));

    z80.registers.set_reg8(Reg8::H, 0x11);
    z80.registers.set_reg8(Reg8::L, 0x0A);

    z80.set_loc8(&Location8::RegIndirect(Reg16::HL), 0xEE);
    assert_hex!(0xEE, z80.memory.memory[0x0A11]);

    z80.set_loc8(&Location8::ImmediateIndirect(0x0C22), 0xF5);
    assert_hex!(0xF5, z80.memory.memory[0x0C22]);
}

#[test]
#[should_panic]
fn set_loc16_immediate_panic() {
    let mut z80 = Z80::default();
    z80.set_loc16(&Location16::Immediate(0x0000), 0x00);
}

#[test]
fn set_loc16() {
    let mut z80 = Z80::default();
    z80.set_loc16(&Location16::Reg(Reg16::DE), 0xDDEE);
    assert_hex!(0xDDEE, z80.registers.get_reg16(&Reg16::DE));

    z80.set_loc16(&Location16::ImmediateIndirect(0x1000), 0x4644);
    assert_hex!(0x44, z80.memory.memory[0x1000]);
    assert_hex!(0x46, z80.memory.memory[0x1001]);
}

#[test]
fn ld8_op() {
    let mut z80 = Z80::default();
    z80.exec(Op::LD8(Location8::Reg(Reg8::A), Location8::Immediate(0xF5)));
    assert_hex!(0xF5, z80.registers.get_reg8(Reg8::A))
}

#[test]
fn ld16_op() {
    let mut z80 = Z80::default();
    z80.exec(Op::LD16(
        Location16::Reg(Reg16::SP),
        Location16::Immediate(0xF5C5),
    ));
    assert_hex!(0xF5C5, z80.registers.get_reg16(&Reg16::SP));
    z80.memory.memory[0x2130] = 0x65;
    z80.memory.memory[0x2131] = 0x78;

    z80.exec(Op::LD16(
        Location16::Reg(Reg16::BC),
        Location16::ImmediateIndirect(0x2130),
    ));
    assert_hex!(0x7865, z80.registers.get_reg16(&Reg16::BC));
}

#[test]
fn push_op() {
    let mut z80 = Z80::default();
    z80.registers.set_reg16(&Reg16::AF, 0x2233);
    z80.registers.set_reg16(&Reg16::SP, 0x1007);
    z80.exec(Op::PUSH(Location16::Reg(Reg16::AF)));
    assert_hex!(0x22, z80.memory.memory[0x1006]);
    assert_hex!(0x33, z80.memory.memory[0x1005]);
    assert_hex!(0x1005, z80.registers.get_reg16(&Reg16::SP));
}

#[test]
fn pop_op() {
    let mut z80 = Z80::default();
    z80.registers.set_reg16(&Reg16::SP, 0x1000);
    z80.memory.memory[0x1000] = 0x55;
    z80.memory.memory[0x1001] = 0x33;
    z80.exec(Op::POP(Location16::Reg(Reg16::HL)));
    assert_hex!(0x3355, z80.registers.get_reg16(&Reg16::HL));
    assert_hex!(0x1002, z80.registers.get_reg16(&Reg16::SP));
}

#[test]
fn add8_op() {
    let mut z80 = Z80::default();
    z80.registers.set_reg8(Reg8::A, 0x64);
    z80.exec(Op::ADD8(
        Location8::Reg(Reg8::A),
        Location8::Immediate(0x44),
    ));
    assert_hex!(0xA8, z80.registers.get_reg8(Reg8::A));
    assert_flags!(
        z80.registers,
        Sign = true,
        Zero = false,
        HalfCarry = true,
        ParityOverflow = false,
        AddSubtract = false,
        Carry = true,
    );

    z80.registers.set_reg8(Reg8::A, 0xFF);
    z80.exec(Op::ADD8(
        Location8::Reg(Reg8::A),
        Location8::Immediate(0x01),
    ));
    assert_hex!(0x00, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::H, 0xCC);
    z80.registers.set_reg8(Reg8::L, 0x20);
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
    z80.registers.set_reg8(Reg8::A, 0x64);
    z80.registers.set_flag(&StatusFlag::Carry, true);
    z80.exec(Op::ADC(Location8::Reg(Reg8::A), Location8::Immediate(0x44)));
    assert_hex!(0xA9, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::A, 0b1010_0000);
    z80.exec(Op::SUB8(
        Location8::Reg(Reg8::A),
        Location8::Immediate(0b0100_0100),
    ));
    assert_bin!(0b0101_1100, z80.registers.get_reg8(Reg8::A));
    assert_flags!(
        z80.registers,
        Sign = false,
        Zero = false,
        HalfCarry = true,
        ParityOverflow = false,
        AddSubtract = true,
        Carry = true,
    );

    z80.registers.set_reg8(Reg8::A, 0x0A); // 10
    z80.exec(Op::SUB8(
        Location8::Reg(Reg8::A),
        Location8::Immediate(0x0B), // 11
    ));
    assert_bin!((0xFF) as u8, z80.registers.get_reg8(Reg8::A)); // -1
    assert_flags!(
        z80.registers,
        Sign = true,
        Zero = false,
        HalfCarry = true,
        ParityOverflow = true,
        AddSubtract = true,
        Carry = true,
    );
}

#[test]
fn cp_op() {
    let mut z80 = Z80::default();
    z80.registers.set_reg8(Reg8::A, 0b1010_0000);
    z80.exec(Op::CP(Location8::Immediate(0b0100_0100)));
    assert_bin!(0b1010_0000, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::A, 1);
    z80.exec(Op::SBC(Location8::Reg(Reg8::A), Location8::Immediate(0)));
    assert_bin!(0, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::A, 0b1010_0000);
    z80.exec(Op::DEC(Location8::Reg(Reg8::A)));
    assert_bin!(0b1001_1111, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::A, 0b1001_1000);
    z80.exec(Op::AND(Location8::Immediate(0b0000_0000)));
    assert_bin!(0b0000_0000, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::A, 0b1001_1000);
    z80.exec(Op::OR(Location8::Immediate(0b0001_1011)));
    assert_bin!(0b1001_1011, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::A, 0b0011_1100);
    z80.exec(Op::XOR(Location8::Immediate(0b0001_1011)));
    assert_bin!(0b0010_0111, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::A, 0b1011_1101);
    z80.exec(Op::CPL);
    assert_bin!(0b0100_0010, z80.registers.get_reg8(Reg8::A));
    assert_flags!(z80.registers, HalfCarry = true, AddSubtract = true,);
}

#[test]
fn neg_op_default() {
    let mut z80 = Z80::default();
    // Sign is positive
    z80.registers.set_reg8(Reg8::A, 0b1001_1000);
    z80.exec(Op::NEG);
    assert_bin!(0b0110_1000, z80.registers.get_reg8(Reg8::A));
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
fn neg_op_sign_negative() {
    let mut z80 = Z80::default();
    // Sign is negative
    z80.registers.set_reg8(Reg8::A, 0b0001_1000);
    z80.exec(Op::NEG);
    assert_bin!(0b1110_1000, z80.registers.get_reg8(Reg8::A));
    assert_flags!(
        z80.registers,
        Sign = true,
        Zero = false,
        HalfCarry = true,
        ParityOverflow = false,
        AddSubtract = true,
        Carry = true,
    );
}
#[test]
fn neg_0x80() {
    let mut z80 = Z80::default();
    z80.registers.set_reg8(Reg8::A, 0x80);
    z80.exec(Op::NEG);
    // TODO: not 100% on 2's compliment of 0x80
    assert_bin!(0b1000_0000, z80.registers.get_reg8(Reg8::A));
    assert_flags!(
        z80.registers,
        Sign = true,
        Zero = false,
        HalfCarry = false,
        ParityOverflow = true,
        AddSubtract = true,
        Carry = true,
    );
}

#[test]
fn neg_zero() {
    let mut z80 = Z80::default();
    // A was 0x00
    z80.registers.set_reg8(Reg8::A, 0b0000_0000);
    z80.exec(Op::NEG);
    assert_bin!(0b0000_0000, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::A, 0b0101_1011);
    z80.exec(Op::RLCA);
    assert_bin!(0b1011_0110, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::A, 0b1001_1011);
    z80.exec(Op::RLA);
    assert_bin!(0b0011_0111, z80.registers.get_reg8(Reg8::A));
    assert_flags!(
        z80.registers,
        HalfCarry = false,
        AddSubtract = false,
        Carry = true,
    );

    z80.registers.set_flag(&StatusFlag::Carry, false);
    z80.registers.set_reg8(Reg8::A, 0b0001_1001);
    z80.exec(Op::RLA);
    assert_bin!(0b0011_0010, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::A, 0b0110_1001);
    z80.exec(Op::RRCA);
    assert_bin!(0b1011_0100, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::A, 0b0101_1100);
    z80.exec(Op::RRA);
    assert_bin!(0b1010_1110, z80.registers.get_reg8(Reg8::A));
    assert_flags!(
        z80.registers,
        HalfCarry = false,
        AddSubtract = false,
        Carry = false,
    );

    z80.registers.set_flag(&StatusFlag::Carry, false);
    z80.registers.set_reg8(Reg8::A, 0b1010_1011);
    z80.exec(Op::RRA);
    assert_bin!(0b0101_0101, z80.registers.get_reg8(Reg8::A));

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
    z80.registers.set_reg8(Reg8::B, 0b1111_0000);
    z80.exec(Op::RLC(Location8::Reg(Reg8::B)));
    assert_bin!(0b1110_0001, z80.registers.get_reg8(Reg8::B));

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
    z80.registers.set_reg8(Reg8::B, 0b1000_0000);
    z80.exec(Op::RL(Location8::Reg(Reg8::B)));
    assert_bin!(0b0000_0000, z80.registers.get_reg8(Reg8::B));
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
    z80.registers.set_reg8(Reg8::B, 0b1000_1011);
    z80.exec(Op::RRC(Location8::Reg(Reg8::B)));
    assert_bin!(0b1100_0101, z80.registers.get_reg8(Reg8::B));
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
    z80.registers.set_reg8(Reg8::B, 0b1110_1110);
    z80.exec(Op::RR(Location8::Reg(Reg8::B)));
    assert_bin!(0b1111_0111, z80.registers.get_reg8(Reg8::B));

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
    z80.registers.set_reg8(Reg8::C, 0b0110_0001);
    z80.exec(Op::SRL(Location8::Reg(Reg8::C)));
    assert_bin!(0b0011_0000, z80.registers.get_reg8(Reg8::C));
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
    z80.registers.set_reg8(Reg8::D, 0b1000_0000);
    z80.exec(Op::SLA(Location8::Reg(Reg8::D)));
    assert_bin!(0b0000_0000, z80.registers.get_reg8(Reg8::D));
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
    z80.registers.set_reg8(Reg8::C, 0b1100_1100);
    z80.exec(Op::SRA(Location8::Reg(Reg8::C)));
    assert_bin!(0b1110_0110, z80.registers.get_reg8(Reg8::C));
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
    z80.registers.set_reg8(Reg8::H, 0xCC);
    z80.registers.set_reg8(Reg8::L, 0x20);
    z80.registers.set_reg8(Reg8::A, 0b0111_1010);
    z80.memory.memory[0x20CC] = 0b0011_0001;

    z80.exec(Op::RLD);

    assert_bin!(0b0111_0011, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::H, 0xCC);
    z80.registers.set_reg8(Reg8::L, 0x20);
    z80.registers.set_reg8(Reg8::A, 0b0000_1010);
    z80.memory.memory[0x20CC] = 0b0000_1110;

    z80.exec(Op::RLD);

    assert_bin!(0b0000_0000, z80.registers.get_reg8(Reg8::A));
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
    z80.registers.set_reg8(Reg8::H, 0xCC);
    z80.registers.set_reg8(Reg8::L, 0x20);
    z80.registers.set_reg8(Reg8::A, 0b1000_0100);
    z80.memory.memory[0x20CC] = 0b0010_0000;

    z80.exec(Op::RRD);

    assert_bin!(0b1000_0000, z80.registers.get_reg8(Reg8::A));
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

#[test]
fn bit_op() {
    let mut z80 = Z80::default();
    z80.registers.set_reg8(Reg8::A, 0b1100_0001);

    let expected = [false, true, true, true, true, true, false, false];
    for (i, expect) in expected.iter().enumerate() {
        z80.exec(Op::BIT(i as u8, Location8::Reg(Reg8::A)));
        assert_flags!(
            z80.registers,
            Zero = *expect,
            HalfCarry = true,
            AddSubtract = false,
        );
    }
}

#[test]
#[should_panic]
fn bit_op_too_big() {
    let mut z80 = Z80::default();
    z80.exec(Op::BIT(8, Location8::Reg(Reg8::A)));
}

#[test]
fn set_op() {
    let mut z80 = Z80::default();
    z80.registers.set_reg8(Reg8::E, 0b1011_0110);

    z80.exec(Op::SET(0, Location8::Reg(Reg8::E)));
    assert_bin!(0b1011_0111, z80.registers.get_reg8(Reg8::E));
    z80.exec(Op::SET(3, Location8::Reg(Reg8::E)));
    assert_bin!(0b1011_1111, z80.registers.get_reg8(Reg8::E));
    z80.exec(Op::SET(6, Location8::Reg(Reg8::E)));
    assert_bin!(0b1111_1111, z80.registers.get_reg8(Reg8::E));
    z80.exec(Op::SET(7, Location8::Reg(Reg8::E)));
    assert_bin!(0b1111_1111, z80.registers.get_reg8(Reg8::E));
}

#[test]
#[should_panic]
fn set_op_too_big() {
    let mut z80 = Z80::default();
    z80.exec(Op::SET(8, Location8::Reg(Reg8::A)));
}

#[test]
fn res_op() {
    let mut z80 = Z80::default();
    z80.registers.set_reg8(Reg8::D, 0b1101_1001);
    z80.exec(Op::RES(0, Location8::Reg(Reg8::D)));
    assert_bin!(0b1101_1000, z80.registers.get_reg8(Reg8::D));
    z80.exec(Op::RES(3, Location8::Reg(Reg8::D)));
    assert_bin!(0b1101_0000, z80.registers.get_reg8(Reg8::D));
    z80.exec(Op::RES(4, Location8::Reg(Reg8::D)));
    assert_bin!(0b1100_0000, z80.registers.get_reg8(Reg8::D));
    z80.exec(Op::RES(5, Location8::Reg(Reg8::D)));
    assert_bin!(0b1100_0000, z80.registers.get_reg8(Reg8::D));
    z80.exec(Op::RES(6, Location8::Reg(Reg8::D)));
    assert_bin!(0b1000_0000, z80.registers.get_reg8(Reg8::D));
    z80.exec(Op::RES(7, Location8::Reg(Reg8::D)));
    assert_bin!(0b0000_0000, z80.registers.get_reg8(Reg8::D));
}

#[test]
#[should_panic]
fn res_op_too_big() {
    let mut z80 = Z80::default();
    z80.exec(Op::RES(8, Location8::Reg(Reg8::A)));
}

#[test]
fn in_op() {
    let mut z80 = Z80::default();
    let buf1 = super::io::BufInput::new(vec![0xF8, 0x33]);
    let buf2 = super::io::BufInput::new(vec![0xBB, 0xB7]);

    z80.registers.set_reg8(Reg8::C, 0x05);

    z80.install_input(0x00, &buf1);
    z80.install_input(0x05, &buf2);

    z80.exec(Op::IN(Location8::Reg(Reg8::A), Location8::Immediate(0x00)));
    assert_hex!(0x33, z80.registers.get_reg8(Reg8::A));
    z80.exec(Op::IN(Location8::Reg(Reg8::A), Location8::Immediate(0x05)));
    assert_hex!(0xB7, z80.registers.get_reg8(Reg8::A));

    z80.exec(Op::IN(Location8::Reg(Reg8::A), Location8::Immediate(0x00)));
    assert_hex!(0xF8, z80.registers.get_reg8(Reg8::A));
    z80.exec(Op::IN(Location8::Reg(Reg8::A), Location8::Reg(Reg8::C)));
    assert_hex!(0xBB, z80.registers.get_reg8(Reg8::A));
}

#[test]
#[should_panic(expected = "no peripheral installed in 0x00")]
fn in_no_device_installed() {
    let mut z80 = Z80::default();
    z80.exec(Op::IN(Location8::Reg(Reg8::A), Location8::Immediate(0x00)));
}

#[test]
fn out_op() {
    let mut z80 = Z80::default();
    let buf1 = super::io::BufOutput::default();
    let buf2 = super::io::BufOutput::default();
    z80.registers.set_reg8(Reg8::C, 0x05);

    z80.install_output(0x00, &buf1);
    z80.install_output(0x05, &buf2);

    z80.registers.set_reg8(Reg8::A, 0xFD);
    z80.registers.set_reg8(Reg8::B, 0x69);

    z80.exec(Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x05)));
    z80.exec(Op::OUT(Location8::Reg(Reg8::B), Location8::Immediate(0x00)));

    z80.registers.set_reg8(Reg8::A, 0x73);
    z80.registers.set_reg8(Reg8::B, 0x5C);

    z80.exec(Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)));
    z80.exec(Op::OUT(Location8::Reg(Reg8::B), Location8::Reg(Reg8::C)));

    assert_eq!(vec!(0x69, 0x73), buf1.result());
    assert_eq!(vec!(0xFD, 0x5C), buf2.result());
}

#[test]
#[should_panic(expected = "no peripheral installed in 0x00")]
fn out_no_device_installed() {
    let mut z80 = Z80::default();
    z80.exec(Op::OUT(Location8::Reg(Reg8::A), Location8::Immediate(0x00)));
}

#[test]
fn halt() {
    let mut z80 = Z80::default();
    z80.exec(Op::HALT);
    assert!(z80.is_halted);
}

#[test]
fn jp() {
    let mut z80 = Z80::default();
    assert_eq!(
        Some(0x0CFF),
        z80.exec_with_offset(Op::JP(
            JumpConditional::Unconditional,
            Location16::Immediate(0x0CFF)
        )),
    );

    z80.set_loc16(&Location16::Reg(Reg16::HL), 0xABBA); // Dancing queen
    assert_eq!(
        Some(0xABBA),
        z80.exec_with_offset(Op::JP(
            JumpConditional::Unconditional,
            Location16::Reg(Reg16::HL)
        )),
    )
}

#[test]
fn jp_conditional() {
    let mut z80 = Z80::default();

    z80.registers.set_flag(&StatusFlag::Zero, true);
    let op1 = Op::JP(JumpConditional::Zero, Location16::Immediate(0x0CFF));
    let op2 = Op::JP(JumpConditional::NonZero, Location16::Immediate(0x0CDD));
    assert_eq!(Some(0x0CFF), z80.exec_with_offset(op1.clone()));
    assert_eq!(None, z80.exec_with_offset(op2.clone()));
    z80.registers.set_flag(&StatusFlag::Zero, false);
    assert_eq!(None, z80.exec_with_offset(op1));
    assert_eq!(Some(0x0CDD), z80.exec_with_offset(op2));

    z80.registers.set_flag(&StatusFlag::Carry, true);
    let op1 = Op::JP(JumpConditional::Carry, Location16::Immediate(0x0CFF));
    let op2 = Op::JP(JumpConditional::NoCarry, Location16::Immediate(0x0CDD));
    assert_eq!(Some(0x0CFF), z80.exec_with_offset(op1.clone()));
    assert_eq!(None, z80.exec_with_offset(op2.clone()));
    z80.registers.set_flag(&StatusFlag::Carry, false);
    assert_eq!(None, z80.exec_with_offset(op1));
    assert_eq!(Some(0x0CDD), z80.exec_with_offset(op2));

    z80.registers.set_flag(&StatusFlag::ParityOverflow, true);
    let op1 = Op::JP(JumpConditional::ParityEven, Location16::Immediate(0x0CDD));
    let op2 = Op::JP(JumpConditional::ParityOdd, Location16::Immediate(0x0CFF));
    assert_eq!(Some(0x0CDD), z80.exec_with_offset(op1.clone()));
    assert_eq!(None, z80.exec_with_offset(op2.clone()));
    z80.registers.set_flag(&StatusFlag::ParityOverflow, false);
    assert_eq!(None, z80.exec_with_offset(op1));
    assert_eq!(Some(0x0CFF), z80.exec_with_offset(op2));

    z80.registers.set_flag(&StatusFlag::Sign, true);
    let op1 = Op::JP(JumpConditional::SignNegative, Location16::Immediate(0x0CFF));
    let op2 = Op::JP(JumpConditional::SignPositive, Location16::Immediate(0x0CDD));
    assert_eq!(Some(0x0CFF), z80.exec_with_offset(op1.clone()));
    assert_eq!(None, z80.exec_with_offset(op2.clone()));
    z80.registers.set_flag(&StatusFlag::Sign, false);
    assert_eq!(None, z80.exec_with_offset(op1));
    assert_eq!(Some(0x0CDD), z80.exec_with_offset(op2));
}

#[test]
fn jr() {
    let mut z80 = Z80::default();
    z80.registers.set_pc(0xA123);

    assert_eq!(
        Some(0xA123 + 129),
        z80.exec_with_offset(Op::JR(JumpConditional::Unconditional, 127))
    );

    z80.registers.set_flag(&StatusFlag::Zero, true);
    let op1 = Op::JR(JumpConditional::Zero, -128);
    let op2 = Op::JR(JumpConditional::NonZero, 127);
    assert_eq!(Some(0xA123 - 126), z80.exec_with_offset(op1.clone()));
    assert_eq!(None, z80.exec_with_offset(op2.clone()));
    z80.registers.set_flag(&StatusFlag::Zero, false);
    assert_eq!(None, z80.exec_with_offset(op1));
    assert_eq!(Some(0xA123 + 129), z80.exec_with_offset(op2));

    z80.registers.set_flag(&StatusFlag::Carry, true);
    let op1 = Op::JR(JumpConditional::Carry, 15);
    let op2 = Op::JR(JumpConditional::NoCarry, -100);
    assert_eq!(Some(0xA123 + 17), z80.exec_with_offset(op1.clone()));
    assert_eq!(None, z80.exec_with_offset(op2.clone()));
    z80.registers.set_flag(&StatusFlag::Carry, false);
    assert_eq!(None, z80.exec_with_offset(op1));
    assert_eq!(Some(0xA123 - 98), z80.exec_with_offset(op2));
}

#[test]
fn djnz() {
    let mut z80 = Z80::default();
    z80.set_loc8(&Location8::Reg(Reg8::B), 2);
    z80.registers.set_pc(0xAB50);

    assert_eq!(Some(0xAB00), z80.exec_with_offset(Op::DJNZ(-82)));
    assert_eq!(None, z80.exec_with_offset(Op::DJNZ(-52)));
    assert_eq!(0, z80.registers.get_reg8(Reg8::B));

    // Don't underflow when we add our offset
    z80.set_loc8(&Location8::Reg(Reg8::B), 2);
    z80.registers.set_pc(0x0010);
    assert_eq!(Some(0x0001), z80.exec_with_offset(Op::DJNZ(-17)));
    assert_eq!(1, z80.registers.get_reg8(Reg8::B));
}

#[test]
fn call() {
    let mut z80 = Z80::default();
    z80.registers.set_pc(0x1A47);
    z80.registers.set_reg16(&Reg16::SP, 0x3002);
    assert_eq!(
        Some(0x2135),
        z80.exec_with_offset(Op::CALL(JumpConditional::Unconditional, 0x2135)),
    );

    assert_eq!(0x4A, z80.memory.memory[0x3000]);
    assert_eq!(0x1A, z80.memory.memory[0x3001]);
    assert_eq!(0x3000, z80.registers.get_reg16(&Reg16::SP));
}

#[test]
fn call_cond_zero() {
    let mut z80 = Z80::default();
    z80.registers.set_pc(0x1A47);
    z80.registers.set_reg16(&Reg16::SP, 0x3002);

    z80.registers.set_flag(&StatusFlag::Zero, true);

    let op1 = Op::CALL(JumpConditional::Zero, 0x2135);
    let op2 = Op::CALL(JumpConditional::NonZero, 0x2135);
    assert_eq!(0x3002, z80.registers.get_reg16(&Reg16::SP));
    assert_eq!(None, z80.exec_with_offset(op2));
    assert_eq!(Some(0x2135), z80.exec_with_offset(op1));

    assert_eq!(0x4A, z80.memory.memory[0x3000]);
    assert_eq!(0x1A, z80.memory.memory[0x3001]);
    assert_eq!(0x3000, z80.registers.get_reg16(&Reg16::SP));

    // Not testing the other states, well covered by the JP tests
}

#[test]
fn ret() {
    let mut z80 = Z80::default();
    z80.registers.set_pc(0x3535);
    z80.registers.set_reg16(&Reg16::SP, 0x2000);
    z80.memory.memory[0x2000] = 0xB5;
    z80.memory.memory[0x2001] = 0x18;
    assert_eq!(
        Some(0x18B5),
        z80.exec_with_offset(Op::RET(JumpConditional::Unconditional)),
    );
    assert_eq!(0x2002, z80.registers.get_reg16(&Reg16::SP));
}

#[test]
fn call_cond_carry() {
    let mut z80 = Z80::default();
    z80.registers.set_pc(0x1A47);
    z80.registers.set_reg16(&Reg16::SP, 0x2000);

    z80.registers.set_flag(&StatusFlag::Carry, true);
    z80.memory.memory[0x2000] = 0xB5;
    z80.memory.memory[0x2001] = 0x18;

    let op1 = Op::RET(JumpConditional::Carry);
    let op2 = Op::RET(JumpConditional::NoCarry);
    assert_eq!(None, z80.exec_with_offset(op2));
    assert_eq!(0x2000, z80.registers.get_reg16(&Reg16::SP));

    assert_eq!(Some(0x18B5), z80.exec_with_offset(op1));
    assert_eq!(0x2002, z80.registers.get_reg16(&Reg16::SP));

    // Not testing the other states, well covered by the JP tests
}
