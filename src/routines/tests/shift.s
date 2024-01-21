.include "macros.s"
.include "configuration.s"

        .CODE
        .org code_segment
        .P02            ; disable 65SC02, 65C02 and 65816 instructions
start:

; testing shifts - ASL LSR ROL ROR all addressing modes
; shifts - accumulator
        ldx #5
tasl:
        set_ax zps,0
        asl a
        tst_ax rASL,fASL,0
        dex
        bpl tasl
        ldx #5
tasl1:
        set_ax zps,$ff
        asl a
        tst_ax rASL,fASL,$ff-fnzc
        dex
        bpl tasl1

        ldx #5
tlsr:
        set_ax zps,0
        lsr a
        tst_ax rLSR,fLSR,0
        dex
        bpl tlsr
        ldx #5
tlsr1:
        set_ax zps,$ff
        lsr a
        tst_ax rLSR,fLSR,$ff-fnzc
        dex
        bpl tlsr1

        ldx #5
trol:
        set_ax zps,0
        rol a
        tst_ax rROL,fROL,0
        dex
        bpl trol
        ldx #5
trol1:
        set_ax zps,$ff-fc
        rol a
        tst_ax rROL,fROL,$ff-fnzc
        dex
        bpl trol1

        ldx #5
trolc:
        set_ax zps,fc
        rol a
        tst_ax rROLc,fROLc,0
        dex
        bpl trolc
        ldx #5
trolc1:
        set_ax zps,$ff
        rol a
        tst_ax rROLc,fROLc,$ff-fnzc
        dex
        bpl trolc1

        ldx #5
tror:
        set_ax zps,0
        ror a
        tst_ax rROR,fROR,0
        dex
        bpl tror
        ldx #5
tror1:
        set_ax zps,$ff-fc
        ror a
        tst_ax rROR,fROR,$ff-fnzc
        dex
        bpl tror1

        ldx #5
trorc:
        set_ax zps,fc
        ror a
        tst_ax rRORc,fRORc,0
        dex
        bpl trorc
        ldx #5
trorc1:
        set_ax zps,$ff
        ror a
        tst_ax rRORc,fRORc,$ff-fnzc
        dex
        bpl trorc1

; shifts - zeropage
        ldx #5
tasl2:
        set_z zps,0
        asl zpt
        tst_z rASL,fASL,0
        dex
        bpl tasl2
        ldx #5
tasl3:
        set_z zps,$ff
        asl zpt
        tst_z rASL,fASL,$ff-fnzc
        dex
        bpl tasl3

        ldx #5
tlsr2:
        set_z zps,0
        lsr zpt
        tst_z rLSR,fLSR,0
        dex
        bpl tlsr2
        ldx #5
tlsr3:
        set_z zps,$ff
        lsr zpt
        tst_z rLSR,fLSR,$ff-fnzc
        dex
        bpl tlsr3

        ldx #5
trol2:
        set_z zps,0
        rol zpt
        tst_z rROL,fROL,0
        dex
        bpl trol2
        ldx #5
trol3:
        set_z zps,$ff-fc
        rol zpt
        tst_z rROL,fROL,$ff-fnzc
        dex
        bpl trol3

        ldx #5
trolc2:
        set_z zps,fc
        rol zpt
        tst_z rROLc,fROLc,0
        dex
        bpl trolc2
        ldx #5
trolc3:
        set_z zps,$ff
        rol zpt
        tst_z rROLc,fROLc,$ff-fnzc
        dex
        bpl trolc3

        ldx #5
tror2:
        set_z zps,0
        ror zpt
        tst_z rROR,fROR,0
        dex
        bpl tror2
        ldx #5
tror3:
        set_z zps,$ff-fc
        ror zpt
        tst_z rROR,fROR,$ff-fnzc
        dex
        bpl tror3

        ldx #5
trorc2:
        set_z zps,fc
        ror zpt
        tst_z rRORc,fRORc,0
        dex
        bpl trorc2
        ldx #5
trorc3:
        set_z zps,$ff
        ror zpt
        tst_z rRORc,fRORc,$ff-fnzc
        dex
        bpl trorc3

; shifts - absolute
        ldx #5
tasl4:
        set_abs zps,0
        asl abst
        tst_abs rASL,fASL,0
        dex
        bpl tasl4
        ldx #5
tasl5:
        set_abs zps,$ff
        asl abst
        tst_abs rASL,fASL,$ff-fnzc
        dex
        bpl tasl5

        ldx #5
tlsr4:
        set_abs zps,0
        lsr abst
        tst_abs rLSR,fLSR,0
        dex
        bpl tlsr4
        ldx #5
tlsr5:
        set_abs zps,$ff
        lsr abst
        tst_abs rLSR,fLSR,$ff-fnzc
        dex
        bpl tlsr5

        ldx #5
trol4:
        set_abs zps,0
        rol abst
        tst_abs rROL,fROL,0
        dex
        bpl trol4
        ldx #5
trol5:
        set_abs zps,$ff-fc
        rol abst
        tst_abs rROL,fROL,$ff-fnzc
        dex
        bpl trol5

        ldx #5
