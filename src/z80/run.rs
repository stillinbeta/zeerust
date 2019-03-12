use super::Z80;
use crate::cpu::opcodes;

impl<'a> Z80<'a> {
    pub fn run(&mut self, program: &[u8]) {
        while !self.is_halted {
            let pc = self.registers.get_pc();
            let opcode_horizon = [
                *program.get(pc as usize).expect("executed too far!"),
                program.get((pc+1) as usize).map_or(0x00, |i| *i),
                program.get((pc+2) as usize).map_or(0x00, |i| *i),
                program.get((pc+3) as usize).map_or(0x00, |i| *i),
            ];
            let (opc, consumed) = opcodes::opcode(opcode_horizon);
            self.exec(opc);
            self.registers.set_pc(pc + consumed as u16);
        }
    }
}
