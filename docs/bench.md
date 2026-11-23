Bench for HAL testing
=====================

The boards and wiring that run every example in `examples/`. Anything not
listed here (sensors, Ethernet modules, ADC jumpers) is wired only while
its example is being tested.

# Boards

| Board | Why |
|---|---|
| PIC32CK SG01 Curiosity Ultra (EV33A17A) | The main CK board: TrustZone (the `PORT_SEC` view), the HSM; `uart_peer`, the far end for the UART examples. |
| PIC32CK GC01 Curiosity Ultra (EV44P93A) | The CK without TrustZone; the far end for `can_ping`/`can_echo`. |
| PIC32CM PL10 Curiosity Nano (EV10P22A) | The PL10 examples. |
| PIC32CM GV-Value Line Curiosity Nano+ Touch (EV80D90A) | The GV00 examples. Its target supply must be 3.3 V: the user guide gives a 5.0 V factory default, over the GV00's 3.63 V (`pymcuprog getvoltage` reads it). |

# Connections

- **USB to the host:** each Curiosity Ultra's PKoB4 debug port (flashing
  and the VCOM log), the PL10's and the GV's nEDBG. A CK board's J203 only
  while running `usb_echo`, and the SG01's J202 (USB-C, high speed) while
  running `usbhs_echo`, by an A-to-C or a C-to-C cable: the CC switches'
  control (USB0_DRD, PA12) is pulled low, which leaves 5.1k on CC, so a
  USB-C host sees a device and turns VBUS on. On a Linux host (for the
  suspend tests), `tools/udev/70-pic32c-hal-usbhs.rules` in
  `/etc/udev/rules.d/` gives 1209:000B's tty to plugdev and lets it
  autosuspend after 2 s and wake the host. Keep anything that back-powers
  a hub off the boards' hubs, or give it a VBUS-blocking cable: a device on
  external power that fed VBUS back made a hub reset itself and drop
  high-speed devices; without it the same hub passes the HS tests.
- **Ethernet:** both CK boards' RJ45 to the bench switch, for `eth_ping`,
  `eth_http` and `eth_https`.
- **CAN:** SG01 J704 to GC01 J704, pin to pin (1 CANL, 2 GND, 3 CANH), with
  SW700 termination on at both ends. J705 is CAN1, not CAN0.
- **UART:** the SG01's EXT2 header (J501) to the GC01's, crossed: TX pin 16
  (PC15) to the other's RX pin 18 (PC16) and RTS pin 17 (PC17) to the
  other's CTS pin 6 (PC18), both ways, and GND. The SG01 runs `uart_peer`,
  the GC01 the example under test; start `uart_peer` first. Keep both
  boards powered while wired, so neither back-feeds the other through the
  UART pins.
- **The SG01's mikroBUS socket stays empty, and `spi` needs its EXT2
  unwired:** PC15-PC17 are both EXT2's UART and the mikroBUS SPI, and `spi`
  needs nothing on them for its loopback. So unplug the UART link from the
  SG01's EXT2 while `spi` runs there. `i2c` uses the on-board EEPROM.
- **Click boards,** in a CK board's mikroBUS socket only while their program
  runs: a MikroElektronika TempHum 15 Click (SHT40) for `moff` and `i2c`, a
  Weather Click (BME280, in its I2C setting) for `i2c`, and an ETH WIZ
  Click (W5500) for `w5500`. The ETH WIZ
  Click's SPI pins are PC15-PC17, so unplug the UART link from that board
  while it's in; its RST (PD21) also resets the on-board Ethernet PHY.
- **GV:** PA10 jumpered to PA11 and PA18 to PA19, for `exti` (PA10-PA11
  for `open_drain` too); PA09 to
  PA08, for `nmi`; an SHT40 and BMP280 module on PA22 (SDA) and PA23
  (SCL), with VTG and GND, for `i2c`. The board's silkscreen has no pin
  names: count along the edge from the user guide's pinout.
- **PL10:** PA08 jumpered to PA09 and PA10 to PA11, for `exti` (PA10-PA11
  for `open_drain` too); PA00 to
  PA01, for `nmi` (off for `i2c`, which has its device on those pins);
  PA04-PA07 (TX, RX, RTS, CTS) to the SG01's EXT2 running `uart_peer`, for
  `uart_flow`, and nothing on them for `spi`'s loopback; PA04 jumpered to
  PA05, for `uart_bridge`. Plug the PL10's USB in before leaving the SG01's
  `uart_peer` on: its TX and RTS power an
  unplugged PL10 through PA05 and PA07.

# Flashing

- Both families with pyOCD and Microchip's packs (`Microchip.PIC32CK-SG_DFP`
  and `-GC_DFP`, `Microchip.PIC32CM-PL_DFP`, `Microchip.PIC32CM-GV_DFP`);
  the PL10 at 100 kHz.
- Never probe-rs to flash or reset the PL10: it can't reset this family,
  and older versions wedge the nEDBG.
- On the CK boards, erase the first boot-flash sector before loading, and
  back up BFM, PFM and CFM before the first flash of a new board.
