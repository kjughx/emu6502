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
; testing index register increment/decrement and transfer
; INX INY DEX DEY TAX TXA TAY TYA
        ldx #$fe
        set_stat $ff
        inx             ;ff
        tst_x $ff,$ff-zero
        inx             ;00
        tst_x 0,$ff-minus
        inx             ;01
        tst_x 1,$ff-minus-zero
        dex             ;00
        tst_x 0,$ff-minus
        dex             ;ff
        tst_x $ff,$ff-zero
        dex             ;fe
        set_stat 0
        inx             ;ff
        tst_x $ff,minus
        inx             ;00
        tst_x 0,zero
        inx             ;01
        tst_x 1,0
        dex             ;00
        tst_x 0,zero
        dex             ;ff
        tst_x $ff,minus

        ldy #$fe
        set_stat $ff
        iny             ;ff
        tst_y $ff,$ff-zero
        iny             ;00
        tst_y 0,$ff-minus
        iny             ;01
        tst_y 1,$ff-minus-zero
        dey             ;00
        tst_y 0,$ff-minus
        dey             ;ff
        tst_y $ff,$ff-zero
        dey             ;fe
        set_stat 0
        iny             ;ff
        tst_y $ff,0+minus
        iny             ;00
        tst_y 0,zero
        iny             ;01
        tst_y 1,0
        dey             ;00
        tst_y 0,zero
        dey             ;ff
        tst_y $ff,minus

        ldx #$ff
        set_stat $ff
        txa
        tst_a $ff,$ff-zero
        php
        inx             ;00
        plp
        txa
        tst_a 0,$ff-minus
        php
        inx             ;01
        plp
        txa
        tst_a 1,$ff-minus-zero
        set_stat 0
        txa
        tst_a 1,0
        php
        dex             ;00
        plp
        txa
        tst_a 0,zero
        php
        dex             ;ff
        plp
        txa
        tst_a $ff,minus

        ldy #$ff
        set_stat $ff
        tya
        tst_a $ff,$ff-zero
        php
        iny             ;00
        plp
        tya
        tst_a 0,$ff-minus
        php
        iny             ;01
        plp
        tya
        tst_a 1,$ff-minus-zero
        set_stat 0
        tya
        tst_a 1,0
        php
        dey             ;00
        plp
        tya
        tst_a 0,zero
        php
        dey             ;ff
        plp
        tya
        tst_a $ff,minus

        load_flag $ff
        pha
        ldx #$ff        ;ff
        txa
        plp
        tay
        tst_y $ff,$ff-zero
        php
        inx             ;00
        txa
        plp
        tay
        tst_y 0,$ff-minus
        php
        inx             ;01
        txa
        plp
        tay
        tst_y 1,$ff-minus-zero
        load_flag 0
        pha
        lda #0
        txa
        plp
        tay
        tst_y 1,0
        php
        dex             ;00
        txa
        plp
        tay
        tst_y 0,zero
        php
        dex             ;ff
        txa
        plp
        tay
        tst_y $ff,minus


        load_flag $ff
        pha
        ldy #$ff        ;ff
        tya
        plp
        tax
        tst_x $ff,$ff-zero
        php
        iny             ;00
        tya
        plp
        tax
        tst_x 0,$ff-minus
        php
        iny             ;01
        tya
        plp
        tax
        tst_x 1,$ff-minus-zero
        load_flag 0
        pha
        lda #0          ;preset status
        tya
        plp
        tax
        tst_x 1,0
        php
        dey             ;00
        tya
        plp
        tax
        tst_x 0,zero
        php
        dey             ;ff
        tya
        plp
        tax
        tst_x $ff,minus

;TSX sets NZ - TXS does not
;  This section also tests for proper stack wrap around.
        ldx #1          ;01
        set_stat $ff
        txs
        php
        lda $101
        cmp_flag $ff
        trap_ne
        set_stat 0
        txs
        php
        lda $101
        cmp_flag 0
        trap_ne
        dex             ;00
        set_stat $ff
        txs
        php
        lda $100
        cmp_flag $ff
        trap_ne
        set_stat 0
        txs
        php
        lda $100
        cmp_flag 0
        trap_ne
        dex             ;ff
        set_stat $ff
        txs
        php
        lda $1ff
        cmp_flag $ff
        trap_ne
        set_stat 0
        txs
        php
        lda $1ff
        cmp_flag 0

        ldx #1
        txs             ;sp=01
        set_stat $ff
        tsx             ;clears Z, N
        php             ;sp=00
        cpx #1
        trap_ne
        lda $101
        cmp_flag $ff-minus-zero
        trap_ne
        set_stat $ff
        tsx             ;clears N, sets Z
        php             ;sp=ff
        cpx #0
        trap_ne
        lda $100
        cmp_flag $ff-minus
        trap_ne
        set_stat $ff
        tsx             ;clears N, sets Z
        php             ;sp=fe
        cpx #$ff
        trap_ne
        lda $1ff
        cmp_flag $ff-zero
        trap_ne

        ldx #1
        txs             ;sp=01
        set_stat 0
        tsx             ;clears Z, N
        php             ;sp=00
        cpx #1
        trap_ne
        lda $101
        cmp_flag 0
        trap_ne
        set_stat 0
        tsx             ;clears N, sets Z
        php             ;sp=ff
        cpx #0
        trap_ne
        lda $100
        cmp_flag zero
        trap_ne
        set_stat 0
        tsx             ;clears N, sets Z
        php             ;sp=fe
        cpx #$ff
        trap_ne
        lda $1ff
        cmp_flag minus
        trap_ne
        pla             ;sp=ff

