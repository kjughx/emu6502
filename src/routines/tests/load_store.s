.include "macros.s"
.include "configuration.s"

        .CODE
        .org code_segment
        .P02            ; disable 65SC02, 65C02 and 65816 instructions
start:
; testing index register load & store LDY LDX STY STX all addressing modes
; LDX / STX - zp,y / abs,y
        ldy #3
tldx:
        set_stat 0
        ldx zp1,y
        php         ;test stores do not alter flags
        txa
        eor #$c3
        plp
        sta abst,y
        php         ;flags after load/store sequence
        eor #$c3
        cmp abs1,y  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx,y  ;test flags
        trap_ne
        dey
        bpl tldx

        ldy #3
tldx1:
        set_stat $ff
        ldx zp1,y
        php         ;test stores do not alter flags
        txa
        eor #$c3
        plp
        sta abst,y
        php         ;flags after load/store sequence
        eor #$c3
        cmp abs1,y  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx,y  ;test flags
        trap_ne
        dey
        bpl tldx1

        ldy #3
tldx2:
        set_stat 0
        ldx abs1,y
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx zpt,y
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1,y   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx,y  ;test flags
        trap_ne
        dey
        bpl tldx2

        ldy #3
tldx3:
        set_stat $ff
        ldx abs1,y
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx zpt,y
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1,y   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx,y  ;test flags
        trap_ne
        dey
        bpl tldx3

        ldy #3      ;testing store result
        ldx #0
tstx:   lda zpt,y
        eor #$c3
        cmp zp1,y
        trap_ne     ;store to zp data
        stx zpt,y   ;clear
        lda abst,y
        eor #$c3
        cmp abs1,y
        trap_ne     ;store to abs data
        txa
        sta abst,y  ;clear
        dey
        bpl tstx

; indexed wraparound test (only zp should wrap)
        ldy #3+$fa
tldx4:  ldx <(zp1-$fa),y   ;wrap on indexed zp
        txa
        sta abst-$fa,y      ;no STX abs,y!
        dey
        cpy #$fa
        bcs tldx4
        ldy #3+$fa
tldx5:  ldx abs1-$fa,y      ;no wrap on indexed abs
        stx <(zpt-$fa),y
        dey
        cpy #$fa
        bcs tldx5
        ldy #3      ;testing wraparound result
        ldx #0
tstx1:  lda zpt,y
        cmp zp1,y
        trap_ne     ;store to zp data
        stx zpt,y   ;clear
        lda abst,y
        cmp abs1,y
        trap_ne     ;store to abs data
        txa
        sta abst,y  ;clear
        dey
        bpl tstx1

; LDY / STY - zp,x / abs,x
        ldx #3
tldy:
        set_stat 0
        ldy zp1,x
        php         ;test stores do not alter flags
        tya
        eor #$c3
        plp
        sta abst,x
        php         ;flags after load/store sequence
        eor #$c3
        cmp abs1,x  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx,x  ;test flags
        trap_ne
        dex
        bpl tldy

        ldx #3
tldy1:
        set_stat $ff
        ldy zp1,x
        php         ;test stores do not alter flags
        tya
        eor #$c3
        plp
        sta abst,x
        php         ;flags after load/store sequence
        eor #$c3
        cmp abs1,x  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx,x  ;test flags
        trap_ne
        dex
        bpl tldy1

        ldx #3
tldy2:
        set_stat 0
        ldy abs1,x
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty zpt,x
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1,x   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx,x  ;test flags
        trap_ne
        dex
        bpl tldy2

        ldx #3
tldy3:
        set_stat $ff
        ldy abs1,x
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty zpt,x
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1,x   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx,x  ;test flags
        trap_ne
        dex
        bpl tldy3

        ldx #3      ;testing store result
        ldy #0
tsty:   lda zpt,x
        eor #$c3
        cmp zp1,x
        trap_ne     ;store to zp,x data
        sty zpt,x   ;clear
        lda abst,x
        eor #$c3
        cmp abs1,x
        trap_ne     ;store to abs,x data
        txa
        sta abst,x  ;clear
        dex
        bpl tsty

; indexed wraparound test (only zp should wrap)
        ldx #3+$fa
tldy4:  ldy <(zp1-$fa),x   ;wrap on indexed zp
        tya
        sta abst-$fa,x      ;no STX abs,x!
        dex
        cpx #$fa
        bcs tldy4
        ldx #3+$fa
