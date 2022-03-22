use crate::display::Display;
use crate::instruction::Instruction;
use crate::keyboard::Keyboard;
use log::debug;
use rand::{thread_rng, Rng};

pub struct CPU {
    pub address_i: u16,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub v_registers: [u8; 16],
    stack: [u16; 16],

    delay_timer: u8,
    sound_timer: u8,

    key_pressed: u8,
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
            key_pressed: 0,
        }
    }
}

impl CPU {
    pub fn reset(&mut self, display: &mut dyn Display) {
        self.address_i = 0;
        self.program_counter = 0x200;
        self.stack_pointer = 0;
        self.stack = [0; 16];
        self.v_registers = [0; 16];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.key_pressed = 0;

        self.cls(display);
    }

    pub fn handler_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn add_vx(&mut self, x: usize, byte: u8) {
        debug!("ADD V{}, {:#01x}", x, byte);
        self.v_registers[x] += byte;
    }

    pub fn add_i_vx(&mut self, x: usize) {
        debug!("ADD I, V{}", x);
        self.address_i += self.v_registers[x] as u16;
    }

    pub fn add_vx_vy(&mut self, x: usize, y: usize) {
        debug!("ADD V{}, V{}", x, y);
        let buffer = self.v_registers[x] as u16 + self.v_registers[y] as u16;
        if buffer > 0xff {
            self.v_registers[0xf] = 0x1;
        } else {
            self.v_registers[0xf] = 0x0;
        }
        self.v_registers[x] = *buffer.to_be_bytes().last().unwrap();
    }

