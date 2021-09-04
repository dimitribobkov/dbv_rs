#include "ruleset.asm"

main:
    load r_a, 0
    load r_b, 25

    move r_b, r_a



    load r_c, 0xFFED

    .loop:
        jmp add_nums
        sw 0xFFED, r_a
        ifn r_a, 250*5000, .loop

    lw r_a, 0xFFED

    load f_r_a, 0x4079999a

    move f_r_a, r_c
    
    hlt


add_nums:
    add r_a, r_a, r_b
    ret