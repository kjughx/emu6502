;
; 6 5 0 2   F U N C T I O N A L   T E S T
;
; Practically cut+paste from https://github.com/Klaus2m5/6502_65C02_functional_tests
;
        .macro  trap
        jmp *           ;failed anyway
        .endmacro
        .macro  trap_eq
        beq *           ;failed equal (zero)
        .endmacro
        .macro  trap_ne
        bne *           ;failed not equal (non zero)
        .endmacro
        .macro  trap_cs
        bcs *           ;failed carry set
        .endmacro
        .macro  trap_cc
        bcc *           ;failed carry clear
        .endmacro
        .macro  trap_mi
        bmi *           ;failed minus (bit 7 set)
        .endmacro
        .macro  trap_pl
        bpl *           ;failed plus (bit 7 clear)
        .endmacro
        .macro  trap_vs
        bvs *           ;failed overflow set
        .endmacro
        .macro  trap_vc
        bvc *           ;failed overflow clear
        .endmacro
; please observe that during the test the stack gets invalidated
; therefore a RTS inside the success macro is not possible
        .macro  success
        jmp *           ;test passed, no errors
        .endmacro

        .macro  load_flag   p1
        lda #p1             ;allow test to change I-flag (no mask)
        .endmacro
        .macro  cmp_flag    p1
        cmp #(p1|fao)&m8    ;expected flags + always on bits
        .endmacro
        .macro  eor_flag    p1
        eor #p1|fao         ;invert expected flags + always on bits
        .endmacro

;macros to set (register|memory|zeropage) & status
            .macro      set_stat    p1          ;setting flags in the processor status register
            load_flag p1
            pha         ;use stack to load status
            plp
            .endmacro

            .macro      set_a       p1,p2       ;precharging accu & status
            load_flag p2
            pha         ;use stack to load status
            lda #p1     ;precharge accu
            plp
            .endmacro

            .macro      set_x       p1,p2       ;precharging index & status
            load_flag p2
            pha         ;use stack to load status
            ldx #p1     ;precharge index x
            plp
            .endmacro

            .macro      set_y       p1,p2       ;precharging index & status
            load_flag p2
            pha         ;use stack to load status
            ldy #p1     ;precharge index y
            plp
            .endmacro

            .macro      set_ax      p1,p2       ;precharging indexed accu & immediate status
            load_flag p2
            pha         ;use stack to load status
            lda p1,x    ;precharge accu
            plp
            .endmacro

            .macro      set_ay      p1,p2       ;precharging indexed accu & immediate status
            load_flag p2
            pha         ;use stack to load status
            lda p1,y    ;precharge accu
            plp
            .endmacro

            .macro      set_z       p1,p2       ;precharging indexed zp & immediate status
            load_flag p2
            pha         ;use stack to load status
            lda p1,x    ;load to zeropage
            sta zpt
            plp
            .endmacro

            .macro      set_zx      p1,p2       ;precharging zp,x & immediate status
            load_flag p2
            pha         ;use stack to load status
            lda p1,x    ;load to indexed zeropage
            sta zpt,x
            plp
            .endmacro

            .macro      set_abs     p1,p2       ;precharging indexed memory & immediate status
            load_flag p2
            pha         ;use stack to load status
            lda p1,x    ;load to memory
            sta abst
            plp
            .endmacro

            .macro      set_absx    p1,p2       ;precharging abs,x & immediate status
            load_flag p2
            pha         ;use stack to load status
            lda p1,x    ;load to indexed memory
            sta abst,x
            plp
            .endmacro

;macros to test (register|memory|zeropage) & status & (mask)
            .macro      tst_stat    p1          ;testing flags in the processor status register
            php         ;save status
            pla         ;use stack to retrieve status
            pha
            cmp_flag p1
            trap_ne
            plp         ;restore status
            .endmacro

            .macro      tst_a       p1,p2        ;testing result in accu & flags
            php         ;save flags
            cmp #p1     ;test result
            trap_ne
            pla         ;load status
            pha
            cmp_flag p2
            trap_ne
            plp         ;restore status
            .endmacro

            .macro      tst_x       p1,p2       ;testing result in x index & flags
            php         ;save flags
            cpx #p1     ;test result
            trap_ne
            pla         ;load status
            pha
            cmp_flag p2
            trap_ne
            plp         ;restore status
            .endmacro

            .macro      tst_y       p1,p2       ;testing result in y index & flags
            php         ;save flags
            cpy #p1     ;test result
            trap_ne
            pla         ;load status
            pha
            cmp_flag p2
            trap_ne
            plp         ;restore status
            .endmacro

            .macro      tst_ax      p1,p2,p3    ;indexed testing result in accu & flags
            php         ;save flags
            cmp p1,x    ;test result
            trap_ne
            pla         ;load status
            eor_flag p3
            cmp p2,x    ;test flags
            trap_ne     ;
            .endmacro

            .macro      tst_ay      p1,p2,p3    ;indexed testing result in accu & flags
            php         ;save flags
            cmp p1,y    ;test result
            trap_ne     ;
            pla         ;load status
            eor_flag p3
            cmp p2,y    ;test flags
            trap_ne
            .endmacro

            .macro      tst_z       p1,p2,p3    ;indexed testing result in zp & flags
            php         ;save flags
            lda zpt
            cmp p1,x    ;test result
            trap_ne
            pla         ;load status
            eor_flag p3
            cmp p2,x    ;test flags
            trap_ne
            .endmacro

            .macro      tst_zx      p1,p2,p3    ;testing result in zp,x & flags
            php         ;save flags
            lda zpt,x
            cmp p1,x    ;test result
            trap_ne
            pla         ;load status
            eor_flag p3
            cmp p2,x    ;test flags
            trap_ne
            .endmacro

            .macro      tst_abs     p1,p2,p3    ;indexed testing result in memory & flags
            php         ;save flags
            lda abst
            cmp p1,x    ;test result
            trap_ne
            pla         ;load status
            eor_flag p3
            cmp p2,x    ;test flags
            trap_ne
            .endmacro

            .macro      tst_absx    p1,p2,p3    ;testing result in abs,x & flags
            php         ;save flags
            lda abst,x
            cmp p1,x    ;test result
            trap_ne
            pla         ;load status
            eor_flag p3
            cmp p2,x    ;test flags
            trap_ne
            .endmacro
