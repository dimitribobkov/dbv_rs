#include "ruleset.asm"

main:
    ; Test registers and loading/moving
    load r_a, 0
    load r_b, 25

    move r_b, r_a

    ; Test jumps and conditional jumps 
    .loop:
        jmp add_nums
        ifn r_a, 250*500, .loop



    ; Test that fp registers work        
    load f_r_b, 0x4079999a

    add f_r_c, f_r_a, 0
    

    ; Test eq
    move r_a, r_b

    eq r_f, r_b, r_a

    ; Test the stack
    push r_a
    push r_b

    pop r_c
    pop r_d

    ; Halt!

    hlt


; Test relative jumps and some basic instructions, including returning
add_nums:
    add r_a, r_a, r_b

    add f_r_a, f_r_a, 0x4079999a

    ret