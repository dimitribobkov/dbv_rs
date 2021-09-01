#include "ruleset.asm"

main:
    load r_a, 25
    load r_b, 25

    .loop:
        add r_c, r_a, r_b
        ifnr r_c, 250, .loop

    ret
