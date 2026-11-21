// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! What the channel and the interrupt handler share: the event queue, the
//! outstanding transmissions and the counts, behind a critical section.

use core::cell::RefCell;
use core::task::Waker;

use can_hal::{BusState, Event, Frame, Timestamped, TransmitId};
use critical_section::Mutex;

use crate::can::frame::{self, Element};
use crate::can::order::{MAILBOXES, Outstanding, outcome};
use crate::can::registers::{
    self, IR_PEA, IR_PED, IR_RF0L, IR_TOO, PSR_BO, PSR_DLEC_SHIFT, PSR_EP, PSR_EW, PSR_LEC_SHIFT, RX_FIFO, RXFS_FFL_MASK, RXFS_FGI_MASK, RXFS_FGI_SHIFT, Ram, Reg, RegisterBlock, Regs, TX_EVENTS, TXEFS_EFFL_MASK, TXEFS_EFGI_MASK, TXEFS_EFGI_SHIFT,
};
use crate::can::{BusError, Clock};
use can_common::queue::Queue;
use can_common::{Counts, Warning};

/// The state a [`Can`](crate::can::Can) channel shares with its interrupt handler,
/// with room for `N` events. Declare one per controller as a `static`, give it
/// to [`Can::new`](crate::can::Can::new).
pub struct State<C: Clock, const N: usize> {
    inner: Mutex<RefCell<Inner<C::Instant, N>>>,
}

pub(crate) struct Inner<T, const N: usize> {
    pub(crate) regs: Option<Regs>,
    pub(crate) events: Queue<Event<Frame, T>, N>,
    pub(crate) outstanding: Outstanding<T>,
    pub(crate) next_id: u32,
    pub(crate) bus_state: BusState,
    pub(crate) waker: Option<Waker>,
    pub(crate) counts: Counts,
    /// Whether bus errors are reported, and the latest not yet taken.
    pub(crate) report_bus_errors: bool,
    pub(crate) bus_error: Option<BusError>,
    pub(crate) error_waker: Option<Waker>,
    pub(crate) warning: Warning,
    /// Whether frames are single-shot, and whether the timeout counter is
    /// watching for one to be marked failed.
    pub(crate) single_shot: bool,
    pub(crate) watching: bool,
    /// The filter list lengths in use: standard, extended.
    pub(crate) filter_lists: (usize, usize),
}

/// Something that happened, with the bit-time stamp of its start of frame.
enum Happening {
    Sent { mailbox: u8 },
    Received(Frame),
}

impl Happening {
    /// Sent before received when both started together: a frame received in
    /// loopback is reported after its own transmission.
    const fn rank(&self) -> u8 {
        match self {
            Self::Sent { .. } => 0,
            Self::Received(_) => 1,
        }
    }
}

/// At most a full Tx event FIFO and a full receive FIFO 0.
const MAX_HAPPENINGS: usize = TX_EVENTS + RX_FIFO;

impl<C: Clock, const N: usize> State<C, N> {
    /// A state with no controller yet.
    ///
    /// # Panics
    ///
    /// When `N` is no more than the transmit buffers, too few for the
    /// transmission reports: at compile time when the state is a `static`.
    #[must_use]
    pub const fn new() -> Self {
        assert!(N > MAILBOXES, "the queue needs room for more events than there are transmit buffers");
        Self {
            inner: Mutex::new(RefCell::new(Inner {
                regs: None,
                events: Queue::new(),
                outstanding: Outstanding::new(),
                next_id: 0,
                bus_state: BusState::ErrorActive,
                waker: None,
                report_bus_errors: false,
                bus_error: None,
                error_waker: None,
                warning: Warning::new(),
                single_shot: false,
                watching: false,
                filter_lists: (0, 0),
                counts: Counts::new(),
            })),
        }
    }

    pub(crate) fn with<R>(&self, f: impl FnOnce(&mut Inner<C::Instant, N>) -> R) -> R {
        critical_section::with(|cs| f(&mut self.inner.borrow_ref_mut(cs)))
    }

    /// Handles the controller's interrupts: transmissions finished, frames
    /// received, receive overruns, bus errors and error state changes, from
    /// its interrupt line 0, where the channel routes them all.
    pub(crate) fn on_interrupt(&self) {
        self.with(|inner| {
            let Some(regs) = inner.regs else { return };
            inner.handle(regs, &C::now());
        });
    }
}

impl<C: Clock, const N: usize> crate::can::instance::Handle for State<C, N> {
    fn on_interrupt(&self) {
        State::on_interrupt(self);
    }
}

