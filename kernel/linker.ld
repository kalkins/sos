ENTRY(_start)

SECTIONS {
  . = 0x200000;

  .text : {
    *(.text*)
  }

  .rodata : {
    *(.rodata*)
  }

  .data : {
    *(.data*)
  }

  . = ALIGN(4096);

  .bss : {
    *(.bss*)
    *(COMMON)
  }

  . = ALIGN(16);

  .stack : {
    _stack_bottom = .;
    . = . + 0x4000;
    _stack_top = .;
  }
}
