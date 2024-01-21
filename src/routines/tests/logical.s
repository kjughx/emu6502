.include "macros.s"
.include "configuration.s"

        .CODE
        .org code_segment
        .P02            ; disable 65SC02, 65C02 and 65816 instructions
start:
; testing bit test & compares BIT CPX CPY CMP all addressing modes
; BIT - zp / abs
        set_a $ff,0
        bit zp1+3   ;00 - should set Z / clear  NV
        tst_a $ff,fz
        set_a 1,0
        bit zp1+2   ;41 - should set V (M6) / clear NZ
        tst_a 1,fv
        set_a 1,0
        bit zp1+1   ;82 - should set N (M7) & Z / clear V
        tst_a 1,fnz
        set_a 1,0
        bit zp1     ;c3 - should set N (M7) & V (M6) / clear Z
        tst_a 1,fnv

        set_a $ff,$ff
        bit zp1+3   ;00 - should set Z / clear  NV
        tst_a $ff,~fnv
        set_a 1,$ff
        bit zp1+2   ;41 - should set V (M6) / clear NZ
        tst_a 1,~fnz
        set_a 1,$ff
        bit zp1+1   ;82 - should set N (M7) & Z / clear V
        tst_a 1,~fv
        set_a 1,$ff
        bit zp1     ;c3 - should set N (M7) & V (M6) / clear Z
        tst_a 1,~fz

        set_a $ff,0
        bit abs1+3  ;00 - should set Z / clear  NV
        tst_a $ff,fz
        set_a 1,0
        bit abs1+2  ;41 - should set V (M6) / clear NZ
        tst_a 1,fv
        set_a 1,0
        bit abs1+1  ;82 - should set N (M7) & Z / clear V
        tst_a 1,fnz
        set_a 1,0
        bit abs1    ;c3 - should set N (M7) & V (M6) / clear Z
        tst_a 1,fnv

        set_a $ff,$ff
        bit abs1+3  ;00 - should set Z / clear  NV
        tst_a $ff,~fnv
        set_a 1,$ff
        bit abs1+2  ;41 - should set V (M6) / clear NZ
        tst_a 1,~fnz
        set_a 1,$ff
        bit abs1+1  ;82 - should set N (M7) & Z / clear V
        tst_a 1,~fv
        set_a 1,$ff
        bit abs1    ;c3 - should set N (M7) & V (M6) / clear Z
        tst_a 1,~fz

; CPX - zp / abs / #
        set_x $80,0
        cpx zp7f
        tst_stat fc
        dex
        cpx zp7f
        tst_stat fzc
        dex
        cpx zp7f
        tst_x $7e,fn
        set_x $80,$ff
        cpx zp7f
        tst_stat ~fnz
        dex
        cpx zp7f
        tst_stat ~fn
        dex
        cpx zp7f
        tst_x $7e,~fzc

        set_x $80,0
        cpx abs7f
        tst_stat fc
        dex
        cpx abs7f
        tst_stat fzc
        dex
        cpx abs7f
        tst_x $7e,fn
        set_x $80,$ff
        cpx abs7f
        tst_stat ~fnz
        dex
        cpx abs7f
        tst_stat ~fn
        dex
        cpx abs7f
        tst_x $7e,~fzc

        set_x $80,0
        cpx #$7f
        tst_stat fc
        dex
        cpx #$7f
        tst_stat fzc
        dex
        cpx #$7f
        tst_x $7e,fn
        set_x $80,$ff
        cpx #$7f
        tst_stat ~fnz
        dex
        cpx #$7f
        tst_stat ~fn
        dex
        cpx #$7f
        tst_x $7e,~fzc

; CPY - zp / abs / #
        set_y $80,0
        cpy zp7f
        tst_stat fc
        dey
        cpy zp7f
        tst_stat fzc
        dey
        cpy zp7f
        tst_y $7e,fn
        set_y $80,$ff
        cpy zp7f
        tst_stat ~fnz
        dey
        cpy zp7f
        tst_stat ~fn
        dey
        cpy zp7f
        tst_y $7e,~fzc

        set_y $80,0
        cpy abs7f
        tst_stat fc
        dey
        cpy abs7f
        tst_stat fzc
        dey
        cpy abs7f
        tst_y $7e,fn
        set_y $80,$ff
        cpy abs7f
        tst_stat ~fnz
        dey
        cpy abs7f
        tst_stat ~fn
        dey
        cpy abs7f
        tst_y $7e,~fzc

        set_y $80,0
        cpy #$7f
        tst_stat fc
        dey
        cpy #$7f
        tst_stat fzc
        dey
        cpy #$7f
        tst_y $7e,fn
        set_y $80,$ff
        cpy #$7f
        tst_stat ~fnz
        dey
        cpy #$7f
        tst_stat ~fn
        dey
        cpy #$7f
        tst_y $7e,~fzc

