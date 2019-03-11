#[derive(Debug, PartialEq)]
pub enum Op {
    ADC(Location8, Location8),  // Add including carry
    ADD8(Location8, Location8), // Add
    INC(Location8),             // Add one

    SBC(Location8, Location8),  // Subtract including borrow
    SUB8(Location8, Location8), // Subtract
    CP(Location8),              // Subtract, only setting flags
    DEC(Location8),             // Decement

    AND(Location8, Location8),
    OR(Location8, Location8),
    XOR(Location8, Location8),
    CPL, // One's Compliment
    NEG, // Negation (two's compliment)
    CCF, // toggle carry flag
    SCF, // set carry flag unconditionally

    NOP, // Do nothing

    DAA, // BCD Nonsense. Not implemented.

    RLCA,           // Rotate Accumulator Left, set Carry
    RLA,            // Rotate Accumulator Left, through carry
    RRCA,           // Rotate Accumulator Right, set Carry
    RRA,            // Rotate Accumulator Left, through carry
    RLC(Location8), // Rotate Left, set Carry
    RL(Location8),  // Rotate Left, through carry
    RRC(Location8), // Rotate Right, set Carry
    RR(Location8),  // Rotate Right, through carry

    SLA(Location8), // Shift Left
    SRL(Location8), // Shift Right
    SRA(Location8), // Shift Right, preserving 7th bit

    RLD, // Rotate nibbles Left through accumulator
    RRD, // Rotate nibbles Right through accumulator

    BIT(u8, Location8), // set Zero flag if BIT is on
    SET(u8, Location8), // SET b bit in location
    RES(u8, Location8), // RESet b bit in location

    IN(Location8, u8),
    OUT(Location8, u8),
    // CALL,
    // CPD,
    // CPDR,
    // CPI,
    // CPIR,
    // DI,
    // DJNZ,
    // EI,
    // EX,
    // EXX,
    // HALT,
    // IM,
    // IN,
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
    // OTDR,
    // OTIR,
    // OUT,
    // OUTD,
    // OUTI,
    // POP,
    // PUSH,
    // RET,
    // RETI,
    // RETN,
    // RST,
    // SLA,
    // SLL,
    // SL1,
    // SRA,
    // SRL,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Location8 {
    Reg(Reg8),
    RegIndirect(Reg16),
    ImmediateIndirect(u16),
    Immediate(u8),
}

#[derive(Debug, PartialEq)]
pub enum Location16 {
    Reg(Reg16),
    RegIndirect(Reg16),
    Immediate(u16),
}

#[derive(Debug, PartialEq)]
pub enum StatusFlag {
    Carry,
    AddSubtract,
    ParityOverflow,
    HalfCarry,
    Zero,
    Sign,
}
