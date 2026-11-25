// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The TRNG as `embassy-crypto`'s random number driver (feature
//! `embassy-crypto-rng`; see the module above for what it hands out).
//!
//! The TRNG starts on first use. Each word is read and checked inside a
//! critical section of its own (at most 84 bus cycles of spinning), so
//! callers at any priority can share it; the DRBG's hashing runs outside, on
//! the caller's stack. The start-up test, and a restart after a failure, run
//! inside one critical section: 1024 words, about 86 000 bus cycles, once.

use core::cell::RefCell;

use critical_section::Mutex;

use super::health::{Health, STARTUP_WORDS};
use super::{CTRLA, DATA, DATARDY, ENABLE, INTENCLR, INTFLAG, read8, read32, write8};

/// What the driver hands out, as the application chose at build time through
/// `PIC32C_HAL_RNG`: `"drbg"` or `"raw"`.
#[cfg(rng_construction = "drbg")]
pub const CONSTRUCTION: &str = "drbg";
/// What the driver hands out, as the application chose at build time through
/// `PIC32C_HAL_RNG`: `"drbg"` or `"raw"`.
#[cfg(rng_construction = "raw")]
pub const CONSTRUCTION: &str = "raw";

/// What the TRNG has done since boot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Stats {
    /// Words read, start-up tests included.
    pub words: u64,
    /// Restarts after a failed health test (at most one: a second failure
    /// panics).
    pub restarts: u32,
}

/// What the TRNG has done since boot.
#[must_use]
pub fn stats() -> Stats {
    critical_section::with(|cs| SOURCE.borrow_ref(cs).stats)
}

struct Source {
    started: bool,
    health: Health,
    stats: Stats,
}

static SOURCE: Mutex<RefCell<Source>> = Mutex::new(RefCell::new(Source {
    started: false,
    health: Health::new(),
    stats: Stats { words: 0, restarts: 0 },
}));

impl Source {
    /// The next health-checked word, or `None` if a test failed and the TRNG
    /// was restarted (and passed its start-up test again), so the caller's
    /// words so far must be thrown away.
    fn next(&mut self) -> Option<u32> {
        if !self.started {
            self.start();
        }
        let word = self.read();
        match self.health.check(word) {
            Ok(()) => Some(word),
            Err(fault) => {
                assert!(self.stats.restarts == 0, "pic32c-hal: TRNG health check failed again ({fault}) after a restart");
                self.stats.restarts += 1;
                write8(CTRLA, 0);
                self.start();
                None
            }
        }
    }

    /// Enables the TRNG and runs the start-up test.
    fn start(&mut self) {
        // Its APB clock is on from reset (MCLK's CLKMSK leaves every
        // peripheral clocked); no interrupt: words are polled.
        write8(INTENCLR, DATARDY);
        write8(CTRLA, ENABLE);
        self.health = Health::new();
        for _ in 0..STARTUP_WORDS {
            let word = self.read();
            if let Err(fault) = self.health.check(word) {
                panic!("pic32c-hal: TRNG start-up health test failed ({fault})");
            }
        }
        self.started = true;
    }

    fn read(&mut self) -> u32 {
        while read8(INTFLAG) & DATARDY == 0 {}
        self.stats.words += 1;
        read32(DATA)
    }
}

/// Fills `words` with health-checked TRNG words; after a failure and a
/// successful restart, from the start again.
fn draw(words: &mut [u32]) {
    'again: loop {
        for slot in words.iter_mut() {
            match critical_section::with(|cs| SOURCE.borrow_ref_mut(cs).next()) {
                Some(word) => *slot = word,
                None => continue 'again,
            }
        }
        return;
    }
}

struct Driver;

impl embassy_crypto::driver::Rng for Driver {
    fn fill_bytes(buf: &mut [u8]) {
        #[cfg(rng_construction = "raw")]
        for chunk in buf.chunks_mut(64) {
            let mut words = [0_u32; 16];
            let words = &mut words[..chunk.len().div_ceil(4)];
            draw(words);
            for (bytes, word) in chunk.chunks_mut(4).zip(words.iter()) {
                bytes.copy_from_slice(&word.to_le_bytes()[..bytes.len()]);
            }
        }

        #[cfg(rng_construction = "drbg")]
        for request in buf.chunks_mut(super::drbg::MAX_REQUEST) {
            drbg_request(request);
        }
    }
}

