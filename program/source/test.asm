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

    if f_r_a, 0x4698522d, test_floating_point_jumps

    load f_r_c, 0
    load f_r_d, 0

    div f_r_e, f_r_c, f_r_d

    sw 0xFFE0, f_r_e

    load f_r_f, 5
    sw 0xFFE1, f_r_f

    lw f_r_c, 0xFFE0
    lw f_r_d, 0xFFE1

    ;move f_r_d, f_r_e
    

    ; Halt!

    hlt


; Test relative jumps and some basic instructions, including returning
add_nums:
    add r_a, r_a, r_b

    add f_r_a, f_r_a, 0x4079999a

    ret

test_floating_point_jumps:
    load f_r_f, 0x4698522d
    ret