tldy5:  ldy abs1-$fa,x      ;no wrap on indexed abs
        sty <(zpt-$fa),x
        dex
        cpx #$fa
        bcs tldy5
        ldx #3      ;testing wraparound result
        ldy #0
tsty1:  lda zpt,x
        cmp zp1,x
        trap_ne     ;store to zp,x data
        sty zpt,x   ;clear
        lda abst,x
        cmp abs1,x
        trap_ne     ;store to abs,x data
        txa
        sta abst,x  ;clear
        dex
        bpl tsty1

; LDX / STX - zp / abs / #
        set_stat 0
        ldx zp1
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx abst
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx #$c3    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx    ;test flags
        trap_ne
        set_stat 0
        ldx zp1+1
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx abst+1
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx #$82    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat 0
        ldx zp1+2
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx abst+2
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx #$41    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat 0
        ldx zp1+3
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx abst+3
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx #0      ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+3  ;test flags
        trap_ne

        set_stat $ff
        ldx zp1
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx abst
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx #$c3    ;test result
        trap_ne     ;
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx    ;test flags
        trap_ne
        set_stat $ff
        ldx zp1+1
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx abst+1
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx #$82    ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat $ff
        ldx zp1+2
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx abst+2
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx #$41    ;test result
        trap_ne     ;
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat $ff
        ldx zp1+3
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx abst+3
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx #0      ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+3  ;test flags
        trap_ne

        set_stat 0
        ldx abs1
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx zpt
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1     ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx    ;test flags
        trap_ne
        set_stat 0
        ldx abs1+1
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx zpt+1
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1+1   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat 0
        ldx abs1+2
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx zpt+2
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1+2   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat 0
        ldx abs1+3
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx zpt+3
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1+3   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+3  ;test flags
        trap_ne

        set_stat $ff
        ldx abs1
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx zpt
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx zp1     ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx    ;test flags
        trap_ne
        set_stat $ff
        ldx abs1+1
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx zpt+1
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx zp1+1   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat $ff
        ldx abs1+2
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx zpt+2
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx zp1+2   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat $ff
        ldx abs1+3
        php         ;test stores do not alter flags
        txa
        eor #$c3
        tax
        plp
        stx zpt+3
        php         ;flags after load/store sequence
        eor #$c3
        tax
        cpx zp1+3   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+3  ;test flags
        trap_ne

        set_stat 0
        ldx #$c3
        php
        cpx abs1    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx    ;test flags
        trap_ne
        set_stat 0
        ldx #$82
        php
        cpx abs1+1  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat 0
        ldx #$41
        php
        cpx abs1+2  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat 0
        ldx #0
        php
        cpx abs1+3  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+3  ;test flags
        trap_ne

        set_stat $ff
        ldx #$c3
        php
        cpx abs1    ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx    ;test flags
        trap_ne
        set_stat $ff
        ldx #$82
        php
        cpx abs1+1  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat $ff
        ldx #$41
        php
        cpx abs1+2  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat $ff
        ldx #0
        php
        cpx abs1+3  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+3  ;test flags
        trap_ne

        ldx #0
        lda zpt
        eor #$c3
        cmp zp1
        trap_ne     ;store to zp data
        stx zpt     ;clear
        lda abst
        eor #$c3
        cmp abs1
        trap_ne     ;store to abs data
        stx abst    ;clear
        lda zpt+1
        eor #$c3
        cmp zp1+1
        trap_ne     ;store to zp data
        stx zpt+1   ;clear
        lda abst+1
        eor #$c3
        cmp abs1+1
        trap_ne     ;store to abs data
        stx abst+1  ;clear
        lda zpt+2
        eor #$c3
        cmp zp1+2
        trap_ne     ;store to zp data
        stx zpt+2   ;clear
        lda abst+2
        eor #$c3
        cmp abs1+2
        trap_ne     ;store to abs data
        stx abst+2  ;clear
        lda zpt+3
        eor #$c3
        cmp zp1+3
        trap_ne     ;store to zp data
        stx zpt+3   ;clear
        lda abst+3
        eor #$c3
        cmp abs1+3
        trap_ne     ;store to abs data
        stx abst+3  ;clear

