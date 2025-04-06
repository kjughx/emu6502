;
; 6 5 0 2   F U N C T I O N A L   T E S T
;
; Practically cut+paste from https://github.com/Klaus2m5/6502_65C02_functional_tests
;
.include "macros.s"
.include "configuration.s"

        .CODE
        .org code_segment
        .P02            ; disable 65SC02, 65C02 and 65816 instructions
start:
; test set and clear flags CLC CLI CLD CLV SEC SEI SED
        set_stat $ff
        clc
        tst_stat $ff-carry
        sec
        tst_stat $ff
    .if I_flag = 3
        cli
        tst_stat $ff-intdis
        sei
        tst_stat $ff
    .endif
        cld
        tst_stat $ff-decmode
        sed
        tst_stat $ff
        clv
        tst_stat $ff-overfl
        set_stat 0
        tst_stat 0
        sec
        tst_stat carry
        clc
        tst_stat 0
    .if I_flag = 3
        sei
        tst_stat intdis
        cli
        tst_stat 0
    .endif
        sed
        tst_stat decmode
        cld
        tst_stat 0
        set_stat overfl
        tst_stat overfl
        clv
        tst_stat 0

        success

brk_ret0:
    trap
brk_ret1:
    trap
