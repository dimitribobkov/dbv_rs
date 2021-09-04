use crate::{Instructions, Memory, OPCODE_TABLE, Registers};

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
                self.memory.push_to_stack(final_value, self.registers.get_stack_pointer());
                self.registers.increment_stack_pointer();
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
                self.registers.set_instruction_pointer(self.registers.jump_pointer);
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
            
            _ => {
                println!("Warning: instruction {:?} has not been implemented!", instruction);
            }
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

        
        self.registers.jump_pointer = self.registers.get_instruction_pointer() as i32;
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


        self.registers.jump_pointer = self.registers.get_instruction_pointer() as i32;
        self.registers.set_instruction_pointer(addr as i32);
        self.has_jumped = true;
    }
}