; LDY / STY - zp / abs / #
        set_stat 0
        ldy zp1
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty abst
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy #$c3    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx    ;test flags
        trap_ne
        set_stat 0
        ldy zp1+1
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty abst+1
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy #$82    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat 0
        ldy zp1+2
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty abst+2
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy #$41    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat 0
        ldy zp1+3
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty abst+3
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy #0      ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+3  ;test flags
        trap_ne

        set_stat $ff
        ldy zp1
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty abst
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy #$c3    ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx    ;test flags
        trap_ne
        set_stat $ff
        ldy zp1+1
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty abst+1
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy #$82   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat $ff
        ldy zp1+2
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty abst+2
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy #$41    ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat $ff
        ldy zp1+3
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty abst+3
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy #0      ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+3  ;test flags
        trap_ne

        set_stat 0
        ldy abs1
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty zpt
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy zp1     ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx    ;test flags
        trap_ne
        set_stat 0
        ldy abs1+1
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty zpt+1
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy zp1+1   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat 0
        ldy abs1+2
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty zpt+2
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy zp1+2   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat 0
        ldy abs1+3
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty zpt+3
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cpy zp1+3   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+3  ;test flags
        trap_ne

        set_stat $ff
        ldy abs1
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty zpt
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cmp zp1     ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx    ;test flags
        trap_ne
        set_stat $ff
        ldy abs1+1
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty zpt+1
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cmp zp1+1   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat $ff
        ldy abs1+2
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty zpt+2
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cmp zp1+2   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat $ff
        ldy abs1+3
        php         ;test stores do not alter flags
        tya
        eor #$c3
        tay
        plp
        sty zpt+3
        php         ;flags after load/store sequence
        eor #$c3
        tay
        cmp zp1+3   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+3  ;test flags
        trap_ne


        set_stat 0
        ldy #$c3
        php
        cpy abs1    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx    ;test flags
        trap_ne
        set_stat 0
        ldy #$82
        php
        cpy abs1+1  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat 0
        ldy #$41
        php
        cpy abs1+2  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat 0
        ldy #0
        php
        cpy abs1+3  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+3  ;test flags
        trap_ne

        set_stat $ff
        ldy #$c3
        php
        cpy abs1    ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx    ;test flags
        trap_ne
        set_stat $ff
        ldy #$82
        php
        cpy abs1+1  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat $ff
        ldy #$41
        php
        cpy abs1+2   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat $ff
        ldy #0
        php
        cpy abs1+3  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+3  ;test flags
        trap_ne

        ldy #0
        lda zpt
        eor #$c3
        cmp zp1
        trap_ne     ;store to zp   data
        sty zpt     ;clear
        lda abst
        eor #$c3
        cmp abs1
        trap_ne     ;store to abs   data
        sty abst    ;clear
        lda zpt+1
        eor #$c3
        cmp zp1+1
        trap_ne     ;store to zp+1 data
        sty zpt+1   ;clear
        lda abst+1
        eor #$c3
        cmp abs1+1
        trap_ne     ;store to abs+1 data
        sty abst+1  ;clear
        lda zpt+2
        eor #$c3
        cmp zp1+2
        trap_ne     ;store to zp+2 data
        sty zpt+2   ;clear
        lda abst+2
        eor #$c3
        cmp abs1+2
        trap_ne     ;store to abs+2 data
        sty abst+2  ;clear
        lda zpt+3
        eor #$c3
        cmp zp1+3
        trap_ne     ;store to zp+3 data
        sty zpt+3   ;clear
        lda abst+3
        eor #$c3
        cmp abs1+3
        trap_ne     ;store to abs+3 data
        sty abst+3  ;clear

; testing load / store accumulator LDA / STA all addressing modes
; LDA / STA - zp,x / abs,x
        ldx #3
tldax:
        set_stat 0
        lda zp1,x
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst,x
        php         ;flags after load/store sequence
        eor #$c3
        cmp abs1,x  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx,x  ;test flags
        trap_ne
        dex
        bpl tldax

        ldx #3
tldax1:
        set_stat $ff
        lda zp1,x
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst,x
        php         ;flags after load/store sequence
        eor #$c3
        cmp abs1,x   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx,x  ;test flags
        trap_ne
        dex
        bpl tldax1

        ldx #3
tldax2:
        set_stat 0
        lda abs1,x
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta zpt,x
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1,x   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx,x  ;test flags
        trap_ne
        dex
        bpl tldax2

        ldx #3
tldax3:
        set_stat $ff
        lda abs1,x
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta zpt,x
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1,x   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx,x  ;test flags
        trap_ne
        dex
        bpl tldax3

        ldx #3      ;testing store result
        ldy #0
tstax:  lda zpt,x
        eor #$c3
        cmp zp1,x
        trap_ne     ;store to zp,x data
        sty zpt,x   ;clear
        lda abst,x
        eor #$c3
        cmp abs1,x
        trap_ne     ;store to abs,x data
        txa
        sta abst,x  ;clear
        dex
        bpl tstax

