use crate::{Instructions, Memory, OPCODE_TABLE, Registers};

use std::fs::read;

pub struct VirtualMachine{
    registers: Registers,
    memory: Memory,

    program: Vec::<(Instructions, Vec::<u32>)>,

    running: bool,

/* Runtime Params */

    has_jumped: bool,
}

impl VirtualMachine{
    pub fn new() -> Self{
        Self{
            registers: Registers::new(),
            memory: Memory::new(),
            program: Vec::<(Instructions, Vec::<u32>)>::new(),
            running: true,
            has_jumped: false,
        }
    }

    pub fn load_program(&mut self, path: &str) -> std::io::Result<()>{
        let result = read(path)?;
        println!("Loading program...");

        let mut i = 0;
        while i < result.len(){
            for value in OPCODE_TABLE{
                if value.0 == Instructions::from(result[i]){
                    let mut instr_params = Vec::<u32>::new();
                    for n in 0..(value.1 as usize){
                        instr_params.push(result[(i + 1) + n] as u32);
                    }

                    self.program.push((Instructions::from(result[i]), instr_params));
                    i += value.1 as usize;
                    break;
                }
            }

            i += 1;
        }

        println!("Loaded program!");

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
            println!("Registers: {:?}", self.registers.registers);

        }

        println!("Registers at the end of running: {:?}", self.registers.registers);

        Ok(())
    }

    pub fn fetch(&self) -> &(Instructions, Vec<u32>){
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
                self.registers.set_register(params[0] as usize, self.registers.get_register(params[1] as usize));
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
                self.registers.set_register(params[0] as usize, self.memory.read_byte(final_value as usize) as i32);
            },
            Instructions::SD => {
                let final_value = params[0] << 8 | params[1];
                self.memory.write_byte(final_value as usize, self.registers.get_register(params[1] as usize) as u8);
            },
            Instructions::LDHW => {
                let final_value = params[1] << 8 | params[2];
                self.registers.set_register(params[0] as usize, self.memory.read_half_word(final_value as usize) as i32);
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
            Instructions::JNZ => {
                if self.registers.get_register(params[0] as usize) != 0{
                    let final_value = (params[1] << 8 | params[2]) as usize;
                    
                    self.handle_jump(final_value);
                }
            },
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

        
        
        self.registers.set_instruction_pointer(addr);
        self.has_jumped = true;
    }
}