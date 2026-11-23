pic32c-hal
==========

[![CI](https://github.com/aeolionics-rs/pic32c-hal/actions/workflows/ci.yml/badge.svg)](https://github.com/aeolionics-rs/pic32c-hal/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#licence)
[![Rust 1.100](https://img.shields.io/badge/rust-1.100-orange.svg)](rust-toolchain.toml)
[![no_std](https://img.shields.io/badge/no__std-yes-green.svg)](https://docs.rust-embedded.org/book/intro/no-std.html)

Rust drivers for Microchip's PIC32C Arm microcontrollers, starting with the
PIC32CK SG/GC (Cortex-M33), the PIC32CM SG00 (Cortex-M23) and the
PIC32CM PL10 and GV00 (Cortex-M0+).
We don't know of any other Rust HAL that covers the PIC32C parts at this time.

This repo is under active development with dependencies on as yet unreleased
versions of:

* can-hal-rs, rather than embedded-can, for CAN-FD; our fork of it, for
  confirmed transmission, bus recovery and statistics
* can-common, support code shared with CAN drivers in our other HALs
* embassy, at a revision of main, for embassy-net on its new xarxa stack
  ahead of a release (and embassy-usb-driver 0.3); embassy main needs Rust
  1.100

The plan is to publish to crates.io once these have stabilized.

- `pic32c-hal`: the drivers, one module each, on the registers directly,
  in Embassy's style: `init(Config)` returns the `Peripherals` singletons,
  which drivers take as `Peri`s, and drivers that need interrupts take a
  `bind_interrupts!` type (both from `embassy-hal-internal`). SPI, I²C and
  every UART type implement `embassy-embedded-hal`'s `SetConfig`, for its
  shared-bus devices (`spi` and `i2c` change rate and mode with it on the
  GC01). Choose the chip
  with feature `pic32ck2051sg01144` or `pic32ck2051gc01144`, each on its
  own PAC; `rt` brings in the vector table. Host
  tests with `cargo test -p pic32c-hal --features
  pic32ck2051sg01144,can-hal,time-driver,usb,usbhs`.
  - `adc`: the 12-bit SAR ADC, one read at a time in the core's
    software-controlled mode (its channel and trigger registers are fixed
    while it's enabled), embassy-stm32's shape: `Adc::new_blocking` /
    `new(adc, Irqs, config)`, `blocking_read` / `read(&mut channel,
    SampleTime)` on a pin (VINP0-11: PA04-PA12, PB00, PB11, PB12),
    `AnyAdcChannel`, `enable_vrefint()` (the 1.2 V bandgap) or
    `enable_temperature()`. Results on a 12-bit scale at every
    resolution. `Calibration` reads the factory's bandgap and temperature
    sensor readings (CAL OTP FCCFG69-71); `millivolts` gives AVDD from the
    bandgap (3292 and 3257 mV on the bench boards). The temperature
    sensor's output is 180-190 mV below its calibration (Microchip's
    erratum 2.18.1, whose correction isn't documented), so `celsius`
    reads about 65 °C low and is left uncorrected for now: see the
    module's known-issue notes.
  - `dma`: the two DMA controllers' 12 channels (`p.DMA0_CH0` to
    `p.DMA1_CH3`; the PIC32CK's own DMA IP, not the SAM D5x DMAC) as
    `Transfer` futures: `new_copy` memory to memory in the widest beats the
    alignment allows (64 KiB in 327 µs, about 200 MB/s, against the CPU's
    548 µs), and `new_read`/`new_write` between memory and a peripheral's
    data register, paced by its DMA `Request` (SERCOMs named; others by
    index). Dropping a transfer stops it. CAN, Ethernet and USB have bus
    masters of their own and don't use it.
  - `cache`: the CMCC, the L1 cache in front of flash; `init` turns it on
    unless `Config.cache` says not to, and the flash driver invalidates it.
  - `can` (feature `can-hal`): CAN0 and CAN1, Bosch `M_CAN` controllers,
    as can-hal channels (our fork's `aeolionics` branch): confirmed
    transmission, CAN FD with bit-rate switching, live acceptance filters
    (`Filterable`), bus status, recovery and statistics. `Can::new` takes
    the controller, a `bind_interrupts!` type, TX and RX pins typed from the
    pin multiplexing table, message RAM (`ClassicMessageRam` or
    `FdMessageRam`, sized for its filter lists) and a `State`, and feeds the
    kernel clock from the generator its `Config` names (2 by default). The
    feature is named for the traits, leaving room for an `embedded-can` one.
    Transmit ordering, the event queue, the bit-timing solver and `Clock`
    come from can-common (aeolionics-rs/can-common), a git dependency.
  - `clocks`: the XOSC, PLL0 and the twelve generic clock generators, from
    a checked `clocks::Config` (`Config::pll_from_xosc` for a crystal to
    120 MHz); `init` brings it up, sets the flash wait states, and records
    the rates for drivers.
  - `gpio`: `Input`, `Output`, `OutputOpenDrain` and `Flex` on pin
    singletons (`p.PD20`, 107 on the 144-pin package) or `AnyPin`, with
    pulls, slew and the embedded-hal 1.0 traits; `gpio::set_as_function`
    hands a pin to a peripheral the HAL doesn't drive yet. On the SG it uses
    PORT's Secure view; the chip feature picks.
  - `flash`: program flash erase and write through the FCW, as an
    `embedded-storage` `NorFlash` (4 KiB pages, 32-byte flash words, 1 KiB
    rows from SRAM).
  - `eth` (feature `eth`): the Ethernet MAC (Cadence GEM) as a
    `xarxa-driver` driver (embassy-net's, on xarxa), RMII to the board's
    KSZ8091, with PHY discovery and link polling over MDIO; with
    embassy-net it gets DHCP and SLAAC addresses and answers ping, IPv4 and
    IPv6.
  - `exti` (feature `exti`): `ExtiInput`, waiting on a pin's levels and
    edges through the EIC (embedded-hal-async `Wait`), the pin's type naming
    its line (`p.EXTINT0` to `p.EXTINT15`); with `rt` the HAL owns the 16
    vectors. Secure view on the SG, as for PORT. `Nmi` arms the NMI pin
    (PD02) for one firing at a time; the application's `NonMaskableInt`
    handler calls `exti::on_nmi`, which takes no lock.
  - `i2c`: a SERCOM as an I2C host (7-bit addresses, 100 kHz to 1 MHz with
    Fast-mode Plus's 1:2 duty, repeated starts between a transaction's
    reads and writes), SDA on pad 0 and SCL on pad 1. `new_blocking` polls;
    `new` waits on the SERCOM's MB and SB interrupts. embedded-hal `I2c`,
    and embedded-hal-async's on the interrupt one. SCL held low 25-35 ms
    gives `Error::Timeout` and a reset.
  - `rtc`: the RTC as a clock and calendar (MODE2, 1 Hz from the 32.768 kHz
    crystal's 1.024 kHz output, or the ULP oscillator's), 2000 to 2063 as
    `DateTime` with Unix-time conversions. `Rtc::new` keeps the time when
    it finds the RTC already running so (it runs through every reset but
    power-on; `was_running` says which), otherwise starts the crystal (not
    on demand, gain CGM 15: at its reset gain of 8 the boards' crystal
    doesn't start) and sets 2000-01-01. `now`, `set`, and `wait_until` on
    ALARM0 (matched a second early, since the flag comes on the tick after
    the match).
  - `sercom`: what the SERCOM drivers share: `Pad<T, PAD, IOSET>`, each
    pin's pad, function and I/O set, generated from the ATDF by
    `tools/pin_tables.py` (CAN's pins likewise). A driver takes all its
    pins in one I/O set, as the datasheet requires, and the compiler infers
    it and refuses a mix.
  - `spi`: a SERCOM as an SPI host (8-bit, modes 0-3, MSB or LSB first,
    SCK up to half the SERCOM clock: 24 MHz from 48 MHz). Pins go by SERCOM
    pad (`sercom::Pad`; SCK pad 1, MOSI pad 0 or 3, MISO another, all in one
    I/O set, checked at compile time). `new_blocking`
    moves bytes with the CPU; `new` takes two `dma` channels, receive and
    transmit together (4 KiB at 24 MHz in 1.4 ms, the line rate). Both
    implement embedded-hal's `SpiBus`, the DMA one embedded-hal-async's
    too. `Config::loopback` reads back through the MOSI pin; chip select
    is the caller's. SCK is capped at the SERCOM's rated 21.3 MHz (48 MHz gives 12 at most, 80 MHz 20); `Config.slew` slew-limits SCK and MOSI (embassy-stm32's `gpio_speed`), which long leads need.
  - `uart`: a SERCOM as a UART (8 data bits, parity, stop bits), whole or
    `split` into `UartTx` and `UartRx`, with embedded-io and
    embedded-io-async; TX on pad 0 and RX on pad 1 to 3, in one I/O set
    (`sercom::Pad`), the SERCOM's clock from the generator its `Config`
    names. As in Embassy: `new_blocking` polls; `new` takes two `dma`
    channels, and its `write` and `read` go by DMA (`read` fills the whole
    buffer; the embedded-io-async `Read` still returns once a byte is in).
    `BufferedUart` (as Embassy's) keeps ring buffers the caller gives
    through the SERCOM's interrupts, with embedded-io-async `Read`, `Write`
    and `ReadReady`; a full receive ring leaves bytes in the SERCOM, so with
    flow control RTS holds the far end and nothing is lost. Each has a
    `_with_rtscts` form for hardware flow control (RX pad 1, RTS pad 2, CTS
    pad 3). `UartRx::into_ring_buffered` (as Embassy's) gives a
    `RingBufferedUartRx`, its DMA channel filling a ring continuously through
    a self-linked descriptor (`dma::ReadableRing`), the CPU doing nothing per
    byte; a reader a whole ring behind gets `Error::Overrun`. The FIFO (16 bytes on
    SERCOM0, 8 on the others) is on by default. Receive DMA started on an
    empty FIFO takes a stale byte from the FIFO's memory first (seen on the
    bench), so with the FIFO on a DMA read starts its transfer only once a
    byte is in: at once if one is, else on the RXC interrupt. `BufferedUart` also has `fill_buf`/`consume` (embedded-io-async `BufRead`, for parsing lines where they lie), the blocking embedded-io `Read`/`Write`, and `WriteReady`; every UART kind and half has `baudrate`/`set_baudrate` (BAUD is enable-protected, so the SERCOM is disabled for the write and a byte on the wire then is lost). `RingBufferedUartRx` has no `BufRead`: its DMA writes the ring under any slice handed out.
  - `wdt`: the watchdog (normal mode, early warning) and the reset cause
    (RSTC.RCAUSE).
  - `uid`: the chip's unique ID.
  - `time` (feature `time-driver`): an `embassy-time` driver on TCC0, one of
    the 32-bit TCCs, extended to 64 bits; `init` starts it from the
    generator `Config.time` names (4 by default), and TCC0 leaves the
    `Peripherals`. A `time-driver-tccN` feature puts it on another TCC
    instead, as embassy-stm32's `time-driver-timN` do: TCC1-TCC3 are
    32-bit too, TCC4-TCC7 16-bit (an interrupt every 32.8 ms at 1 MHz),
    which leaves the 32-bit TCCs for PWM. TCC0 and TCC6 pass `timer` on
    the GC01.
  - `usb` (feature `usb`): an `embassy-usb-driver` for the full-speed USB
    device controller (the SAM D5x/E5x USB IP), on its pins (PD04, PD03),
    clocked from the 48 MHz generator it's given.
  - `usbhs` (feature `usbhs`): an `embassy-usb-driver` for the high-speed
    USB controller (USBHS: a Mentor MUSB core with Microchip's wrapper and
    an on-chip PHY), `usbhs::Driver::new(p.USBHS, Irqs, usbhs::Config::default())`,
    on its dedicated pins (the SG01 Curiosity Ultra's J202, USB-C). Device
    mode at 480 Mbit/s, or full speed (`Config.high_speed`): control, bulk
    up to 512 bytes (double-buffered) and interrupt up to 1,024, through
    the core's own FIFO RAM by CPU copy, or by its DMA (`Config.dma`, off
    by default: no faster per packet, but it frees the CPU). Its PHY's PLL
    takes the XOSC at 12 or 24 MHz, which `clocks` passes on
    (XOSCCTRLA.USBHSDIV); the driver turns on the USB regulator, loads the
    PHY's trim from CAL OTP, and turns on its VBUS comparators (off out of
    reset, PHY24 bit 1: without them the core never attaches). VBUS events
    (watched every 32 ms while unpowered or suspended) and remote wakeup
    use embassy-time, so the application needs a time driver.
    `usbhs::is_high_speed()` says what the bus chose. Benched against
    macOS and Linux: CDC-ACM, CDC-NCM with embassy-net, full speed, cable
    pulled and replugged, suspend, resume and remote wakeup. Known issue:
    on a marginal link a corrupted packet can leave the core deaf until
    the board is reset (see the module docs). No isochronous.
- `pac/pic32ck-sg01-pac` and `pac/pic32ck-gc01-pac`: the PIC32CK2051SG01144's
  and GC01144's peripheral access crates, generated by svd2rust 0.37 from
  Microchip's PIC32CK-SG_DFP 1.11.288 and PIC32CK-GC_DFP 1.5.239 SVDs
  (Apache-2.0; each one's `generate.sh` regenerates it, with `cargo install
  svd2rust@0.37.1 form`). The GC's lacks the Secure PORT and EIC views and
  the HSM.
- `pac/pic32cm-pl10-pac`: the PIC32CM6408PL10048's PAC, from the
  PIC32CM-PL_DFP 1.6.499 SVD (svd2rust 0.37.1 and form, as long_sentences
  generated it).
- PIC32CM PL10 (Cortex-M0+, feature `pic32cm6408pl10048`, target
  `thumbv6m-none-eabi`): `clocks` (OSCHF 1-24 MHz, four generators),
  `gpio` (a 1-bit slew limit; no open drain in PORT, so `OutputOpenDrain`
  switches the pin's direction, with an optional pull-up), `uart` blocking and
  `BufferedUart` (SERCOM v1: no FIFO, 16-bit DATA, one vector per SERCOM,
  bus clocks turned on by the driver), `uart`'s DMA and the ring receiver,
  `dma` on its DMAC (the SG00's IP and driver, with two channels and one
  vector), `spi` and `i2c` (the CK's drivers less CTRLC, with SERCOM v1's
  narrower DATA, on generator 0 by default), `flash` (a driver of its own
  for its NVMCTRL, behind the FCW driver's API: 512-byte pages, 32-bit
  word writes, blocking and async on READY), `wdt` (the CK's driver: its
  WDT moves SYNCBUSY.CLEAR, and its RSTC has its own reset causes),
  `uid` (the signature row's 128-bit serial number, not a UUID), `rtc`
  (the CK's driver over its own OSC32KCTRL layout; OSC32K for the internal
  source; a system reset request clears its RTC, unlike the CK's and the
  SG00's), `exti` (the CK's driver with one vector for all 16 lines and no
  PINSTATE; NMI on PA01, PA07 or PA30; the internal pull doesn't hold
  while the EIC has a pin, so it needs an external pull or a driver),
  `adc` (a driver of its own for its ADC, IP 06272, behind the CK's ADC
  API: VDD, an external or four internal references; VDDIO/10 and the
  temperature sensor with its factory calibration inside the chip; results
  on the 12-bit scale at 8, 10 and 12 bits), and the time driver on TC0 and TC1 paired as 32 bits (its TCC0 is 16-bit), or with
  `time-driver-tc0`, `-tc1` or `-tc2` on one TC as 16 bits (`blink` on
  TC0+TC1 and on TC0 alone: ticks 1,002-1,003 ms apart for a minute). Atomics go through
  `portable-atomic`. The other modules are CK only for now.
- `examples/pic32cm-pl10-curiosity-nano`: board support for the PL10
  Curiosity Nano (EV10P22A; VCOM on SERCOM1 PB00/PB01, LED0 PB02), examples
  `blink` (24 MHz, LED, a line a second with the time), `uart_echo`
  (`BufferedUart`), and `dma`, `uart_dma`, `uart_ring`, `spi` (on the
  board's SPI pins, SERCOM0 PA04/PA05/PA07, in loopback) and `i2c` (on its
  I2C pins, SERCOM0 PA00/PA01, with a sensor wired), `flash`,
  `flash_async`, `watchdog`, `uid`, `rtc`, `exti` (with PA08 jumpered
  to PA09 and PA10 to PA11), `adc`, `adc_pin` (PA03's readings, as
  `name=value` lines, for a voltage source wired to it) and `uart_flow`
  (RTS/CTS on SERCOM0, PA04-PA07, against a Curiosity Ultra's
  `uart_peer`), `nmi` (PA00 jumpered to PA01, the NMI pin) and
  `open_drain` (on the PA10-PA11 jumper), the SG00 board's sized for 8 KB of SRAM. Build with `cargo build --release -p
  pic32cm-pl10-curiosity-nano --target thumbv6m-none-eabi --example blink`;
  flash with pyOCD and the PIC32CM-PL_DFP pack at 100 kHz (probe-rs can't
  reset this chip). All pass on the board: 1002 ms a tick, 8000 bytes
  echoed intact at 115,200 baud; `dma`'s copies (1.5 KiB of words at
  11 MB/s, both channels at once, a copy dropped part way, a UART line
  paced by SERCOM1), 100 blocks echoed by DMA, and 200,000 bytes through a
  1 KiB ring at 115,200 and at 460,800 (paced), with an `Overrun` when the
  reader falls a ring behind; `spi` at 1 to 12 MHz (the most from 24 MHz),
  the four modes and LSB first, blocking and by DMA (1 KiB in 1.28 ms at
  12 MHz); and `i2c` against an `SHT40` and a BMP280 on one module (its
  4.7 kΩ pull-ups): the scan, a NACK, reads whole and in parts, async at
  100 kHz, 400 kHz and 1 MHz, and both sensors by their public drivers;
  `flash` and `flash_async` on scratch pages at `0xF800` and `0xC000`
  (erase 10 ms a page, about 116 µs a word, the CPU stalled meanwhile,
  misaligned and out-of-range requests refused); and `watchdog`: start,
  stop and refusals, fed at 100 ms (no warning) and 750 ms (a warning
  each period), then starved (a warning at 506 ms, a reset at about 1 s,
  RCAUSE watchdog), then a system reset request (RCAUSE system); `uid`;
  `rtc`: the crystal refused as `NoCrystal` in 3.3 s (the board's crystal
  isn't connected until two straps are cut and two pads bridged), then on
  OSC32K 1.5 % off OSCHF over 10 s, the calendar through 29 February and a
  new year, an alarm, and the RTC found cleared after a system reset; and
  `exti`: on each jumper, rising, falling, either edge, high and low each
  woken about 100 µs after the output moved, a level already met at once,
  and 100 alternate edges, none missed; and on the one vector, two lines
  rising together both woken, and one line's edge leaving the other's
  wait alone; and `adc`: VDD from VDDIO/10 against the 1.024, 2.048 and
  2.5 V references, 3.34 to 3.38 V (the debugger reports 3.31 V), the
  three resolutions agreeing, async reads matching, and the temperature
  sensor reading 33.5 °C with an SHT40 beside the board at 24.7 °C (about
  9 °C high: see the driver's known issue); and `adc_pin` with a 12-bit
  DAC on PA03 stepped from 0.2 V to 3.1 V: linear to within 2.6 counts
  (RMS 1.1) against VDD and against the 2.048 V reference; and
  `uart_flow` at 1 Mbaud against the SG01's `uart_peer`: CTS holding a
  6 KiB burst (seen 54 times, every byte back) and RTS holding the echo
  through 200 ms unread (no overflow), on the DMA `Uart` and on a
  `BufferedUart` with 256-byte rings, each control without flow control
  losing bytes; and `nmi` on PA01: high and low levels firing once at
  once (the handler disarms the level) and not for the other level,
  rising, falling and either edge firing once about 25 µs after PA00
  moved and not before, the wrong edge and a disarmed pin not firing;
  `open_drain` on PA10-PA11: low read low at both ends and, let go,
  high through PA11's pull-up; the line, taken low then left floating,
  staying low while PA10 let go (it never drives high); both ends open
  drain high only while both let go; `toggle`, and the reset state on
  drop; 585 kHz square waves through the PORT's pull-up at 24 MHz.
- `pac/pic32cm-sg00-pac`: the PIC32CM5112SG00100's PAC, from the
  PIC32CM-SG00_DFP 1.2.83 SVD (svd2rust 0.37.1 and form, with the CK's
  `fix_svd.py`).
- PIC32CM SG00 (Cortex-M23, feature `pic32cm5112sg00100`, target
  `thumbv8m.base-none-eabi`): mostly the PIC32CK's IP at other addresses.
  `clocks` is the CK's module (XOSC, PLL0 and twelve generators; its PLL
  outputs are numbered 0 to 4, it needs the additional regulator, and
  takes one flash wait state per 24 MHz to 72 MHz), `gpio`, `uart` (SERCOM
  v6 with its FIFO, DMA and the ring receiver), `spi`, `i2c`, `wdt`,
  `uid`, the time driver on a TCC (all seven are 16-bit),
  `can` (an `M_CAN` 3.3.0 behind a wrapper, so its registers start 0x100 in
  and the message RAM's offset is in the wrapper), and `dma` on its DMAC
  (the SAM D21/C21 kind, `dma_u2223`: 8 channels, descriptors in SRAM, the
  CK's `dma` API; a ring is two half-ring descriptors linked to each
  other), and `flash` on its FCW (04971: NVMOP in CTRLOP, 16-byte flash
  words, one panel, every write pre-programmed; blocking and async), and
  the CK's `rtc`, `exti`, `adc` and `usb` at the SG00's addresses: the RTC's
  alarm on `RTC_CMP0` (its RTC has four lines), the EIC's one view and
  `EIC_EXTINT0`-`15`, the NMI on PD00; the ADC's AIN0-11 and the bandgap on
  input 14, on `ADC_REQ1` (no temperature sensor or factory readings: the
  data sheet gives neither); the full-speed USB on PD03/PD02, GCLK channel
  38, its four vectors under the SG00's names.
- `examples/pic32cm-sg00-curiosity-pro`: board support for the SG00
  Curiosity Pro (EV53V42A; VCOM on SERCOM4 PD19/PD20, LED0 PA20, LED1
  PA18, SW0 PB07, SW1 PB11), examples `blink`, `uart_echo`, `watchdog`
  (with the unique ID), `can_loopback` (internal loopback), `can_listen`
  (bus monitoring) and `can_ping` (against an echo node; with
  `tools/host/can_echo.py` on a Linux host), `dma`,
  `uart_dma` (256-byte blocks echoed by DMA, a block at a time),
  `uart_ring` (the ring receiver, driven from the host), `spi` (in
  loopback on the mikroBUS pins, SERCOM2), `rtc` (the 32.768 kHz crystal
  Y303), `exti` (mikroBUS AN, PA07, and SW0), `adc` (the bandgap against
  AVDD and the board's MCP1501 on VREFH, the potentiometer on PB06),
  `usb_echo` (CDC-ACM on the target Micro-AB; `tools/host/cdc_echo_test.py`
  drives it), `flash` and `flash_async`
  (scratch pages at offsets 0x7_C000 and 0x4_0000), `i2c` (an `SHT4x`
  and a BMP280 or BME280 wired to EXT1 or the mikroBUS socket; the board
  has no I2C device of its own), `gpio` (the board alone), `open_drain`
  and `nmi` (PD01 jumpered to PD00, the NMI pin: EXT1 pins 11 and 12) and
  `uart_flow` (RTS/CTS on SERCOM3, PD09 and PA00-PA02, against a Curiosity
  Ultra's `uart_peer`). Build with `cargo build
  --release -p pic32cm-sg00-curiosity-pro --target thumbv8m.base-none-eabi
  --example blink`; flash with pyOCD and the PIC32CM-SG00_DFP pack, the
  board's debugger in CMSIS-DAP mode. All pass on the board: 1,002-1,003
  ms a tick for a minute, 20,000 bytes echoed intact at 115,200 baud, the
  watchdog's tests and reset, every classic and FD frame through CAN0 at
  500 kbit/s and 2 Mbit/s; `dma`'s copies (16 KiB of words at 53 MB/s,
  all eight channels at once, a copy dropped part way), 400 blocks echoed
  by DMA at 921,600 baud, 200,000 bytes through a 2 KiB ring at 115,200
  and at 921,600 (paced), an `Overrun` when the reader falls a ring
  behind, `spi`'s 22 checks, to 12 MHz by DMA, and `flash`'s and
  `flash_async`'s: page erase 19.4 ms, 1 KiB row 7.9 ms, 16-byte word
  129 µs, reads right after an erase or write (the FCR's prefetch cache
  needs no invalidating), the CPU stalled through each as PFM is one
  panel; and `i2c`'s 12, against an SHT40 and a BMP280: scan, NACK, reads
  blocking and on interrupts at 100 kHz, 400 kHz and 1 MHz, and both
  sensors' readings. PD01 and PD00 work together as SERCOM1's pads 0
  and 1 (the chip file's set 3); which other pins share their set is still
  the open question below. Then `rtc`: −13 ppm against the 12 MHz
  crystal, the calendar through 29 February and a new year, the alarm in
  2.97 s, the time kept through a reset; `exti`: every edge and level
  waited for, woken 23 µs after the flip, 100 back-to-back edges with none
  missed; `adc`: the bandgap at 1,490 counts (AVDD 3,298 mV), 1,194 mV
  against the MCP1501, 10 and 8 bits agreeing, async reads matching;
  `usb_echo`: every length to 10,000 bytes echoed, 527 kB/s both ways; and
  CAN0 on a real bus at 250 kbit/s: `can_listen` heard 10,000 frames at
  1,000/s error-active with no errors, and `can_ping` against a CoreMP135
  running `can_echo.py` passed 10 rounds (440 frames each way: 11- and
  29-bit IDs, data and remote, 0-8 bytes, eight at once), error-active at
  0/0; and CAN1 as FD at 500 kbit/s and 2 Mbit/s: `can_ping` built with
  `CAN_FD=1` passed 10 rounds in 957 ms against the CoreMP135 (1,080 frames
  each way, FD ones of every length to 64 bytes with and without bit-rate
  switching, eight 64-byte ones at once), error-active at 0/0. Then
  `gpio`: the pulls on the empty mikroBUS socket's AN and RST both ways,
  both buttons released, LED1 reading back what it drives, the reset
  state on drop; `open_drain` (the chip's own) on PD01-PD00: low and let
  go read at both ends, both ways round, `toggle`, the reset state on
  drop, 2.48 MHz square waves through the board's I2C pull-ups at 72 MHz
  (those pull-ups keep it from showing that a pin let go never drives
  high); and `uart_flow` against the SG01's `uart_peer` at 1 Mbaud: 8 KiB
  back intact with CTS seen holding 178 times, 2 KiB back after 200 ms
  unread with no overflow, the same through a `BufferedUart`'s 256-byte
  rings, and the controls without flow control losing bytes. `nmi` does
  not pass: see the to-do list.
- `pac/pic32cm-gv00-pac`: the PIC32CM3204GV00064's PAC, from the
  PIC32CM-GV_DFP 1.3.47 SVD (svd2rust 0.37.1 and form). The SVD has no
  PTC, so neither has the PAC.
- PIC32CM GV00 (Cortex-M0+, feature `pic32cm3204gv00064`, target
  `thumbv6m-none-eabi`): a generation older than the others, the SAM D20's
  IP, and no DMA controller. `clocks` of its own (SYSCTRL, PM and the
  older GCLK, written through ID fields): OSC8M and its prescaler, XOSC32K
  and OSC32K, the DFLL48M open loop from its factory calibration or closed
  loop locked to a generator, eight generators, the flash wait states for
  the CPU's rate; `gpio` (`port_v0`: `port_v1` with drive strength in place
  of slew control; open drain by direction, as on the PL10); `uart` blocking and `BufferedUart` (`sercom_v0`: the
  registers at other offsets, synchronisation in STATUS.SYNCBUSY, no ERROR
  interrupt, no FIFO, no RTS/CTS); `spi` blocking, and async with the
  SERCOM's interrupt moving each byte in place of DMA (SCK rated to 8 MHz)
  and `i2c` blocking and on interrupts (Standard and Fast mode:
  no Fast-mode Plus, so 400 kHz at most); `flash` (a driver of its own for
  its NVMCTRL, behind the others' API: 256-byte rows erased, 32-bit words
  written through the 64-byte page buffer and a Write Page command,
  blocking and async on READY; `init` sets CTRLB.MANW, which the data
  sheet says resets to 1 but reads 0); `wdt` (the others' driver: its
  synchronisation in STATUS, its clock generator 2 at 1.024 kHz,
  OSCULP32K / 32, which the clocks' configurations set with DIVSEL; the
  reset causes in the PM); `rtc` (the others' driver at the SAM D20's
  offsets, a CLOCK read asked for first, its clock a generator it sets to
  the crystal or OSCULP32K / 32; only a power-on reset resets it, so the
  time is kept through other resets); `exti` (the others' driver with one
  vector, no ASYNCH or PINSTATE, its edges sampled by `GCLK_EIC` from
  generator 0; the button read high with the PORT's pull-up while the EIC
  had it, which suggests the pull holds, unlike the PL10's) and the NMI on
  PA08; `adc` (a driver of its own for its ADC, IP U2204, behind the
  others' API: a gain stage, AVDD full scale by default (half AVDD at half
  gain), the internal 1.0 V, AVDD / 1.48 or the VREFA/VREFB pins; AIN0-19,
  the bandgap, VDDIO/4, the temperature sensor and the DAC's output inside
  the chip; the factory linearity and bias calibration loaded; its clock
  `GCLK_ADC` through a prescaler kept to 2.1 MHz); `dac` (its 10-bit DAC,
  which no other chip here has: VOUT on PA02 and the ADC through its output
  buffer, or the comparators alone; AVDD, 1.0 V or VREFA); `uid` (its
  128-bit serial number); and the time driver on TC0 and TC1 paired as 32 bits, or with
  `time-driver-tc0`, `-tc1` or `-tc2` on one TC as 16 bits (the SAM D20's
  TC: a read of the count is asked for first). The other modules aren't
  ported yet (see the to-do list).
- `examples/pic32cm-gv-curiosity-nano`: board support for the PIC32CM
  GV-Value Line Curiosity Nano+ Touch (EV80D90A; VCOM on SERCOM2
  PA12/PA13, the user LED PA24 lit high, the button PB17, the 32.768 kHz
  crystal on PA00/PA01), examples `blink` (the CPU at 8 MHz, or with
  `GV_DFLL=1` at build time at 47.97 MHz from the DFLL48M locked to the
  crystal; the LED, a line a second with the time, and the serial number),
  `uart_echo` (`BufferedUart`), `spi` (on the board's SPI pins, SERCOM5
  PB02/PB03/PB00, in loopback) and `i2c` (on its I2C pins, SERCOM3
  PA22/PA23, with a sensor wired; the PL10's `bme280` driver test doesn't
  fit in 32 KB, so the Bosch part is read raw), `flash` and `flash_async`
  (scratch rows at offsets 0x7C00 and 0x6000), `watchdog`, `rtc` (from
  the crystal), `exti` (PA10 jumpered to PA11 and PA18 to PA19, and the
  button), `nmi` (PA09 jumpered to PA08, the NMI pin), `open_drain` (on
  the PA10-PA11 jumper), `adc` (the inputs
  inside the chip) and `dac` (read back by the ADC, inside the chip and
  on PA02). Build with `cargo build --release -p
  pic32cm-gv-curiosity-nano --target thumbv6m-none-eabi --example blink`;
  flash with pyOCD and the PIC32CM-GV_DFP pack (the PL10 board crate's
  examples have the same names and target, so each build replaces the
  other's ELF). All pass on the board: 1,002-1,003 ms a tick (OSC8M about
  0.5 % fast against the host's clock), the DFLL48M locked coarse and fine with one
  flash wait state; 8,000 bytes echoed intact at 115,200 baud in one
  write and in 64-byte writes, and 20,000 in 256-byte writes; `spi` at 1
  to 8 MHz (a faster request held to 7.995 MHz) from 47.97 MHz, the four
  modes and LSB first, unequal lengths and in place, 256 bytes in 707 µs
  at 8 MHz, and async on the interrupt: the same transfers, 256 bytes in
  1.61 ms at 8 MHz and 3.41 ms at 1 MHz (about 6 µs of interrupt a byte
  at 48 MHz, so the CPU is mostly in it at 8 MHz: a ticker task ran 12
  times in that 1.61 ms), a transfer dropped part way stopped with the
  bus working after; and `i2c` against an SHT40 and a BMP280 on one module (its
  4.7 kΩ pull-ups): the scan, a NACK, reads whole and in parts, on
  interrupts at 100 and 400 kHz, 1 MHz refused, and the SHT40 by the
  `sht4x` driver; `flash` and `flash_async` (row erase 1.17 ms, a 64-byte
  page 360 µs, 256 bytes 1.34 ms, the CPU stalled through each as flash is
  one array; partial pages, a second write into a page, runs across a row
  boundary, misaligned and out-of-range requests refused, an erase
  dropped part way); and `watchdog`: start, stop and refusals, fed at
  100 ms (no warning) and 750 ms (a warning each period), then starved (a
  warning at 501 ms, a reset at about 1 s, RCAUSE watchdog), then a system
  reset request (RCAUSE system); `rtc`: from the crystal, −3,633 ppm
  against OSC8M over 10 s (OSC8M's error), the calendar through 29
  February and a new year, an alarm in 2.88 s, the time kept through a
  system reset, then OSCULP32K set up afresh (0.9 % slow); `exti`: on each
  jumper, rising, falling, either edge, high and low each woken about
  203 µs after the output moved (at 8 MHz), a level already met at once,
  100 alternate edges, none missed, and two lines on the one vector, both
  woken together and one left waiting; and `nmi`: levels and edges each
  firing once about 34 µs after PA09 moved, and not for the wrong edge or
  disarmed; `adc`: AVDD 3,322 mV from the bandgap (the debugger reports
  3.32 V), VDD 3,340 mV from VDDIO/4 against 1.0 V, the three resolutions
  agreeing, async reads matching, 14 µs a read (30 µs with the long sample)
  at a 2 MHz `CLK_ADC`, and the temperature sensor at 36.6 °C by the data
  sheet's typical figures beside an SHT40's 25 °C (about 11 °C high: see
  the to-do list); and `dac` against the ADC at AVDD: 4.000 counts a code
  inside the chip and 4.001 on PA02 (4 ideal), offset −1, worst deviation
  2 counts over codes 32-992, a full-scale step within 24 counts at once
  and settled 10 µs later; against 1.0 V on both, 4.03 a code; and
  `open_drain` on PA10-PA11: as on the PL10, with PA11's pull-down
  holding the line low while PA10 let go; 181 kHz square waves through
  the pull-up at 8 MHz.
- `examples/pic32ck-curiosity-ultra`: board support for the PIC32CK SG01 and GC01
  Curiosity Ultra boards, with pic32c-hal's bench programs as its examples
  (in its `examples/`). The library is the board support alone (boot, `init` with the boards'
  clocks and the cache on (`config` and `init_with` to change them), a UART log on the debugger's virtual COM port, cycle-counter
  time), on pic32c-hal, cortex-m, cortex-m-rt and critical-section only;
  what several examples share (CAN0 on the board's transceiver, mDNS,
  SNTP, TLS) is in `examples/common/`, and the rest are dev-dependencies.
  Build one with
  `cargo build --release -p pic32ck-curiosity-ultra --target
  thumbv8m.main-none-eabihf --features sg01 --example blink` (or `gc01`;
  `eth_https` and `crypto_bench` also need `--features tls`); the ELF is
  in `target/thumbv8m.main-none-eabihf/release/examples/`. Programs: `blink`,
  `clocks`, `timer` (build with `TIMER_START=7FB3B4C0` or `FFB3B4C0` to cross
  2^31 or 2^32 within seconds), `flash` (scratch pages at the top of each
  panel; `FLASH_NO_CACHE=1` leaves the cache off), `dma` (copies of every word size and awkward alignment, from SRAM and flash, against the CPU's copy; the executor running meanwhile; a transfer dropped part way; both controllers at once; a line written to the VCOM by DMA), `flash_async` (the async
  driver on the same pages, with a ticker task showing the executor runs
  through a panel-2 erase), `watchdog`, `gpio`
  (pulls, outputs, open drain, the buttons), `exti` (edges and levels on
  PA10 made by flipping its pull, SW0's levels, a press), `nmi` (SW1),
  `eth_ping` (embassy-net on the RJ45: DHCP, SLAAC, DNS, ping), `i2c` (the target I2C bus, SERCOM4 on PB09/PB10: a scan, the AT24MAC402's EUI-48 and serial number, a page of its EEPROM written and restored, two reads in a row, then on interrupts at 100 kHz, 400 kHz and 1 MHz; then each sensor it finds that the bench knows, by public drivers through `I2c`'s embedded-hal traits (`sht4x`, async; `bme280`, blocking), an SHT4x, BMP280 or BME280 such as a MikroElektronika TempHum 15 or Weather Click in the mikroBUS socket: its serial number or chip ID and a measurement within reason), `spi` (SERCOM2 on the empty mikroBUS socket's pins, in loopback: rates to 24 MHz, the four modes, LSB first, unequal lengths and in place, blocking and by DMA, then full duplex with MISO on PC17), `trng` (the TRNG's rate and sanity statistics over 2 Mbit), `eth_https` (feature `tls`: `pic32ck-<MAC's last 3 bytes>.local` by mDNS with `_https._tcp`, the board crate's `mdns`; its clock from SNTP, the board crate's `sntp`: the DHCP gateway, then `pool.ntp.org`, clockless if neither answers; `eth_http`'s routes over TLS 1.3 with rustls and RustCrypto, a bench-only self-signed certificate in `certs/`; `tools/host/eth_https_test.py` drives it; feature `tls` builds C and assembly, so `arm-none-eabi-gcc` must be on the PATH), `crypto_bench` (feature `tls`: each handshake operation timed, RustCrypto against the assembly crates), `eth_http` (a TCP echo on port 7 and
  picoserve on port 80; `tools/host/eth_http_test.py` drives it), `adc` (the bandgap's spread and AVDD from it, the temperature sensor reported, 10 and 8 bits on the 12-bit scale, read times, the Arduino header's A0-A5 shown as they float, async reads; `ADC_JUMPERS=1` with A1/PA09 jumpered to 3V3 and A2/PA10 to GND checks 4095 and 0), `moff` (the die temperature sensor's offset, erratum 2.18.1's Moff, against the air's temperature from an SHT4x, which it needs: a MikroElektronika TempHum 15 Click in the mikroBUS socket; about 228 counts on the GC01, steady), `w5500` (a WIZnet W5500 on a MikroElektronika ETH WIZ Click in the mikroBUS socket, CS PD15, RST PD21, INT PD18: a hardware reset, VERSIONR, a software reset, registers, the PHY's link, and 2 KiB buffer round trips at 1-20 MHz and mode 3 with SCK slew-limited; the click shares PC15-PC17 with EXT2's UART, so unplug the UART link meanwhile), `rtc` (the crystal started or found running, its rate against the CPU's crystal over 10 s, the calendar through 29 February and a new year, an alarm 3 s ahead, and the time kept through a software reset), `uart_flow` (RTS/CTS against `uart_peer`'s slow-draining echo on a second board's EXT2: CTS holding an 8 KiB burst and RTS holding the echo through 200 ms unread, on the DMA `Uart` and on a `BufferedUart` with 256-byte rings, each with a control run without flow control that loses bytes), `uart_link` (SERCOM2 on EXT2: TX PC15 pin 16, RX PC16 pin 18, RTS PC17 pin 17, CTS PC18 pin 6, flow control on; against `uart_peer` on another board, or another board running `uart_link` too: 1 KiB blocks at 1 Mbaud, 20 on a `BufferedUart` then 20 on a `Uart` with DMA; `UART_LINK_BAUD`, and `UART_LINK_FIFO=0` with the FIFO off), `uart_lines` (300 sentences of 20-82 characters read back by `fill_buf`/`consume` through a 128-byte ring, `available`, `set_baudrate` to 115200 and back against `uart_peer`'s 1 Mbaud echo, the blocking embedded-io traits), `uart_ring` (a 512-byte `RingBufferedUartRx` against `uart_peer`'s echo: 16 KiB in order through some 32 wraps, the wake on one byte, and an overrun reported then recovered; `UART_RING_FIFO=0` with the FIFO off), `uart_peer` (the far end for `uart_flow`, `uart_ring` and `uart_lines` on a second CK board's EXT2, crossed: a 1 Mbaud echo with RTS/CTS that takes up to 512 bytes a round, gathered for 20 ms, then pauses 10 ms, about 50 KB/s, so its 2 KiB receive ring fills and its RTS holds the far end; `UART_PEER_BAUD` sets the rate), `uart_echo` (async echo on the
  VCOM; `UART_BAUD` sets the rate and `UART_FIFO=0` turns the FIFO off,
  and `tools/host/uart_echo_test.py` on the host drives it) and `usb_echo` (CDC-ACM on J203), `usbhs_echo` (CDC-ACM in 512-byte packets on J202 at high speed, a zero-length packet after a full one when nothing follows, the bus's events logged, remote wakeup after 5 s suspended; `USBHS_FULL_SPEED=1` for full speed and 64-byte packets, `USBHS_DMA=1` for the DMA; `tools/host/cdc_hs_test.py` drives it: every length from 1 to 1,100 bytes, then 64 MiB streamed and checked at 13.8-14.3 MB/s each way on the SG01 against macOS, 15.9 against Linux (13.6 with the DMA; 0.53 at full speed), against `usb_echo`'s ~550 kB/s both ways on the full-speed controller), `usbhs_ncm` (CDC-NCM in 512-byte packets under embassy-net, static link-local addresses, TCP echo/discard/chargen on ports 7/9/19; `tools/host/tcp_bench.py` drives it: on the GC01, 3.6 MB/s echo each way, 5.3 MB/s to the board, 6.4 MB/s from it, against 3.1, 4.8 and 5.6 on embassy-net 0.9's smoltcp, whose 8 KiB windows were the limit), and for CAN0 `can_loopback` (internal and external
loopback, classic and FD, filters; `LOOPBACK_CLASSIC=1` for a classic
channel), and `can_ping` against `can_echo` on another node (250 kbit/s, or FD
at 500 kbit/s and `CAN_FD_DATA`, 2 Mbit/s by default, when both are built
with `CAN_FD=1`). They report over the PKoB4's virtual COM port. `soak`
runs USB full and high speed, Ethernet and CAN0 at once, each an echo as in
those programs, for as long as `tools/host/soak.py` (and a CAN node sending
test frames) keeps them busy, with a line of counters every 10 s.

- `docs/bench.md`: the boards and wiring that run every example.
- `tools/host/`: the host-side scripts that drive the bench programs (see
  its README).
- `docs/can-plan.md`: the plan the M_CAN driver was built to, kept as
  history.

# Building against local checkouts

`can-common` and `can-hal` are git dependencies, which `Cargo.lock` pins
to commits. For changes to them not yet pushed, check them out beside this
repository and patch them in `.cargo/local.toml`, which `.cargo/config.toml`
includes if it exists and git ignores:

```toml
[patch."https://github.com/aeolionics-rs/can-common"]
can-common = { path = "../can-common" }

[patch."https://github.com/aeolionics-rs/can-hal-rs"]
can-hal-rs = { path = "../can-hal-rs/can-hal" }
```

# To do

PIC32CK:

- PWM and timers (TCC), on neither family yet.
- The event system (EVSYS), for timer-triggered ADC conversions and the
  like.
- SQI (quad SPI), against the board's QSPI flash; erratum 2.9.1 allows
  single-wire writes only.
- ADC: `Reference::External` taking the PB01 pin, Embassy-style; scanning
  several channels by DMA; timer triggers; the temperature sensor's offset
  (erratum 2.18.1, "Moff"), documented as a known issue until Microchip
  explains it.
- SPI: client mode, the FIFO, hardware chip select. `w5500` waits for its
  ETH WIZ Click to be run.
- I²C by DMA.
- DMA channel priorities, and the CRC engine (errata 2.3.1, 2.3.4 and 2.3.7
  restrict it).
- BFM/CFM access: back them up first, and agree before any CFM write.
- USBHS: recovering from a deaf core on a marginal link (module docs):
  perhaps a soft disconnect when FRAME stops counting while attached and
  not suspended (pulsing SOFTRST.NRSTX made it worse); the DMA's
  whole-transfer mode behind `read_transfer` and `write_transfer`, should
  a class use them (embassy-usb's don't); embassy-usb's CDC classes give
  the interrupt endpoint bInterval 255, invalid at high speed (Linux
  takes 11).
- Not started: AC (mind erratum 2.1.2), PTC, SDHC, EBI, I²S, CCL, FREQM,
  PDEC.

PIC32CM PL10:

- The ADC's temperature sensor reads about 9 °C high with the factory
  calibration and the data sheet's equation; the preliminary data sheet
  doesn't settle it. The external reference needs wiring (not yet run).
- `rtc` from the crystal, on a board with the crystal connected (not yet
  run).
- Stack overflow runs silently into `.bss` on the 8 KB part: flip-link or
  stack painting in the board crate.

PIC32CM SG00:

- The NMI on PD00 doesn't see the pin on the bench board (silicon
  revision A.0, DSU.DID `0x0AC08053`): armed for a low level it fires
  whatever PD00 does, for a high level or an edge (synchronous or
  asynchronous) never, as if its input were held low inside the chip.
  Probed by hand over SWD from the boot ROM (2026-11-23), with the same
  result on every PMUX function of PD00, with the EIC enabled on
  `CLK_ULP32K` or on `GCLK_EIC` from generator 0, the NMI's filter on or
  off, with PORT's continuous sampling, and with PD00 and the NMI each
  Secure or Non-secure (PORT.NONSEC, EIC.NONSEC); with every pin but SWD's
  pulled up on function A it still fired for low, so it isn't on another
  pin. PORT reads PD00 right, and PD01's EXTINT1 follows a level driven
  from PD00, so group D reaches the EIC. The data sheet (DS60001921A table
  5-3) and the ATDF both give PD00. Tested with Microchip's Harmony
  TrustZone EIC example too (csp_apps_pic32cm_sg00_gc00, XC32 5.10, DFP
  1.2.83), with a Secure NMI on PD00 added as Harmony's `eic_u2804`
  template generates it and EXTINT9 on PC09 as a control on the same net:
  EXTINT9 saw every edge, the NMI handler never ran, asynchronous or
  synchronous and filtered, configured before or after the EIC enable.
  The errata (DS80001141A) has one NMI entry, 2.3.1, about spurious NMIs
  in synchronous edge mode; its workaround's order was among those tried.
  The repro is branch `sg00-nmi-repro` of
  [aeolionics-contrib/csp_apps_pic32cm_sg00_gc00](https://github.com/aeolionics-contrib/csp_apps_pic32cm_sg00_gc00/tree/sg00-nmi-repro),
  to raise with Microchip. Until it's explained, `Nmi` builds for the SG00
  but doesn't work on this silicon; an EXTINT line is the fallback.
- The ADC's temperature sensor: SUPC has TSEN, but the data sheet lists no
  ADC input for it and the CAL OTP no factory readings (the CK's
  FCCFG69-71), so `adc` offers neither yet.
- cortex-m 0.7 sets NVIC priorities with byte writes (only ARMv6-M gets
  word writes), and the M23 ignored them on the bench: `set_priority`
  does nothing there. Needs a word-write path (or a cortex-m fix) before
  anything relies on priorities.
- `boot!` (BFM stage-0 and the SRAM ECC `__pre_init`) is the same in both
  board crates: a candidate for the HAL.
- PD00/PD01's I/O set: the ATDF gives them none, the data sheet's table
  7-11 says set 1 (with PC15/PC14); the chip file puts them in set 3 (with
  PC12/PC13). Only PD00/PD01 together have run.

PIC32CM GV00:

- No DMA controller, so no `dma` and no DMA modes in the SERCOM drivers.
  The async `spi` moves a byte per interrupt; keeping a second byte in
  the transmit buffer would halve its interrupts, if the receiver's
  two-deep buffer allows it (not yet tried).
- The temperature sensor: the data sheet gives only typical figures
  (0.667 V at 25 °C, 2.4 mV/°C, ±10 °C) and no factory calibration (the
  ATDF names a `TEMP_LOG` area the data sheet doesn't describe); by those
  figures it read about 11 °C high on the bench board.
- The AC, PTC and event system, as on the other chips.
- The time driver on TC3 to TC7.
- SERCOM v0's TX on pad 2 (TXPO 1), which the UART's constructors don't
  offer yet (they take TX on pad 0, as on the other chips).
- The DFLL48M's quick-lock sequence (the data sheet's DFLLCTRL.BPLCKC,
  which the SVD doesn't have), and its factory fine value (bits 73:64 of
  the calibration area; not on dies up to revision C).

Both families:

- Low power: standby, with wake sources.

Waiting:

- Plan for supporting the SG01 HSM host driver.
- Resolution of the temperature sensor's offset issue.
- Upstream reports:
  - `embassy-net-wiznet`'s zero-length frames
  - `embassy-net`'s silent drop
  - `embedded-alloc`'s `used()`.
- More than two HTTPS connection slots for browsers, in `eth_https`.

# Licence

MIT OR Apache-2.0, at your option; the PACs are Microchip's, Apache-2.0.
