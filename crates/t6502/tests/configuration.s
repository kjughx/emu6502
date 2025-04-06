;
; 6 5 0 2   F U N C T I O N A L   T E S T
;
; Practically cut+paste from https://github.com/Klaus2m5/6502_65C02_functional_tests
;
; C O N F I G U R A T I O N

;ROM_vectors writable (0=no, 1=yes)
;if ROM vectors can not be used interrupts will not be trapped
;as a consequence BRK can not be tested but will be emulated to test RTI
ROM_vectors = 1

;load_data_direct (0=move from code segment, 1=load directly)
;loading directly is preferred but may not be supported by your platform
;0 produces only consecutive object code, 1 is not suitable for a binary image
load_data_direct = 1

;I_flag behavior (0=force enabled, 1=force disabled, 2=prohibit change, 3=allow
;change) 2 requires extra code and is not recommended. SEI & CLI can only be
;tested if you allow changing the interrupt status (I_flag = 3)
I_flag = 3

;configure memory - try to stay away from memory used by the system
;zero_page memory start address, $52 (82) consecutive Bytes required
;                                add 2 if I_flag = 2
zero_page = $a  

;data_segment memory start address, $7B (123) consecutive Bytes required
; check that this matches the linker configuration file
data_segment = $200  
    .if (data_segment & $ff) <> 0
        .error "low byte of data_segment MUST be $00 !!"
    .endif

;code_segment memory start address, 13.1kB of consecutive space required
;                                   add 2.5 kB if I_flag = 2
; check that this matches the linker configuration file
code_segment = $400
rom_segment = $8000

;self modifying code may be disabled to allow running in ROM
;0=part of the code is self modifying and must reside in RAM
;1=tests disabled: branch range
disable_selfmod = 0

;RAM integrity test option. Checks for undesired RAM writes.
;set lowest non RAM or RAM mirror address page (-1=disable, 0=64k, $40=16k)
;leave disabled if a monitor, OS or background interrupt is allowed to alter RAM
ram_top = -1

;disable test decimal mode ADC & SBC, 0=enable, 1=disable,
;2=disable including decimal flag in processor status
disable_decimal = 1

    .define equ =

carry   equ %00000001   ;flag bits in status
zero    equ %00000010
intdis  equ %00000100
decmode equ %00001000
break   equ %00010000
reserv  equ %00100000
overfl  equ %01000000
minus   equ %10000000

fc      equ carry
fz      equ zero
fzc     equ carry+zero
fv      equ overfl
fvz     equ overfl+zero
fn      equ minus
fnc     equ minus+carry
fnz     equ minus+zero
fnzc    equ minus+zero+carry
fnv     equ minus+overfl

fao     equ break+reserv    ;bits always on after PHP, BRK
fai     equ fao+intdis      ;+ forced interrupt disable
faod    equ fao+decmode     ;+ ignore decimal
faid    equ fai+decmode     ;+ ignore decimal
m8      equ $ff             ;8 bit mask
m8i     equ $ff&~intdis     ;8 bit mask - interrupt disable


        .ZEROPAGE
		.res zero_page, 0
        .org zero_page

;break test interrupt save
irq_a:  .res    1,0             ;a register
irq_x:  .res    1,0             ;x register
    .if I_flag = 2
;masking for I bit in status
flag_I_on:  .res    1,0         ;or mask to load flags
flag_I_off: .res    1,0         ;and mask to load flags
    .endif
