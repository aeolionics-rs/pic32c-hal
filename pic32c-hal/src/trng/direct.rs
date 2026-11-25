// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The TRNG as a driver of its own: [`Trng`], without the
//! `embassy-crypto-rng` feature.

use core::convert::Infallible;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;

use super::{CTRLA, DATA, DATARDY, ENABLE, INTENCLR, INTENSET, INTFLAG, read8, read32, write8};
use crate::interrupt::InterruptExt;
use crate::interrupt::typelevel::{self, Binding, Handler};
use crate::{Peri, peripherals};

static WAKER: AtomicWaker = AtomicWaker::new();

/// The TRNG's handler: bind `TRNG` to it with
/// [`bind_interrupts!`](crate::bind_interrupts).
pub struct InterruptHandler {
    _private: (),
}

impl Handler<typelevel::TRNG> for InterruptHandler {
    unsafe fn on_interrupt() {
        // Mask it; the waiting side reads DATA (which clears the flag) and
        // unmasks it for the next word.
        write8(INTENCLR, DATARDY);
        WAKER.wake();
    }
}

/// The TRNG, enabled while this lives.
pub struct Trng<'d> {
    _trng: PhantomData<&'d mut peripherals::TRNG>,
}

impl<'d> Trng<'d> {
    /// Enables the TRNG. Its APB clock is on from reset (MCLK's CLKMSK
    /// leaves every peripheral clocked).
    pub fn new(_trng: Peri<'d, peripherals::TRNG>, _irqs: impl Binding<typelevel::TRNG, InterruptHandler>) -> Self {
        write8(INTENCLR, DATARDY);
        write8(CTRLA, ENABLE);
        let irq = crate::interrupt::TRNG;
        irq.unpend();
        // SAFETY: the handler only masks the TRNG's interrupt and wakes.
        unsafe { irq.enable() };
        Self { _trng: PhantomData }
    }

    /// The next word, spinning until it's ready (at most 84 bus cycles after
    /// the last).
    #[must_use]
    pub fn next_u32_blocking(&mut self) -> u32 {
        while read8(INTFLAG) & DATARDY == 0 {}
        read32(DATA)
    }

    /// Fills `dest`, spinning on each word.
    pub fn fill_blocking(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(4) {
            let word = self.next_u32_blocking().to_le_bytes();
            chunk.copy_from_slice(&word[..chunk.len()]);
        }
    }

    /// The next word, waiting on the interrupt.
    pub async fn next_u32(&mut self) -> u32 {
        poll_fn(|cx| {
            WAKER.register(cx.waker());
            if read8(INTFLAG) & DATARDY != 0 {
                Poll::Ready(read32(DATA))
            } else {
                write8(INTENSET, DATARDY);
                Poll::Pending
            }
        })
        .await
    }

    /// Fills `dest`, waiting on the interrupt for each word.
    pub async fn fill(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(4) {
            let word = self.next_u32().await.to_le_bytes();
            chunk.copy_from_slice(&word[..chunk.len()]);
        }
    }

    fn next_u64_blocking(&mut self) -> u64 {
        u64::from(self.next_u32_blocking()) | (u64::from(self.next_u32_blocking()) << 32)
    }
}

impl Drop for Trng<'_> {
    fn drop(&mut self) {
        crate::interrupt::TRNG.disable();
        write8(INTENCLR, DATARDY);
        write8(CTRLA, 0);
    }
}

impl rand_core_06::RngCore for Trng<'_> {
    fn next_u32(&mut self) -> u32 {
        self.next_u32_blocking()
    }

    fn next_u64(&mut self) -> u64 {
        self.next_u64_blocking()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.fill_blocking(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core_06::Error> {
        self.fill_blocking(dest);
        Ok(())
    }
}

impl rand_core_06::CryptoRng for Trng<'_> {}

impl rand_core_09::RngCore for Trng<'_> {
    fn next_u32(&mut self) -> u32 {
        self.next_u32_blocking()
    }

    fn next_u64(&mut self) -> u64 {
        self.next_u64_blocking()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.fill_blocking(dest);
    }
}

impl rand_core_09::CryptoRng for Trng<'_> {}

impl rand_core_010::TryRng for Trng<'_> {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        Ok(self.next_u32_blocking())
    }

    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        Ok(self.next_u64_blocking())
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Infallible> {
        self.fill_blocking(dest);
        Ok(())
    }
}

impl rand_core_010::TryCryptoRng for Trng<'_> {}
