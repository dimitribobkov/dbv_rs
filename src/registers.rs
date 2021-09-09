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

    pub fn set_exception_register(&mut self, flag: u8, value: usize){
        let v = is_bit_set(flag);
        if v != -1{
            self.exception_register[v as usize] = (value, true);
        }else{
            eprintln!("Error - flag passed had no bits set");
        }
    }

    pub fn get_exception_register(&self, flag: u8) -> (usize, bool){
        let v = is_bit_set(flag);
        if v != -1{
            return self.exception_register[v as usize];
        }else{
            eprintln!("Error - flag passed had no bits set");
        }

        return (0, false);
    }

    /* Exception flags */

    pub fn set_exception_flags(&mut self, value: u8){
        self.exception_flags |= value;
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