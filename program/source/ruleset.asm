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

    push {value: i32}                                                       => 0x01 @ value ; push value to the stack
    push r_{reg: register}                                                  => 0x45 @ reg`8 ; push value to the stack
    pop r_{reg: register}                                                   => 0x02 @ reg`8 ; pop to reg
    
    load r_{reg: register}, {value: i32}                                    => 0x03 @ reg`8 @ value ; Set the contents of reg to value (32 bit)
    move r_{reg_a: register}, r_{reg_b: register}                           => 0x04 @ reg_a`8 @ reg_b`8 ; Copy the contents from reg a to reg b
    slt r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x05  @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_a = reg_b < reg_c
    
    ; Arithmetic
    add r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x06 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b + reg_c, storing the results in reg a
    sub r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x07 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b - reg_c, storing the results in reg a
    mul r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x08 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b * reg_c, storing the results in reg a
    div r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x09 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b / reg_c, storing the results in reg a

    add r_{reg_a: register}, r_{reg_b: register}, {value: i32}             => 0x0A @ reg_a`8 @ reg_b`8 @ value ; reg_b + value, storing the results in reg a
    sub r_{reg_a: register}, r_{reg_b: register}, {value: i32}             => 0x0B @ reg_a`8 @ reg_b`8 @ value ; reg_b - value, storing the results in reg a
    mul r_{reg_a: register}, r_{reg_b: register}, {value: i32}             => 0x0C @ reg_a`8 @ reg_b`8 @ value ; reg_b * value, storing the results in reg a
    div r_{reg_a: register}, r_{reg_b: register}, {value: i32}             => 0x0D @ reg_a`8 @ reg_b`8 @ value ; reg_b / value, storing the results in reg a

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

    ; Jumps and branching
    if r_{reg: register}, {value: i32}, {addr: i16}                         => 0x1C @ reg`8 @ value @ {REL = (addr - pc), assert(REL > 127 && REL < -128), REL[15:0]} ; if reg == val, goto rel_addr
    ifn r_{reg: register}, {value: i32}, {addr: i16}                        => 0x1D @ reg`8 @ value @ {REL = (addr - pc), assert(REL > 127 && REL < -128), REL[15:0]} ; if reg != val, goto addr
    jmp {addr: i16}                                                         => 0x1E @ {REL = (addr - pc), assert(REL > 127 && REL < -128), REL[15:0]} ; Jumps to addr
    jnz r_{reg: register}, {addr: i16}                                      => 0x1F @ reg`8 @ {REL = (addr - pc), assert(REL > 127 && REL < -128), REL[15:0]} ; Jumps to addr

    if r_{reg: register}, {value: i32}, {rel_addr: i8}                      => 0x20 @ reg`8 @ value @ {REL = (rel_addr - pc), assert(REL <= 127 && REL >= -128), REL[7:0]} ; if reg == val, goto rel_addr
    ifn r_{reg: register}, {value: i32}, {rel_addr: i8}                     => 0x21 @ reg`8 @ value @ {REL = (rel_addr - pc), assert(REL <= 127 && REL >= -128), REL[7:0]} ; if reg != val, goto rel_addr
    jmp {rel_addr: i8}                                                      => 0x22 @ {REL = (rel_addr - pc), assert(REL <= 127 && REL >= -128), REL[7:0]} ; Jumps relatively to addr
    jnz r_{reg: register}, {rel_addr: i8}                                   => 0x23 @ reg`8 @ {REL = (rel_addr - pc), assert(REL <= 127 && REL >= -128), REL[7:0]} ; Jumps relatively to addr


    ; Jumps based on floating point
    if r_{reg: register}, {value: i32}, {addr: i16}                         => 0x33 @ reg`8 @ value @ {REL = (addr - pc), assert(REL > 127 && REL < -128), REL[15:0]} ; if reg == val, goto rel_addr
    ifn r_{reg: register}, {value: i32}, {addr: i16}                        => 0x34 @ reg`8 @ value @ {REL = (addr - pc), assert(REL > 127 && REL < -128), REL[15:0]} ; if reg != val, goto addr
    jnz r_{reg: register}, {addr: i16}                                      => 0x37 @ reg`8 @ {REL = (addr - pc), assert(REL > 127 && REL < -128), REL[15:0]} ; Jumps to addr

    if f_r_{reg: register}, {value: i32}, {rel_addr: i8}                    => 0x35 @ reg`8 @ value @ {REL = (rel_addr - pc), assert(REL <= 127 && REL >= -128), REL[7:0]} ; if reg == val, goto rel_addr
    ifn f_r_{reg: register}, {value: i32}, {rel_addr: i8}                   => 0x36 @ reg`8 @ value @ {REL = (rel_addr - pc), assert(REL <= 127 && REL >= -128), REL[7:0]} ; if reg != val, goto rel_addr
    jnz f_r_{reg: register}, {rel_addr: i8}                                 => 0x38 @ reg`8 @ {REL = (rel_addr - pc), assert(REL <= 127 && REL >= -128), REL[7:0]} ; Jumps relatively to addr


    ret                                                                     => 0x24 ; Return from a jump
    
    
    ; Floating point operations
    load f_r_{reg: register}, {value: s32}                                  => 0x25 @ reg`8 @ value ; Set the contents of reg to value (32 bit, fp32)
    move f_r_{reg_a: register}, f_r_{reg_b: register}                       => 0x26 @ reg_a`8 @ reg_b`8 ; Copy the contents from freg a to freg b
    move f_r_{reg_a: register}, r_{reg_b: register}                         => 0x27 @ reg_a`8 @ reg_b`8 ; Copy the contents from freg a to reg b
    move r_{reg_a: register}, f_r_{reg_b: register}                         => 0x28 @ reg_a`8 @ reg_b`8 ; Copy the contents from reg a to freg b

    add f_r_{reg_a: register}, f_r_{reg_b: register}, f_r_{reg_c: register} => 0x29 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b + reg_c, storing the results in reg a
    sub f_r_{reg_a: register}, f_r_{reg_b: register}, f_r_{reg_c: register} => 0x2A @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b - reg_c, storing the results in reg a
    mul f_r_{reg_a: register}, f_r_{reg_b: register}, f_r_{reg_c: register} => 0x2B @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b * reg_c, storing the results in reg a
    div f_r_{reg_a: register}, f_r_{reg_b: register}, f_r_{reg_c: register} => 0x2C @ reg_a`8 @ reg_b`8 @ reg_c`8 ; reg_b / reg_c, storing the results in reg a

    add f_r_{reg_a: register}, f_r_{reg_b: register}, {value: s32}          => 0x2D @ reg_a`8 @ reg_b`8 @ value ; reg_b + value, storing the results in reg a
    sub f_r_{reg_a: register}, f_r_{reg_b: register}, {value: s32}          => 0x2E @ reg_a`8 @ reg_b`8 @ value; reg_b - value, storing the results in reg a
    mul f_r_{reg_a: register}, f_r_{reg_b: register}, {value: s32}          => 0x2F @ reg_a`8 @ reg_b`8 @ value ; reg_b * value, storing the results in reg a
    div f_r_{reg_a: register}, f_r_{reg_b: register}, {value: s32}          => 0x30 @ reg_a`8 @ reg_b`8 @ value ; reg_b / value, storing the results in reg a

    lw f_r_{reg: register}, {mem_addr: i16}                                 => 0x31 @ reg`8 @ mem_addr ; Load word (32 bit) into reg from mem_addr 
    sw {mem_addr: i16}, f_r_{reg: register}                                 => 0x32 @ mem_addr @ reg`8 ; Store word (32 bit) from reg into mem_addr 


    ; Equality Testing
    eq r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}        => 0x39 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a == reg_b, reg_c = 1 else reg_c = 0
    neq r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x3A @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a != reg_b, reg_c = 1 else reg_c = 0
    leq r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x3B @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a <= reg_b, reg_c = 1 else reg_c = 0
    geq r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}       => 0x3C @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a >= reg_b, reg_c = 1 else reg_c = 0
    lt r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}        => 0x3D @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a < reg_b, reg_c = 1 else reg_c = 0
    gt r_{reg_a: register}, r_{reg_b: register}, r_{reg_c: register}        => 0x3E @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a > reg_b, reg_c = 1 else reg_c = 0

    ; Floating point Equality Testing
    eq f_r_{reg_a: register}, f_r_{reg_b: register}, f_r_{reg_c: register}  => 0x3F @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a == reg_b, reg_c = 1 else reg_c = 0
    neq f_r_{reg_a: register}, f_r_{reg_b: register}, f_r_{reg_c: register} => 0x40 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a != reg_b, reg_c = 1 else reg_c = 0
    leq f_r_{reg_a: register}, f_r_{reg_b: register}, f_r_{reg_c: register} => 0x41 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a <= reg_b, reg_c = 1 else reg_c = 0
    geq f_r_{reg_a: register}, f_r_{reg_b: register}, f_r_{reg_c: register} => 0x42 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a >= reg_b, reg_c = 1 else reg_c = 0
    lt f_r_{reg_a: register}, f_r_{reg_b: register}, f_r_{reg_c: register}  => 0x43 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a < reg_b, reg_c = 1 else reg_c = 0
    gt f_r_{reg_a: register}, f_r_{reg_b: register}, f_r_{reg_c: register}  => 0x44 @ reg_a`8 @ reg_b`8 @ reg_c`8 ; if reg_a > reg_b, reg_c = 1 else reg_c = 0
}   