; LDA / STA - (zp),y / abs,y / (zp,x)
        ldy #3
tlday:
        set_stat 0
        lda (ind1),y
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst,y
        php         ;flags after load/store sequence
        eor #$c3
        cmp abs1,y  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx,y  ;test flags
        trap_ne
        dey
        bpl tlday

        ldy #3
tlday1:
        set_stat $ff
        lda (ind1),y
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst,y
        php         ;flags after load/store sequence
        eor #$c3
        cmp abs1,y  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx,y  ;test flags
        trap_ne
        dey
        bpl tlday1

        ldy #3      ;testing store result
        ldx #0
tstay:  lda abst,y
        eor #$c3
        cmp abs1,y
        trap_ne     ;store to abs data
        txa
        sta abst,y  ;clear
        dey
        bpl tstay

        ldy #3
tlday2:
        set_stat 0
        lda abs1,y
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta (indt),y
        php         ;flags after load/store sequence
        eor #$c3
        cmp (ind1),y    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx,y  ;test flags
        trap_ne
        dey
        bpl tlday2

        ldy #3
tlday3:
        set_stat $ff
        lda abs1,y
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta (indt),y
        php         ;flags after load/store sequence
        eor #$c3
        cmp (ind1),y   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx,y  ;test flags
        trap_ne
        dey
        bpl tlday3

        ldy #3      ;testing store result
        ldx #0
tstay1: lda abst,y
        eor #$c3
        cmp abs1,y
        trap_ne     ;store to abs data
        txa
        sta abst,y  ;clear
        dey
        bpl tstay1

        ldx #6
        ldy #3
tldax4:
        set_stat 0
        lda (ind1,x)
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta (indt,x)
        php         ;flags after load/store sequence
        eor #$c3
        cmp abs1,y  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx,y  ;test flags
        trap_ne
        dex
        dex
        dey
        bpl tldax4

        ldx #6
        ldy #3
tldax5:
        set_stat $ff
        lda (ind1,x)
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta (indt,x)
        php         ;flags after load/store sequence
        eor #$c3
        cmp abs1,y  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx,y  ;test flags
        trap_ne
        dex
        dex
        dey
        bpl tldax5

        ldy #3      ;testing store result
        ldx #0
tstay2: lda abst,y
        eor #$c3
        cmp abs1,y
        trap_ne     ;store to abs data
        txa
        sta abst,y  ;clear
        dey
        bpl tstay2

; indexed wraparound test (only zp should wrap)
        ldx #3+$fa
tldax6: lda <(zp1-$fa),x   ;wrap on indexed zp
        sta abst-$fa,x      ;no STX abs,x!
        dex
        cpx #$fa
        bcs tldax6
        ldx #3+$fa
tldax7: lda abs1-$fa,x      ;no wrap on indexed abs
        sta <(zpt-$fa),x
        dex
        cpx #$fa
        bcs tldax7

        ldx #3      ;testing wraparound result
        ldy #0
tstax1: lda zpt,x
        cmp zp1,x
        trap_ne     ;store to zp,x data
        sty zpt,x   ;clear
        lda abst,x
        cmp abs1,x
        trap_ne     ;store to abs,x data
        txa
        sta abst,x  ;clear
        dex
        bpl tstax1

        ldy #3+$f8
        ldx #6+$f8
tlday4: lda (<(ind1-$f8),x) ;wrap on indexed zp indirect
        sta abst-$f8,y
        dex
        dex
        dey
        cpy #$f8
        bcs tlday4
        ldy #3      ;testing wraparound result
        ldx #0
tstay4: lda abst,y
        cmp abs1,y
        trap_ne     ;store to abs data
        txa
        sta abst,y  ;clear
        dey
        bpl tstay4

        ldy #3+$f8
tlday5: lda abs1-$f8,y  ;no wrap on indexed abs
        sta (inwt),y
        dey
        cpy #$f8
        bcs tlday5
        ldy #3      ;testing wraparound result
        ldx #0
tstay5: lda abst,y
        cmp abs1,y
        trap_ne     ;store to abs data
        txa
        sta abst,y  ;clear
        dey
        bpl tstay5

        ldy #3+$f8
        ldx #6+$f8
tlday6: lda (inw1),y    ;no wrap on zp indirect indexed
        sta (<(indt-$f8),x)
        dex
        dex
        dey
        cpy #$f8
        bcs tlday6
        ldy #3      ;testing wraparound result
        ldx #0
