///! This module provides the symbolic representation of all z80 instructions
///! You can construct these yourself, or you can parse binaries using `zeerust::cpu::opcodes`.

/// Op represents a single operation.
/// This representation (and backing implementation) is more expressive than
/// the processor itself.
/// For example `ADD8(Location8::Reg(Reg8::D), Location8::Immediate(10))` is a valid representation, but
/// the Z80 features no such instruction.
/// Usually executing an instruction like this will just work, but in some cases a panic will occur
/// (Such as attempting to store to an immediate, which doesn't make any sense).
/// It is probably best to stick to the "guide rails" of the Z80 operations.
#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    /// ADd including Carry
    ADC(Location8, Location8),
    /// ADD (8-bit)
    ADD8(Location8, Location8),
    /// INCrement
    INC(Location8),

    /// SuBtract including borrow (Carry bit)
    SBC(Location8, Location8),
    /// SUBtraction (8-bit)
    SUB8(Location8, Location8),
    /// DECrement
    DEC(Location8),

    /// bitwise AND
    AND(Location8),
    /// bitwise OR
    OR(Location8),
    /// bitwise XOR
    XOR(Location8),
    /// two's ComPliment
    CP(Location8),

    /// One's ComPLiment
    CPL, // One's Compliment
    /// sign NEGation (two's compliment)
    NEG, // Negation (two's compliment)
    /// toggle the Carry Flag
    CCF, // toggle carry flag
    /// Set the Carry Flag unconditionally
    SCF,

    /// Do nothing (No-OPeration)
    NOP,
    /// HALT execution (until woken)
    HALT, // End execution (until woken)

    /// BCD nonsense. Not implemented
    DAA,

    /// Rotate Accumulator Left, set Carry
    RLCA,
    /// Rotate Accumulator Left, through carry
    RLA,
    /// Rotate Accumulator Right, set Carry
    RRCA,
    /// Rotate Accumulator Left, through carry
    RRA,
    /// Rotate Left, set Carry
    RLC(Location8),
    /// Rotate Left, through carry
    RL(Location8),
    /// Rotate Right, set Carry
    RRC(Location8),
    /// Rotate Right, through carry
    RR(Location8),

    /// Shift Left
    SLA(Location8),
    /// Shift Right
    SRL(Location8),
    /// Shift Right, preserving 7th bit
    SRA(Location8),

    /// Rotate nibbles Left through accumulator
    RLD,
    /// Rotate nibbles Right through accumulator
    RRD,

    /// set zero flag if BIT is on
    BIT(u8, Location8),
    /// SET b bit in location
    SET(u8, Location8),
    /// RESet b bit in location
    RES(u8, Location8),

    /// INput from a peripheral
    IN(Location8, Location8),
    /// OUTput to a peripheral
    OUT(Location8, Location8),

    /// JumP to the given position
    JP(JumpConditional, Location16),
    /// Jump to the given Relative position
    JR(JumpConditional, i8),
    /// Decrement register b, then Jump if register b is Non Zero
    DJNZ(i8),
    /// CALL a method
    CALL(JumpConditional, u16),
    /// RETurn from a method call
    RET(JumpConditional),

    /// Pop an address off of the stack
    POP(Location16),
    /// Push an address onto a stack
    PUSH(Location16),
    /// LoaD the given address (8-bit)
    LD8(Location8, Location8),
    /// LoaD the given address (16-bit)
    LD16(Location16, Location16),
    // TODO
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
    // RETI,
    // RETN,
    // RST,
    // SLA,
    // SLL,
    // SL1,
    // SRA,
    // SRL,
}

/// 8 bit registers
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
    /// A'
    AP,
    /// F'
    FP,
    /// B'
    BP,
    /// C'
    CP,
    /// D'
    DP,
    /// E'
    EP,
    /// H'
    HP,
    /// L'
    LP,
}

/// 16-bit registers
#[derive(Debug, PartialEq, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    /// AF'
    AFP,
    /// BC'
    BCP,
    /// DE'
    DEP,
    /// HL'
    HLP,

    IX,
    IY,
    /// Stack Pointer
    SP,
}

/// Anywhere an 8-bit value could could come from or be stored to
#[derive(Debug, PartialEq, Clone)]
pub enum Location8 {
    /// A register
    Reg(Reg8),
    /// A location in memory, pointed to by a 16-bit register
    RegIndirect(Reg16),
    /// A location in memory, pointed to by a literal number
    ImmediateIndirect(u16),
    /// A literal number
    Immediate(u8),
}

/// Anywhere a 16-bit value could could come from or be stored to
#[derive(Debug, PartialEq, Clone)]
pub enum Location16 {
    /// A 16-bit combined register
    Reg(Reg16),
    // RegIndirect(Reg16), // Is this used anywhere?
    /// A location in memory, pointed to by a literal number
    ImmediateIndirect(u16),
    /// A literal number
    Immediate(u16),
}

/// Status Flags. Implemented in the Z80 as a bitfield on register F
#[derive(Debug, PartialEq, Clone)]
pub enum StatusFlag {
    /// Bit 0. Indicates carry or borrows from bit 7
    Carry,
    /// Bit 1. Usually 0 after addition, 1 after subtraction
    AddSubtract,
    /// Bit 2. Indicates overflow after arithmetic, or parity after bitwise operations
    /// Parity is set if the number of 1s in the number is even, otherwise it is reset
    ParityOverflow,
    // Bit 3 unused
    /// Bit 4. Indicates carry or borrows from bit 3
    HalfCarry,
    /// Bit 6. Set if result of an operation was zero
    Zero,
    /// Bit 7. Set if the 7th bit is 1 after an arithmatic operation, i.e. number is negative if considered as signed
    Sign,
}

/// Jumps and Returns can be conditional on certain flags being set
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum JumpConditional {
    /// Both Jump and Return have unconditional versions.
    /// Rather than a seperate Op, these will have this flag.
    /// It always evaluates to true
    Unconditional,
    /// True if the Zero flag is reset
    NonZero,
    /// True if the Zero flag is set
    Zero,
    /// True if the Carry flag is reset
    NoCarry,
    /// True if the Carry flag is set
    Carry,
    /// True if the ParityOverflow bit is reset
    ParityOdd,
    /// True if the ParityOverflow bit is set
    ParityEven,
    /// True if the Sign bit is reset
    SignPositive,
    /// True if the Sign bit is set
    SignNegative,
}
