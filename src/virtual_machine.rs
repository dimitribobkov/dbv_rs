use crate::{Instructions, Memory, OPCODE_TABLE, Registers, Exceptions};

use std::fs::read;


pub struct VirtualMachine{
    registers: Registers,
    memory: Memory,

    program: Vec::<(Instructions, Vec::<i32>)>,

    running: bool,

/* Runtime Params */

    has_jumped: bool,
}

impl VirtualMachine{
    pub fn new() -> Self{
        Self{
            registers: Registers::new(),
            memory: Memory::new(),
            program: Vec::<(Instructions, Vec::<i32>)>::new(),
            running: true,
            has_jumped: false,
        }
    }

    pub fn load_program(&mut self, path: &str) -> std::io::Result<()>{
        let result = read(path)?;

        println!("Loading program...");
        
        

        let mut i = 0;
        while i < result.len(){
            let instruction = Instructions::from(result[i]);
            for value in OPCODE_TABLE{
                if value.0 == instruction{
                    let mut instr_params = Vec::<i32>::new();
                    for n in 0..(value.1 as usize){
                        let signed_value =  result[(i + 1) + n] as u8;
                        instr_params.push(signed_value as i32);
                    }

                    self.program.push((instruction, instr_params));
                    i += value.1 as usize;
                    break;
                }
            }


            i += 1;
        }

        println!("Loaded program!");

        println!("Program: {:?}", self.program);

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), &'static str>{
        'running: loop {
            self.eval();

            if !self.has_jumped{
                self.registers.increment_instruction_pointer(1);
            }

