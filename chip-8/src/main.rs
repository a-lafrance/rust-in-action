mod isa;

use isa::Cpu;

fn main() {
    let mut cpu = Cpu::default();

    cpu.store_reg(0, 5);
    cpu.store_reg(1, 10);

    cpu.store_bytes(0x0, &[0x21, 0x00, 0x21, 0x00, 0x00, 0x00]);
    cpu.store_bytes(0x100, &[0x80, 0x14, 0x80, 0x14, 0x00, 0xEE]);

    cpu.run();
    println!("{}", cpu.load_reg(0));
}
