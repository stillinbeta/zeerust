#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    ADC(Location8, Location8),  // Add including carry
    ADD8(Location8, Location8), // Add
    INC(Location8),             // Add one

    SBC(Location8, Location8),  // Subtract including borrow
    SUB8(Location8, Location8), // Subtract
    DEC(Location8),             // Decement

    AND(Location8),
    OR(Location8),
    XOR(Location8),
    CP(Location8), // Subtract, only setting flags

    CPL, // One's Compliment
    NEG, // Negation (two's compliment)
    CCF, // toggle carry flag
    SCF, // set carry flag unconditionally

    NOP,  // Do nothing
    HALT, // End execution (until woken)

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

    IN(Location8, Location8),  // INput from a peripheral
    OUT(Location8, Location8), // OUTput to a peripheral

    JP(JumpConditional, Location16), // JumP to the given position
    JR(JumpConditional, i8),         // Jump to the given Relative position
    DJNZ(i8),                        // Do a Jump if register b is Non Zero

    POP(Location16),           // Pop an address off of the stack
    PUSH(Location16),          // Push an address onto a stack
    LD8(Location8, Location8), // LoaD the given address
    LD16(Location16, Location16),
    // CALL,
    // CPD,
    // CPDR,
    // CPI,
    // CPIR,
    // DI,
    // EI,
    // EX,
    // EXX,
    // IM,
    // IN,
    // IND,
    // INDR,
    // INI,
    // INIR,
    // LDD,
    // LDDR,
    // LDI,
    // OTDR,
    // OTIR,
    // OUTD,
    // OUTI,
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

#[derive(Debug, PartialEq, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    AFP,
    BCP,
    DEP,
    HLP,

    IX,
    IY,
    SP,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Location8 {
    Reg(Reg8),
    RegIndirect(Reg16),
    ImmediateIndirect(u16),
    Immediate(u8),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Location16 {
    Reg(Reg16),
    // RegIndirect(Reg16), // Is this used anywhere?
    Immediate(u16),
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatusFlag {
    Carry,
    AddSubtract,
    ParityOverflow,
    HalfCarry,
    Zero,
    Sign,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum JumpConditional {
    Unconditional,
    NonZero,
    Zero,
    NoCarry,
    Carry,
    ParityOdd,
    ParityEven,
    SignPositive,
    SignNegative,
}
