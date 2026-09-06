ENTRY(_start)

SECTIONS {
  . = 0x40000000 + 2M;

  image_begin = .;

  .init.text ALIGN(4K) : {
    text_begin = .;
    *(.head.text)
    *(.init.text)
  }

  .text ALIGN(4K) : {
    *(.text .text.*)
  }

  .init.rodata ALIGN(4K) : {
    data_begin = .;
    *(.init.rodata)
  }

  .rodata ALIGN(4K) : {
    *(.rodata .rodata.*)
  }

  .init.data ALIGN(4K) : {
    *(.init.data)
  }

  .data ALIGN(4K) : {
    *(.data .data.*)
  }

  .init.bss ALIGN(4K) : {
    *(.init.bss)
  }

  .bss ALIGN(4K) : {
    *(.bss .bss.*)
    *(COMMON)
  }

  .vector_tables ALIGN(4K) : {
    *(.vector_tables)
  }

  . = ALIGN(4K);
  init_trans_table_begin = .;
  . += 64 * 4K;
  init_trans_table_end = .;

  . = ALIGN(4K);
  init_stack_begin = .;
  . = init_stack_begin + 4K;
  init_stack_end = .;

  data_end = .;

  image_end = .;
}
