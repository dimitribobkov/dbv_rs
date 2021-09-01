mod instructions;
mod memory;
mod registers;
mod virtual_machine;

use instructions::{Instructions, OPCODE_TABLE};
use memory::Memory;
use registers::Registers;
use virtual_machine::VirtualMachine;

fn main() {
    let mut vm = VirtualMachine::new();
    vm.load_program("./test.bin").unwrap();

    vm.run().unwrap();
}
