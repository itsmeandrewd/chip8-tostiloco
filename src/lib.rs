mod chip8;
mod cpu;
mod screen_display;
mod instruction;

use std::panic;
use crate::cpu::CPU;
use log::{info, Level};
use wasm_bindgen::prelude::*;
use crate::chip8::CHIP8;
use crate::screen_display::WebGLDisplay;

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

    //let mut chip8: CHIP8 = Default::default();
    let mut chip8 = CHIP8 {
        cpu: CPU {
            address_i: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            v_registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0
        },
        memory: [0; 4096],
        display: WebGLDisplay::default()
    };

    chip8.load_rom_into_memory(rom_bytes);
    chip8.fetch_and_execute_instruction();
    //cpu.execute();
    //console_log!(&format!("Hello {}", rom_bytes[0]));
}
