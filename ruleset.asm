; Required in any asm files you want to compile for dbv
#subruledef register
{
    a => 0x0
    b => 0x1
    c => 0x2
    d => 0x3
    e => 0x4
    f => 0x5
}

#ruledef
{
    
    hlt                                                                     => 0x00

    push {value: i32}                                                       => 0x01 @ reg`8 ; push value to the stack
    pop r_{reg: register}                                                   => 0x02 @ reg`8 ; pop to reg
    load r_{reg: register}, {value: i32}                                    => 0x03 @ reg`8 @ value ; Set the contents of reg to value (32 bit)
    move r_{reg_a: register}, r_{reg_b: register}                           => 0x04 @ reg_a`8 @ reg_b`8 ; Copy the contents from reg b to reg a
    slt r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x05  @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_a = reg_b < reg_c
    
    ; Arithmetic
    add r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x06 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b + reg_c, storing the results in reg a
    sub r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x07 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b - reg_c, storing the results in reg a
    mul r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x08 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b * reg_c, storing the results in reg a
    div r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x09 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b / reg_c, storing the results in reg a

    addi r_{reg_a: register}, r_{reg_b: register}, {value: i32}             => 0x0A @ reg_a`8 @ reg_b`8 @ value ; reg_b + value, storing the results in reg a
    subi r_{reg_a: register}, r_{reg_b: register}, {value: i32}             => 0x0B @ reg_a`8 @ reg_b`8 @ value ; reg_b - value, storing the results in reg a
    muli r_{reg_a: register}, r_{reg_b: register}, {value: i32}             => 0x0C @ reg_a`8 @ reg_b`8 @ value ; reg_b * value, storing the results in reg a
    divi r_{reg_a: register}, r_{reg_b: register}, {value: i32}             => 0x0D @ reg_a`8 @ reg_b`8 @ value ; reg_b / value, storing the results in reg a

    and r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x0E @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b & reg_c, storing the results in reg a
    or r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}        => 0x0F @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b | reg_c, storing the results in reg a
    sl r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}        => 0x10 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b << reg_c, storing the results in reg a
    sr r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}        => 0x11 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b >> reg_c, storing the results in reg a
    
    sli r_{reg_a: register}, r_{reg_b: register}, {value: i32}              => 0x12 @ reg_a`8 @ reg_b`8 @ value ; reg_b << value, storing the results in reg a
    sri r_{reg_a: register}, r_{reg_b: register}, {value: i32}              => 0x13 @ reg_a`8 @ reg_b`8 @ value ; reg_b >> value, storing the results in reg a
    
    sal r_{reg: register}                                                   => 0x14 @ reg`8 ; reg = reg << 1
    sar r_{reg: register}                                                   => 0x15 @ reg`8 ; reg = reg >> 1
    
    ; Memory Operations
    lb r_{reg: register}, {mem_addr: i16}                                   => 0x16 @ reg`8 @ mem_addr ; Load byte (8) bit) into reg from mem_addr 
    sb {mem_addr: i16}, r_{reg: register}                                   => 0x17 @ mem_addr @ reg`8 ; Store byte (8 bit) from reg into mem_addr 

    lhw r_{reg: register}, {mem_addr: i16}                                  => 0x18 @ reg`8 @ mem_addr ; Load halfword (16 bit) into reg from mem_addr 
    shw {mem_addr: i16}, r_{reg: register}                                  => 0x19 @ mem_addr @ reg ; Store hlafword (16 bit) from reg into mem_addr 
    
    lw r_{reg: register}, {mem_addr: i16}                                   => 0x1A @ reg`8 @ mem_addr ; Load word (32 bit) into reg from mem_addr 
    sw {mem_addr: i16}, r_{reg: register}                                   => 0x1B @ mem_addr @ reg`8 ; Store word (32 bit) from reg into mem_addr 

    ; Jumps and branching (no relative jumps yet :))
    if r_{reg: register}, {value: i32}, {addr: i16}                         => 0x1C @ reg`8 @ value @ {REL = (addr - pc), assert(REL > 127 && REL < -128), REL[15:0]} ; if reg == val, goto rel_addr
    ifn r_{reg: register}, {value: i32}, {addr: i16}                        => 0x1D @ reg`8 @ value @ {REL = (addr - pc), assert(REL > 127 && REL < -128), REL[15:0]} ; if reg != val, goto addr
    jmp {addr: i16}                                                         => 0x1E @ {REL = (addr - pc), assert(REL > 127 && REL < -128), REL[15:0]} ; Jumps to addr
    jnz r_{reg: register}, {addr: i16}                                      => 0x1F @ reg`8 @ {REL = (addr - pc), assert(REL > 127 && REL < -128), REL[15:0]} ; Jumps to addr

    if r_{reg: register}, {value: i32}, {rel_addr: i8}                      => 0x20 @ reg`8 @ value @ {REL = (rel_addr - pc), assert(REL <= 127 && REL >= -128), REL[7:0]} ; if reg == val, goto rel_addr
    ifn r_{reg: register}, {value: i32}, {rel_addr: i8}                     => 0x21 @ reg`8 @ value @ {REL = (rel_addr - pc), assert(REL <= 127 && REL >= -128), REL[7:0]} ; if reg != val, goto rel_addr
    jmp {rel_addr: i8}                                                      => 0x22 @ {REL = (rel_addr - pc), assert(REL <= 127 && REL >= -128), REL[7:0]} ; Jumps relatively to addr
    jnz r_{reg: register}, {rel_addr: i8}                                   => 0x23 @ reg`8 @ {REL = (rel_addr - pc), assert(REL <= 127 && REL >= -128), REL[7:0]} ; Jumps relatively to addr

    ret                                                                     => 0x24 ; Return from a jump
}