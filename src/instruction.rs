pub(crate) struct Instruction {
    pub raw_bytes: u16,
    pub first: u8,
    pub nnn: u16,
    pub kk: u8,
    pub x: usize,
    pub y: usize,
    pub n: u8,
}

impl Instruction {
    pub(crate) fn new(bytes: u16) -> Instruction {
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
