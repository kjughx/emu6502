LDX #$FF        ; 
SEI             ; set interrupt disable
TXS             ; transfer X to stack
CLD             ; clear decimal flag
JSR $FD02       ; check for cart
BNE $FCEF       ; .Z=0? then no cart detected
JMP $8000       ; direct to cartridge cold start via vector
STX $D016       ; sets bit 5 (MCM) off, bit 3 (38 cols) off
JSR $FDA3       ; initialise I/O
JSR $FD50       ; initialise memory
JSR $FD15       ; set I/O vectors ($0314..$0333) to kernal defaults
JSR $FF5B       ; more initialising... mostly set system IRQ to correct value and start
CLI             ; clear interrupt flag
JMP $A000       ; direct to BASIC cold start via vector
