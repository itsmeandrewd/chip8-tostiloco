use crate::instruction::Instruction;
use crate::screen_display::WebGLDisplay;
use crate::CPU;

// where in memory roms should start being read from
const ROM_START_ADDRESS: usize = 0x200;

pub struct CHIP8 {
    pub cpu: CPU,
    pub memory: [u8; 4096],
    pub(crate) display: WebGLDisplay,
}

impl Default for CHIP8 {
    fn default() -> Self {
        Self {
            cpu: CPU::default(),
            memory: [0; 4096],
            display: WebGLDisplay::default(),
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

    pub fn fetch_and_execute_instruction(&mut self) {
        let instruction = self.fetch_instruction();
        self.cpu.execute_instruction(instruction, &mut self.memory, &mut self.display);
    }
}
