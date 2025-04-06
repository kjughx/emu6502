SRCS = \
	   branch.s \
	   jump_call.s \
	   stack.s \
	   flag.s \
	   increment_decrement.s \
	   load_store.s \
	   logical.s \
	   shift.s \
	   arithmetic.s

OBJS = $(SRCS:%.s=%.bin)

all: $(OBJS)

%.bin: %.s
	./cc65/bin/ca65 -l $*.lst $<
	./cc65/bin/ld65 $*.o -o ../../hardware/cpu/instructions/$@ -m $*.map -C script.ld
	@rm -f $*.o $*.map


clean:
	@rm -rf *.o *.lst ../../hardware/cpu/instructions/*.bin *.map
