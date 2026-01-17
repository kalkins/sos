ENTRY(_start)

SECTIONS {
  . = 0x200000;

  .text : ALIGN(4096) {
    *(.text*)
  }

  .rodata : ALIGN(4096) {
    *(.rodata*)
  }

  .data : ALIGN(4096) {
    *(.data*)
  }

  .bss : ALIGN(4096) {
    *(.bss*)
    *(COMMON)
  }

  .stack : ALIGN(4096) {
    _stack_bottom = .;
    . = . + 0x4000;
    _stack_top = .;
  }

  /DISCARD/ : {
    *(.eh_frame)
    *(.note .note.*)
  }
}
