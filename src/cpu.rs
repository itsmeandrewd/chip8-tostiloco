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

/*impl Default for CPU {
    fn default() -> Self {
        Self { program_counter: 0x200, ..Default::default() }
    }
}*/

impl CPU {

}
