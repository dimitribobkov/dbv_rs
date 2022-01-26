# dbv_rs
DBV virtual machine written in rust

This is a small project I've undertaken to implement my own custom ISA in rust. 

The project involves designing a ISA with common instructions, floating point and exceptions. So far, this project has all 3 implemented however
it is still a work in progress as exceptions are not all currently functional, and floating point requires testing.

The instruction set can be found within src/instructions.rs, however this will be better documented in future.

This project also includes a virtual machine that runs on top of the ISA. The virtual machine implements all the functions of the VM and includes
registers (which are 32-bit for both floating and non-floating point) as well as a memory controller which implements RAM and the stack. This VM will be
further developed to see how far I can push the project.


The next goal once the ISA is stabilized and fully functional will be to write a compiler from a language such as C to readable code.
