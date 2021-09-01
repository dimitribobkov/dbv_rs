#include "ruleset.asm"

main:
    ifn 2, 50, add
    ifn 3, 25, subtract
	hlt


subtract:
    sub 3, 2, 1
    jmp main

add:
    load 0, 25
    load 1, 25
    add 2, 0, 1
    jmp main