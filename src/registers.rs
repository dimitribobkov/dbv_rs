pub struct Registers{
    pub registers: [i32; 6], // 6 general purpose registers
    
    pub fp_registers: [f32; 6], // 6 general purpose floating point registers

    pub exception_register: u32,
    pub exception_flags: u32,

    pub jump_pointer: i32, // A special register to store the IP when jumping (so you can return from a jump)

    pub instruction_pointer: i32,
    pub stack_pointer: isize,
}

impl Registers{
    pub fn new() -> Self{
        Self{
            registers: [0; 6],

            fp_registers: [0.0; 6],

            exception_register: 0,
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

    pub fn set_exception_register(&mut self, value: u32){
        self.exception_register = value;
    }

    pub fn get_exception_register(&self) -> u32{
        self.exception_register
    }

    /* Exception flags */

    pub fn set_exception_flags(&mut self, value: u32){
        self.exception_flags = value;
    }

    pub fn get_exception_flags(&self) -> u32{
        self.exception_flags
    }

    /* Jump pointer */

    pub fn set_jump_pointer(&mut self, value: i32){
        self.jump_pointer = value;
    }

    pub fn get_jump_pointer(&self) -> i32{
        self.jump_pointer
    }
}