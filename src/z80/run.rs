extern crate log;
use log::debug;

use super::Z80;
use crate::cpu::opcodes;
use crate::ops::{Op, Reg16, Reg8};

impl<'a> Z80<'a> {
    /// Load a function into memory.
    /// This is done by mapping the provided bytes into memory, starting at 0x0000
    /// You only have 16 kibibytes to work with, so be careful!
    pub fn load(&mut self, program: &[u8]) {
        for (i, b) in program.iter().enumerate() {
            self.memory.memory[i] = *b
        }
    }

    /// Parse the CPU instruction at the given location.
    /// If the location exists in memory, return the opcode and opcode size in bytes
    /// Otherwise, return none.
    ///
    /// # Panics
    /// Panics if no valid opcode is found and the specified location
    pub fn parse_opcode(&self, location: usize) -> Option<(Op, usize)> {
        let mem = self.memory.memory;
        let byte = match mem.get(location) {
            Some(byte) => *byte,
            None => return None,
        };

        let opcode_horizon = [
            byte,
            mem.get(location + 1).map_or(0x00, |i| *i),
            mem.get(location + 2).map_or(0x00, |i| *i),
            mem.get(location + 3).map_or(0x00, |i| *i),
        ];
        Some(opcodes::opcode(opcode_horizon))
    }

    /// Execute a single instruction.
    /// The program counter will be updated to the new position, ready to call step again
    ///
    /// # Panics
    /// Panics if program counter is beyond the end of CPU memory
    pub fn step(&mut self) {
        let pc = self.registers.get_pc();
        let (opc, consumed) = self.parse_opcode(pc as usize).expect("out of memory range");
        debug!("Running {:?}", opc);
        debug!(
            "A: {:02x}, B: {:02x}, C: {:02x}, D: {:02x}, HL: {:04x}, F: {:08b}, PC: {:02x}",
            self.registers.get_reg8(Reg8::A),
            self.registers.get_reg8(Reg8::B),
            self.registers.get_reg8(Reg8::C),
            self.registers.get_reg8(Reg8::D),
            self.registers.get_reg16(&Reg16::HL),
            self.registers.get_reg8(Reg8::F),
            self.registers.get_pc(),
        );
        let pc = self
            .exec_with_offset(opc) //dbg!(opc))
            .unwrap_or(pc + consumed as u16);
        self.registers.set_pc(pc as u16)
    }

    /// Start executing.
    /// The program counter is set to 0x0000, and instructions are executed until a HALT is encountered.
    /// If the program does not contain a HALT, the emulator will simply continue until it runs out of memory.
    pub fn run(&mut self) {
        while !self.is_halted {
            self.step()
        }
    }
}
