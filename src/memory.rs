const STACK_SIZE: usize = 0xFF;
const MEMORY_SIZE: usize = 0xFFFF;

pub struct Memory{
    pub stack: [i32; STACK_SIZE],
    pub memory: [u8; MEMORY_SIZE],
}

impl Memory{
    pub fn new() -> Self{
        Self{
            stack: [0; STACK_SIZE],
            memory: [0; MEMORY_SIZE],
        }
    }

    /* Byte - 8 bits */

    pub fn write_byte(&mut self, addr: usize, value: u8){
        self.memory[addr] = value;
    }

    pub fn read_byte(&self, addr: usize) -> u8{
        self.memory[addr]
    }

    /* Half Word - 16 bits */

    pub fn write_half_word(&mut self, addr: usize, value: u16){
        let upper = (value >> 8) as u8;
        let lower = value as u8;
        self.memory[addr] = upper;
        self.memory[addr + 1] = lower;
    }

    pub fn read_half_word(&self, addr: usize) -> u16{
        let upper = (self.memory[addr] as u16) << 8;
        let lower = self.memory[addr + 1] as u16;

        upper | lower
    }

    /* Word - 32 bits */

    pub fn write_word(&mut self, addr: usize, value: u32){
        let upper = (value >> 24) as u8; // gets bits 24 - 31
        let upper24 = (value >> 16) as u8;// gets bits 16 - 23
        let upper16 = (value >> 8) as u8; // gets bits 8 - 15
        let lower = value as u8; // gets bits 0 - 7 (inclusive)

        self.memory[addr] = upper;
        self.memory[addr + 1] = upper24;
        self.memory[addr + 2] = upper16;
        self.memory[addr + 3] = lower;
    }

    pub fn read_word(&self, addr: usize) -> u32{
        let upper = (self.memory[addr] as u32) << 24;
        let upper24 = (self.memory[addr + 1] as u32) << 16;
        let upper16 = (self.memory[addr + 2] as u32) << 8;
        let lower = self.memory[addr + 3] as u32;

        upper | upper24 | upper16 | lower
    }

    /* Stack */

    pub fn push_to_stack(&mut self, value: i32, stack_pointer: isize){
        if stack_pointer == -1{
            println!("Warning: Stack Pointer is negative. Skipping...");
            return
        }
        self.stack[stack_pointer as usize] = value;
    }

    pub fn pop_from_stack(&mut self, stack_pointer: isize) -> i32{
        if stack_pointer == -1{
            println!("Warning: Stack Pointer is negative. Skipping...");
            return 0;
        }

        self.stack[stack_pointer as usize]
    }
}