use moldy_chip::cpu::CPU;

#[test]
fn ld_i() {
    let mut cpu = CPU::default();
    cpu.ld_i(0x133);
    assert_eq!(cpu.address_i, 0x133);
}