            if !self.running{
                println!("HLT! Finished running!");
                break 'running;
            }
        }

        println!("Registers at the end of running: {:?}", self.registers.registers);
        println!("FP_Registers at the end of running: {:?}", self.registers.fp_registers);

        println!("\nPC: {}\nSP: {}", self.registers.get_instruction_pointer(), self.registers.get_stack_pointer());

        Ok(())
    }

    pub fn fetch(&self) -> &(Instructions, Vec<i32>){
        &self.program[self.registers.get_instruction_pointer()]
    }

    pub fn eval(&mut self){
        self.has_jumped = false;

        let data = self.fetch();
        let instruction = data.0;
        let params = data.1.clone(); // Sorry :(

        
        match instruction{
            Instructions::HLT => {
                self.running = false;
            },
            Instructions::PSH => {
                let final_value = params[0] << 24 | params[1] << 16 | params[2] << 8 | params[3];
                self.registers.increment_stack_pointer();
                self.memory.push_to_stack(final_value, self.registers.get_stack_pointer());
            },
            Instructions::POP => {
                self.registers.set_register(params[0] as usize, self.memory.pop_from_stack(self.registers.get_stack_pointer()) as i32); 
                self.registers.decrement_stack_pointer();
            },
            Instructions::SET => {
                let final_value = params[1] << 24 | params[2] << 16 | params[3] << 8 | params[4];
                self.registers.set_register(params[0] as usize, final_value as i32);
            },
            Instructions::MOV => {
                self.registers.set_register(params[1] as usize, self.registers.get_register(params[0] as usize));
            },
            Instructions::SLT => {
                self.registers.set_register(params[0] as usize, 
                    (self.registers.get_register(params[1] as usize) < self.registers.get_register(params[2] as usize)) as i32);
            }
            Instructions::ADD => {
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) + self.registers.get_register(params[2] as usize));
            },
            Instructions::SUB => {
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) - self.registers.get_register(params[2] as usize));
            },
            Instructions::MUL => {
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) * self.registers.get_register(params[2] as usize));
            },
            Instructions::DIV => {
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) / self.registers.get_register(params[2] as usize));
            },
            Instructions::ADDI => {
                let final_value = params[2] << 24 | params[3] << 16 | params[4] << 8 | params[5];
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) + final_value as i32);
            },
            Instructions::SUBI => {
                let final_value = params[2] << 24 | params[3] << 16 | params[4] << 8 | params[5];
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) - final_value as i32);
            },
            Instructions::MULI => {
                let final_value = params[2] << 24 | params[3] << 16 | params[4] << 8 | params[5];
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) * final_value as i32);
            },
            Instructions::DIVI => {
                let final_value = params[2] << 24 | params[3] << 16 | params[4] << 8 | params[5];
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) / final_value as i32);
            },
            Instructions::AND => {
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) & self.registers.get_register(params[2] as usize));
            },
            Instructions::OR => {
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) | self.registers.get_register(params[2] as usize));
            },
            Instructions::SL => {
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) << self.registers.get_register(params[2] as usize));
            },
            Instructions::SR => {
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) >> self.registers.get_register(params[2] as usize));
            },
            Instructions::SLI => {
                let final_value = params[2] << 24 | params[3] << 16 | params[4] << 8 | params[5];
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) << final_value as i32);
            },
            Instructions::SRI => {
                let final_value = params[2] << 24 | params[3] << 16 | params[4] << 8 | params[5];
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[1] as usize) >> final_value as i32);
            },
            Instructions::SAL => {
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[0] as usize) << 1);
            },
            Instructions::SAR => {
                self.registers.set_register(params[0] as usize, 
                    self.registers.get_register(params[0] as usize) >> 1);
            },
            Instructions::LD => {
                let final_value = params[1] << 8 | params[2];
                self.registers.set_register(params[0] as usize, self.memory.read_byte(final_value as usize) as i8 as i32);
            },
            Instructions::SD => {
                let final_value = params[0] << 8 | params[1];
                self.memory.write_byte(final_value as usize, self.registers.get_register(params[1] as usize) as u8);
            },
            Instructions::LDHW => {
                let final_value = params[1] << 8 | params[2];
                self.registers.set_register(params[0] as usize, self.memory.read_half_word(final_value as usize) as i16 as i32);
            },
            Instructions::SDHW => {
                let final_value = params[0] << 8 | params[1];
                self.memory.write_half_word(final_value as usize, self.registers.get_register(params[2] as usize) as u16);
            },
            Instructions::LDW => {
                let final_value = params[1] << 8 | params[2];
                self.registers.set_register(params[0] as usize, self.memory.read_word(final_value as usize) as i32);
            },
            Instructions::SDW => {
                let final_value = params[0] << 8 | params[1];
                self.memory.write_word(final_value as usize, self.registers.get_register(params[2] as usize) as u32);
            },

            Instructions::IF => {
                let value = params[1] << 24 | params[2] << 16 | params[3] << 8 | params[4];
                if self.registers.get_register(params[0] as usize) == value as i32{
                    let final_value = (params[5] << 8 | params[6]) as usize;

                    self.handle_jump(final_value);
                }
            },
            Instructions::IFN => {
                let value = params[1] << 24 | params[2] << 16 | params[3] << 8 | params[4];
                if self.registers.get_register(params[0] as usize) != value as i32{
                    let final_value = (params[5] << 8 | params[6]) as usize;

                    self.handle_jump(final_value);
                }
            },

            Instructions::JMP => {
                let final_value = (params[0] << 8 | params[1]) as usize;
                self.handle_jump(final_value);
            },

            Instructions::JNZ => {
                if self.registers.get_register(params[0] as usize) != 0{
                    let final_value = (params[1] << 8 | params[2]) as usize;
                    
                    self.handle_jump(final_value);
                }
            },


            Instructions::IFR => {
                let value = params[1] << 24 | params[2] << 16 | params[3] << 8 | params[4];
                if self.registers.get_register(params[0] as usize) == value as i32{
                    let final_value = params[5] as i8 as isize;

                    self.handle_relative_jump(final_value);
                }
            },
            Instructions::IFNR => {
                let value = params[1] << 24 | params[2] << 16 | params[3] << 8 | params[4];

                if self.registers.get_register(params[0] as usize) != value{
                    let final_value = params[5] as i8 as isize;

                    self.handle_relative_jump(final_value);
                }
            },
            Instructions::JMPR => {
                let final_value = params[0] as i8 as isize;
                
                self.handle_relative_jump(final_value);
            },
            Instructions::JNZR => {
                if self.registers.get_register(params[0] as usize) != 0{
                    let final_value = params[1] as i32 as isize;
                    
                    self.handle_relative_jump(final_value);
                }
            },

            Instructions::RET => {
                self.registers.set_instruction_pointer(self.registers.get_jump_pointer());
            },

            Instructions::SETF => {
                // From bits converts our regular hex value to a proper floating point. Must be a u32 to work, so we cast it.
                let value = f32::from_bits((params[1] << 24 | params[2] << 16 | params[3] << 8 | params[4]) as u32);
                self.registers.set_f_register(params[0] as usize, value);     
            },
            
            Instructions::MOVF => {
                self.registers.set_f_register(params[0] as usize, self.registers.get_f_register(params[1] as usize));
            },

            Instructions::MOVFI => {
                self.registers.set_register(params[1] as usize, self.registers.get_f_register(params[0] as usize) as i32);
            },

            Instructions::MOVIF => {
                self.registers.set_f_register(params[1] as usize, self.registers.get_register(params[0] as usize) as f32);
            },

            Instructions::ADDF => {
                let value = self.registers.get_f_register(params[1] as usize) + self.registers.get_f_register(params[2] as usize);
                
                // Set exception flag if the result is NaN
                if value.is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
                
                self.registers.set_f_register(params[0] as usize, value);
            },

            Instructions::SUBF => {
                let value = self.registers.get_f_register(params[1] as usize) - self.registers.get_f_register(params[2] as usize);
                
                // Set exception flag if the result is NaN
                if value.is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }

                self.registers.set_f_register(params[0] as usize, value);
            },

            Instructions::MULF => {
                let value = self.registers.get_f_register(params[1] as usize) * self.registers.get_f_register(params[2] as usize);
                
                // Set exception flag if the result is NaN
                if value.is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
                
                self.registers.set_f_register(params[0] as usize, value);
            },

            Instructions::DIVF => {
                // Set exception if dividing by 0
                if self.registers.get_f_register(params[2] as usize) == 0.0{
                    self.registers.set_exception_flags(Exceptions::ZERO_DIVIDE);
                }

                // Set exception if NaN
                if self.registers.get_f_register(params[1] as usize).is_nan() || self.registers.get_f_register(params[2] as usize).is_nan(){
                    self.registers.set_exception_flags(Exceptions::NAN);
                }

                let value = self.registers.get_f_register(params[1] as usize) / self.registers.get_f_register(params[2] as usize);
                self.registers.set_f_register(params[0] as usize, value);
            },
            
            Instructions::ADDFI => {
                let value = self.registers.get_f_register(params[1] as usize) + f32::from_bits((params[2] << 24 | params[3] << 16 | params[4] << 8 | params[5]) as u32);
                
                // Set exception flag if the result is NaN
                if value.is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
                
                self.registers.set_f_register(params[0] as usize, value);
            },

            Instructions::SUBFI => {
                let value = self.registers.get_f_register(params[1] as usize) - f32::from_bits((params[2] << 24 | params[3] << 16 | params[4] << 8 | params[5]) as u32);
               
                // Set exception flag if the result is NaN
                if value.is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
               
                self.registers.set_f_register(params[0] as usize, value);
            },

            Instructions::MULFI => {
                let value = self.registers.get_f_register(params[1] as usize) * f32::from_bits((params[2] << 24 | params[3] << 16 | params[4] << 8 | params[5]) as u32);
                
                // Set exception flag if the result is NaN
                if value.is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
                
                self.registers.set_f_register(params[0] as usize, value);
            },

            Instructions::DIVFI => {
                // Set exception if dividing by 0
                if (f32::from_bits((params[2] << 24 | params[3] << 16 | params[4] << 8 | params[5]) as u32)) == 0.0{
                    self.registers.set_exception_flags(Exceptions::ZERO_DIVIDE);
                }
                

                let value = self.registers.get_f_register(params[1] as usize) / f32::from_bits((params[2] << 24 | params[3] << 16 | params[4] << 8 | params[5]) as u32);
                
                // Set exception flag if the result is NaN
                if value.is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
                
                self.registers.set_f_register(params[0] as usize, value);
            },

            Instructions::LDWF => {
                let final_value = params[1] << 8 | params[2];
                
                self.registers.set_f_register(params[0] as usize, f32::from_bits(self.memory.read_word(final_value as usize)));
            },
            Instructions::SDWF => {
                let final_value = params[0] << 8 | params[1];
                let data_as_int: u32 = unsafe { std::mem::transmute(self.registers.get_f_register(params[2] as usize)) };
                self.memory.write_word(final_value as usize, data_as_int);
            },

            Instructions::IFF => {
                let value = params[1] << 24 | params[2] << 16 | params[3] << 8 | params[4];

                // Set exception flag if the result is NaN
                if f32::from_bits(value as u32).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }

                if self.registers.get_f_register(params[0] as usize) == f32::from_bits(value as u32){
                    let final_value = (params[5] << 8 | params[6]) as usize;

                    self.handle_jump(final_value);
                }
            },
            Instructions::IFNF => {
                let value = params[1] << 24 | params[2] << 16 | params[3] << 8 | params[4];

                // Set exception flag if the result is NaN
                if f32::from_bits(value as u32).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }

                if self.registers.get_f_register(params[0] as usize) != f32::from_bits(value as u32){
                    let final_value = (params[5] << 8 | params[6]) as usize;

                    self.handle_jump(final_value);
                }
            },

            Instructions::IFRF => {
                let value = params[1] << 24 | params[2] << 16 | params[3] << 8 | params[4];

                // Set exception flag if the result is NaN
                if f32::from_bits(value as u32).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }

                if self.registers.get_f_register(params[0] as usize) == f32::from_bits(value as u32){
                    let final_value = params[5] as i8 as isize;

                    self.handle_relative_jump(final_value);
                }
            },
            Instructions::IFNRF => {
                let value = params[1] << 24 | params[2] << 16 | params[3] << 8 | params[4];

                // Set exception flag if the result is NaN
                if f32::from_bits(value as u32).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }

                if self.registers.get_f_register(params[0] as usize) != f32::from_bits(value as u32){
                    let final_value = params[5] as i8 as isize;

                    self.handle_relative_jump(final_value);
                }
            },

            Instructions::EQ => {
                if self.registers.get_register(params[1] as usize) == self.registers.get_register(params[2] as usize){
                    self.registers.set_register(params[0] as usize, 1);
                }else{
                    self.registers.set_register(params[0] as usize, 0);
                }
            },

            Instructions::NEQ => {
                if self.registers.get_register(params[1] as usize) != self.registers.get_register(params[2] as usize){
                    self.registers.set_register(params[0] as usize, 1);
                }else{
                    self.registers.set_register(params[0] as usize, 0);
                }
            },

            Instructions::LEQ => {
                if self.registers.get_register(params[1] as usize) <= self.registers.get_register(params[2] as usize){
                    self.registers.set_register(params[0] as usize, 1);
                }else{
                    self.registers.set_register(params[0] as usize, 0);
                }
            },

            Instructions::GEQ => {
                if self.registers.get_register(params[1] as usize) >= self.registers.get_register(params[2] as usize){
                    self.registers.set_register(params[0] as usize, 1);
                }else{
                    self.registers.set_register(params[0] as usize, 0);
                }
            },

            Instructions::LT => {
                if self.registers.get_register(params[1] as usize) < self.registers.get_register(params[2] as usize){
                    self.registers.set_register(params[0] as usize, 1);
                }else{
                    self.registers.set_register(params[0] as usize, 0);
                }
            },

            Instructions::GT => {
                if self.registers.get_register(params[1] as usize) > self.registers.get_register(params[2] as usize){
                    self.registers.set_register(params[0] as usize, 1);
                }else{
                    self.registers.set_register(params[0] as usize, 0);
                }
            },

            Instructions::EQF => {
                if self.registers.get_f_register(params[1] as usize) == self.registers.get_f_register(params[2] as usize){
                    self.registers.set_f_register(params[0] as usize, 1.0);
                }else{
                    self.registers.set_f_register(params[0] as usize, 0.0);
                }
            },

            Instructions::NEQF => {
                if self.registers.get_f_register(params[1] as usize) != self.registers.get_f_register(params[2] as usize){
                    self.registers.set_f_register(params[0] as usize, 1.0);
                }else{
                    self.registers.set_f_register(params[0] as usize, 0.0);
                }
            },

            Instructions::LEQF => {
                if self.registers.get_f_register(params[1] as usize) <= self.registers.get_f_register(params[2] as usize){
                    self.registers.set_f_register(params[0] as usize, 1.0);
                }else{
                    self.registers.set_f_register(params[0] as usize, 0.0);
                }
            },

            Instructions::GEQF => {
                if self.registers.get_f_register(params[1] as usize) >= self.registers.get_f_register(params[2] as usize){
                    self.registers.set_f_register(params[0] as usize, 1.0);
                }else{
                    self.registers.set_f_register(params[0] as usize, 0.0);
                }
            },

            Instructions::LTF => {
                if self.registers.get_f_register(params[1] as usize) < self.registers.get_f_register(params[2] as usize){
                    self.registers.set_f_register(params[0] as usize, 1.0);
                }else{
                    self.registers.set_f_register(params[0] as usize, 0.0);
                }
            },

            Instructions::GTF => {
                if self.registers.get_f_register(params[1] as usize) > self.registers.get_f_register(params[2] as usize){
                    self.registers.set_f_register(params[0] as usize, 0.0);
                }else{
                    self.registers.set_f_register(params[0] as usize, 0.0);
                }
            },

            Instructions::PSHR => {
                let value = self.registers.get_register(params[0] as usize);
                self.registers.increment_stack_pointer();
                self.memory.push_to_stack(value, self.registers.stack_pointer);
            },

            Instructions::REC => {
                self.registers.set_f_register(params[0] as usize, self.registers.get_f_register(params[1] as usize).recip());

                if self.registers.get_f_register(params[0] as usize).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
            },

            Instructions::SQRT => {
                self.registers.set_f_register(params[0] as usize, self.registers.get_f_register(params[1] as usize).sqrt());

                if self.registers.get_f_register(params[0] as usize).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
            },

            Instructions::RND => {
                self.registers.set_f_register(params[0] as usize, self.registers.get_f_register(params[1] as usize).round());

                if self.registers.get_f_register(params[0] as usize).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
            },

            Instructions::SIN => {
                self.registers.set_f_register(params[0] as usize, self.registers.get_f_register(params[1] as usize).sin());

                if self.registers.get_f_register(params[0] as usize).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
            },

            Instructions::COS => {
                self.registers.set_f_register(params[0] as usize, self.registers.get_f_register(params[1] as usize).cos());

                if self.registers.get_f_register(params[0] as usize).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
            },

            Instructions::TAN => {
                self.registers.set_f_register(params[0] as usize, self.registers.get_f_register(params[1] as usize).tan());

                if self.registers.get_f_register(params[0] as usize).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
            },

            Instructions::ASIN => {
                self.registers.set_f_register(params[0] as usize, self.registers.get_f_register(params[1] as usize).asin());

                if self.registers.get_f_register(params[0] as usize).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
            },

            Instructions::ACOS => {
                self.registers.set_f_register(params[0] as usize, self.registers.get_f_register(params[1] as usize).acos());

                if self.registers.get_f_register(params[0] as usize).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
            },

            Instructions::ATAN => {
                self.registers.set_f_register(params[0] as usize, self.registers.get_f_register(params[1] as usize).atan());

                if self.registers.get_f_register(params[0] as usize).is_nan() {
                    self.registers.set_exception_flags(Exceptions::NAN);
                }
            },

            Instructions::SEX => {
                let flag = params[0] as u8;
                let addr = (params[1] as u16) << 8 | params[2] as u16;
                self.registers.set_exception_register(Exceptions::from(flag), addr as usize);
                println!("Exception ({:#010b}) at address {:#x?}", flag, addr);
            },

            _ => {
                println!("Error: instruction {:?} has not been implemented!", instruction);
                self.registers.set_exception_flags(Exceptions::INVALID_OPERATION);
            }
        }

        // Exception handling - the lower the byte, the more important it is
        if self.registers.get_exception_flags() != 0{
            let mut handled = false;
            for i in 0..7{
                let bit = self.registers.get_exception_flags() & (1 << i);
                // Check the bit is set. If it is, handle the exception. Else, continue
                if bit != 0{
                    let reg = self.registers.get_exception_register(Exceptions::from(bit));
                    if !reg.1{ // If no handler is set, just continue
                        continue;
                    }

                    self.handle_jump(reg.0 as usize);
                    self.has_jumped = true;
                    handled = true;
                    break;
                }
                
            }
            if !handled{
                eprintln!("Error: Exception not handled! (Flag: {:#010b})", self.registers.get_exception_flags());
                panic!("Exception not handled");
            }

            self.registers.reset_exception_flags();
            
        }
    }

    // Automatically converts jumps into the appropriate format to be used by the VM
    fn handle_jump(&mut self, mut addr: usize){
        let mut param_count = 0;
        let mut i = 0;
        for val in self.program.clone(){
            if i == addr{
                break;
            }
            param_count += val.1.len();
            i += 1;
            i += val.1.len();

        }

        addr -= param_count;

        
        self.registers.set_jump_pointer(self.registers.get_instruction_pointer() as i32);
        self.registers.set_instruction_pointer(addr as i32);
        self.has_jumped = true;
    }

    // Automatically converts jumps into the appropriate format to be used by the VM
    fn handle_relative_jump(&mut self, mut addr: isize){
        let mut param_count: isize = 0;
        let mut i: isize = 0;
        
        for val in self.program.clone(){
            if self.registers.get_instruction_pointer() as isize == i - param_count{
                break;
            }
            param_count += val.1.len() as isize;

            i += 1;
            i += val.1.len() as isize;

        }
        
        
        addr += i;

        i = 0;
        param_count = 0;

        for val in self.program.clone(){
            if i == addr{
                break;
            }
            param_count += val.1.len() as isize;
            i += 1;
            i += val.1.len() as isize;

        }

        if addr > 0{                
            addr -= param_count;
        }else{               
            
            addr += param_count;
        }


        self.registers.set_jump_pointer(self.registers.get_instruction_pointer() as i32);
        self.registers.set_instruction_pointer(addr as i32);
        self.has_jumped = true;
    }
}

