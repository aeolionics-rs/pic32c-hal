// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The PIC32CK's true random number generator (TRNG, at `0x4502_4000` on the
//! SG01 and the GC01 alike): a free-running entropy source that latches a
//! new 32-bit word into DATA every 84 cycles of its APB clock once enabled.
//!
//! The datasheet promises no more than that: no health tests, no
//! conditioning, no entropy rating. Where FIPS 140-2/140-3 compliance is
//! required, Microchip recommends seeding a DRBG from an external source that
//! passes NIST SP 800-22 instead (DS60001795 section 42).
//!
//! There are two ways to use it, and a program picks one:
//!
//! - **As a driver of its own** (without the `embassy-crypto-rng` feature):
//!   [`Trng::new`] takes the `TRNG` peripheral and a
//!   [`bind_interrupts!`](crate::bind_interrupts) type binding its vector
//!   (`TRNG`) to [`InterruptHandler`]. [`Trng::fill`] waits on the
//!   interrupt; the blocking calls and the `rand_core` traits (0.6, 0.9 and
//!   0.10, for the crates that take each) spin on the ready flag, which is
//!   never more than 84 bus cycles away. The words are the hardware's own,
//!   with no conditioning: use them to seed a cryptographic DRBG, or where a
//!   TLS stack asks for a `CryptoRng` (as embedded-tls and Mbed TLS do, each
//!   conditioning its input).
//! - **As `embassy-crypto`'s random number driver** (feature
//!   `embassy-crypto-rng`): `embassy_crypto::rng_fill_bytes`, and everything
//!   in embassy-crypto that draws keys and nonces, take their bytes from the
//!   TRNG. The `TRNG` singleton leaves `Peripherals`, so the two can't be
//!   mixed. Every word is health-checked ([NIST SP 800-90B] section 4.4: a
//!   start-up test over 1024 words, then the repetition count and adaptive
//!   proportion tests on every word). On a failure the TRNG is restarted and
//!   tested again, once per boot; a second failure panics, so no output is
//!   ever handed out from a source that failed. What the driver hands out is
//!   chosen by the application at build time, through the `PIC32C_HAL_RNG`
//!   environment variable (set it in the application's `.cargo/config.toml`
//!   under `[env]`):
//!   - `drbg` (the default): each call instantiates an `HMAC_DRBG` with SHA-256
//!     ([NIST SP 800-90A] section 10.1.2) from fresh TRNG words and the
//!     chip's unique ID, and returns its output.
//!   - `raw`: the health-checked TRNG words themselves. The build warns.
//!
//!   [`CONSTRUCTION`] says which one a program was built with, and [`stats`]
//!   how many words the TRNG has given. [`Rng`] hands the same bytes to
//!   crates that take a `rand_core` generator (0.6, 0.9 and 0.10).
//!
//! [NIST SP 800-90A]: https://csrc.nist.gov/pubs/sp/800/90/a/r1/final
//! [NIST SP 800-90B]: https://csrc.nist.gov/pubs/sp/800/90/b/final

use core::ptr::{read_volatile, write_volatile};

#[cfg(not(feature = "embassy-crypto-rng"))]
mod direct;
#[cfg(not(feature = "embassy-crypto-rng"))]
pub use direct::{InterruptHandler, Trng};

#[cfg(feature = "embassy-crypto-rng")]
mod driver;
#[cfg(feature = "embassy-crypto-rng")]
pub use driver::{CONSTRUCTION, Rng, Stats, stats};

#[cfg(all(feature = "embassy-crypto-rng", any(test, rng_construction = "drbg")))]
mod drbg;
#[cfg(any(test, feature = "embassy-crypto-rng"))]
mod health;

const TRNG: usize = 0x4502_4000;
const CTRLA: usize = TRNG;
const INTENCLR: usize = TRNG + 0x08;
#[cfg(not(feature = "embassy-crypto-rng"))]
const INTENSET: usize = TRNG + 0x09;
const INTFLAG: usize = TRNG + 0x0A;
const DATA: usize = TRNG + 0x20;

const ENABLE: u8 = 1 << 1;
const DATARDY: u8 = 1 << 0;

fn read8(address: usize) -> u8 {
    // SAFETY: a TRNG register; the driver owns the peripheral.
    unsafe { read_volatile(address as *const u8) }
}

fn write8(address: usize, value: u8) {
    // SAFETY: as above.
    unsafe { write_volatile(address as *mut u8, value) }
}

fn read32(address: usize) -> u32 {
    // SAFETY: as above.
    unsafe { read_volatile(address as *const u32) }
}
