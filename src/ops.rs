pub enum Op {
    ADC(Location8, Location8),  // Add including carry
    ADD8(Location8, Location8), // Add

    SBC(Location8, Location8),  // Subtract including borrow
    SUB8(Location8, Location8), // Subtract

    AND(Location8, Location8),
    OR(Location8, Location8),
    XOR(Location8, Location8),

    // BIT,
    // CALL,
    // CCF,
    // CP,
    // CPD,
    // CPDR,
    // CPI,
    // CPIR,
    // CPL,
    // DAA,
    // DEC,
    // DI,
    // DJNZ,
    // EI,
    // EX,
    // EXX,
    // HALT,
    // IM,
    // IN,
    // INC,
    // IND,
    // INDR,
    // INI,
    // INIR,
    // JP,
    // JR,
    LD8(Location8, Location8),
    // LDD,
    // LDDR,
    // LDI,
    // LDIR,
    // NEG,
    // NOP,
    // OTDR,
    // OTIR,
    // OUT,
    // OUTD,
    // OUTI,
    // POP,
    // PUSH,
    // RES,
    // RET,
    // RETI,
    // RETN,
    // RL,
    // RLA,
    // RLC,
    // RLCA,
    // RLD,
    // RR,
    // RRA,
    // RRC,
    // RRCA,
    // RRD,
    // RST,
    // SCF,
    // SET,
    // SLA,
    // SLL,
    // SL1,
    // SRA,
    // SRL,
}

pub enum Reg8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AP,
    FP,
    BP,
    CP,
    DP,
    EP,
    HP,
    LP,
}

pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    AFP,
    BCP,
    DEP,
    HLP,
}

pub enum Location8 {
    Reg(Reg8),
    RegIndirect(Reg16),
    Immediate(u8), // Indexed()
}

pub enum Location16 {
    Reg(Reg16),
    RegIndirect(Reg16),
    Immediate(u16),
}

pub enum StatusFlag {
    Carry,
    AddSubtract,
    ParityOverflow,
    HalfCarry,
    Zero,
    Sign,
}
