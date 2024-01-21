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
;testing stack operations PHA PHP PLA PLP
        ldx #$ff        ;initialize stack
        txs
        lda #$55
        pha
        lda #$aa
        pha
        cmp $1fe        ;on stack ?
        trap_ne
        tsx
        txa             ;overwrite accu
        cmp #$fd        ;sp decremented?
        trap_ne
        pla
        cmp #$aa        ;successful retreived from stack?
        trap_ne
        pla
        cmp #$55
        trap_ne
        cmp $1ff        ;remains on stack?
        trap_ne
        tsx
        cpx #$ff        ;sp incremented?
        trap_ne

; test PHA does not alter flags or accumulator but PLA does
        ldx #$55        ;x & y protected
        ldy #$aa
        set_a 1,$ff     ;push
        pha
        tst_a 1,$ff
        set_a 0,0
        pha
        tst_a 0,0
        set_a $ff,$ff
        pha
        tst_a $ff,$ff
        set_a 1,0
        pha
        tst_a 1,0
        set_a 0,$ff
        pha
        tst_a 0,$ff
        set_a $ff,0
        pha
        tst_a $ff,0
        set_a 0,$ff     ;pull
        pla
        tst_a $ff,$ff-zero
        set_a $ff,0
        pla
        tst_a 0,zero
        set_a $fe,$ff
        pla
        tst_a 1,$ff-zero-minus
        set_a 0,0
        pla
        tst_a $ff,minus
        set_a $ff,$ff
        pla
        tst_a 0,$ff-minus
        set_a $fe,0
        pla
        tst_a 1,0
        cpx #$55        ;x & y unchanged?
        trap_ne
        cpy #$aa
        trap_ne

        success
        
brk_ret0:
    trap
brk_ret1:
    trap

