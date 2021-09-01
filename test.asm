#include "ruleset.asm"

main:
    ifn 2, 50, add
    if 2, 25, quit


subtract:
    sub 2, 2, 0

add:
    load 0, 25
    load 1, 25
    add 2, 0, 1
    sw 0x0, 2

quit:
    hlt