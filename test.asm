#ruledef
{
    
    hlt                                         => 0x00
    push {value: s32}                           => 0x01 @ reg ; push value to the stack
    pop {reg: s8}                               => 0x02 @ reg ; pop to reg
    load {reg: s8}, {value: s32}                => 0x03 @ reg @ value ; Set the contents of reg to value (32 bit)
    move {reg_a: s8}, {reg_b: s8}               => 0x04 @ reg_a @ reg_b ; Copy the contents from reg b to reg a
    if {reg: s8}, {value: s32}, {addr: s16}     => 0x05 @ reg @ value @ addr ; if reg == val, goto addr
    ifn {reg: s8}, {value: s32}, {addr: s16}    => 0x06 @ reg @ value @ addr ; if reg != val, goto addr
    jmp {addr: s16}                             => 0x07 @ addr ; Jumps to addr
    slt {reg_a: s8}, {reg_b: s8}, {reg_c: s8}   => 0x08  @ reg_a @ reg_b @ reg_c ; reg_a = reg_b < reg_c
    
    ; Arithmetic
    add {reg_a: s8}, {reg_b: s8}, {reg_c: s8}   => 0x09 @ reg_a @ reg_b @ reg_c ; reg_b + reg_c, storing the results in reg a
    sub {reg_a: s8}, {reg_b: s8}, {reg_c: s8}   => 0x0A @ reg_a @ reg_b @ reg_c ; reg_b - reg_c, storing the results in reg a
    mul {reg_a: s8}, {reg_b: s8}, {reg_c: s8}   => 0x0B @ reg_a @ reg_b @ reg_c ; reg_b * reg_c, storing the results in reg a
    div {reg_a: s8}, {reg_b: s8}, {reg_c: s8}   => 0x0C @ reg_a @ reg_b @ reg_c ; reg_b / reg_c, storing the results in reg a

    addi {reg_a: s8}, {reg_b: s8}, {value: s32}   => 0x0D @ reg_a @ reg_b @ value ; reg_b + value, storing the results in reg a
    subi {reg_a: s8}, {reg_b: s8}, {value: s32}   => 0x0E @ reg_a @ reg_b @ value ; reg_b - value, storing the results in reg a
    muli {reg_a: s8}, {reg_b: s8}, {value: s32}   => 0x0F @ reg_a @ reg_b @ value ; reg_b * value, storing the results in reg a
    divi {reg_a: s8}, {reg_b: s8}, {value: s32}   => 0x10 @ reg_a @ reg_b @ value ; reg_b / value, storing the results in reg a

    and {reg_a: s8}, {reg_b: s8}, {reg_c: s8}   => 0x11 @ reg_a @ reg_b @ reg_c ; reg_b & reg_c, storing the results in reg a
    or {reg_a: s8}, {reg_b: s8}, {reg_c: s8}   => 0x12 @ reg_a @ reg_b @ reg_c ; reg_b | reg_c, storing the results in reg a
    sl {reg_a: s8}, {reg_b: s8}, {reg_c: s8}   => 0x13 @ reg_a @ reg_b @ reg_c ; reg_b << reg_c, storing the results in reg a
    sr {reg_a: s8}, {reg_b: s8}, {reg_c: s8}   => 0x14 @ reg_a @ reg_b @ reg_c ; reg_b >> reg_c, storing the results in reg a
    
    sli {reg_a: s8}, {reg_b: s8}, {value: s32}   => 0x15 @ reg_a @ reg_b @ value ; reg_b << value, storing the results in reg a
    sri {reg_a: s8}, {reg_b: s8}, {value: s32}   => 0x16 @ reg_a @ reg_b @ value ; reg_b >> value, storing the results in reg a
    
    sal {reg}                                    => 0x17 @ reg ; reg = reg << 1
    sar {reg}                                    => 0x18 @ reg ; reg = reg >> 1
    
    ; Memory Operations
    lb {reg: s8}, {mem_addr: s16}                   => 0x19 @ reg @ mem_addr ; Load byte (8) bit) into reg from mem_addr 
    sb {mem_addr: s16}, {reg: s8}                   => 0x1A @ mem_addr @ reg ; Store byte (8 bit) from reg into mem_addr 

    lhw {reg: s8}, {mem_addr: s16}                   => 0x1B @ reg @ mem_addr ; Load halfword (16 bit) into reg from mem_addr 
    shw {mem_addr: s16}, {reg: s8}                   => 0x1C @ mem_addr @ reg ; Store hlafword (16 bit) from reg into mem_addr 
    
    lw {reg: s8}, {mem_addr: s16}                   => 0x1D @ reg @ mem_addr ; Load word (32 bit) into reg from mem_addr 
    sw {mem_addr: s16}, {reg: s8}                   => 0x1E @ mem_addr @ reg ; Store word (32 bit) from reg into mem_addr 
    
}


main:
    load 0, 0xFFEF
    sw 0xFE, 0
    lw 1, 0xFE
    add 2, 0, 1
    hlt