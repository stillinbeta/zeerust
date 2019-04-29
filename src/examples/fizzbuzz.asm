	ld B, 0
loop:
	inc B
	ld E, B
	call printFizzBuzz
	ld A, B
	sub A, 20
	jp m, loop
	halt

printFizzBuzz:
	ld A, E
	ld C, 15
	call divmod
	jp z, fzbz
	ld A, E
	ld C, 5
	call divmod
	jp z, bz
	ld A, E
	ld C, 3
	call divmod
	jp z, fz
	ld A, E
	call printNum
	ret
fz:	ld HL, fizz
	jp end
bz:	ld HL, buzz
	jp end
fzbz:	ld HL, fizzbuzz
end:	call print0
	ret


printNum:
	ld C, 10              ; call with a 2-digit number in A
	call divmod
	ld C, A               ; save the remainder
	ld A, D
	call printDigit
	ld A, C               ; load the remainder
	call printDigit
	ld A, '\n'
	out(0), A
	ret

printDigit:
	add A, 30h          ; Call with 0-9 in A
	out (0), A
	ret

divmod:
	ld D, 0
divloop:
	sub A, C
	add a, 0
	ret z
	jp m, fixd
	inc D
	jp divloop
	ret
fixd:	add A, C
	ret

print0:
	ld A, (HL)              ; Set HL to a zero-terminated string to call
	add A, 0
	ret z
	out (0), A
	inc H
	jp print0

fizz: db "Fizz\n",0
buzz: db "Buzz\n",0
fizzbuzz: db "FizzBuzz\n",0
number: db "A Number"