; CMP - zp / abs / #
        set_a $80,0
        cmp zp7f
        tst_a $80,fc
        set_a $7f,0
        cmp zp7f
        tst_a $7f,fzc
        set_a $7e,0
        cmp zp7f
        tst_a $7e,fn
        set_a $80,$ff
        cmp zp7f
        tst_a $80,~fnz
        set_a $7f,$ff
        cmp zp7f
        tst_a $7f,~fn
        set_a $7e,$ff
        cmp zp7f
        tst_a $7e,~fzc

        set_a $80,0
        cmp abs7f
        tst_a $80,fc
        set_a $7f,0
        cmp abs7f
        tst_a $7f,fzc
        set_a $7e,0
        cmp abs7f
        tst_a $7e,fn
        set_a $80,$ff
        cmp abs7f
        tst_a $80,~fnz
        set_a $7f,$ff
        cmp abs7f
        tst_a $7f,~fn
        set_a $7e,$ff
        cmp abs7f
        tst_a $7e,~fzc

        set_a $80,0
        cmp #$7f
        tst_a $80,fc
        set_a $7f,0
        cmp #$7f
        tst_a $7f,fzc
        set_a $7e,0
        cmp #$7f
        tst_a $7e,fn
        set_a $80,$ff
        cmp #$7f
        tst_a $80,~fnz
        set_a $7f,$ff
        cmp #$7f
        tst_a $7f,~fn
        set_a $7e,$ff
        cmp #$7f
        tst_a $7e,~fzc

        ldx #4          ;with indexing by X
        set_a $80,0
        cmp zp1,x
        tst_a $80,fc
        set_a $7f,0
        cmp zp1,x
        tst_a $7f,fzc
        set_a $7e,0
        cmp zp1,x
        tst_a $7e,fn
        set_a $80,$ff
        cmp zp1,x
        tst_a $80,~fnz
        set_a $7f,$ff
        cmp zp1,x
        tst_a $7f,~fn
        set_a $7e,$ff
        cmp zp1,x
        tst_a $7e,~fzc

        set_a $80,0
        cmp abs1,x
        tst_a $80,fc
        set_a $7f,0
        cmp abs1,x
        tst_a $7f,fzc
        set_a $7e,0
        cmp abs1,x
        tst_a $7e,fn
        set_a $80,$ff
        cmp abs1,x
        tst_a $80,~fnz
        set_a $7f,$ff
        cmp abs1,x
        tst_a $7f,~fn
        set_a $7e,$ff
        cmp abs1,x
        tst_a $7e,~fzc

        ldy #4          ;with indexing by Y
        ldx #8          ;with indexed indirect
        set_a $80,0
        cmp abs1,y
        tst_a $80,fc
        set_a $7f,0
        cmp abs1,y
        tst_a $7f,fzc
        set_a $7e,0
        cmp abs1,y
        tst_a $7e,fn
        set_a $80,$ff
        cmp abs1,y
        tst_a $80,~fnz
        set_a $7f,$ff
        cmp abs1,y
        tst_a $7f,~fn
        set_a $7e,$ff
        cmp abs1,y
        tst_a $7e,~fzc

        set_a $80,0
        cmp (ind1,x)
        tst_a $80,fc
        set_a $7f,0
        cmp (ind1,x)
        tst_a $7f,fzc
        set_a $7e,0
        cmp (ind1,x)
        tst_a $7e,fn
        set_a $80,$ff
        cmp (ind1,x)
        tst_a $80,~fnz
        set_a $7f,$ff
        cmp (ind1,x)
        tst_a $7f,~fn
        set_a $7e,$ff
        cmp (ind1,x)
        tst_a $7e,~fzc

        set_a $80,0
        cmp (ind1),y
        tst_a $80,fc
        set_a $7f,0
        cmp (ind1),y
        tst_a $7f,fzc
        set_a $7e,0
        cmp (ind1),y
        tst_a $7e,fn
        set_a $80,$ff
        cmp (ind1),y
        tst_a $80,~fnz
        set_a $7f,$ff
        cmp (ind1),y
        tst_a $7f,~fn
        set_a $7e,$ff
        cmp (ind1),y
        tst_a $7e,~fzc

