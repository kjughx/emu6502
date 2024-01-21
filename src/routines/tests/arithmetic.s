.include "macros.s"
.include "configuration.s"

; full binary add/subtract test
; iterates through all combinations of operands and carry input
; uses increments/decrements to predict result & result flags
        .CODE
        .org code_segment
        .P02            ; disable 65SC02, 65C02 and 65816 instructions
start:
; full binary add/subtract test
; iterates through all combinations of operands and carry input
; uses increments/decrements to predict result & result flags
        cld
        ldx #ad2        ;for indexed test
        ldy #$ff        ;max range
        lda #0          ;start with adding zeroes & no carry
        sta adfc        ;carry in - for diag
        sta ad1         ;operand 1 - accumulator
        sta ad2         ;operand 2 - memory or immediate
        sta ada2        ;non zp
        sta adrl        ;expected result bits 0-7
        sta adrh        ;expected result bit 8 (carry out)
        lda #$ff        ;complemented operand 2 for subtract
        sta sb2
        sta sba2        ;non zp
        lda #2          ;expected Z-flag
        sta adrf
tadd:   clc             ;test with carry clear
        jsr chkadd
        inc adfc        ;now with carry
        inc adrl        ;result +1
        php             ;save N & Z from low result
        php
        pla             ;accu holds expected flags
        and #$82        ;mask N & Z
        plp
        bne tadd1
        inc adrh        ;result bit 8 - carry
tadd1:  ora adrh        ;merge C to expected flags
        sta adrf        ;save expected flags except overflow
        sec             ;test with carry set
        jsr chkadd
        dec adfc        ;same for operand +1 but no carry
        inc ad1
        bne tadd        ;iterate op1
        lda #0          ;preset result to op2 when op1 = 0
        sta adrh
        inc ada2
        inc ad2
        php             ;save NZ as operand 2 becomes the new result
        pla
        and #$82        ;mask N00000Z0
        sta adrf        ;no need to check carry as we are adding to 0
        dec sb2         ;complement subtract operand 2
        dec sba2
        lda ad2
        sta adrl
        bne tadd        ;iterate op2

        success

; core subroutine of the full binary add/subtract test
; iterates through all combinations of operands and carry input
; uses increments/decrements to predict result & result flags
chkadd: lda adrf        ;add V-flag if overflow
        and #$83        ;keep N-----ZC / clear V
        pha
        lda ad1         ;test sign unequal between operands
        eor ad2
        bmi ckad1       ;no overflow possible - operands have different sign
        lda ad1         ;test sign equal between operands and result
        eor adrl
        bpl ckad1       ;no overflow occured - operand and result have same sign
        pla
        ora #$40        ;set V
        pha
ckad1:  pla
        sta adrf        ;save expected flags
; binary ADC / SBC zp
        php             ;save carry for subtract
        lda ad1
        adc ad2         ;perform add
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
        php             ;save carry for next add
        lda ad1
        sbc sb2         ;perform subtract
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
; binary ADC / SBC abs
        php             ;save carry for subtract
        lda ad1
        adc ada2        ;perform add
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
        php             ;save carry for next add
        lda ad1
        sbc sba2        ;perform subtract
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
; binary ADC / SBC #
        php             ;save carry for subtract
        lda ad2
        sta ex_adci+1   ;set ADC # operand
        lda ad1
        jsr ex_adci     ;execute ADC # in RAM
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
        php             ;save carry for next add
        lda sb2
        sta ex_sbci+1   ;set SBC # operand
        lda ad1
        jsr ex_sbci     ;execute SBC # in RAM
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
; binary ADC / SBC zp,x
        php             ;save carry for subtract
        lda ad1
        adc 0,x         ;perform add
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
        php             ;save carry for next add
        lda ad1
        sbc sb2-ad2,x   ;perform subtract
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
; binary ADC / SBC abs,x
        php             ;save carry for subtract
        lda ad1
        adc ada2-ad2,x  ;perform add
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
        php             ;save carry for next add
        lda ad1
        sbc sba2-ad2,x  ;perform subtract
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
; binary ADC / SBC abs,y
        php             ;save carry for subtract
        lda ad1
        adc ada2-$ff,y  ;perform add
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
        php             ;save carry for next add
        lda ad1
        sbc sba2-$ff,y  ;perform subtract
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
; binary ADC / SBC (zp,x)
        php             ;save carry for subtract
        lda ad1
        adc (<adi2-ad2,x) ;perform add
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
        php             ;save carry for next add
        lda ad1
        sbc (<sbi2-ad2,x) ;perform subtract
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
; binary ADC / SBC (abs),y
        php             ;save carry for subtract
        lda ad1
        adc (adiy2),y   ;perform add
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
        php             ;save carry for next add
        lda ad1
        sbc (sbiy2),y   ;perform subtract
        php
        cmp adrl        ;check result
        trap_ne         ;bad result
        pla             ;check flags
        and #$c3        ;mask NV----ZC
        cmp adrf
        trap_ne         ;bad flags
        plp
        rts


brk_ret0:
    trap
brk_ret1:
    trap