    pub fn and_vx_vy(&mut self, x: usize, y: usize) {
        debug!("AND V{}, V{}", x, y);
        self.v_registers[x] = self.v_registers[x] & self.v_registers[y];
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

    pub fn ld_bcd_vx(&mut self, x: usize, memory: &mut [u8]) {
        debug!("LD BCD, V{}", x);
        memory[self.address_i as usize] = self.v_registers[x] / 100;
        memory[self.address_i as usize + 1] = self.v_registers[x] % 100 / 10;
        memory[self.address_i as usize + 2] = self.v_registers[x] % 10;
    }

    pub fn ld_dt_vx(&mut self, x: usize) {
        debug!("LD DT, V{}", x);
        self.delay_timer = self.v_registers[x];
    }

    pub fn ld_f_vx(&mut self, x: usize) {
        debug!("LD F, V{}", x);
    }

    pub fn ld_i(&mut self, addr: u16) {
        debug!("LD I, {:#02x}", addr);
        self.address_i = addr;
    }

    pub fn ld_st_vx(&mut self, x: usize) {
        debug!("LD ST, V{}", x);
        self.sound_timer = self.v_registers[x];
    }

    pub fn ld_vx(&mut self, x: usize, byte: u8) {
        debug!("LD V{}, {:#01x}", x, byte);
        self.v_registers[x] = byte;
    }

    pub fn ld_vx_dt(&mut self, x: usize) {
        debug!("LD V{}, DT", x);
        self.v_registers[x] = self.delay_timer;
    }

    pub fn ld_vx_i(&mut self, x: usize, memory: &[u8]) {
        debug!("LD V{}, I", x);
        for n in 0..=x {
            self.v_registers[n] = memory[self.address_i as usize + n]
        }
    }

    pub fn ld_vx_k(&mut self, x: usize, keyboard: &mut dyn Keyboard) {
        debug!("LD V{}, K", x);
        let key_down = keyboard.get_key();

        if key_down == 0 {
            self.program_counter -= 2;
        } else {
            self.v_registers[x] = key_down;
        }
    }

    pub fn ld_vx_vy(&mut self, x: usize, y: usize) {
        debug!("LD V{}, V{}", x, y);
        self.v_registers[x] = self.v_registers[y];
    }

    pub fn ret(&mut self) {
        debug!("RET");
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }

    pub fn rnd(&mut self, x: usize, byte: u8) {
        let mut rng = thread_rng();
        let random_num = rng.gen_range(0, 256);
        self.v_registers[x] = random_num as u8 & byte;
    }

    pub fn se_vx(&mut self, vx: usize, byte: u8) {
        debug!("SE V{}, {:#01x}", vx, byte);
        if self.v_registers[vx] == byte {
            self.program_counter += 2;
        }
    }

    pub fn shr_vx_vy(&mut self, x: usize, y: usize) {
        debug!("SHR V{}, V{}", x, y);
        if self.v_registers[x] & 0x1 == 0x1 {
            self.v_registers[0xf] = 0x1;
        } else {
            self.v_registers[0xf] = 0x0;
        }
        self.v_registers[x] = self.v_registers[x] >> 1;
    }

    pub fn skp_vx(&mut self, x: usize, keyboard: &mut dyn Keyboard) {
        debug!("SKP V{}", x);
        if keyboard.get_key() == self.v_registers[x] {
            self.program_counter += 2;
        }
    }

    pub fn sknp_vx(&mut self, x: usize, keyboard: &mut dyn Keyboard) {
        debug!("SKNP V{}", x);
        if keyboard.get_key() != self.v_registers[x] {
            self.program_counter += 2;
        }
    }

    pub fn sne_vx(&mut self, vx: usize, byte: u8) {
        debug!("SNE V{}, {:#01x}", vx, byte);
        if self.v_registers[vx] != byte {
            self.program_counter += 2;
        }
    }

    pub fn sub_vx_vy(&mut self, x: usize, y: usize) {
        debug!("SUB V{}, V{}", x, y);
        if self.v_registers[x] > self.v_registers[y] {
            self.v_registers[0xf] = 0x1;
        } else {
            self.v_registers[0xf] = 0x0;
        }
        self.v_registers[x] = self.v_registers[x].wrapping_sub(self.v_registers[y]);
    }

    pub fn xor_vx_vy(&mut self, x: usize, y: usize) {
        debug!("XOR V{}, V{}", x, y);
        self.v_registers[x] ^= self.v_registers[y];
    }

    pub fn drw(&mut self, x: usize, y: usize, n: usize, memory: &[u8], display: &mut dyn Display) {
        debug!("DRW V{}, V{}, {:#01x}", x, y, n);
        self.v_registers[0xf] = 0x0;

        let pixel_size = 20.0;
        for row in 0..n {
            let pixel = memory[self.address_i as usize + row];
            for col in 0..8 {
                if (pixel & (0x80 >> col)) != 0 {
                    let cur_pixel = display.get_pixel(
                        (self.v_registers[x] + col) as usize,
                        (self.v_registers[y] + row as u8) as usize,
                    );
                    if cur_pixel {
                        self.v_registers[0xf] = 0x1;
                    }
                    display.draw_pixel(
                        (self.v_registers[x] + col) as usize,
                        (self.v_registers[y] + row as u8) as usize,
                        pixel_size,
                        (cur_pixel as u8 ^ 1) == 1,
                    );
                }
            }
        }
    }

    pub fn execute_instruction(
        &mut self,
        instruction: Instruction,
        memory: &mut [u8],
        display: &mut dyn Display,
        keyboard: &mut dyn Keyboard,
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
            0x8 => match instruction.n {
                0x0 => self.ld_vx_vy(instruction.x, instruction.y),
                0x2 => self.and_vx_vy(instruction.x, instruction.y),
                0x3 => self.xor_vx_vy(instruction.x, instruction.y),
                0x4 => self.add_vx_vy(instruction.x, instruction.y),
                0x5 => self.sub_vx_vy(instruction.x, instruction.y),
                0x6 => self.shr_vx_vy(instruction.x, instruction.y),
                _ => self.unknown_instruction(&instruction),
            },
            0xa => self.ld_i(instruction.nnn),
            0xc => self.rnd(instruction.x, instruction.kk),
            0xd => self.drw(
                instruction.x,
                instruction.y,
                instruction.n as usize,
                memory,
                display,
            ),
            0xe => match instruction.kk {
                0x9e => self.skp_vx(instruction.x, keyboard),
                0xa1 => self.sknp_vx(instruction.x, keyboard),
                _ => self.unknown_instruction(&instruction),
            },
            0xf => match instruction.kk {
                0x07 => self.ld_vx_dt(instruction.x),
                0x0a => self.ld_vx_k(instruction.x, keyboard),
                0x15 => self.ld_dt_vx(instruction.x),
                0x18 => self.ld_st_vx(instruction.x),
                0x1e => self.add_i_vx(instruction.x),
                0x33 => self.ld_bcd_vx(instruction.x, memory),
                0x65 => self.ld_vx_i(instruction.x, memory),
                _ => self.unknown_instruction(&instruction),
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
    use super::*;
    use crate::display::null::NullDisplay;
    use crate::keyboard::null::NullKeyboard;

    #[test]
    fn add_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x7c05);

        cpu.v_registers[0xc] = 0x12;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0xc], 0x17);
    }

    #[test]
    fn add_i_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0xfb1e);

        cpu.address_i = 0x7;
        cpu.v_registers[0xb] = 0x3;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.address_i, 0xa);
    }

    #[test]
    fn add_vx_vy() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x8ba4);

        cpu.v_registers[0xb] = 0x2;
        cpu.v_registers[0xa] = 0x2;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0xf], 0x0);
        assert_eq!(cpu.v_registers[0xb], 0x4);

        let instruction = Instruction::new(0x8ba4);
        cpu.v_registers[0xb] = 0xff;
        cpu.v_registers[0xa] = 0x2;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0xf], 0x1);
        assert_eq!(cpu.v_registers[0xb], 0x1);
    }

    #[test]
    fn and_vx_vy() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x8012);

        cpu.v_registers[0] = 0x3;
        cpu.v_registers[1] = 0xe;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0], 0x2);
    }

    #[test]
    fn call() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x2123);

        cpu.program_counter = 0xcbd;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);

        assert_eq!(cpu.program_counter, 0x123);
        assert_eq!(cpu.stack[cpu.stack_pointer as usize - 1], 0xcbf);
    }

    #[test]
    fn cls() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x00e0);

        assert!(!display.cleared);
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert!(display.cleared);
    }

    #[test]
    fn jp() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x1aba);

        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.program_counter, 0xaba);
    }

    #[test]
    fn ld_bcd_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut memory = [0u8; 7];
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0xfe33);

        cpu.v_registers[0xe] = 123;
        cpu.address_i = 3;
        cpu.execute_instruction(instruction, &mut memory, &mut display, &mut keyboard);
        assert_eq!(memory[3], 1);
        assert_eq!(memory[4], 2);
        assert_eq!(memory[5], 3);
    }

    #[test]
    fn ld_dt_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0xf315);

        cpu.v_registers[0x3] = 0xbb;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.delay_timer, 0xbb);
    }

    #[test]
    fn ld_st_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0xfa18);

        cpu.v_registers[0xa] = 0x7;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.sound_timer, 0x7);
    }

    #[test]
    fn ld_vx_dt() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0xf407);

        cpu.delay_timer = 0xf;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0x4], cpu.delay_timer);
    }

    #[test]
    fn ld_vx_i() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0xf365);
        let mut memory = [0x0, 0xf, 0xe, 0xd, 0xc, 0xb];
        cpu.address_i = 0x1;

        cpu.execute_instruction(instruction, &mut memory, &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0], 0xf);
        assert_eq!(cpu.v_registers[1], 0xe);
        assert_eq!(cpu.v_registers[2], 0xd);
        assert_eq!(cpu.v_registers[3], 0xc);
    }

    #[test]
    fn ld_vx_k() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0xf10a);

        cpu.program_counter = 0x2;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.program_counter, 0x2);
        assert_eq!(cpu.v_registers[0x1], 0x0);

        keyboard.set_key(0xd);
        let instruction = Instruction::new(0xf10a);
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.program_counter, 0x4);
        assert_eq!(cpu.v_registers[0x1], 0xd);
    }

    #[test]
    fn ld_vx_vy() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x8de0);

        cpu.v_registers[0xd] = 0xff;
        cpu.v_registers[0xe] = 0x12;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0xd], 0x12);
    }

    #[test]
    fn ld_i() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0xa123);

        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.address_i, 0x123);
    }

    #[test]
    fn ld_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x6513);

        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0x5], 0x13);
    }

    #[test]
    fn ret() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x2123);

        cpu.program_counter = 0xcbd;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);

        let instruction = Instruction::new(0x00ee);
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);

        assert_eq!(cpu.program_counter, 0xcbf);
    }

    #[test]
    fn se_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x3307);

        cpu.program_counter = 0x5;
        cpu.v_registers[0x3] = 0x8;

        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.program_counter, 0x7);

        cpu.program_counter = 0x5;
        let instruction = Instruction::new(0x3308);
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.program_counter, 0x9);
    }

    #[test]
    fn shr_vx_vy() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x8106);

        cpu.v_registers[0x1] = 0b10111010; // 186
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0xf], 0x0);
        assert_eq!(cpu.v_registers[0x1], 0b1011101);

        let instruction = Instruction::new(0x8106);
        cpu.v_registers[0x1] = 0b11111011; // 251
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0xf], 0x1);
        assert_eq!(cpu.v_registers[0x1], 0b1111101);
    }

    #[test]
    fn skp_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0xe39e);

        cpu.program_counter = 0x2;
        cpu.v_registers[0x3] = 0xa;
        keyboard.set_key(0xa);
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.program_counter, 0x6);
    }

    #[test]
    fn sknp_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0xe2a1);

        cpu.program_counter = 0x2;
        cpu.v_registers[0x2] = 0xa;
        keyboard.set_key(0xb);
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.program_counter, 0x6);
    }

    #[test]
    fn sne_vx() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x4320);

        cpu.program_counter = 0x5;
        cpu.v_registers[0x3] = 0x21;

        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.program_counter, 0x9);

        cpu.program_counter = 0x5;
        let instruction = Instruction::new(0x4321);
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.program_counter, 0x7);
    }

    #[test]
    fn sub_vx_vy() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x83b5);

        cpu.v_registers[0x3] = 0x9;
        cpu.v_registers[0xb] = 0x4;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0xf], 0x1);
        assert_eq!(cpu.v_registers[0x3], 0x5);

        let instruction = Instruction::new(0x83b5);
        cpu.v_registers[0x3] = 0x9;
        cpu.v_registers[0xb] = 0xf;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0xf], 0x0);
        assert_eq!(cpu.v_registers[0x3], 0xfa);
    }

    #[test]
    fn xor_vx_vy() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0x8e23);

        cpu.v_registers[0xe] = 0xff;
        cpu.v_registers[0x2] = 0x10;
        cpu.execute_instruction(instruction, &mut [0], &mut display, &mut keyboard);
        assert_eq!(cpu.v_registers[0xe], 0xef);
    }

    #[test]
    fn drw() {
        let mut cpu = CPU::default();
        let mut display = NullDisplay::default();
        let mut keyboard = NullKeyboard::default();
        let instruction = Instruction::new(0xdb03);
        let mut memory: [u8; 4096] = [0; 4096];

        // drawing the following 'tree' sprite offset 1 pixel from left
        // ...*..
        // ..*.*.
        // .*..*.
        let sprite_data: [u8; 3] = [0b00100000, 0b01010000, 0b10010000];
        cpu.address_i = 0x500;
        cpu.v_registers[0xb] = 0x1;
        memory[0x500..0x500 + sprite_data.len()].copy_from_slice(&sprite_data);
        cpu.execute_instruction(instruction, &mut memory, &mut display, &mut keyboard);

        // row 1
        assert!(!display.get_pixel(0, 0));
        assert!(!display.get_pixel(1, 0));
        assert!(!display.get_pixel(2, 0));
        assert!(display.get_pixel(3, 0));
        assert!(!display.get_pixel(4, 0));
        assert!(!display.get_pixel(5, 0));

        // row 2
        assert!(!display.get_pixel(0, 1));
        assert!(!display.get_pixel(1, 1));
        assert!(display.get_pixel(2, 1));
        assert!(!display.get_pixel(3, 1));
        assert!(display.get_pixel(4, 1));
        assert!(!display.get_pixel(5, 1));

        // row 3
        assert!(!display.get_pixel(0, 2));
        assert!(display.get_pixel(1, 2));
        assert!(!display.get_pixel(2, 2));
        assert!(!display.get_pixel(3, 2));
        assert!(display.get_pixel(4, 2));
        assert!(!display.get_pixel(5, 2));
    }
}
