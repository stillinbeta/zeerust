      ld HL, data
jump: ld A, (HL)
      add A, 0
      jp Z, end
      out (0), A
      inc H
      jp jump
end:  halt
data: defb "Hello World\n",0
