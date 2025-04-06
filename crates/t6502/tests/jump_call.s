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
; PC modifying instructions except branches (NOP, JMP, JSR, RTS, BRK, RTI)
; testing NOP
        ldx #$24
        ldy #$42
        set_a $18,0
        nop
        tst_a $18,0
        cpx #$24
        trap_ne
        cpy #$42
        trap_ne
        ldx #$db
        ldy #$bd
        set_a $e7,$ff
        nop
        tst_a $e7,$ff
        cpx #$db
        trap_ne
        cpy #$bd
        trap_ne

; jump absolute
        ldx #$ff
        txs
        set_stat $0
        lda #'F'
        ldx #'A'
        ldy #'R'        ;N=0, V=0, Z=0, C=0
        jmp test_far
        nop
        nop
        trap_ne         ;runover protection
        inx
        inx
far_ret:
        trap_eq         ;returned flags OK?
        trap_pl
        trap_cc
        trap_vc
        cmp #('F'^$aa)  ;returned registers OK?
        trap_ne
        cpx #('A'+1)
        trap_ne
        cpy #('R'-3)
        trap_ne
        dex
        iny
        iny
        iny
        eor #$aa        ;N=0, V=1, Z=0, C=1
        jmp test_near
        nop
        nop
        trap_ne         ;runover protection
        inx
        inx
test_near:
        trap_eq         ;passed flags OK?
        trap_mi
        trap_cc
        trap_vc
        cmp #'F'        ;passed registers OK?
        trap_ne
        cpx #'A'
        trap_ne
        cpy #'R'
        trap_ne

; jump indirect
        set_stat 0
        lda #'I'
        ldx #'N'
        ldy #'D'        ;N=0, V=0, Z=0, C=0
        jmp (ptr_tst_ind)
        nop
        trap_ne         ;runover protection
        dey
        dey
ind_ret:
        php             ;either SP or Y count will fail, if we do not hit
        dey
        dey
        dey
        plp
        trap_eq         ;returned flags OK?
        trap_pl
        trap_cc
        trap_vc
        cmp #('I'^$aa)  ;returned registers OK?
        trap_ne
        cpx #('N'+1)
        trap_ne
        cpy #('D'-6)
        trap_ne
        tsx             ;SP check
        cpx #$ff
        trap_ne

; jump subroutine & return from subroutine
        set_stat 0
        lda #'J'
        ldx #'S'
        ldy #'R'        ;N=0, V=0, Z=0, C=0
        jsr test_jsr
jsr_ret = *-1           ;last address of jsr = return address
        php             ;either SP or Y count will fail, if we do not hit
        dey
        dey
        dey
        plp
        trap_eq         ;returned flags OK?
        trap_pl
        trap_cc
        trap_vc
        cmp #('J'^$aa)  ;returned registers OK?
        trap_ne
        cpx #('S'+1)
        trap_ne
        cpy #('R'-6)
        trap_ne
        tsx             ;sp?
        cpx #$ff
        trap_ne

; break & return from interrupt
        load_flag 0     ;with interrupts enabled if allowed!
        pha
        lda #'B'
        ldx #'R'
        ldy #'K'
        plp             ;N=0, V=0, Z=0, C=0
        brk

        dey             ;should not be executed

brk_ret0:               ;address of break return
        php             ;either SP or Y count will fail, if we do not hit
        dey
        dey
        dey
        cmp #'B'^$aa    ;returned registers OK?
        ;the IRQ vector was never executed if A & X stay unmodified
        trap_ne
        cpx #'R'+1
        trap_ne
        cpy #'K'-6
        trap_ne
        pla             ;returned flags OK (unchanged)?
        cmp_flag 0
        trap_ne
        tsx             ;sp?
        cpx #$ff
        trap_ne

        load_flag $ff   ;with interrupts disabled if allowed!
        pha
        lda #$ff-'B'
        ldx #$ff-'R'
        ldy #$ff-'K'
        plp             ;N=1, V=1, Z=1, C=1
        brk

        dey             ;should not be executed

brk_ret1:               ;address of break return
        php             ;either SP or Y count will fail, if we do not hit
        dey
        dey
        dey
        cmp #($ff-'B')^$aa  ;returned registers OK?
        ;the IRQ vector was never executed if A & X stay unmodified
        trap_ne
        cpx #$ff-'R'+1
        trap_ne
        cpy #$ff-'K'-6
        trap_ne
        pla             ;returned flags OK (unchanged)?
        cmp_flag $ff
        trap_ne
        tsx             ;sp?
        cpx #$ff
        trap_ne


        dey             ;should not be executed

        success

; target for the jump absolute test
        dey
        dey
test_far:
        php             ;either SP or Y count will fail, if we do not hit
        dey
        dey
        dey
        plp
        trap_cs         ;flags loaded?
        trap_vs
        trap_mi
        trap_eq
        cmp #'F'        ;registers loaded?
        trap_ne
        cpx #'A'
        trap_ne
        cpy #('R'-3)
        trap_ne
        pha             ;save a,x
        txa
        pha
        tsx
        cpx #$fd        ;check SP
        trap_ne
        pla             ;restore x
        tax
        set_stat $ff
        pla             ;restore a
        inx             ;return registers with modifications
        eor #$aa        ;N=1, V=1, Z=0, C=1
        jmp far_ret

; target for the jump indirect test
;       .align 2
        .if * & 1       ; workaround for problems with .align 2
            .byte 0     ;
        .endif          ;
ptr_tst_ind:
        .word   test_ind
ptr_ind_ret:
        .word   ind_ret
        trap            ;runover protection
        dey
        dey

test_ind:
        php             ;either SP or Y count will fail, if we do not hit
        dey
        dey
        dey
        plp
        trap_cs         ;flags loaded?
        trap_vs
        trap_mi
        trap_eq
        cmp #'I'        ;registers loaded?
        trap_ne
        cpx #'N'
        trap_ne
        cpy #('D'-3)
        trap_ne
        pha             ;save a,x
        txa
        pha
        tsx
        cpx #$fd        ;check SP
        trap_ne
        pla             ;restore x
        tax
        set_stat $ff
        pla             ;restore a
        inx             ;return registers with modifications
        eor #$aa        ;N=1, V=1, Z=0, C=1
        jmp (ptr_ind_ret)
        trap            ;runover protection
        jmp start       ;catastrophic error - cannot continue

test_jsr:
        php             ;either SP or Y count will fail, if we do not hit
        dey
        dey
        dey
        plp
        trap_cs         ;flags loaded?
        trap_vs
        trap_mi
        trap_eq
        cmp #'J'        ;registers loaded?
        trap_ne
        cpx #'S'
        trap_ne
        cpy #('R'-3)
        trap_ne
        pha             ;save a,x
        txa
        pha
        tsx             ;sp -4? (return addr,a,x)
        cpx #$fb
        trap_ne
        lda $1ff        ;propper return on stack
        cmp #>jsr_ret
        trap_ne
        lda $1fe
        cmp #<jsr_ret
        trap_ne
        set_stat $ff
        pla             ;pull x,a
        tax
        pla
        inx             ;return registers with modifications
        eor #$aa        ;N=1, V=1, Z=0, C=1
        rts
        trap            ;runover protection
        jmp start       ;catastrophic error - cannot continue