; testing memory increment/decrement - INC DEC all addressing modes
; zeropage
        ldx #0
        lda #$7e
        sta zpt
tinc:
        set_stat 0
        inc zpt
        tst_z rINC,fINC,0
        inx
        cpx #2
        bne tinc1
        lda #$fe
        sta zpt
tinc1:  cpx #5
        bne tinc
        dex
        inc zpt
tdec:
        set_stat 0
        dec zpt
        tst_z rINC,fINC,0
        dex
        bmi tdec1
        cpx #1
        bne tdec
        lda #$81
        sta zpt
        bne tdec
tdec1:
        ldx #0
        lda #$7e
        sta zpt
tinc10:
        set_stat $ff
        inc zpt
        tst_z rINC,fINC,$ff-fnz
        inx
        cpx #2
        bne tinc11
        lda #$fe
        sta zpt
tinc11: cpx #5
        bne tinc10
        dex
        inc zpt
tdec10:
        set_stat $ff
        dec zpt
        tst_z rINC,fINC,$ff-fnz
        dex
        bmi tdec11
        cpx #1
        bne tdec10
        lda #$81
        sta zpt
        bne tdec10
tdec11:

; absolute memory
        ldx #0
        lda #$7e
        sta abst
tinc2:
        set_stat 0
        inc abst
        tst_abs rINC,fINC,0
        inx
        cpx #2
        bne tinc3
        lda #$fe
        sta abst
tinc3:  cpx #5
        bne tinc2
        dex
        inc abst
tdec2:
        set_stat 0
        dec abst
        tst_abs rINC,fINC,0
        dex
        bmi tdec3
        cpx #1
        bne tdec2
        lda #$81
        sta abst
        bne tdec2
tdec3:
        ldx #0
        lda #$7e
        sta abst
tinc12:
        set_stat $ff
        inc abst
        tst_abs rINC,fINC,$ff-fnz
        inx
        cpx #2
        bne tinc13
        lda #$fe
        sta abst
tinc13:  cpx #5
        bne tinc12
        dex
        inc abst
tdec12:
        set_stat $ff
        dec abst
        tst_abs rINC,fINC,$ff-fnz
        dex
        bmi tdec13
        cpx #1
        bne tdec12
        lda #$81
        sta abst
        bne tdec12
tdec13:

; zeropage indexed
        ldx #0
        lda #$7e
tinc4:  sta zpt,x
        set_stat 0
        inc zpt,x
        tst_zx rINC,fINC,0
        lda zpt,x
        inx
        cpx #2
        bne tinc5
        lda #$fe
tinc5:  cpx #5
        bne tinc4
        dex
        lda #2
tdec4:  sta zpt,x
        set_stat 0
        dec zpt,x
        tst_zx rINC,fINC,0
        lda zpt,x
        dex
        bmi tdec5
        cpx #1
        bne tdec4
        lda #$81
        bne tdec4
tdec5:
        ldx #0
        lda #$7e
tinc14: sta zpt,x
        set_stat $ff
        inc zpt,x
        tst_zx rINC,fINC,$ff-fnz
        lda zpt,x
        inx
        cpx #2
        bne tinc15
        lda #$fe
tinc15: cpx #5
        bne tinc14
        dex
        lda #2
tdec14: sta zpt,x
        set_stat $ff
        dec zpt,x
        tst_zx rINC,fINC,$ff-fnz
        lda zpt,x
        dex
        bmi tdec15
        cpx #1
        bne tdec14
        lda #$81
        bne tdec14
tdec15:

; memory indexed
        ldx #0
        lda #$7e
tinc6:  sta abst,x
        set_stat 0
        inc abst,x
        tst_absx rINC,fINC,0
        lda abst,x
        inx
        cpx #2
        bne tinc7
        lda #$fe
tinc7:  cpx #5
        bne tinc6
        dex
        lda #2
tdec6:  sta abst,x
        set_stat 0
        dec abst,x
        tst_absx rINC,fINC,0
        lda abst,x
        dex
        bmi tdec7
        cpx #1
        bne tdec6
        lda #$81
        bne tdec6
tdec7:
        ldx #0
        lda #$7e
tinc16: sta abst,x
        set_stat $ff
        inc abst,x
        tst_absx rINC,fINC,$ff-fnz
        lda abst,x
        inx
        cpx #2
        bne tinc17
        lda #$fe
tinc17: cpx #5
        bne tinc16
        dex
        lda #2
tdec16: sta abst,x
        set_stat $ff
        dec abst,x
        tst_absx rINC,fINC,$ff-fnz
        lda abst,x
        dex
        bmi tdec17
        cpx #1
        bne tdec16
        lda #$81
        bne tdec16
tdec17:

        success

brk_ret0:
    trap
brk_ret1:
    trap


