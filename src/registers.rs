pub struct Registers{
    pub registers: [i32; 6], // 6 general purpose registers
    
    pub fp_registers: [f32; 6], // 6 general purpose floating point registers

    pub exception_register: [(usize, bool); 8],
    pub exception_flags: u8,

    pub jump_pointer: i32, // A special register to store the IP when jumping (so you can return from a jump)

    pub instruction_pointer: i32,
    pub stack_pointer: isize,
}

impl Registers{
    pub fn new() -> Self{
        Self{
            registers: [0; 6],

            fp_registers: [0.0; 6],

            exception_register: [(0, false); 8],
            exception_flags: 0,

            jump_pointer: 0,

            instruction_pointer: 0,
            stack_pointer: -1
        }
    }

    /* Registers */

    pub fn set_register(&mut self, register: usize, value: i32){
        self.registers[register] = value;
    }

    pub fn get_register(&self, register: usize) -> i32{
        self.registers[register]
    }

    pub fn set_f_register(&mut self, register: usize, value: f32){
        self.fp_registers[register] = value;
    }

    pub fn get_f_register(&self, register: usize) -> f32{
        self.fp_registers[register]
    }

    /* Instruction pointer */

    pub fn increment_instruction_pointer(&mut self, n: i32){
        self.instruction_pointer += n;
    }

    pub fn set_instruction_pointer(&mut self, value: i32){
        self.instruction_pointer = value;
    }

    pub fn get_instruction_pointer(&self) -> usize{
        self.instruction_pointer as usize
    }

    /* Stack pointer */

    pub fn increment_stack_pointer(&mut self){
        self.stack_pointer += 1;
    }

    pub fn decrement_stack_pointer(&mut self){
        self.stack_pointer -= 1;
    }

    pub fn get_stack_pointer(&self) -> isize{
        self.stack_pointer
    }


    /* Exception register */

    pub fn set_exception_register(&mut self, flag: Exceptions, value: usize){
        let v = is_bit_set(flag as u8);
        if v != -1{
            self.exception_register[v as usize] = (value, true);
        }else{
            eprintln!("Error - flag passed had no bits set");
        }
    }

    pub fn get_exception_register(&self, flag: Exceptions) -> (usize, bool){
        let v = is_bit_set(flag as u8);
        if v != -1{
            return self.exception_register[v as usize];
        }else{
            eprintln!("Error - flag passed had no bits set");
        }

        return (0, false);
    }

    /* Exception flags */

    pub fn set_exception_flags(&mut self, value: Exceptions){
        self.exception_flags |= value as u8;
    }

    pub fn get_exception_flags(&self) -> u8{
        self.exception_flags
    }

    pub fn reset_exception_flags(&mut self){
        self.exception_flags = 0b0000_0000;
    }

    /* Jump pointer */

    pub fn set_jump_pointer(&mut self, value: i32){
        self.jump_pointer = value;
    }

    pub fn get_jump_pointer(&self) -> i32{
        self.jump_pointer
    }
}   

fn is_bit_set(b: u8) -> i8
{
    for i in 0..7{
        if (b & (1 << i)) != 0{
            return i;
        }
    }
    return -1;
}

/// # Exception flags
/// 
/// Contains the flags for the exception register as an enum for easy access.
pub enum Exceptions{
    /// DENORMAL - The floating point number is too small to be represented as a normal number
    DENORMAL = 0b0010_0000,
    /// ZERO_DIVIDE - The floating point number is zero and a divide by zero was attempted
    ZERO_DIVIDE = 0b0001_0000,
    /// NAN - The floating point number is not a number
    NAN = 0b0000_1000,

    /// PREFETCH_ABORT - The instruction is attempting to execute code not in the memory space
    PREFETCH_ABORT = 0b0000_0100,
    /// INVALID_ACCESS - The program attempted to access an invalid address in memory
    INVALID_ACCESS = 0b0000_0010,
    /// INVALID_OPERATION - The instruction does not exist or is invalid
    INVALID_OPERATION = 0b0000_0001,
}

impl From<u8> for Exceptions{
    fn from(b: u8) -> Self{
        match b{
            0b0010_0000 => Exceptions::DENORMAL,
            0b0001_0000 => Exceptions::ZERO_DIVIDE,
            0b0000_1000 => Exceptions::NAN,
            0b0000_0100 => Exceptions::PREFETCH_ABORT,
            0b0000_0010 => Exceptions::INVALID_ACCESS,
            0b0000_0001 => Exceptions::INVALID_OPERATION,
            _ => Exceptions::INVALID_OPERATION,
        }
    }
}

impl From<Exceptions> for u8{
    fn from(e: Exceptions) -> Self{
        match e{
            Exceptions::DENORMAL => 0b0010_0000,
            Exceptions::ZERO_DIVIDE => 0b0001_0000,
            Exceptions::NAN => 0b0000_1000,
            Exceptions::PREFETCH_ABORT => 0b0000_0100,
            Exceptions::INVALID_ACCESS => 0b0000_0010,
            Exceptions::INVALID_OPERATION => 0b0000_0001,
        }
    }
}