; testing logical instructions - AND EOR ORA all addressing modes
; AND
        ldx #3          ;immediate
tand:   lda zpAN,x
        sta ex_andi+1   ;set AND # operand
        set_ax  absANa,0
        jsr ex_andi     ;execute AND # in RAM
        tst_ax  absrlo,absflo,0
        dex
        bpl tand
        ldx #3
tand1:  lda zpAN,x
        sta ex_andi+1   ;set AND # operand
        set_ax  absANa,$ff
        jsr ex_andi     ;execute AND # in RAM
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl tand1

        ldx #3      ;zp
tand2:  lda zpAN,x
        sta zpt
        set_ax  absANa,0
        and zpt
        tst_ax  absrlo,absflo,0
        dex
        bpl tand2
        ldx #3
tand3:  lda zpAN,x
        sta zpt
        set_ax  absANa,$ff
        and zpt
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl tand3

        ldx #3      ;abs
tand4:  lda zpAN,x
        sta abst
        set_ax  absANa,0
        and abst
        tst_ax  absrlo,absflo,0
        dex
        bpl tand4
        ldx #3
tand5:  lda zpAN,x
        sta abst
        set_ax  absANa,$ff
        and abst
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl tand6

        ldx #3      ;zp,x
tand6:
        set_ax  absANa,0
        and zpAN,x
        tst_ax  absrlo,absflo,0
        dex
        bpl tand6
        ldx #3
tand7:
        set_ax  absANa,$ff
        and zpAN,x
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl tand7

        ldx #3      ;abs,x
tand8:
        set_ax  absANa,0
        and absAN,x
        tst_ax  absrlo,absflo,0
        dex
        bpl tand8
        ldx #3
tand9:
        set_ax  absANa,$ff
        and absAN,x
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl tand9

        ldy #3      ;abs,y
tand10:
        set_ay  absANa,0
        and absAN,y
        tst_ay  absrlo,absflo,0
        dey
        bpl tand10
        ldy #3
tand11:
        set_ay  absANa,$ff
        and absAN,y
        tst_ay  absrlo,absflo,$ff-fnz
        dey
        bpl tand11

        ldx #6      ;(zp,x)
        ldy #3
tand12:
        set_ay  absANa,0
        and (indAN,x)
        tst_ay  absrlo,absflo,0
        dex
        dex
        dey
        bpl tand12
        ldx #6
        ldy #3
tand13:
        set_ay  absANa,$ff
        and (indAN,x)
        tst_ay  absrlo,absflo,$ff-fnz
        dex
        dex
        dey
        bpl tand13

        ldy #3      ;(zp),y
tand14:
        set_ay  absANa,0
        and (indAN),y
        tst_ay  absrlo,absflo,0
        dey
        bpl tand14
        ldy #3
tand15:
        set_ay  absANa,$ff
        and (indAN),y
        tst_ay  absrlo,absflo,$ff-fnz
        dey
        bpl tand15

; EOR
        ldx #3          ;immediate - self modifying code
teor:   lda zpEO,x
        sta ex_eori+1   ;set EOR # operand
        set_ax  absEOa,0
        jsr ex_eori     ;execute EOR # in RAM
        tst_ax  absrlo,absflo,0
        dex
        bpl teor
        ldx #3
teor1:  lda zpEO,x
        sta ex_eori+1   ;set EOR # operand
        set_ax  absEOa,$ff
        jsr ex_eori     ;execute EOR # in RAM
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl teor1

        ldx #3      ;zp
teor2:   lda zpEO,x
        sta zpt
        set_ax  absEOa,0
        eor zpt
        tst_ax  absrlo,absflo,0
        dex
        bpl teor2
        ldx #3
teor3:  lda zpEO,x
        sta zpt
        set_ax  absEOa,$ff
        eor zpt
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl teor3

        ldx #3      ;abs
teor4:  lda zpEO,x
        sta abst
        set_ax  absEOa,0
        eor abst
        tst_ax  absrlo,absflo,0
        dex
        bpl teor4
        ldx #3
teor5:  lda zpEO,x
        sta abst
        set_ax  absEOa,$ff
        eor abst
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl teor6

        ldx #3      ;zp,x
teor6:
        set_ax  absEOa,0
        eor zpEO,x
        tst_ax  absrlo,absflo,0
        dex
        bpl teor6
        ldx #3
teor7:
        set_ax  absEOa,$ff
        eor zpEO,x
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl teor7

        ldx #3      ;abs,x
