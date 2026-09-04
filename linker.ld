ENTRY(_start)

SECTIONS {
  . = 0x40000000 + 2M;

  .head.text ALIGN (4K): {
    *(.head.text);
  }

  .text ALIGN(4K) : {
    *(.text .text.*);
  }

  .rodata ALIGN(4K) : {
    *(.rodata .rodata.*)
  }

  .stack ALIGN(4K) : {
    stack_begin = .;
    . = stack_begin + 2M;
    stack_end = .;
  }

  .data ALIGN(4K) : {
    *(.data .data.*)
  }

  .bss ALIGN(4K) : {
    *(.bss .bss.*)
    *(COMMON)
  }
}
