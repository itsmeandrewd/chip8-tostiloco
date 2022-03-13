pub(crate) struct Instruction {
    pub raw_bytes: u16,
    pub(crate) first: u8,
    nnn: u16,
    pub(crate) kk: u16,
    x: u8,
    y: u8,
    n: u8,
}

impl Instruction {
    pub(crate) fn new(bytes: u16) -> Instruction {
        return Instruction {
            raw_bytes: bytes,
            first: (bytes >> 12 & 0xf) as u8,
            nnn: bytes & 0xfff,
            kk: bytes & 0xff,
            x: (bytes >> 8 & 0xf) as u8,
            y: (bytes >> 4 & 0xf) as u8,
            n: (bytes & 0xf) as u8,
        };
    }
}
