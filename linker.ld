ENTRY(_start)

SECTIONS {
  . = 0x40000000 + 0x10000;

  .text : {
    *(.text);
  }

  .rodata : {
    *(.rodata)
  }

  .data : {
    *(.data)
  }

  .bss : {
    *(.bss)
    *(COMMON)
  }
}
