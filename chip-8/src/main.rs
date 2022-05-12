mod isa;

use isa::Cpu;

fn main() {
    let mut cpu = Cpu::default();

    cpu.store_reg(0, 5);
    cpu.store_reg(1, 10);
    cpu.store_reg(2, 10);
    cpu.store_reg(3, 10);

    cpu.store_byte(0, 0x80);
    cpu.store_byte(1, 0x14);
    cpu.store_byte(2, 0x80);
    cpu.store_byte(3, 0x24);
    cpu.store_byte(4, 0x80);
    cpu.store_byte(5, 0x34);

    cpu.run();
    println!("{}", cpu.load_reg(0));
}
