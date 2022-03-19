use crate::display::Display;
use crate::instruction::Instruction;
use log::debug;

pub struct CPU {
    pub address_i: u16,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub v_registers: [u8; 16],
    stack: [u16; 16],

    delay_timer: u8,
    sound_timer: u8,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            address_i: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            stack: [0; 16],
            v_registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}

impl CPU {

    pub fn reset(&mut self, display: &mut dyn Display) {
        self.address_i = 0;
        self.program_counter = 0x200;
        self.stack_pointer = 0;
        self.stack = [0; 16];
        self.v_registers = [0;16];
        self.delay_timer = 0;
        self.sound_timer = 0;

        self.cls(display);
    }

    pub fn add_vx(&mut self, x: usize, byte: u8) {
        debug!("ADD V{}, {:#01x}", x, byte);
        self.v_registers[x] += byte;
    }

    pub fn add_i_vx(&mut self, x: usize) {
        debug!("ADD I, V{}", x);
        self.address_i += self.v_registers[x] as u16;
    }

    pub fn call(&mut self, addr: u16) {
        debug!("CALL {:#02x}", addr);
        self.stack[self.stack_pointer as usize] = self.program_counter + 2;
        self.stack_pointer += 1;
        self.program_counter = addr;
    }

    pub fn cls(&mut self, display: &mut dyn Display) {
        debug!("CLS");
        display.clear();
    }

    pub fn jp(&mut self, addr: u16) {
        debug!("JP {:#02x}", addr);
        self.program_counter = addr;
    }

    pub fn ld_dt_vx(&mut self, x: usize) {
        debug!("LD DT, V{}", x);
        self.delay_timer = self.v_registers[x];
    }

    pub fn ld_i(&mut self, addr: u16) {
        debug!("LD I, {:#02x}", addr);
        self.address_i = addr;
    }

    pub fn ld_vx(&mut self, x: usize, byte: u8) {
        debug!("LD V{}, {:#01x}", x, byte);
        self.v_registers[x] = byte;
    }

    pub fn ld_vx_dt(&mut self, x: usize) {
        debug!("LD V{}, DT", x);
        self.v_registers[x] = self.delay_timer;
    }

    pub fn ret(&mut self) {
        debug!("RET");
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }

    pub fn se_vx(&mut self, vx: usize, byte: u8) {
        debug!("SE V{}, {:#01x}", vx, byte);
        if self.v_registers[vx] == byte {
            self.program_counter += 2;
        }
    }

    pub fn sne_vx(&mut self, vx: usize, byte: u8) {
        debug!("SNE V{}, {:#01x}", vx, byte);
        if self.v_registers[vx] != byte {
            self.program_counter += 2;
        }
    }

    pub fn drw(
        &mut self,
        vx: usize,
        vy: usize,
        n: usize,
        memory: &[u8],
        display: &mut dyn Display,
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
                if sprite_pixel && display.get_pixel(x_coord, y_coord) {
                    display.draw_pixel(x_coord, y_coord, pixel_size, false);
                    self.v_registers[0xf] = 1;
                } else if sprite_pixel {
                    display.draw_pixel(x_coord, y_coord, pixel_size, true);
                }

                x_coord -= 1;
                if x_coord >= display.get_width() {
                    break;
                }
            }
            y_coord += 1;
            //x_coord = 0;
            x_coord = (self.v_registers[vx] % 64) as usize;
            if y_coord >= display.get_height() {
                break;
            }
        }
    }

    pub fn execute_instruction(
        &mut self,
        instruction: Instruction,
        memory: &mut [u8],
        display: &mut dyn Display,
    ) {
        match instruction.first {
            0x0 => match instruction.kk {
                0xe0 => self.cls(display),
                0xee => self.ret(),
                _ => self.unknown_instruction(&instruction),
            },
            0x1 => self.jp(instruction.nnn),
            0x2 => self.call(instruction.nnn),
            0x3 => self.se_vx(instruction.x, instruction.kk),
            0x4 => self.sne_vx(instruction.x, instruction.kk),
            0x6 => self.ld_vx(instruction.x, instruction.kk),
            0x7 => self.add_vx(instruction.x, instruction.kk),
            0xa => self.ld_i(instruction.nnn),
            0xd => self.drw(
                instruction.x,
                instruction.y,
                instruction.n as usize,
                memory,
                display,
            ),
            0xf => match instruction.kk {
                0x07 => self.ld_vx_dt(instruction.x),
                0x15 => self.ld_dt_vx(instruction.x),
                0x1e => self.add_i_vx(instruction.x),
                _ => self.unknown_instruction(&instruction)
            },
            _ => self.unknown_instruction(&instruction),
        }

        if instruction.first != 0x2 && instruction.first != 0x1 && instruction.raw_bytes != 0x00ee {
            // dont move the pc with JP, CALL, or RET instructions
            self.program_counter += 2;
        }
    }

    fn unknown_instruction(&self, instruction: &Instruction) {
        panic!(
            "Encountered unknown instruction {:#02x}",
            instruction.raw_bytes
        );
    }
}

