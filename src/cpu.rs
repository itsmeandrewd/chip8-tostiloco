use crate::screen_display::WebGLDisplay;
use log::debug;

pub struct CPU {
    pub address_i: u16,
    pub program_counter: usize,
    pub stack_pointer: u8,
    pub v_registers: [u8; 16],

    delay_timer: u8,
    sound_timer: u8,

    pub(crate) display: WebGLDisplay,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            address_i: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            v_registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,

            display: Default::default()
        }
    }
}

impl CPU {
    pub fn cls(&mut self) {
        debug!("CLS");
        self.display.clear();
    }

    pub fn ld_i(&mut self, addr: u16) {
        debug!("LD I, {:#02x}", addr);
        self.address_i = addr;
    }

    pub fn ld_vx(&mut self, x: usize, byte: u8) {
        debug!("LD V{}, {:#01x}", x, byte);
        self.v_registers[x] = byte;
    }

    pub fn drw(&mut self, vx: usize, vy: usize, n: u8, memory: &[u8]) {
        debug!("DRW V{}, V{}, {:#01x}", vx, vy, n);
        let x_coord = self.v_registers[vx] % 64;
        let y_coord = self.v_registers[vy] % 32;
        self.v_registers[0xf] = 0;

        for i in 0..self.display.get_height() {}

        self.display.draw(vx as u8, vy as u8, n);
    }
}
