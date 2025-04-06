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

;pretest small branch offset
        ldx #5
        jmp psb_test
psb_bwok:
        ldy #5
        bne psb_forw
        trap        ;branch should be taken
        dey         ;forward landing zone
        dey
        dey
        dey
        dey
psb_forw:
        dey
        dey
        dey
        dey
        dey
        beq psb_fwok
        trap        ;forward offset

        dex         ;backward landing zone
        dex
        dex
        dex
        dex
psb_back:
        dex
        dex
        dex
        dex
        dex
        beq psb_bwok
        trap        ;backward offset
psb_test:
        bne psb_back
        trap        ;branch should be taken
psb_fwok:

;testing relative addressing with BEQ
        ldy #$fe        ;testing maximum range, not -1/-2 (invalid/self adr)
range_loop:
        dey             ;next relative address
        tya
        tax             ;precharge count to end of loop
        bpl range_fw    ;calculate relative address
        clc             ;avoid branch self or to relative address of branch
        adc #2
        nop             ;offset landing zone - tolerate +/-5 offset to branch
        nop
        nop
        nop
        nop
range_fw:
        nop
        nop
        nop
        nop
        nop
        eor #$7f        ;complement except sign
        sta range_adr   ;load into test target
        lda #0          ;should set zero flag in status register
        jmp range_op

        dex             ; offset landing zone - backward branch too far
        dex
        dex
        dex
        dex
        ;relative address target field with branch under test in the middle
        dex             ;-128 - max backward
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-120
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-110
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-100
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-90
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-80
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-70
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-60
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-50
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-40
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-30
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-20
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-10
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;-3
range_op:                ;test target with zero flag=0, z=1 if previous dex
range_adr   = *+1       ;modifiable relative address
        beq *+64        ;+64 if called without modification
        dex             ;+0
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+10
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+20
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+30
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+40
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+50
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+60
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+70
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+80
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+90
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+100
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+110
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex
        dex             ;+120
        dex
        dex
        dex
        dex
        dex
        dex
        nop             ;offset landing zone - forward branch too far
        nop
        nop
        nop
        nop
        beq range_ok    ;+127 - max forward
        trap            ; bad range
        nop             ;offset landing zone - tolerate +/-5 offset to branch
        nop
        nop
        nop
        nop
range_ok:
        nop
        nop
        nop
        nop
        nop
        cpy #0
        beq range_end
        jmp range_loop
range_end:               ;range test successful

;partial test BNE & CMP, CPX, CPY immediate
        cpy #1          ;testing BNE true
        bne test_bne
        trap
test_bne:
        lda #0
        cmp #0          ;test compare immediate
        trap_ne
        trap_cc
        trap_mi
        cmp #1
        trap_eq
        trap_cs
        trap_pl
        tax
        cpx #0          ;test compare x immediate
        trap_ne
        trap_cc
        trap_mi
        cpx #1
        trap_eq
        trap_cs
        trap_pl
        tay
        cpy #0          ;test compare y immediate
        trap_ne
        trap_cc
        trap_mi
        cpy #1
        trap_eq
        trap_cs
        trap_pl

;testing branch decisions BPL BMI BVC BVS BCC BCS BNE BEQ
        ldx #$ff        ;initialize stack
        txs
        set_stat $ff    ;all on
        bpl nbr1        ;branches should not be taken
        bvc nbr2
        bcc nbr3
        bne nbr4
        bmi br1         ;branches should be taken
        trap
br1:    bvs br2
        trap
br2:    bcs br3
        trap
br3:    beq br4
        trap
nbr1:
        trap            ;previous bpl taken
nbr2:
        trap            ;previous bvc taken
nbr3:
        trap            ;previous bcc taken
nbr4:
        trap            ;previous bne taken
br4:    php
        tsx
        cpx #$fe        ;sp after php?
        trap_ne
        pla
        cmp_flag $ff    ;returned all flags on?
        trap_ne
        tsx
        cpx #$ff        ;sp after php?
        trap_ne
        set_stat 0      ;all off
        bmi nbr11       ;branches should not be taken
        bvs nbr12
        bcs nbr13
        beq nbr14
        bpl br11        ;branches should be taken
        trap
br11:   bvc br12
        trap
