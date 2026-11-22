// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A receiver whose DMA channel fills a ring buffer continuously, as
//! Embassy's `RingBufferedUartRx`: the CPU does nothing per byte, and reads
//! take what has come. From a [`UartRx`] with DMA, by
//! [`UartRx::into_ring_buffered`].
//!
//! The ring takes bytes whether or not anyone reads, so flow control never
//! holds the far end: a reader that falls a whole ring behind gets
//! [`Error::Overrun`](super::Error::Overrun), and carries on from the newest
//! bytes.
//!
//! No `BufRead`: the DMA channel writes the ring while the CPU reads it, so
//! a slice of it handed out could change under the caller after an
//! overrun. Reads copy out instead.

use core::future::poll_fn;
use core::task::Poll;

use super::{BUFOVF, DATA, Error, FERR, INTENSET, PERR, RXC, STATUS, State, UartRx, fifo_byte, read16, write8, write16};
use crate::dma::{ReadableRing, Request};
use crate::mode::Async;

/// A receiver filling a ring buffer by DMA.
pub struct RingBufferedUartRx<'d> {
    base: usize,
    state: &'static State,
    ring: ReadableRing<'d>,
}

impl<'d> UartRx<'d, Async> {
    /// This receiver as one whose DMA channel fills `ring` continuously,
    /// from the first read on.
    ///
    /// # Panics
    ///
    /// If the UART has no receive DMA channel (it wasn't made by
    /// [`Uart::new`](super::Uart::new)), or `ring` is empty or over 64 KiB.
    #[must_use]
    pub fn into_ring_buffered(mut self, ring: &'d mut [u8]) -> RingBufferedUartRx<'d> {
        let channel = self.dma.take().expect("a Uart made with DMA channels");
        // SAFETY: this SERCOM's DATA, a UART making receive requests.
        let ring = unsafe { ReadableRing::new(channel, Request::sercom_rx(self.number), (self.base + DATA) as *const u8, ring) };
        RingBufferedUartRx { base: self.base, state: self.state, ring }
    }
}

impl RingBufferedUartRx<'_> {
    /// Starts the ring if it isn't running: with the FIFO on, only once a
    /// byte is in, as receive DMA started on an empty FIFO takes a stale
    /// byte first.
    async fn ensure_started(&mut self) {
        if self.ring.is_running() {
            return;
        }
        fifo_byte(self.base, self.state).await;
        self.ring.start();
    }

    /// The receive error the SERCOM noted, if any, cleared.
    fn take_error(&self) -> Option<Error> {
        let status = read16(self.base, STATUS) & (PERR | FERR | BUFOVF);
        if status == 0 {
            return None;
        }
        write16(self.base, STATUS, status);
        Some(if status & BUFOVF != 0 {
            Error::Overrun
        } else if status & FERR != 0 {
            Error::Framing
        } else {
            Error::Parity
        })
    }

    /// Reads what has come, waiting until something has: how many bytes.
    /// The first read starts the ring.
    ///
    /// # Errors
    ///
    /// [`Error::Overrun`] if the reader fell a ring behind (reading then
    /// carries on from the newest bytes) or the SERCOM overran; a framing or
    /// parity error the SERCOM noted.
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if buffer.is_empty() {
            return Ok(0);
        }
        self.ensure_started().await;
        poll_fn(|cx| {
            self.ring.register_waker(cx.waker());
            self.state.rx_waker.register(cx.waker());
            if let Some(error) = self.take_error() {
                return Poll::Ready(Err(error));
            }
            match self.ring.read(buffer) {
                Err(_) => Poll::Ready(Err(Error::Overrun)),
                Ok(0) => {
                    // Wake on the next byte, not only at the ring's half and
                    // full marks: RXC once, which the handler masks again.
                    write8(self.base, INTENSET, RXC);
                    match self.ring.read(buffer) {
                        Ok(0) => Poll::Pending,
                        Ok(n) => Poll::Ready(Ok(n)),
                        Err(_) => Poll::Ready(Err(Error::Overrun)),
                    }
                }
                Ok(n) => Poll::Ready(Ok(n)),
            }
        })
        .await
    }

    /// Fills `buffer` exactly.
    ///
    /// # Errors
    ///
    /// As [`read`](Self::read); what came before the error is in `buffer`.
    pub async fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        let mut at = 0;
        while at < buffer.len() {
            at += self.read(&mut buffer[at..]).await?;
        }
        Ok(())
    }

    /// Stops filling the ring; the next read starts it afresh, empty.
    pub fn stop(&mut self) {
        self.ring.stop();
    }

    /// The baud rate, which the transmitter shares.
    #[must_use]
    pub fn baudrate(&self) -> u32 {
        self.state.baudrate(self.base)
    }

    /// Changes the baud rate, for the transmitter too: as
    /// [`Uart::set_baudrate`](super::Uart::set_baudrate). The ring keeps
    /// what it holds.
    ///
    /// # Errors
    ///
    /// As [`Uart::set_baudrate`](super::Uart::set_baudrate).
    pub fn set_baudrate(&mut self, baudrate: u32) -> Result<(), super::ConfigError> {
        self.state.set_baudrate(self.base, baudrate)
    }

    /// Changes the configuration, for the transmitter too: as
    /// [`Uart::set_config`](super::Uart::set_config). The ring keeps what
    /// it holds.
    ///
    /// # Errors
    ///
    /// As [`Uart::set_config`](super::Uart::set_config).
    pub fn set_config(&mut self, config: &super::Config) -> Result<(), super::ConfigError> {
        self.state.configure(self.base, config)
    }
}

impl embassy_embedded_hal::SetConfig for RingBufferedUartRx<'_> {
    type Config = super::Config;
    type ConfigError = super::ConfigError;

    fn set_config(&mut self, config: &super::Config) -> Result<(), super::ConfigError> {
        RingBufferedUartRx::set_config(self, config)
    }
}

impl embedded_io::ErrorType for RingBufferedUartRx<'_> {
    type Error = Error;
}

impl embedded_io_async::Read for RingBufferedUartRx<'_> {
    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        RingBufferedUartRx::read(self, buffer).await
    }
}
