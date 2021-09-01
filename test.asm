#include "ruleset.asm"

main:
    ifnr r_c, 50, add
    ifnr r_d, 25, subtract
    ifr r_e, 0, multiply
	jmpr halt


subtract:
    sub r_d, r_c, r_b
    jmpr main

add:
    load r_a, 25
    load r_b, 25
    add r_c, r_a, r_b
    jmpr main

multiply:
    mul r_e, r_c, r_d
    jmpr main

halt:
    hlt