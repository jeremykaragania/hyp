.section .text

.global dsb
dsb:
  dsb ish
  ret

.global isb
isb:
  isb
  ret
