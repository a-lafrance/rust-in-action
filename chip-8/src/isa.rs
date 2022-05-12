pub struct Cpu {
    registers: [u8; Cpu::N_REGS],
    memory: [u8; Cpu::MEM_SIZE],
    pc: usize,
}

impl Cpu {
    const N_REGS: usize = 16;
    const MEM_SIZE: usize = 0x1000;

    fn fetch_instr(&mut self) -> u16 {
        let lower = self.memory[self.pc] as u16;
        let upper = self.memory[self.pc + 1] as u16;
        self.pc += 2;

        lower << 8 | upper
    }

    pub fn load_reg(&self, reg: usize) -> u8 {
        self.registers[reg]
    }

    pub fn store_reg(&mut self, reg: usize, val: u8) {
        self.registers[reg] = val;
    }

    pub fn load_byte(&self, addr: usize) -> u8 {
        self.memory[addr]
    }

    pub fn store_byte(&mut self, addr: usize, val: u8) {
        self.memory[addr] = val;
    }

    fn decode(&self, opcode: u16) -> (u8, u8, u8, u8) {
        (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        )
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch_instr();

            match self.decode(opcode) {
                (0, 0, 0, 0) => return,
                (0x8, x, y, 0x4) => self.add(x, y),
                _ => todo!(),
            }
        }
    }

    fn add(&mut self, r1: u8, r2: u8) {
        let x = self.load_reg(r1 as usize);
        let y = self.load_reg(r2 as usize);
        let (result, overflow) = x.overflowing_add(y);

        self.store_reg(r1 as usize, result);
        self.store_reg(Cpu::N_REGS - 1, if overflow { 1 } else { 0 });
    }
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu {
            registers: [0; Cpu::N_REGS],
            memory: [0; Cpu::MEM_SIZE],
            pc: 0,
        }
    }
}