tstay6: lda abst,y
        cmp abs1,y
        trap_ne     ;store to abs data
        txa
        sta abst,y  ;clear
        dey
        bpl tstay6

; LDA / STA - zp / abs / #
        set_stat 0
        lda zp1
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst
        php         ;flags after load/store sequence
        eor #$c3
        cmp #$c3    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx    ;test flags
        trap_ne
        set_stat 0
        lda zp1+1
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst+1
        php         ;flags after load/store sequence
        eor #$c3
        cmp #$82    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat 0
        lda zp1+2
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst+2
        php         ;flags after load/store sequence
        eor #$c3
        cmp #$41    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat 0
        lda zp1+3
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst+3
        php         ;flags after load/store sequence
        eor #$c3
        cmp #0      ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+3  ;test flags
        trap_ne
        set_stat $ff
        lda zp1
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst
        php         ;flags after load/store sequence
        eor #$c3
        cmp #$c3    ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx    ;test flags
        trap_ne
        set_stat $ff
        lda zp1+1
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst+1
        php         ;flags after load/store sequence
        eor #$c3
        cmp #$82    ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat $ff
        lda zp1+2
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst+2
        php         ;flags after load/store sequence
        eor #$c3
        cmp #$41    ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat $ff
        lda zp1+3
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta abst+3
        php         ;flags after load/store sequence
        eor #$c3
        cmp #0      ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+3  ;test flags
        trap_ne
        set_stat 0
        lda abs1
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta zpt
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1     ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx    ;test flags
        trap_ne
        set_stat 0
        lda abs1+1
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta zpt+1
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1+1   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat 0
        lda abs1+2
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta zpt+2
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1+2   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat 0
        lda abs1+3
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta zpt+3
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1+3   ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+3  ;test flags
        trap_ne
        set_stat $ff
        lda abs1
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta zpt
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1     ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx    ;test flags
        trap_ne
        set_stat $ff
        lda abs1+1
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta zpt+1
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1+1   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat $ff
        lda abs1+2
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta zpt+2
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1+2   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat $ff
        lda abs1+3
        php         ;test stores do not alter flags
        eor #$c3
        plp
        sta zpt+3
        php         ;flags after load/store sequence
        eor #$c3
        cmp zp1+3   ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+3  ;test flags
        trap_ne
        set_stat 0
        lda #$c3
        php
        cmp abs1    ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx    ;test flags
        trap_ne
        set_stat 0
        lda #$82
        php
        cmp abs1+1  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat 0
        lda #$41
        php
        cmp abs1+2  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat 0
        lda #0
        php
        cmp abs1+3  ;test result
        trap_ne
        pla         ;load status
        eor_flag 0
        cmp fLDx+3  ;test flags
        trap_ne

        set_stat $ff
        lda #$c3
        php
        cmp abs1    ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx    ;test flags
        trap_ne
        set_stat $ff
        lda #$82
        php
        cmp abs1+1  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+1  ;test flags
        trap_ne
        set_stat $ff
        lda #$41
        php
        cmp abs1+2  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+2  ;test flags
        trap_ne
        set_stat $ff
        lda #0
        php
        cmp abs1+3  ;test result
        trap_ne
        pla         ;load status
        eor_flag <~fnz ;mask bits not altered
        cmp fLDx+3  ;test flags
        trap_ne

        ldx #0
        lda zpt
        eor #$c3
        cmp zp1
        trap_ne     ;store to zp data
        stx zpt     ;clear
        lda abst
        eor #$c3
        cmp abs1
        trap_ne     ;store to abs data
        stx abst    ;clear
        lda zpt+1
        eor #$c3
        cmp zp1+1
        trap_ne     ;store to zp data
        stx zpt+1   ;clear
        lda abst+1
        eor #$c3
        cmp abs1+1
        trap_ne     ;store to abs data
        stx abst+1  ;clear
        lda zpt+2
        eor #$c3
        cmp zp1+2
        trap_ne     ;store to zp data
        stx zpt+2   ;clear
        lda abst+2
        eor #$c3
        cmp abs1+2
        trap_ne     ;store to abs data
        stx abst+2  ;clear
        lda zpt+3
        eor #$c3
        cmp zp1+3
        trap_ne     ;store to zp data
        stx zpt+3   ;clear
        lda abst+3
        eor #$c3
        cmp abs1+3
        trap_ne     ;store to abs data
        stx abst+3  ;clear

        success

brk_ret0:
    trap
brk_ret1:
    trap


