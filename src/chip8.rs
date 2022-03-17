use crate::instruction::Instruction;
use crate::CPU;

// where in memory roms should start being read from
const ROM_START_ADDRESS: usize = 0x200;

pub struct CHIP8 {
    pub cpu: CPU,
    pub memory: [u8; 4096],
}

impl Default for CHIP8 {
    fn default() -> Self {
        Self {
            cpu: CPU::default(),
            memory: [0; 4096],
        }
    }
}

impl CHIP8 {
    pub(crate) fn load_rom_into_memory(&mut self, rom_bytes: &[u8]) {
        self.memory[ROM_START_ADDRESS..ROM_START_ADDRESS + rom_bytes.len()]
            .copy_from_slice(&rom_bytes);
    }

    fn fetch_instruction(&self) -> Instruction {
        let bytes = (self.memory[self.cpu.program_counter] as u16) << 8
            | (self.memory[self.cpu.program_counter + 1] as u16);

        Instruction::new(bytes)
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.first {
            0x0 => match instruction.kk {
                0xe0 => self.cpu.cls(),
                _ => self.unknown_instruction(&instruction),
            },
            0x1 => self.cpu.jp(instruction.nnn),
            0x3 => self.cpu.se_vx(instruction.x, instruction.kk),
            0x6 => self.cpu.ld_vx(instruction.x, instruction.kk),
            0x7 => self.cpu.add_vx(instruction.x, instruction.kk),
            0xa => self.cpu.ld_i(instruction.nnn),
            0xd => self.cpu.drw(
                instruction.x,
                instruction.y,
                instruction.n as usize,
                &self.memory,
            ),
            _ => self.unknown_instruction(&instruction),
        }

        if instruction.first != 0x2 && instruction.first != 0x1 {
            // dont move the pc with JP or CALL instructions
            self.cpu.program_counter += 2;
        }
    }

    fn unknown_instruction(&self, instruction: &Instruction) {
        panic!(
            "Encountered unknown instruction {:#02x}",
            instruction.raw_bytes
        );
    }

    pub fn fetch_and_execute_instruction(&mut self) {
        let instruction = self.fetch_instruction();
        self.execute_instruction(instruction);
    }
}