impl<C: Clock, const N: usize> Default for State<C, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, const N: usize> Inner<T, N> {
    /// Takes in everything the controller has finished: Tx events, received
    /// frames and transmissions cancelled or given up, then the error state.
    /// Called from the interrupt handler, and before a transmit buffer is
    /// reused, so that a buffer never holds two unfinished transmissions.
    pub(crate) fn handle(&mut self, regs: Regs, now: &T) {
        let r = regs.get();
        let ram = regs.ram();
        // Clear the flags first: anything that happens from here on raises
        // them again.
        let ir = r.ir.read();
        r.ir.write(ir);
        if ir & IR_RF0L != 0 {
            self.counts.rx_overruns += 1;
        }

        // The transmit buffers' state, before the Tx events are read: a
        // frame sent by now has its event in the FIFO.
        let pending = r.txbrp.read();
        let cancelled = r.txbcf.read();
        let sent = r.txbto.read();

        let mut happened: [Option<(u16, Happening)>; MAX_HAPPENINGS] = [const { None }; MAX_HAPPENINGS];
        let mut count = 0;

        // Transmissions sent, from the Tx event FIFO: the message marker is
        // the buffer the frame was in.
        for _ in 0..TX_EVENTS {
            let txefs = r.txefs.read();
            if txefs & TXEFS_EFFL_MASK == 0 {
                break;
            }
            let index = (txefs >> TXEFS_EFGI_SHIFT) & TXEFS_EFGI_MASK;
            let e1 = ram.tx_event_word(index as usize, 1).read();
            r.txefa.write(index);
            let mailbox = (e1 >> registers::H1_MM_SHIFT) as u8;
            happened[count] = Some((frame::time(e1), Happening::Sent { mailbox }));
            count += 1;
        }

        // Received frames, from FIFO 0.
        for _ in 0..RX_FIFO {
            let rxfs = r.rxf0s.read();
            if rxfs & RXFS_FFL_MASK == 0 {
                break;
            }
            let index = (rxfs >> RXFS_FGI_SHIFT) & RXFS_FGI_MASK;
            let element = read_element(ram, |word| ram.rx_word(index as usize, word));
            r.rxf0a.write(index);
            happened[count] = Some((frame::time(element.h1), Happening::Received(frame::decode(&element))));
            count += 1;
        }

        // Into the order they had on the bus, by start of frame. The stamps
        // are 16 bits of bit time, so ordering by difference holds while they
        // span less than 32768 bits (65 ms at 500 kbit/s).
        let happened = &mut happened[..count];
        if let Some(&Some((reference, _))) = happened.first() {
            #[expect(clippy::cast_possible_wrap, reason = "the difference, as a signed offset")]
            happened.sort_unstable_by_key(|h| h.as_ref().map(|(time, happening)| (time.wrapping_sub(reference) as i16, happening.rank())));
        }
        for (_, happening) in happened.iter_mut().filter_map(Option::take) {
            match happening {
                Happening::Sent { mailbox } => {
                    self.counts.tx_frames += 1;
                    self.outstanding.finish(mailbox, Some(now.clone()));
                }
                Happening::Received(frame) => self.push_received(frame, now),
            }
        }

        // Transmissions that ended without being sent: cancelled, or, without
        // automatic retransmission, failed. Their buffer was no longer
        // pending, its cancellation had finished and it hadn't been sent (a
        // frame asked to be cancelled may be sent all the same, when both
        // flags are set).
        for mailbox in self.unfinished_mailboxes() {
            let bit = 1 << mailbox;
            if pending & bit == 0 && sent & bit == 0 && cancelled & bit != 0 {
                self.outstanding.finish(mailbox, None);
            }
        }
        self.push_reports();
        if self.single_shot {
            let waiting = self.unfinished_mailboxes().any(|mailbox| pending & (1 << mailbox) != 0);
            self.watch_failures(r, ir, waiting);
        }

        let _ = self.update_bus_state(r);
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    /// Without retransmission, a frame that fails raises no interrupt of its
    /// own: the `M_CAN` marks its buffer cancelled (TXBCF) but sets no TCF.
    /// The bus error that failed it does interrupt, but before the error
    /// frame is over and the buffer is marked, so the frame is still pending
    /// then. So after a bus error with a frame still `waiting`, the timeout
    /// counter is restarted to interrupt once an error frame would be over,
    /// and again while frames are waiting; once none is, it is left alone.
    pub(crate) fn watch_failures(&mut self, r: &RegisterBlock, ir: u32, waiting: bool) {
        let error = ir & (IR_PEA | IR_PED) != 0;
        let timed_out = self.watching && ir & IR_TOO != 0;
        if waiting && (error || timed_out) {
            // Restarted, with the flag it may have raised while unwatched
            // cleared, so the next comes a whole period from now.
            r.tocv.write(0);
            r.ir.write(IR_TOO);
            if !self.watching {
                r.ie.modify(|ie| ie | IR_TOO);
                self.watching = true;
            }
        } else if self.watching && !waiting {
            r.ie.modify(|ie| ie & !IR_TOO);
            self.watching = false;
        }
    }

    fn unfinished_mailboxes(&mut self) -> impl Iterator<Item = u8> + use<T, N> {
        let mut mailboxes = [None; MAILBOXES];
        for (slot, mailbox) in mailboxes.iter_mut().zip(self.outstanding.unfinished()) {
            *slot = Some(mailbox);
        }
        mailboxes.into_iter().flatten()
    }

    /// Queues the reports of the transmissions that are next in order.
    fn push_reports(&mut self) {
        while let Some((id, done)) = self.outstanding.pop_report() {
            let outcome = outcome(done);
            if matches!(outcome, can_hal::Outcome::Failed(_)) {
                self.counts.tx_failed += 1;
            }
            // There is always room: received frames leave space for every
            // outstanding report and a change of bus state.
            let _ = self.events.push(Event::Transmitted { id, outcome });
        }
    }

    fn push_received(&mut self, frame: Frame, now: &T) {
        self.counts.rx_frames += 1;
        if self.events.free() > self.outstanding.len() + 1 {
            let _ = self.events.push(Event::Received(Timestamped::new(frame, now.clone())));
        } else {
            self.counts.rx_dropped += 1;
        }
    }

    /// Reads the protocol status, queuing a change of bus state, noting a
    /// bus error and waking `next_warning_change` when the warning level
    /// changes. Every read of PSR goes through here: reading it resets
    /// its error codes, which would otherwise be lost.
    ///
    /// On going bus-off the controller enters initialisation and holds its
    /// transmissions until it recovers; they are cancelled instead, and so
    /// reported as given up. Returns what PSR read.
    pub(crate) fn update_bus_state(&mut self, r: &RegisterBlock) -> u32 {
        let psr = r.psr.read();
        if self.report_bus_errors {
            // The nominal phase's error, else the data phase's.
            let error = BusError::from_lec((psr >> PSR_LEC_SHIFT) & 0b111).or_else(|| BusError::from_lec((psr >> PSR_DLEC_SHIFT) & 0b111));
            if let Some(error) = error {
                self.bus_error = Some(error);
                self.counts.bus_errors += 1;
                if let Some(waker) = self.error_waker.take() {
                    waker.wake();
                }
            }
        }
        let warning = psr & PSR_EW != 0;
        self.warning.update(warning);
        let state = bus_state(psr);
        if state == self.bus_state {
            return psr;
        }
        self.bus_state = state;
        if state == BusState::BusOff {
            for mailbox in self.unfinished_mailboxes() {
                r.txbcr.write(1 << mailbox);
            }
        }
        let _ = self.events.push(Event::BusStateChanged(state));
        psr
    }

    pub(crate) fn take_id(&mut self) -> TransmitId {
        let id = TransmitId(self.next_id);
        self.next_id = self.next_id.wrapping_add(1);
        id
    }
}

/// An element's header and the data its DLC calls for, through `word`
/// (word `i` of the element, headers first).
fn read_element(ram: Ram, word: impl Fn(usize) -> &'static Reg) -> Element {
    let h0 = word(0).read();
    let h1 = word(1).read();
    let mut data = [0u8; 64];
    let words = frame::data_len(h1).div_ceil(4).min(ram.data_words());
    for (i, chunk) in data.as_chunks_mut::<4>().0.iter_mut().enumerate().take(words) {
        *chunk = word(2 + i).read().to_le_bytes();
    }
    Element { h0, h1, data }
}

/// Writes `contents` to transmit buffer `index`: its header, then the data
/// its DLC calls for.
pub(crate) fn write_element(ram: Ram, index: usize, contents: &Element) {
    ram.tx_word(index, 0).write(contents.h0);
    ram.tx_word(index, 1).write(contents.h1);
    let words = frame::data_len(contents.h1).div_ceil(4).min(ram.data_words());
    for (i, chunk) in contents.data.as_chunks::<4>().0.iter().enumerate().take(words) {
        ram.tx_word(index, 2 + i).write(u32::from_le_bytes(*chunk));
    }
}

/// The bus state PSR describes.
pub(crate) const fn bus_state(psr: u32) -> BusState {
    if psr & PSR_BO != 0 {
        BusState::BusOff
    } else if psr & PSR_EP != 0 {
        BusState::ErrorPassive
    } else {
        BusState::ErrorActive
    }
}