br12:   bcc br13
        trap
br13:   bne br14
        trap
nbr11:
        trap            ;previous bmi taken
nbr12:
        trap            ;previous bvs taken
nbr13:
        trap            ;previous bcs taken
nbr14:
        trap            ;previous beq taken
br14:   php
        pla
        cmp_flag 0      ;flags off except break (pushed by sw) + reserved?
        trap_ne
        ;crosscheck flags
        set_stat zero
        bne brzs1
        beq brzs2
brzs1:
        trap            ;branch zero/non zero
brzs2:  bcs brzs3
        bcc brzs4
brzs3:
        trap            ;branch carry/no carry
brzs4:  bmi brzs5
        bpl brzs6
brzs5:
        trap            ;branch minus/plus
brzs6:  bvs brzs7
        bvc brzs8
brzs7:
        trap            ;branch overflow/no overflow
brzs8:
        set_stat carry
        beq brcs1
        bne brcs2
brcs1:
        trap            ;branch zero/non zero
brcs2:  bcc brcs3
        bcs brcs4
brcs3:
        trap            ;branch carry/no carry
brcs4:  bmi brcs5
        bpl brcs6
brcs5:
        trap            ;branch minus/plus
brcs6:  bvs brcs7
        bvc brcs8
brcs7:
        trap            ;branch overflow/no overflow

brcs8:
        set_stat minus
        beq brmi1
        bne brmi2
brmi1:
        trap            ;branch zero/non zero
brmi2:  bcs brmi3
        bcc brmi4
brmi3:
        trap            ;branch carry/no carry
brmi4:  bpl brmi5
        bmi brmi6
brmi5:
        trap            ;branch minus/plus
brmi6:  bvs brmi7
        bvc brmi8
brmi7:
        trap            ;branch overflow/no overflow
brmi8:
        set_stat overfl
        beq brvs1
        bne brvs2
brvs1:
        trap            ;branch zero/non zero
brvs2:  bcs brvs3
        bcc brvs4
brvs3:
        trap            ;branch carry/no carry
brvs4:  bmi brvs5
        bpl brvs6
brvs5:
        trap            ;branch minus/plus
brvs6:  bvc brvs7
        bvs brvs8
brvs7:
        trap            ;branch overflow/no overflow
brvs8:
        set_stat $ff-zero
        beq brzc1
        bne brzc2
brzc1:
        trap            ;branch zero/non zero
brzc2:  bcc brzc3
        bcs brzc4
brzc3:
        trap            ;branch carry/no carry
brzc4:  bpl brzc5
        bmi brzc6
brzc5:
        trap            ;branch minus/plus
brzc6:  bvc brzc7
        bvs brzc8
brzc7:
        trap            ;branch overflow/no overflow
brzc8:
        set_stat $ff-carry
        bne brcc1
        beq brcc2
brcc1:
        trap            ;branch zero/non zero
brcc2:  bcs brcc3
        bcc brcc4
brcc3:
        trap            ;branch carry/no carry
brcc4:  bpl brcc5
        bmi brcc6
brcc5:
        trap            ;branch minus/plus
brcc6:  bvc brcc7
        bvs brcc8
brcc7:
        trap            ;branch overflow/no overflow
brcc8:
        set_stat $ff-minus
        bne brpl1
        beq brpl2
brpl1:
        trap            ;branch zero/non zero
brpl2:  bcc brpl3
        bcs brpl4
brpl3:
        trap            ;branch carry/no carry
brpl4:  bmi brpl5
        bpl brpl6
brpl5:
        trap            ;branch minus/plus
brpl6:  bvc brpl7
        bvs brpl8
brpl7:
        trap            ;branch overflow/no overflow
brpl8:
        set_stat $ff-overfl
        bne brvc1
        beq brvc2
brvc1:
        trap            ;branch zero/non zero
brvc2:  bcc brvc3
        bcs brvc4
brvc3:
        trap            ;branch carry/no carry
brvc4:  bpl brvc5
        bmi brvc6
brvc5:
        trap            ;branch minus/plus
brvc6:  bvs brvc7
        bvc brvc8
brvc7:
        trap            ;branch overflow/no overflow
brvc8:

        success

brk_ret0:
    trap
brk_ret1:
    trap

