.section .head.text, "ax"

.macro boot_el level
  ldr x0, =vector_table_el\level
  msr vbar_el\level, x0

  // TODO: Add high virtual address space translation table.
  ldr x0, =init_trans_table_begin
  msr ttbr0_el\level, x0

  /*
    Initialize TCR_EL with:
    T0SZ = 16
    T1SZ = 16
    TG1 = 4KB
  */
  mov x0, #16
  orr x0, x0, #(16 << 16)
  orr x0, x0, #(2 << 30)
  msr tcr_el\level, x0
  isb

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

  ldr x0, =init_trans_table_begin
  bl create_init_mapping

  mrs x0, currentel
  lsr x0, x0, #2
  cmp x0, #2
  beq boot_el2

boot_el1:
  boot_el level=1

boot_el2:
  boot_el level=2

1:
  b main
