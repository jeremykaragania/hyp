.section .text

.global spinlock_lock
spinlock_lock:
  mov w1, #1
spin:
  ldxr w2, [x0]
  cbnz w2, spin

  stxr w3, w1, [x0]
  cbnz w3, spin

  ret
