use crate::audio::browser::BrowserAudioSource;
use crate::audio::mock::MockAudioSource;
use crate::audio::AudioSource;
use crate::display::mock::MockDisplay;
use crate::display::webgl::WebGLDisplay;
use crate::instruction::Instruction;
use crate::keyboard::browser::BrowserKeyboard;
use crate::keyboard::mock::MockKeyboard;
use crate::{Display, Keyboard, CPU};

// where in memory roms should start being read from
const ROM_START_ADDRESS: usize = 0x200;
const FONT_START_ADDRESS: usize = 0x0;

const FONT_MAP: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[allow(dead_code)]
pub enum Chip8Platform {
    BROWSER,
    //DESKTOP,
    MOCK,
}

pub struct Chip8Bus {
    pub memory: [u8; 4096],
    pub display: Box<dyn Display>,
    pub keyboard: Box<dyn Keyboard>,
    pub audio: Box<dyn AudioSource>,
}

impl Chip8Bus {
    fn new(platform: Chip8Platform) -> Self {
        match platform {
            Chip8Platform::BROWSER => Self {
                memory: [0; 4096],
                display: Box::new(WebGLDisplay::default()),
                keyboard: Box::new(BrowserKeyboard::default()),
                audio: Box::new(BrowserAudioSource::default()),
            },
            /*Chip8Platform::DESKTOP => Self {
                memory: [0; 4096],
                display: Box::new(WebGLDisplay::default()),
                keyboard: Box::new(BrowserKeyboard::default()),
            },*/
            Chip8Platform::MOCK => Self {
                memory: [0; 4096],
                display: Box::new(MockDisplay::default()),
                keyboard: Box::new(MockKeyboard::default()),
                audio: Box::new(MockAudioSource::default()),
            },
        }
    }
}

pub struct Chip8 {
    pub cpu: CPU,
    pub bus: Chip8Bus,
}

impl Chip8 {
    pub fn new(platform: Chip8Platform) -> Self {
        Self {
            cpu: CPU::default(),
            bus: Chip8Bus::new(platform),
        }
    }
}

impl Chip8 {
    pub fn reset(&mut self) {
        self.cpu.reset(&mut self.bus.display);
        self.load_font_into_memory();
    }

    pub fn load_rom_into_memory(&mut self, rom_bytes: &[u8]) {
        self.bus.memory[ROM_START_ADDRESS..ROM_START_ADDRESS + rom_bytes.len()]
            .copy_from_slice(&rom_bytes);
    }

    fn load_font_into_memory(&mut self) {
        self.bus.memory[FONT_START_ADDRESS..FONT_MAP.len()].copy_from_slice(&FONT_MAP);
    }

    fn fetch_instruction(&self) -> Instruction {
        let bytes = (self.bus.memory[self.cpu.program_counter as usize] as u16) << 8
            | (self.bus.memory[self.cpu.program_counter as usize + 1] as u16);

        Instruction::new(bytes)
    }

    pub fn fetch_and_execute_instruction(&mut self) {
        let instruction = self.fetch_instruction();
        self.cpu.execute_instruction(instruction, &mut self.bus);
    }
}
