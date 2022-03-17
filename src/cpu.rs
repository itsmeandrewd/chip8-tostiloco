use crate::screen_display::WebGLDisplay;
use log::debug;

pub struct CPU {
    pub address_i: u16,
    pub program_counter: usize,
    pub stack_pointer: u8,
    pub v_registers: [u8; 16],

    delay_timer: u8,
    sound_timer: u8,
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
    pub fn cls(&mut self, display: &mut WebGLDisplay) {
        debug!("CLS");
        display.clear();
    }

    pub fn ld_i(&mut self, addr: u16) {
        debug!("LD I, {:#02x}", addr);
        self.address_i = addr;
    }

    pub fn ld_vx(&mut self, x: usize, byte: u8) {
        debug!("LD V{}, {:#01x}", x, byte);
        self.v_registers[x] = byte;
    }

    pub fn se_vx(&mut self, vx: usize, byte: u8) {
        debug!("SE V{}, {:#01x}", vx, byte);
        if self.v_registers[vx] == byte {
            self.program_counter += 2;
        }
    }

    pub fn jp(&mut self, addr: u16) {
        debug!("JP {:#02x}", addr);
        self.program_counter = addr as usize;
    }

    pub fn add_vx(&mut self, x: usize, byte: u8) {
        debug!("ADD V{}, {:#01x}", x, byte);
        self.v_registers[x] += byte;
    }

    pub fn drw(
        &mut self,
        vx: usize,
        vy: usize,
        n: usize,
        memory: &[u8],
        display: &mut WebGLDisplay,
    ) {
        debug!("DRW V{}, V{}, {:#01x}", vx, vy, n);
        let mut x_coord = (self.v_registers[vx] % 64) as usize;
        let mut y_coord = (self.v_registers[vy] % 32) as usize;
        self.v_registers[0xf] = 0;

        let pixel_size = 10.0;
        for row in 0..n {
            let sprite_data = memory[(self.address_i as usize) + row];
            for bit in 0..8 {
                let sprite_pixel = (sprite_data & (1 << bit)) != 0;
                if sprite_pixel && display.is_pixel_on(x_coord, y_coord) {
                    display.draw_box(x_coord, y_coord, pixel_size, false);
                    self.v_registers[0xf] = 1;
                } else if sprite_pixel {
                    display.draw_box(x_coord, y_coord, pixel_size, true);
                }

                x_coord += 1;
                if x_coord >= display.get_width() {
                    break;
                }
            }
            y_coord += 1;
            x_coord = 0;
            if y_coord >= display.get_height() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ld_vx() {
        let mut cpu = CPU::default();
        cpu.ld_vx(0x5, 0x13);
        assert_eq!(cpu.v_registers[0x5], 0x13);
    }

    #[test]
    fn se_vx() {
        let mut cpu = CPU::default();
        cpu.program_counter = 0x5;
        cpu.v_registers[0x3] = 0x8;

        cpu.se_vx(0x3, 0x7);
        assert_eq!(cpu.program_counter, 0x5);

        cpu.se_vx(0x3, 0x8);
        assert_eq!(cpu.program_counter, 0x7);
    }
}
