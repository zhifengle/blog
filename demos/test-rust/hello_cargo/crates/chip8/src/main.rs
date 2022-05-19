struct Cpu {
    // current_operation: u16,
    registers: [u8; 16],

    // spec 是 u16; 但是 Rust 只支持 usize 索引
    // 本质是 program counter . 书里面是为了方便读者理解
    position_in_memory: usize,
    // chip 8 只有 4096 RAM; 0x1000  2**12, 16 ** 3
    // nnn 是地址  0x1000 - 1 是能够存储的最大地址
    // chip 8 0x100 512 这是为系统保留的地址
    memory: [u8; 0x1000],

    stack: [u16; 16],
    stack_pointer: usize,
}

impl Cpu {
    // 从内存里面读取操作码
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2
        // self.current_operation
    }
    fn run(&mut self) {
        // Rust 没有 4-bit
        // 0x73ee   High byte + Low byte;   add 3(register) 0xee;  7 add, 3 register
        // ee       High nibble + Low nibble        argument kk
        //
        // 0x1200   Address(nnn)  0x200
        loop {
            let opcode = self.read_opcode();
            // 读取完毕指向下一条指令
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            // assert_eq!(c, 0x8);
            // assert_eq!(x, 0x0);
            // assert_eq!(y, 0x1);
            // assert_eq!(d, 0x4);

            let nnn = opcode & 0x0FFF;
            // let kk = opcode & 0x00ff;
            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        // 这种方式没有考虑到溢出的问题
        // self.registers[x as usize] += self.registers[y as usize];
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        if overflow {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }
    }
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;
        if sp > stack.len() {
            panic!("Stack overflow!");
        }
        // 入栈
        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        // 设置跳转位置
        self.position_in_memory = addr as usize;
    }
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack overflow!");
        }
        self.stack_pointer -= 1;
        // 回跳
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }
}

// main loop
// 1. 读取opcode
// 2. 解码opcode 为命令
// 3. 匹配命令为操作码
// 4. 指向对应的命令函数

/// ref: rust-in-action  part2 chap5.7

// caller  0x2nnn  CALL opcode
// RETURN (0x00EE)
fn main() {
    let mut cpu = Cpu {
        // current_operation: 0,
        // 0xf 最后一个寄存器是 carry
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };
    // 0x8014; 8 调用两个寄存器  0  1  寄存器 0, 1,  4 addition
    // cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // let mut memory: [u8; 4096] = [0; 4096];
    // let mem = &memory;
    // let add_twice = [0x80, 0x14, 0x80, 0x14, 0x00, 0xEE];
    // mem[0x100..0x106].copy_from_slice(&add_twice);

    let mem = &mut cpu.memory;
    // CALL 0x100
    mem[0] = 0x21;
    mem[1] = 0x00;
    mem[2] = 0x21;
    mem[3] = 0x00;

    mem[4] = 0x00;
    mem[5] = 0x00;

    // 0x8014; add register1 to register0
    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();
    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