embassy_crypto::rng_impl!(Driver);

/// The same bytes as `embassy_crypto::rng_fill_bytes`, for crates that take a
/// `rand_core` generator (0.6, 0.9 or 0.10): a handle with no state, so make
/// one wherever it's needed.
#[derive(Clone, Copy, Debug, Default)]
pub struct Rng;

impl Rng {
    fn fill(dest: &mut [u8]) {
        <Driver as embassy_crypto::driver::Rng>::fill_bytes(dest);
    }

    fn next_u32() -> u32 {
        let mut bytes = [0; 4];
        Self::fill(&mut bytes);
        u32::from_le_bytes(bytes)
    }

    fn next_u64() -> u64 {
        let mut bytes = [0; 8];
        Self::fill(&mut bytes);
        u64::from_le_bytes(bytes)
    }
}

impl rand_core_06::RngCore for Rng {
    fn next_u32(&mut self) -> u32 {
        Self::next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        Self::next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        Self::fill(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core_06::Error> {
        Self::fill(dest);
        Ok(())
    }
}

impl rand_core_06::CryptoRng for Rng {}

impl rand_core_09::RngCore for Rng {
    fn next_u32(&mut self) -> u32 {
        Self::next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        Self::next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        Self::fill(dest);
    }
}

impl rand_core_09::CryptoRng for Rng {}

impl rand_core_010::TryRng for Rng {
    type Error = core::convert::Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(Self::next_u32())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(Self::next_u64())
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        Self::fill(dest);
        Ok(())
    }
}

impl rand_core_010::TryCryptoRng for Rng {}

// Raw builds keep the feature's DRBG dependencies (a feature can't depend on
// a cfg) but don't use them; the linker drops them.
#[cfg(rng_construction = "raw")]
use {hmac as _, sha2 as _};

/// Entropy input for one instantiation: 32 words, 256 bits at the health
/// tests' assumed 8 bits a word, `HMAC_DRBG`'s full security strength.
#[cfg(rng_construction = "drbg")]
const ENTROPY_WORDS: usize = 32;
/// The nonce: 16 more words, 128 bits at the same assumption (SP 800-90A
/// section 8.6.7 asks for half the security strength).
#[cfg(rng_construction = "drbg")]
const NONCE_WORDS: usize = 16;

/// One `HMAC_DRBG` instantiation, from fresh TRNG words and the chip's unique
/// ID as personalization string, and its output.
#[cfg(rng_construction = "drbg")]
fn drbg_request(out: &mut [u8]) {
    let mut words = [0_u32; ENTROPY_WORDS + NONCE_WORDS];
    draw(&mut words);
    let mut seed = [0_u8; 4 * (ENTROPY_WORDS + NONCE_WORDS)];
    for (bytes, word) in seed.as_chunks_mut::<4>().0.iter_mut().zip(words.iter()) {
        *bytes = word.to_le_bytes();
    }
    let (entropy, nonce) = seed.split_at(4 * ENTROPY_WORDS);
    let mut drbg = super::drbg::Drbg::instantiate(entropy, nonce, &crate::uid::unique_id());
    wipe(&mut seed);
    wipe(words_as_bytes(&mut words));
    drbg.generate(out);
}

#[cfg(rng_construction = "drbg")]
fn words_as_bytes(words: &mut [u32]) -> &mut [u8] {
    // SAFETY: any u32 is four valid bytes, and u8's alignment is 1.
    unsafe { core::slice::from_raw_parts_mut(words.as_mut_ptr().cast::<u8>(), core::mem::size_of_val(words)) }
}

/// Zeroes secret bytes in a way the compiler keeps.
#[cfg(rng_construction = "drbg")]
fn wipe(bytes: &mut [u8]) {
    for byte in bytes.iter_mut() {
        // SAFETY: a valid, aligned byte.
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
