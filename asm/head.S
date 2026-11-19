.section .head.text, "ax"

.macro boot_el level
  ldr x0, =vector_table_el\level
  msr vbar_el\level, x0

  // TODO: Add high virtual address space translation table.
  ldr x0, =TABLE_POOL
  msr ttbr0_el\level, x0

  /*
    Initialize TCR_EL with:
    T0SZ = 16
    TG0 = 4KB
  */
  mov x0, #16
  msr tcr_el\level, x0
  isb

  // Initialize Memory Attribute Indirection Register.
  // For now, we just creat two memory attributes for Normal and device memory.
  // Attr0 (0b01000100):
  //    Normal memory, Outer Non-cacheable, Inner Non-cacheable.
  // Attr1 (0b00000000):
  //    Device memory, non Gathering, non Re-ordering, non Early Write
  //    Acknowledgement.
  // TODO: For Normal memory, we haven't specified caching because we're not
  // sure what we actually want. We should come back and decide.
  mov x0, #68
  msr mair_el\level, x0

  // Enable the MMU.
  mrs x0, sctlr_el\level
  orr x0, x0, #1
  msr sctlr_el\level, x0
  isb

  b 1f
.endm

.global _start
_start:
  mrs x0, mpidr_el1
  and x0, x0, #0xffff
  cbz x0, boot

sleep:
  wfi
  b sleep

boot:
  ldr x0, =init_stack_end
  mov sp, x0

  bl init_table_alloc
  bl create_init_mapping

  bl current_el
  cmp x0, #2
  beq boot_el2

boot_el1:
  boot_el level=1

boot_el2:
  boot_el level=2

1:
  b main

.global current_el
current_el:
  mrs x0, currentel
  lsr x0, x0, #2
  ret
