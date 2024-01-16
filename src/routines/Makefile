
all: wozmon

wozmon: wozmon.s
	./cc65/bin/ca65 wozmon.s
	./cc65/bin/ld65 -C script.ld wozmon.o -o wozmon.bin

clean:
	@rm -f wozmon.o a.out
