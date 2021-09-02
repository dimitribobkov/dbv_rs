#include "ruleset.asm"

main:
    load r_a, 0
    load r_b, 25

    .loop:
        jmp add_nums
        ifn r_a, 250*5000, .loop

    hlt


add_nums:
    add r_a, r_a, r_b
    ret