#[cfg(test)]
mod test {
    use crate::display::null::NullDisplay;
    use super::*;

    #[test]
    fn add_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0x7c05);

        cpu.v_registers[0xc] = 0x12;
        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert_eq!(cpu.v_registers[0xc], 0x17);
    }

    #[test]
    fn add_i_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0xfb1e);

        cpu.address_i = 0x7;
        cpu.v_registers[0xb] = 0x3;
        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert_eq!(cpu.address_i, 0xa);
    }

    #[test]
    fn call() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0x2123);

        cpu.program_counter = 0xcbd;
        cpu.execute_instruction(instruction, &mut [0], &mut display);

        assert_eq!(cpu.program_counter, 0x123);
        assert_eq!(cpu.stack[cpu.stack_pointer as usize - 1], 0xcbf);
    }

    #[test]
    fn cls() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0x00e0);

        assert!(!display.cleared);
        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert!(display.cleared);
    }

    #[test]
    fn jp() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0x1aba);

        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert_eq!(cpu.program_counter, 0xaba);
    }

    #[test]
    fn ld_dt_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0xf315);

        cpu.v_registers[0x3] = 0xbb;
        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert_eq!(cpu.delay_timer, 0xbb);
    }

    #[test]
    fn ld_vx_dt() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0xf407);

        cpu.delay_timer = 0xf;
        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert_eq!(cpu.v_registers[0x4], cpu.delay_timer);
    }

    #[test]
    fn ld_i() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0xa123);

        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert_eq!(cpu.address_i, 0x123);
    }

    #[test]
    fn ld_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0x6513);

        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert_eq!(cpu.v_registers[0x5], 0x13);
    }

    #[test]
    fn se_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0x3307);

        cpu.program_counter = 0x5;
        cpu.v_registers[0x3] = 0x8;

        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert_eq!(cpu.program_counter, 0x7);

        cpu.program_counter = 0x5;
        let instruction = Instruction::new(0x3308);
        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert_eq!(cpu.program_counter, 0x9);
    }

    #[test]
    fn sne_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0x4320);

        cpu.program_counter = 0x5;
        cpu.v_registers[0x3] = 0x21;

        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert_eq!(cpu.program_counter, 0x9);

        cpu.program_counter = 0x5;
        let instruction = Instruction::new(0x4321);
        cpu.execute_instruction(instruction, &mut [0], &mut display);
        assert_eq!(cpu.program_counter, 0x7);
    }

    #[test]
    fn ret() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0x2123);

        cpu.program_counter = 0xcbd;
        cpu.execute_instruction(instruction, &mut [0], &mut display);

        let instruction = Instruction::new(0x00ee);
        cpu.execute_instruction(instruction, &mut [0], &mut display);

        assert_eq!(cpu.program_counter, 0xcbf);
    }

    #[test]
    fn drw() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let instruction = Instruction::new(0xd103);
        let mut memory: [u8; 4096] = [0; 4096];

        // drawing the following 'tree' sprite offset 1 pixel from left
        // ...*..
        // ..*.*.
        // .*..*.
        let sprite_data: [u8; 3] = [
          0b00100, 0b01010, 0b10010
        ];
        cpu.address_i = 0x500;
        memory[0x500..0x500 + sprite_data.len()].copy_from_slice(&sprite_data);
        cpu.execute_instruction(instruction, &mut memory, &mut display);

        // row 1
        assert_eq!(display.vram[0], 0);
        assert_eq!(display.vram[1], 0);
        assert_eq!(display.vram[2], 0);
        assert_eq!(display.vram[3], 1);
        assert_eq!(display.vram[4], 0);
        assert_eq!(display.vram[5], 0);

    }
}
