      ld B, 9
jump: ld A, B
      add A, 48                  ; int -> ascii
      out (0), A
      ld A, '\n'
      out (0), A
      djnz jump
      halt
