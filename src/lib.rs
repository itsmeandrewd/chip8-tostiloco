mod chip8;
mod cpu;
mod display;
mod instruction;
mod keyboard;

use crate::chip8::CHIP8;
use crate::cpu::CPU;
use crate::display::Display;
use log::Level;
use std::panic;
use wasm_bindgen::prelude::*;
use crate::keyboard::Keyboard;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

static mut EMULATOR: Option<CHIP8> = None;

pub fn init_logging() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("Error initializing log!");
}

#[wasm_bindgen]
pub fn boot_emulator() {
    init_logging();
    unsafe {
        let mut emulator = CHIP8::default();
        emulator.display.initialize();
        emulator.keyboard.initialize();
        EMULATOR = Some(emulator);
    }
}

#[wasm_bindgen]
pub fn key_down(key_code: u8) {
    unsafe {
        let emulator: &mut CHIP8 = EMULATOR.as_mut().unwrap();
        emulator.keyboard.set_key(key_code);
    }
}

#[wasm_bindgen]
pub fn key_up() {
    unsafe {
        let emulator: &mut CHIP8 = EMULATOR.as_mut().unwrap();
        emulator.keyboard.set_key(0);
    }
}

#[wasm_bindgen]
pub fn tick() {
    unsafe {
        let emulator: &mut CHIP8 = EMULATOR.as_mut().unwrap();
        emulator.fetch_and_execute_instruction();
    }
}

#[wasm_bindgen]
pub fn handle_timers() {
    unsafe {
        EMULATOR.as_mut().unwrap().cpu.handler_timers();
    }
}

#[wasm_bindgen]
pub fn load_rom(rom_bytes: &[u8]) {
    unsafe {
        let emulator = EMULATOR.as_mut().unwrap();
        emulator.reset();
        emulator.load_rom_into_memory(rom_bytes);
    }
}
