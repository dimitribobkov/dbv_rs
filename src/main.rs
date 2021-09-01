mod instructions;
mod memory;
mod registers;
mod virtual_machine;

use instructions::{Instructions, opcode_table};
use memory::Memory;
use registers::Registers;
use virtual_machine::VirtualMachine;

fn main() {
    let mut vm = VirtualMachine::new();
    vm.load_program("./test.bin");

    vm.run().unwrap();
}