zpt:                        ;6 bytes store/modify test area
;add/subtract operand generation and result/flag prediction
adfc:   .res    1,0             ;carry flag before op
ad1:    .res    1,0             ;operand 1 - accumulator
ad2:    .res    1,0             ;operand 2 - memory / immediate
adrl:   .res    1,0             ;expected result bits 0-7
adrh:   .res    1,0             ;expected result bit 8 (carry)
adrf:   .res    1,0             ;expected flags NV0000ZC (only binary mode)
sb2:    .res    1,0             ;operand 2 complemented for subtract
zp_bss:
zps:    .byte   $80,1           ;additional shift pattern to test zero result & flag
zp1:    .byte   $c3,$82,$41,0   ;test patterns for LDx BIT ROL ROR ASL LSR
zp7f:   .byte   $7f             ;test pattern for compare  
;logical zeropage operands
zpOR:   .byte   0,$1f,$71,$80   ;test pattern for OR
zpAN:   .byte   $0f,$ff,$7f,$80 ;test pattern for AND
zpEO:   .byte   $ff,$0f,$8f,$8f ;test pattern for EOR
;indirect addressing pointers
ind1:   .word   abs1            ;indirect pointer to pattern in absolute memory
        .word   abs1+1
        .word   abs1+2
        .word   abs1+3
        .word   abs7f
inw1:   .word   abs1-$f8        ;indirect pointer for wrap-test pattern
indt:   .word   abst            ;indirect pointer to store area in absolute memory
        .word   abst+1
        .word   abst+2
        .word   abst+3
inwt:   .word   abst-$f8        ;indirect pointer for wrap-test store
indAN:  .word   absAN           ;indirect pointer to AND pattern in absolute memory
        .word   absAN+1
        .word   absAN+2
        .word   absAN+3
indEO:  .word   absEO           ;indirect pointer to EOR pattern in absolute memory
        .word   absEO+1
        .word   absEO+2
        .word   absEO+3
indOR:  .word   absOR           ;indirect pointer to OR pattern in absolute memory
        .word   absOR+1
        .word   absOR+2
        .word   absOR+3
;add/subtract indirect pointers
adi2:   .word   ada2            ;indirect pointer to operand 2 in absolute memory
sbi2:   .word   sba2            ;indirect pointer to complemented operand 2 (SBC)
adiy2:  .word   ada2-$ff        ;with offset for indirect indexed
sbiy2:  .word   sba2-$ff
zp_bss_end:
   
        .DATA
        .org data_segment

test_case:  .res    1,0         ;current test number
ram_chksm:  .res    2,0         ;checksum for RAM integrity test
;add/subtract operand copy - abs tests write area
abst:                           ;6 bytes store/modify test area
ada2:   .res    1,0             ;operand 2
sba2:   .res    1,0             ;operand 2 complemented for subtract
        .res    4,0             ;fill remaining bytes
data_bss:
    .if load_data_direct = 1
ex_andi:and #0              ;execute immediate opcodes
        rts
ex_eori:eor #0              ;execute immediate opcodes
        rts
ex_orai:ora #0              ;execute immediate opcodes
        rts
ex_adci:adc #0              ;execute immediate opcodes
        rts
ex_sbci:sbc #0              ;execute immediate opcodes
        rts
    .else
ex_andi:.res    3
ex_eori:.res    3
ex_orai:.res    3
ex_adci:.res    3
ex_sbci:.res    3
    .endif
;zps    .byte   $80,1           ;additional shift patterns test zero result & flag
abs1:   .byte   $c3,$82,$41,0   ;test patterns for LDx BIT ROL ROR ASL LSR
abs7f:  .byte   $7f             ;test pattern for compare
;loads
fLDx:   .byte   fn,fn,0,fz              ;expected flags for load
;shifts
rASL:                                   ;expected result ASL & ROL -carry
rROL:   .byte   0,2,$86,$04,$82,0
rROLc:  .byte   1,3,$87,$05,$83,1       ;expected result ROL +carry
rLSR:                                   ;expected result LSR & ROR -carry
rROR:   .byte   $40,0,$61,$41,$20,0
rRORc:  .byte   $c0,$80,$e1,$c1,$a0,$80 ;expected result ROR +carry
fASL:                                   ;expected flags for shifts
fROL:   .byte   fzc,0,fnc,fc,fn,fz      ;no carry in
fROLc:  .byte   fc,0,fnc,fc,fn,0        ;carry in
fLSR:
fROR:   .byte   0,fzc,fc,0,fc,fz        ;no carry in
fRORc:  .byte   fn,fnc,fnc,fn,fnc,fn    ;carry in
;increments (decrements)
rINC:   .byte   $7f,$80,$ff,0,1         ;expected result for INC/DEC
fINC:   .byte   0,fn,fn,fz,0            ;expected flags for INC/DEC
;logical memory operand
absOR:  .byte   0,$1f,$71,$80           ;test pattern for OR
absAN:  .byte   $0f,$ff,$7f,$80         ;test pattern for AND
absEO:  .byte   $ff,$0f,$8f,$8f         ;test pattern for EOR
;logical accu operand
absORa: .byte   0,$f1,$1f,0             ;test pattern for OR
absANa: .byte   $f0,$ff,$ff,$ff         ;test pattern for AND
absEOa: .byte   $ff,$f0,$f0,$0f         ;test pattern for EOR
;logical results
absrlo: .byte   0,$ff,$7f,$80
absflo: .byte   fz,fn,0,fn
data_bss_end:

