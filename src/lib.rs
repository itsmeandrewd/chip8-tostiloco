mod chip8;
mod cpu;
mod instruction;
mod screen_display;

use crate::chip8::CHIP8;
use crate::cpu::CPU;
use log::Level;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

pub fn init_logging() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("Error initializing log!");
}

#[wasm_bindgen]
pub fn boot_emulator(rom_bytes: &[u8]) {
    init_logging();

    let mut chip8: CHIP8 = Default::default();
    chip8.cpu.display.init();
    chip8.load_rom_into_memory(rom_bytes);
    chip8.fetch_and_execute_instruction();
    chip8.fetch_and_execute_instruction();
    chip8.fetch_and_execute_instruction();
    chip8.fetch_and_execute_instruction();
    chip8.fetch_and_execute_instruction();
    chip8.fetch_and_execute_instruction();
    chip8.fetch_and_execute_instruction();
    chip8.fetch_and_execute_instruction();
    chip8.fetch_and_execute_instruction();
    chip8.fetch_and_execute_instruction();
    chip8.fetch_and_execute_instruction();
}
