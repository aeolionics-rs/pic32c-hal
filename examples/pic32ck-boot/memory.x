/* pic32ck-boot: the bootloader in boot flash (BFM, 0x08000000), where the boot ROM loads the
 * stack pointer and PC from the first two words: cortex-m-rt's vector table goes there. The
 * lower panel's 64 KiB of BFM; RAM all of it (the application's startup zeroes it again). */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}
