pub struct Cpu {
    registers: [u8; Cpu::N_REGS],
    memory: [u8; Cpu::MEM_SIZE],
    stack: [u16; Cpu::STACK_SIZE],
    pc: usize,
    sp: usize,
}

impl Cpu {
    const N_REGS: usize = 16;
    const MEM_SIZE: usize = 0x1000;
    const STACK_SIZE: usize = 16;

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

    pub fn store_bytes(&mut self, start: usize, body: &[u8]) {
        let end = start + body.len();
        self.memory[start..end].copy_from_slice(body);
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
                (0x2, _, _, _) => self.call(opcode & 0x0FFF),
                (0, 0, 0xE, 0xE) => self.ret(),
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

    fn call(&mut self, addr: u16) {
        // save return addr
        // inc sp
        self.push_to_stack();

        // jump to addr
        self.pc = addr as usize;
    }

    fn ret(&mut self) {
        // dec sp
        // pop return addr
        let return_addr = self.pop_from_stack();

        // jump back
        self.pc = return_addr as usize;
    }

    fn push_to_stack(&mut self) {
        if self.sp >= self.stack.len() {
            panic!("stack overflow");
        }

        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
    }

    fn pop_from_stack(&mut self) -> u16 {
        if self.sp == 0 {
            panic!("stack underflow");
        }

        self.sp -= 1;
        self.stack[self.sp]
    }
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu {
            registers: [0; Cpu::N_REGS],
            memory: [0; Cpu::MEM_SIZE],
            stack: [0; Cpu::STACK_SIZE],
            pc: 0,
            sp: 0,
        }
    }
}
