#include "ruleset.asm"

main:
    load r_a, 0
    load r_b, 25

    move r_b, r_a

    .loop:
        jmp add_nums
        sw 0xFFED, r_a
        ifn r_a, 250*5000, .loop

    lw r_a, 0xFFED

    load f_r_a, 0x40a00000
        
    load f_r_b, 0x4079999a

    add f_r_c, f_r_a, f_r_b
    
    hlt


add_nums:
    add r_a, r_a, r_b
    ret