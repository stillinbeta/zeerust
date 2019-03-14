ASM ?= "z80asm"
FILES = hello_world.bin zeerust.bin countdown.bin

all: $(FILES)

%.bin: %.asm
	$(ASM) -o $@ $^

.PHONY: clean
clean:
	rm -f $(FILES)
