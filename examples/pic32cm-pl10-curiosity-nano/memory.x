/* PIC32CM6408PL10048, from the PIC32CM-PL_DFP pack description. Flash
 * starts at 0x0C000000; the configuration rows at 0x0D000000 (ROMCFG) and
 * 0x0D000400 (BOOTCFG) are deliberately not mapped. */
MEMORY
{
  FLASH : ORIGIN = 0x0C000000, LENGTH = 64K
  RAM   : ORIGIN = 0x20000000, LENGTH = 8K
}
