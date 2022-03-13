use crate::instruction::Instruction;
use log::{debug, info};

pub struct CPU {
    pub(crate) address_i: u16,
    pub(crate) program_counter: usize,
    pub(crate) stack_pointer: u8,
    pub(crate) v_registers: [u8; 16],

    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,
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
        }
    }
}

impl CPU {
    pub fn ld_i(&mut self, addr: u16) {
        debug!("LD I, {:#02x}", addr);
        self.address_i = addr;
    }

    pub fn ld_vx(&mut self, x: usize, byte: u8) {
        debug!("LD V{}, {:#01x}", x, byte);
        self.v_registers[x] = byte;
    }
}