trolc4:
        set_abs zps,fc
        rol abst
        tst_abs rROLc,fROLc,0
        dex
        bpl trolc4
        ldx #5
trolc5:
        set_abs zps,$ff
        rol abst
        tst_abs rROLc,fROLc,$ff-fnzc
        dex
        bpl trolc5

        ldx #5
tror4:
        set_abs zps,0
        ror abst
        tst_abs rROR,fROR,0
        dex
        bpl tror4
        ldx #5
tror5:
        set_abs zps,$ff-fc
        ror abst
        tst_abs rROR,fROR,$ff-fnzc
        dex
        bpl tror5

        ldx #5
trorc4:
        set_abs zps,fc
        ror abst
        tst_abs rRORc,fRORc,0
        dex
        bpl trorc4
        ldx #5
trorc5:
        set_abs zps,$ff
        ror abst
        tst_abs rRORc,fRORc,$ff-fnzc
        dex
        bpl trorc5

; shifts - zp indexed
        ldx #5
tasl6:
        set_zx zps,0
        asl zpt,x
        tst_zx rASL,fASL,0
        dex
        bpl tasl6
        ldx #5
tasl7:
        set_zx zps,$ff
        asl zpt,x
        tst_zx rASL,fASL,$ff-fnzc
        dex
        bpl tasl7

        ldx #5
tlsr6:
        set_zx zps,0
        lsr zpt,x
        tst_zx rLSR,fLSR,0
        dex
        bpl tlsr6
        ldx #5
tlsr7:
        set_zx zps,$ff
        lsr zpt,x
        tst_zx rLSR,fLSR,$ff-fnzc
        dex
        bpl tlsr7

        ldx #5
trol6:
        set_zx zps,0
        rol zpt,x
        tst_zx rROL,fROL,0
        dex
        bpl trol6
        ldx #5
trol7:
        set_zx zps,$ff-fc
        rol zpt,x
        tst_zx rROL,fROL,$ff-fnzc
        dex
        bpl trol7

        ldx #5
trolc6:
        set_zx zps,fc
        rol zpt,x
        tst_zx rROLc,fROLc,0
        dex
        bpl trolc6
        ldx #5
trolc7:
        set_zx zps,$ff
        rol zpt,x
        tst_zx rROLc,fROLc,$ff-fnzc
        dex
        bpl trolc7

        ldx #5
tror6:
        set_zx zps,0
        ror zpt,x
        tst_zx rROR,fROR,0
        dex
        bpl tror6
        ldx #5
tror7:
        set_zx zps,$ff-fc
        ror zpt,x
        tst_zx rROR,fROR,$ff-fnzc
        dex
        bpl tror7

        ldx #5
trorc6:
        set_zx zps,fc
        ror zpt,x
        tst_zx rRORc,fRORc,0
        dex
        bpl trorc6
        ldx #5
trorc7:
        set_zx zps,$ff
        ror zpt,x
        tst_zx rRORc,fRORc,$ff-fnzc
        dex
        bpl trorc7

; shifts - abs indexed
        ldx #5
tasl8:
        set_absx zps,0
        asl abst,x
        tst_absx rASL,fASL,0
        dex
        bpl tasl8
        ldx #5
tasl9:
        set_absx zps,$ff
        asl abst,x
        tst_absx rASL,fASL,$ff-fnzc
        dex
        bpl tasl9

        ldx #5
tlsr8:
        set_absx zps,0
        lsr abst,x
        tst_absx rLSR,fLSR,0
        dex
        bpl tlsr8
        ldx #5
tlsr9:
        set_absx zps,$ff
        lsr abst,x
        tst_absx rLSR,fLSR,$ff-fnzc
        dex
        bpl tlsr9

        ldx #5
trol8:
        set_absx zps,0
        rol abst,x
        tst_absx rROL,fROL,0
        dex
        bpl trol8
        ldx #5
trol9:
        set_absx zps,$ff-fc
        rol abst,x
        tst_absx rROL,fROL,$ff-fnzc
        dex
        bpl trol9

        ldx #5
trolc8:
        set_absx zps,fc
        rol abst,x
        tst_absx rROLc,fROLc,0
        dex
        bpl trolc8
        ldx #5
trolc9:
        set_absx zps,$ff
        rol abst,x
        tst_absx rROLc,fROLc,$ff-fnzc
        dex
        bpl trolc9

        ldx #5
tror8:
        set_absx zps,0
        ror abst,x
        tst_absx rROR,fROR,0
        dex
        bpl tror8
        ldx #5
tror9:
        set_absx zps,$ff-fc
        ror abst,x
        tst_absx rROR,fROR,$ff-fnzc
        dex
        bpl tror9

        ldx #5
trorc8:
        set_absx zps,fc
        ror abst,x
        tst_absx rRORc,fRORc,0
        dex
        bpl trorc8
        ldx #5
trorc9:
        set_absx zps,$ff
        ror abst,x
        tst_absx rRORc,fRORc,$ff-fnzc
        dex
        bpl trorc9

        success

brk_ret0:
    trap
brk_ret1:
    trap


