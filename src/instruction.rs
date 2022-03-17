pub struct Instruction {
    pub raw_bytes: u16,
    pub first: u8,
    pub nnn: u16,
    pub kk: u8,
    pub x: usize,
    pub y: usize,
    pub n: u8,
}

impl Instruction {
    pub fn new(bytes: u16) -> Instruction {
        return Instruction {
            raw_bytes: bytes,
            first: (bytes >> 12 & 0xf) as u8,
            nnn: bytes & 0xfff,
            kk: (bytes & 0xff) as u8,
            x: (bytes >> 8 & 0xf) as usize,
            y: (bytes >> 4 & 0xf) as usize,
            n: (bytes & 0xf) as u8,
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let inst = Instruction::new(0xabcd);
        assert_eq!(inst.first, 0xa);
        assert_eq!(inst.nnn, 0xbcd);
        assert_eq!(inst.kk, 0xcd);
        assert_eq!(inst.x, 0xb);
        assert_eq!(inst.y, 0xc);
        assert_eq!(inst.n, 0xd);

        let inst = Instruction::new(0x1234);
        assert_eq!(inst.first, 0x1);
        assert_eq!(inst.nnn, 0x234);
        assert_eq!(inst.kk, 0x34);
        assert_eq!(inst.x, 0x2);
        assert_eq!(inst.y, 0x3);
        assert_eq!(inst.n, 0x4);
    }
}