teor8:
        set_ax  absEOa,0
        eor absEO,x
        tst_ax  absrlo,absflo,0
        dex
        bpl teor8
        ldx #3
teor9:
        set_ax  absEOa,$ff
        eor absEO,x
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl teor9

        ldy #3      ;abs,y
teor10:
        set_ay  absEOa,0
        eor absEO,y
        tst_ay  absrlo,absflo,0
        dey
        bpl teor10
        ldy #3
teor11:
        set_ay  absEOa,$ff
        eor absEO,y
        tst_ay  absrlo,absflo,$ff-fnz
        dey
        bpl teor11

        ldx #6      ;(zp,x)
        ldy #3
teor12:
        set_ay  absEOa,0
        eor (indEO,x)
        tst_ay  absrlo,absflo,0
        dex
        dex
        dey
        bpl teor12
        ldx #6
        ldy #3
teor13:
        set_ay  absEOa,$ff
        eor (indEO,x)
        tst_ay  absrlo,absflo,$ff-fnz
        dex
        dex
        dey
        bpl teor13

        ldy #3      ;(zp),y
teor14:
        set_ay  absEOa,0
        eor (indEO),y
        tst_ay  absrlo,absflo,0
        dey
        bpl teor14
        ldy #3
teor15:
        set_ay  absEOa,$ff
        eor (indEO),y
        tst_ay  absrlo,absflo,$ff-fnz
        dey
        bpl teor15

; OR
        ldx #3          ;immediate - self modifying code
tora:   lda zpOR,x
        sta ex_orai+1   ;set ORA # operand
        set_ax  absORa,0
        jsr ex_orai     ;execute ORA # in RAM
        tst_ax  absrlo,absflo,0
        dex
        bpl tora
        ldx #3
tora1:  lda zpOR,x
        sta ex_orai+1   ;set ORA # operand
        set_ax  absORa,$ff
        jsr ex_orai     ;execute ORA # in RAM
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl tora1

        ldx #3      ;zp
tora2:  lda zpOR,x
        sta zpt
        set_ax  absORa,0
        ora zpt
        tst_ax  absrlo,absflo,0
        dex
        bpl tora2
        ldx #3
tora3:  lda zpOR,x
        sta zpt
        set_ax  absORa,$ff
        ora zpt
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl tora3

        ldx #3      ;abs
tora4:  lda zpOR,x
        sta abst
        set_ax  absORa,0
        ora abst
        tst_ax  absrlo,absflo,0
        dex
        bpl tora4
        ldx #3
tora5:  lda zpOR,x
        sta abst
        set_ax  absORa,$ff
        ora abst
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl tora6

        ldx #3      ;zp,x
tora6:
        set_ax  absORa,0
        ora zpOR,x
        tst_ax  absrlo,absflo,0
        dex
        bpl tora6
        ldx #3
tora7:
        set_ax  absORa,$ff
        ora zpOR,x
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl tora7

        ldx #3      ;abs,x
tora8:
        set_ax  absORa,0
        ora absOR,x
        tst_ax  absrlo,absflo,0
        dex
        bpl tora8
        ldx #3
tora9:
        set_ax  absORa,$ff
        ora absOR,x
        tst_ax  absrlo,absflo,$ff-fnz
        dex
        bpl tora9

        ldy #3      ;abs,y
tora10:
        set_ay  absORa,0
        ora absOR,y
        tst_ay  absrlo,absflo,0
        dey
        bpl tora10
        ldy #3
tora11:
        set_ay  absORa,$ff
        ora absOR,y
        tst_ay  absrlo,absflo,$ff-fnz
        dey
        bpl tora11

        ldx #6      ;(zp,x)
        ldy #3
tora12:
        set_ay  absORa,0
        ora (indOR,x)
        tst_ay  absrlo,absflo,0
        dex
        dex
        dey
        bpl tora12
        ldx #6
        ldy #3
tora13:
        set_ay  absORa,$ff
        ora (indOR,x)
        tst_ay  absrlo,absflo,$ff-fnz
        dex
        dex
        dey
        bpl tora13

        ldy #3      ;(zp),y
tora14:
        set_ay  absORa,0
        ora (indOR),y
        tst_ay  absrlo,absflo,0
        dey
        bpl tora14
        ldy #3
tora15:
        set_ay  absORa,$ff
        ora (indOR),y
        tst_ay  absrlo,absflo,$ff-fnz
        dey
        bpl tora15
    .if I_flag = 3
        cli
    .endif

        success

brk_ret0:
    trap
brk_ret1:
    trap



