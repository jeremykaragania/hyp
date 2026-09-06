.section .head.text, "ax"

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
  beq init_el2

init_el1:
  // TODO: Add high virtual address space translation table.
  ldr x0, =init_trans_table_begin
  msr ttbr0_el1, x0

  /*
    Initialize TCR_EL1 with:
    T0SZ = 16
    T1SZ = 16
    TG1 = 4KB
  */
  mov x0, #16
  orr x0, x0, #(16 << 16)
  orr x0, x0, #(2 << 30)
  msr tcr_el1, x0
  isb

  // Enable the MMU.
  mrs x0, sctlr_el1
  orr x0, x0, #1
  msr sctlr_el1, x0
  isb
  b 1f

init_el2:
  // TODO: Add high virtual address space translation table.
  ldr x0, =init_trans_table_begin
  msr ttbr0_el2, x0

  /*
    Initialize TCR_EL2 with:
    T0SZ = 16
    T1SZ = 16
    TG1 = 4KB
  */
  mov x0, #16
  orr x0, x0, #(16 << 16)
  orr x0, x0, #(2 << 30)
  msr tcr_el2, x0
  isb

  // Enable the MMU.
  mrs x0, sctlr_el2
  orr x0, x0, #1
  msr sctlr_el2, x0
  isb

1:
  b main
