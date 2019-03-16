[![Build Status](https://travis-ci.org/stillinbeta/zeerust.svg?branch=master)](https://travis-ci.org/stillinbeta/zeerust)

# Zeerust

_[Something — a character design, a building, anything — used to be someone's idea of futuristic.][zeerust]_

Zeerust is a Z80 emulator written entirely in rust.
It contains modules for parsing Z80 opcodes, executing a symbolic representation, and attaching input and output devices.

There is also a binary that will print any bytes written to `OUT (0)` to stdout.

Take a look at the `tests/` directory for some example programs and usage!

## Debugging

Debug output will be provided when compiled in debug mode:

```
$ target/debug/zeerust tests/zeerust.bin
DEBUG - Running LD8(Reg(A), Immediate(90))
DEBUG - A: 00, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 00
DEBUG - Running OUT(Reg(A), Immediate(0))
DEBUG - A: 5a, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 02
DEBUG - Running LD8(Reg(A), Immediate(69))
DEBUG - A: 5a, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 04
DEBUG - Running OUT(Reg(A), Immediate(0))
DEBUG - A: 45, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 06
DEBUG - Running OUT(Reg(A), Immediate(0))
DEBUG - A: 45, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 08
DEBUG - Running LD8(Reg(A), Immediate(82))
DEBUG - A: 45, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 0a
DEBUG - Running OUT(Reg(A), Immediate(0))
DEBUG - A: 52, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 0c
DEBUG - Running LD8(Reg(A), Immediate(85))
DEBUG - A: 52, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 0e
DEBUG - Running OUT(Reg(A), Immediate(0))
DEBUG - A: 55, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 10
DEBUG - Running LD8(Reg(A), Immediate(83))
DEBUG - A: 55, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 12
DEBUG - Running OUT(Reg(A), Immediate(0))
DEBUG - A: 53, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 14
DEBUG - Running LD8(Reg(A), Immediate(84))
DEBUG - A: 53, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 16
DEBUG - Running OUT(Reg(A), Immediate(0))
DEBUG - A: 54, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 18
DEBUG - Running HALT
DEBUG - A: 54, B: 00, C: 00, D: 00, HL: 0000, F: 00000000, PC: 1a
ZEERUST%
```

## TODO

* [x] Loading registers
* [x] Arithmetic (8-bit)
* [x] Opcode parsing
* [x] Other operations!
* [x] CALL/RET
* [x] Jumping / Looping
* [x] 8-bit Bitwise operations
* [x] Input/Output
* [ ] 16-bit arithmetic
* [ ] Interrupts
* [ ] BCD support (`DAA`)
* [ ] Memory mapping
* [ ] ZX Spectrum or TI83 graphical emulation
* [ ] Debugger
* [ ] ???

[zeerust]: https://tvtropes.org/pmwiki/pmwiki.php/Main/Zeerust

## Show me!

```
cargo install --path .
zeerust tests/zeerust.bin
ZEERUST%
```
