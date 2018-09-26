ENTRY(_reset);

MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM   : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS
{
  .text :
  {
    /* Vector table */
    LONG(ORIGIN(RAM) + LENGTH(RAM))
    LONG(_reset + 1);

    /* Reset handler */
    _reset = .;
    *(.text._reset)

    *(.text*)
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}