nmi_trap:
        trap            ;check stack for conditions at NMI
        jmp start       ;catastrophic error - cannot continue
res_trap:
        trap            ;unexpected RESET
        jmp start       ;catastrophic error - cannot continue

        dey
        dey
irq_trap:               ;BRK test or unextpected BRK or IRQ
        php             ;either SP or Y count will fail, if we do not hit
        dey
        dey
        dey
        ;next traps could be caused by unexpected BRK or IRQ
        ;check stack for BREAK and originating location
        ;possible jump/branch into weeds (uninitialized space)
        cmp #$ff-'B'    ;BRK pass 2 registers loaded?
        beq break2
        cmp #'B'        ;BRK pass 1 registers loaded?
        trap_ne
        cpx #'R'
        trap_ne
        cpy #'K'-3
        trap_ne
        sta irq_a       ;save registers during break test
        stx irq_x
        tsx             ;test break on stack
        lda $102,x
        cmp_flag 0      ;break test should have B=1 & unused=1 on stack
        trap_ne         ; - no break flag on stack
        pla
        cmp_flag intdis ;should have added interrupt disable
        trap_ne
        tsx
        cpx #$fc        ;sp -3? (return addr, flags)
        trap_ne
        lda $1ff        ;propper return on stack
        cmp #>brk_ret0
        trap_ne
        lda $1fe
        cmp #<brk_ret0
        trap_ne
        load_flag $ff
        pha
        ldx irq_x
        inx             ;return registers with modifications
        lda irq_a
        eor #$aa
        plp             ;N=1, V=1, Z=1, C=1 but original flags should be restored
        rti
        trap            ;runover protection
        jmp start       ;catastrophic error - cannot continue

break2:                 ;BRK pass 2
        cpx #$ff-'R'
        trap_ne
        cpy #$ff-'K'-3
        trap_ne
        sta irq_a       ;save registers during break test
        stx irq_x
        tsx             ;test break on stack
        lda $102,x
        cmp_flag $ff    ;break test should have B=1
        trap_ne         ; - no break flag on stack
        pla
        ora #decmode    ;ignore decmode cleared if 65c02
        cmp_flag $ff    ;actual passed flags
        trap_ne
        tsx
        cpx #$fc        ;sp -3? (return addr, flags)
        trap_ne
        lda $1ff        ;propper return on stack
        cmp #>brk_ret1
        trap_ne
        lda $1fe
        cmp #<brk_ret1
        trap_ne
        load_flag intdis
        pha
        ldx irq_x
        inx             ;return registers with modifications
        lda irq_a
        eor #$aa
        plp             ;N=0, V=0, Z=0, C=0 but original flags should be restored
        rti
        trap            ;runover protection
        jmp start       ;catastrophic error - cannot continue

;copy of data to initialize BSS segment
    .if (load_data_direct = 1) & (ROM_vectors = 1)
        .segment "VECTORS"
        .org $fffa       ;vectors
        .word   nmi_trap
        .word   res_trap
        .word   irq_trap
    .endif
