pic32c-hal
==========

[![CI](https://github.com/aeolionics-rs/pic32c-hal/actions/workflows/ci.yml/badge.svg)](https://github.com/aeolionics-rs/pic32c-hal/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#licence)
[![Rust 1.99](https://img.shields.io/badge/rust-1.99-orange.svg)](rust-toolchain.toml)
[![no_std](https://img.shields.io/badge/no__std-yes-green.svg)](https://docs.rust-embedded.org/book/intro/no-std.html)

Rust drivers for Microchip's PIC32C Arm microcontrollers, starting with the
PIC32CK SG/GC (Cortex-M33) and PIC32CM-PL (Cortex-M0+).
We don't know of any other Rust HAL that covers the PIC32C parts at this time.

This repo is under active development with dependencies on as yet unreleased
versions of:

* can-hal-rs, rather than embedded-can, for CAN-FD; our fork of it, for
  confirmed transmission, bus recovery and statistics
* can-common, support code shared with CAN drivers in our other HALs

We also expect to move to embassy-net's xarxa-based release ahead of time.
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
  - `eth` (feature `eth`): the Ethernet MAC (Cadence GEM) as an
    `embassy-net-driver`, RMII to the board's KSZ8091, with PHY discovery
    and link polling over MDIO; with embassy-net it gets DHCP and SLAAC
    addresses and answers ping, IPv4 and IPv6.
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
  `gpio` (no open drain; a 1-bit slew limit), `uart` blocking and
  `BufferedUart` (SERCOM v1: no FIFO, 16-bit DATA, one vector per SERCOM,
  bus clocks turned on by the driver; no DMA yet), and the time driver on
  TC0 and TC1 paired as 32 bits (its TCC0 is 16-bit), or with
  `time-driver-tc0`, `-tc1` or `-tc2` on one TC as 16 bits (`blink` on
  TC0+TC1 and on TC0 alone: ticks 1,002-1,003 ms apart for a minute). Atomics go through
  `portable-atomic`. The other modules are CK only for now.
- `examples/pic32cm-pl10-curiosity-nano`: board support for the PL10
  Curiosity Nano (EV10P22A; VCOM on SERCOM1 PB00/PB01, LED0 PB02), examples
  `blink` (24 MHz, LED, a line a second with the time) and `uart_echo`
  (`BufferedUart`). Build with `cargo build --release -p
  pic32cm-pl10-curiosity-nano --target thumbv6m-none-eabi --example blink`;
  flash with pyOCD and the PIC32CM-PL_DFP pack at 100 kHz (probe-rs can't
  reset this chip). Both pass on the board: 1002 ms a tick, 8000 bytes
  echoed intact at 115,200 baud.
- `pac/pic32cm-sg00-pac`: the PIC32CM5112SG00100's PAC, from the
  PIC32CM-SG00_DFP 1.2.83 SVD (svd2rust 0.37.1 and form, with the CK's
  `fix_svd.py`).
- PIC32CM SG00 (Cortex-M23, feature `pic32cm5112sg00100`, target
  `thumbv8m.base-none-eabi`): mostly the PIC32CK's IP at other addresses.
  `clocks` is the CK's module (XOSC, PLL0 and twelve generators; its PLL
  outputs are numbered 0 to 4, it needs the additional regulator, and
  takes one flash wait state per 24 MHz to 72 MHz), `gpio`, `uart` (SERCOM
  v6 with its FIFO; no DMA yet), `i2c` (built, not yet run), `wdt`, `uid`,
  the time driver on a TCC (all seven are 16-bit), and `can` (an `M_CAN`
  3.3.0 behind a wrapper, so its registers start 0x100 in and the message
  RAM's offset is in the wrapper). Not yet: its DMAC (the SAM D5x kind),
  flash (another FCW), and the rest.
- `examples/pic32cm-sg00-curiosity-pro`: board support for the SG00
  Curiosity Pro (EV53V42A; VCOM on SERCOM4 PD19/PD20, LED0 PA20, LED1
  PA18, SW0 PB07, SW1 PB11), examples `blink`, `uart_echo`, `watchdog`
  (with the unique ID) and `can_loopback` (internal loopback only). Build
  with `cargo build --release -p pic32cm-sg00-curiosity-pro --target
  thumbv8m.base-none-eabi --example blink`; flash with pyOCD and the
  PIC32CM-SG00_DFP pack, the board's debugger in CMSIS-DAP mode. All four
  pass on the board: 1,002-1,003 ms a tick for a minute, 20,000 bytes
  echoed intact at 115,200 baud, the watchdog's tests and reset, and every
  classic and FD frame through CAN0 at 500 kbit/s and 2 Mbit/s.
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
  `eth_ping` (embassy-net on the RJ45: DHCP, SLAAC, DNS, ping), `i2c` (the target I2C bus, SERCOM4 on PB09/PB10: a scan, the AT24MAC402's EUI-48 and serial number, a page of its EEPROM written and restored, two reads in a row, then on interrupts at 100 kHz, 400 kHz and 1 MHz; then each sensor it finds that `examples/common/sensors.rs` knows, an SHT4x, BMP280 or BME280 such as a MikroElektronika TempHum 15 or Weather Click in the mikroBUS socket: its serial number or chip ID and a measurement within reason), `spi` (SERCOM2 on the empty mikroBUS socket's pins, in loopback: rates to 24 MHz, the four modes, LSB first, unequal lengths and in place, blocking and by DMA, then full duplex with MISO on PC17), `trng` (the TRNG's rate and sanity statistics over 2 Mbit), `eth_https` (feature `tls`: `pic32ck-<MAC's last 3 bytes>.local` by mDNS with `_https._tcp`, the board crate's `mdns`; its clock from SNTP, the board crate's `sntp`: the DHCP gateway, then `pool.ntp.org`, clockless if neither answers; `eth_http`'s routes over TLS 1.3 with rustls and RustCrypto, a bench-only self-signed certificate in `certs/`; `tools/host/eth_https_test.py` drives it; feature `tls` builds C and assembly, so `arm-none-eabi-gcc` must be on the PATH), `crypto_bench` (feature `tls`: each handshake operation timed, RustCrypto against the assembly crates), `eth_http` (a TCP echo on port 7 and
  picoserve on port 80; `tools/host/eth_http_test.py` drives it), `adc` (the bandgap's spread and AVDD from it, the temperature sensor reported, 10 and 8 bits on the 12-bit scale, read times, the Arduino header's A0-A5 shown as they float, async reads; `ADC_JUMPERS=1` with A1/PA09 jumpered to 3V3 and A2/PA10 to GND checks 4095 and 0), `moff` (the die temperature sensor's offset, erratum 2.18.1's Moff, against the air's temperature from an SHT4x, which it needs: a MikroElektronika TempHum 15 Click in the mikroBUS socket; about 228 counts on the GC01, steady), `w5500` (a WIZnet W5500 on a MikroElektronika ETH WIZ Click in the mikroBUS socket, CS PD15, RST PD21, INT PD18: a hardware reset, VERSIONR, a software reset, registers, the PHY's link, and 2 KiB buffer round trips at 1-20 MHz and mode 3 with SCK slew-limited; the click shares PC15-PC17 with EXT2's UART, so unplug the UART link meanwhile), `rtc` (the crystal started or found running, its rate against the CPU's crystal over 10 s, the calendar through 29 February and a new year, an alarm 3 s ahead, and the time kept through a software reset), `uart_flow` (RTS/CTS against `uart_peer`'s slow-draining echo on a second board's EXT2: CTS holding an 8 KiB burst and RTS holding the echo through 200 ms unread, on the DMA `Uart` and on a `BufferedUart` with 256-byte rings, each with a control run without flow control that loses bytes), `uart_link` (SERCOM2 on EXT2: TX PC15 pin 16, RX PC16 pin 18, RTS PC17 pin 17, CTS PC18 pin 6, flow control on; against `uart_peer` on another board, or another board running `uart_link` too: 1 KiB blocks at 1 Mbaud, 20 on a `BufferedUart` then 20 on a `Uart` with DMA; `UART_LINK_BAUD`, and `UART_LINK_FIFO=0` with the FIFO off), `uart_lines` (300 sentences of 20-82 characters read back by `fill_buf`/`consume` through a 128-byte ring, `available`, `set_baudrate` to 115200 and back against `uart_peer`'s 1 Mbaud echo, the blocking embedded-io traits), `uart_ring` (a 512-byte `RingBufferedUartRx` against `uart_peer`'s echo: 16 KiB in order through some 32 wraps, the wake on one byte, and an overrun reported then recovered; `UART_RING_FIFO=0` with the FIFO off), `uart_peer` (the far end for `uart_flow`, `uart_ring` and `uart_lines` on a second CK board's EXT2, crossed: a 1 Mbaud echo with RTS/CTS that takes up to 512 bytes a round, gathered for 20 ms, then pauses 10 ms, about 50 KB/s, so its 2 KiB receive ring fills and its RTS holds the far end; `UART_PEER_BAUD` sets the rate), `uart_echo` (async echo on the
  VCOM; `UART_BAUD` sets the rate and `UART_FIFO=0` turns the FIFO off,
  and `tools/host/uart_echo_test.py` on the host drives it) and `usb_echo` (CDC-ACM on J203), `usbhs_echo` (CDC-ACM in 512-byte packets on J202 at high speed, a zero-length packet after a full one when nothing follows, the bus's events logged, remote wakeup after 5 s suspended; `USBHS_FULL_SPEED=1` for full speed and 64-byte packets, `USBHS_DMA=1` for the DMA; `tools/host/cdc_hs_test.py` drives it: every length from 1 to 1,100 bytes, then 64 MiB streamed and checked at 13.8-14.3 MB/s each way on the SG01 against macOS, 15.9 against Linux (13.6 with the DMA; 0.53 at full speed), against `usb_echo`'s ~550 kB/s both ways on the full-speed controller), `usbhs_ncm` (CDC-NCM in 512-byte packets under embassy-net, static link-local addresses, TCP echo/discard/chargen on ports 7/9/19; `tools/host/tcp_bench.py` drives it: 3.1 MB/s echo each way, 5.0 MB/s to the board, 5.7 MB/s from it, smoltcp's 8 KiB windows the limit), and for CAN0 `can_loopback` (internal and external
loopback, classic and FD, filters; `LOOPBACK_CLASSIC=1` for a classic
channel), and `can_ping` against `can_echo` on another node (250 kbit/s, or FD
at 500 kbit/s and `CAN_FD_DATA`, 2 Mbit/s by default, when both are built
with `CAN_FD=1`). They report over the PKoB4's virtual COM port.

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

- SPI and I²C on SERCOM v1, with their DMA and FIFO parts gated off as the
  UART's are.
- Watchdog and RTC: the CK's IP families, with small differences.
- External interrupts on its EIC (U2254).
- New drivers: the DMAC (which brings back UART DMA and the ring
  receiver), flash (NVMCTRL), the ADC.
- Unique ID, and a software open-drain output.
- Stack overflow runs silently into `.bss` on the 8 KB part: flip-link or
  stack painting in the board crate.

PIC32CM SG00:

- The DMAC (SAM D5x style, u2223), then UART DMA, the ring receiver and
  SPI.
- Flash on its FCW (a different register map from the CK's).
- CAN on a bus (only internal loopback has run); `exti`, `rtc`, `adc` and
  `usb` (the CK's IP versions) checked on the board.
- cortex-m 0.7 sets NVIC priorities with byte writes (only ARMv6-M gets
  word writes), and the M23 ignored them on the bench: `set_priority`
  does nothing there. Needs a word-write path (or a cortex-m fix) before
  anything relies on priorities.
- `boot!` (BFM stage-0 and the SRAM ECC `__pre_init`) is the same in both
  board crates: a candidate for the HAL.

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
