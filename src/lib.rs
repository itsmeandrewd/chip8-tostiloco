mod chip8;
mod cpu;
mod display;
mod instruction;
mod keyboard;
mod audio;

use crate::chip8::{Chip8, Chip8Platform};
use crate::cpu::CPU;
use crate::display::Display;
use crate::keyboard::Keyboard;
use log::Level;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

static mut EMULATOR: Option<Chip8> = None;

pub fn init_logging() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("Error initializing log!");
}

#[wasm_bindgen]
pub fn boot_emulator() {
    init_logging();
    unsafe {
        let mut emulator = Chip8::new(Chip8Platform::BROWSER);
        emulator.bus.display.initialize();
        emulator.bus.keyboard.initialize();
        EMULATOR = Some(emulator);
    }
}

#[wasm_bindgen]
pub fn key_down(key_code: u8) {
    unsafe {
        let emulator: &mut Chip8 = EMULATOR.as_mut().unwrap();
        emulator.bus.keyboard.set_key(key_code);
    }
}

#[wasm_bindgen]
pub fn key_up() {
    unsafe {
        let emulator: &mut Chip8 = EMULATOR.as_mut().unwrap();
        emulator.bus.keyboard.set_key(0);
    }
}

#[wasm_bindgen]
pub fn tick() {
    unsafe {
        let emulator: &mut Chip8 = EMULATOR.as_mut().unwrap();
        emulator.fetch_and_execute_instruction();
    }
}

#[wasm_bindgen]
pub fn handle_timers() {
    unsafe {
        let emulator: &mut Chip8 = EMULATOR.as_mut().unwrap();
        emulator.cpu.handler_timers(&mut emulator.bus.audio);
    }
}

#[wasm_bindgen]
pub fn load_rom(rom_bytes: &[u8]) {
    unsafe {
        let emulator = EMULATOR.as_mut().unwrap();
        emulator.reset();
        emulator.load_rom_into_memory(rom_bytes);

        emulator.bus.audio.initialize();
        emulator.bus.audio.start_sound();
    }
}
