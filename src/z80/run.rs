extern crate log;
use log::debug;

use super::Z80;
use crate::cpu::opcodes;
use crate::ops::{Reg16, Reg8};

impl<'a> Z80<'a> {
    pub fn load(&mut self, program: &[u8]) {
        program
            .iter()
            .enumerate()
            .map(|(i, b)| self.memory.memory[i] = *b)
            .collect()
    }
    pub fn run(&mut self) {
        let mem = self.memory.memory;
        while !self.is_halted {
            let pc = self.registers.get_pc();
            let opcode_horizon = [
                *mem.get(pc as usize).expect("executed too far!"),
                mem.get((pc + 1) as usize).map_or(0x00, |i| *i),
                mem.get((pc + 2) as usize).map_or(0x00, |i| *i),
                mem.get((pc + 3) as usize).map_or(0x00, |i| *i),
            ];
            let (opc, consumed) = opcodes::opcode(opcode_horizon);
            debug!("Running {:?}", opc);
            debug!(
                "A: {:02x}, B: {:02}, C: {:02x}, D: {:02x}, HL: {:04x}, F: {:08b}, PC: {:02x}",
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
            self.registers.set_pc(pc as u16);
        }
    }
}
