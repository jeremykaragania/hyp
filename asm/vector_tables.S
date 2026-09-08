.section .vector_tables, "ax"

// EL1 vector table.
.global vector_table_el1
.balign 0x800
vector_table_el1:

// Current EL with SP0
sync_curr_sp0_el1:
b .

.balign 0x80
irq_curr_sp0_el1:
b .

.balign 0x80
fiq_curr_sp0_el1:
b .

.balign 0x80
serror_curr_sp0_el1:
b .

// Current EL with SPx.
.balign 0x80
sync_curr_spx_el1:
b .

.balign 0x80
irq_curr_spx_el1:
b .

.balign 0x80
fiq_curr_spx_el1:
b .

.balign 0x80
serror_curr_spx_el1:
b .

// Lower EL using AArch64.
.balign 0x80
sync_low_aarch64_el1:
b .

.balign 0x80
irq_low_aarch64_el1:
b .

.balign 0x80
fiq_low_aarch64_el1:
b .

.balign 0x80
serror_low_aarch64_el1:
b .

// Lower EL using AArch32.
.balign 0x80
sync_low_aarch32_el1:
b .

.balign 0x80
irq_low_aarch32_el1:
b .

.balign 0x80
fiq_low_aarch32_el1:
b .

.balign 0x80
serror_low_aarch32_el1:
b .

// EL2 vector table.
.global vector_table_el2
vector_table_el2:

// Current EL with SP0
sync_curr_sp0_el2:
b .

.balign 0x80
irq_curr_sp0_el2:
b .

.balign 0x80
fiq_curr_sp0_el2:
b .

.balign 0x80
serror_curr_sp0_el2:
b .

// Current EL with SPx.
.balign 0x80
sync_curr_spx_el2:
b .

.balign 0x80
irq_curr_spx_el2:
b .

.balign 0x80
fiq_curr_spx_el2:
b .

.balign 0x80
serror_curr_spx_el2:
b .

// Lower EL using AArch64.
.balign 0x80
sync_low_aarch64_el2:
b .

.balign 0x80
irq_low_aarch64_el2:
b .

.balign 0x80
fiq_low_aarch64_el2:
b .

.balign 0x80
serror_low_aarch64_el2:
b .

// Lower EL using AArch32.
.balign 0x80
sync_low_aarch32_el2:
b .

.balign 0x80
irq_low_aarch32_el2:
b .

.balign 0x80
fiq_low_aarch32_el2:
b .

.balign 0x80
serror_low_aarch32_el2:
b .

// EL2 vector table.
.global vector_table_el3
vector_table_el3:

// Current EL with SP0
sync_curr_sp0_el3:
b .

.balign 0x80
irq_curr_sp0_el3:
b .

.balign 0x80
fiq_curr_sp0_el3:
b .

.balign 0x80
serror_curr_sp0_el3:
b .

// Current EL with SPx.
.balign 0x80
sync_curr_spx_el3:
b .

.balign 0x80
irq_curr_spx_el3:
b .

.balign 0x80
fiq_curr_spx_el3:
b .

.balign 0x80
serror_curr_spx_el3:
b .

// Lower EL using AArch64.
.balign 0x80
sync_low_aarch64_el3:
b .

.balign 0x80
irq_low_aarch64_el3:
b .

.balign 0x80
fiq_low_aarch64_el3:
b .

.balign 0x80
serror_low_aarch64_el3:
b .

// Lower EL using AArch32.
.balign 0x80
sync_low_aarch32_el3:
b .

.balign 0x80
irq_low_aarch32_el3:
b .

.balign 0x80
fiq_low_aarch32_el3:
b .

.balign 0x80
serror_low_aarch32_el3:
b .
