// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! CAN0 and CAN1, Bosch `M_CAN` controllers (3.2.x) whose message RAM is
//! placed by register, as [can-hal](can_hal) channels (feature `can-hal`).
//! The driver core is can-hal's for now: room is left for an
//! `embedded-can` feature beside it, which would need a core of its own
//! types.
//!
//! The channel confirms its transmissions: it implements can-hal's
//! [`ConfirmedTransmit`] and [`ReceiveEvents`] for [`CanFrame`] and, with a
//! data phase ([`Config::with_data`]) and [`FdMessageRam`], for FD
//! [`Frame`]s, reporting each frame [`Sent`](can_hal::Outcome::Sent) from
//! the controller's Tx event FIFO, and interleaving those reports with the
//! frames received in the order they started on the bus. It also implements
//! [`Filterable`], [`BusStatus`], [`Recover`] and [`Statistics`]. Classic
//! frames keep their DLC, 9 to 15 included, and received remote frames carry
//! no data.
//!
//! [`Can::new`] takes the controller, its interrupt binding, its pins, its
//! message RAM ([`ClassicMessageRam`] for a classic channel,
//! [`FdMessageRam`] for an FD one) and a [`State`], both `static`s, and
//! feeds the kernel clock from [`Config`]'s generator. Solve the bit timing
//! for that generator's rate ([`clocks::generator_hz`](crate::clocks::generator_hz)).
//!
//! ```ignore
//! bind_interrupts!(struct Irqs { CAN0 => can::InterruptHandler<peripherals::CAN0>; });
//! static STATE: State<MyClock, 32> = State::new();
//! static RAM: ClassicMessageRam = ClassicMessageRam::new();
//!
//! let hz = clocks::generator_hz(2).unwrap();
//! let nominal = BitTiming::solve_nominal(hz, 250_000, SamplePoint::from_per_mille(875))?;
//! let mut can = Can::new(p.CAN0, Irqs, p.PB16, p.PB17, &RAM, &STATE, Config::new(nominal).with_generator(2))?;
//! can.start()?;
//! ```

mod frame;
pub(crate) mod instance;
mod order;
mod registers;
mod state;
mod timing;

use core::fmt;
use core::future::poll_fn;
use core::task::Poll;
use core::time::Duration;

use crate::Peri;
use crate::gpio;
use crate::interrupt::typelevel::{Binding, Handler, Interrupt as _};

use can_hal::{AbortError, BusState, BusStatistics, BusStatus, CanFrame, CanId, ConfirmedTransmit, ErrorCounters, Event, Filter, Filterable, Frame, ReceiveEvents, Recover, Statistics, TransmitId, TryTransmitError};

pub use instance::{Instance, InterruptHandler, RxPin, TxPin};
pub use registers::{ClassicMessageRam, EXTENDED_FILTERS, FdMessageRam, MessageRam, STANDARD_FILTERS};
pub use state::State;
pub use timing::BitTiming;

use can_common::queue::Queue;
use order::{MAILBOXES, Outstanding};
use registers::{
    CCCR_BRSE, CCCR_CCE, CCCR_DAR, CCCR_FDOE, CCCR_INIT, CCCR_MON, CCCR_TEST, COUNTS, DBTP_TDC, ECR_RP, EFEC_FIFO0, EFT_CLASSIC, FILTERS_SIZE_SHIFT, GFC_ANFE_REJECT, GFC_ANFS_REJECT, H1_MM_SHIFT, ILE_EINT0, IR_BO, IR_EP, IR_EW, IR_MRAF, IR_PEA,
    IR_PED, IR_RF0L, IR_RF0N, IR_TCF, IR_TEFL, IR_TEFN, MRCFG_OFFSET_SHIFT, PSR_ACT_MASK, RXF0C_F0S_SHIFT, Ram, RegisterBlock, Regs, SFEC_FIFO0, SFID1_SHIFT, SFT_CLASSIC, T1_EFC, TEST_LBCK, TOCC_ETOC, TOCC_TOP_SHIFT, TSCC_TSS_INTERNAL,
    TXBC_TFQS_SHIFT, TXEFC_EFS_SHIFT, TXFQS_TFQF, TXFQS_TFQPI_MASK, TXFQS_TFQPI_SHIFT,
};

pub use can_common::Clock;

/// What the controller does with the bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// Takes part in the bus.
    #[default]
    Normal,
    /// Bus monitoring: listens, and never transmits or acknowledges.
    /// Transmit requests wait.
    Silent,
    /// External loopback: receives its own frames and drives them on the bus
    /// too, but ignores the bus's acknowledgement.
    Loopback,
    /// Internal loopback: receives its own frames and neither drives the bus
    /// nor listens to it; a self-test, safe whatever the pins are wired to,
    /// and one that needs no pins at all.
    SilentLoopback,
}

/// How to set up the controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// The nominal bit timing, of classic frames and of FD frames'
    /// arbitration.
    pub nominal: BitTiming,
    /// The data phase's timing, for FD frames with bit-rate switching. With
    /// none, the channel sends and receives classic frames only. Needs an
    /// [`FdMessageRam`].
    pub data: Option<BitTiming>,
    /// How many standard and extended filter elements to use, at most what
    /// the message RAM has room for; with none, all of it. A list of length
    /// 0 can't filter its identifier kind, whose frames are then all
    /// accepted.
    pub filter_lists: Option<(usize, usize)>,
    /// The mode.
    pub mode: Mode,
    /// Whether the controller retries a frame until it is sent (the CAN
    /// default), or tries once and gives up (single-shot).
    ///
    /// Single-shot, the channel takes the bus error interrupts whether or
    /// not it reports them, and the timeout counter: a frame that fails
    /// raises no interrupt of its own.
    pub retransmit: bool,
    /// Whether to report each bus error, through [`Can::next_bus_error`].
    /// Off by default: on a faulty bus there can be thousands a second.
    pub bus_errors: bool,
    /// The generic clock generator to feed the kernel clock from: 2 by
    /// default, which [`clocks::Config::pll_from_xosc`](crate::clocks::Config::pll_from_xosc)
    /// runs at 80 MHz. The timings must be solved for its rate.
    pub generator: usize,
}

impl Config {
    /// A normal channel with the given timing, retrying frames until sent.
    #[must_use]
    pub const fn new(nominal: BitTiming) -> Self {
        Self {
            nominal,
            data: None,
            filter_lists: None,
            mode: Mode::Normal,
            retransmit: true,
            bus_errors: false,
            generator: 2,
        }
    }

    /// The same, taking FD frames too, with `data` the timing of their data
    /// phase when they switch bit rate.
    #[must_use]
    pub const fn with_data(mut self, data: BitTiming) -> Self {
        self.data = Some(data);
        self
    }

    /// The same, with `standard` and `extended` filter elements in use: the
    /// live filters [`Filterable`] can set, and how many elements the
    /// controller scans per frame.
    #[must_use]
    pub const fn with_filter_lists(mut self, standard: usize, extended: usize) -> Self {
        self.filter_lists = Some((standard, extended));
        self
    }

    /// The same, in `mode`.
    #[must_use]
    pub const fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    /// The same, retrying frames or not.
    #[must_use]
    pub const fn with_retransmit(mut self, retransmit: bool) -> Self {
        self.retransmit = retransmit;
        self
    }

    /// The same, the kernel clock fed from `generator`.
    #[must_use]
    pub const fn with_generator(mut self, generator: usize) -> Self {
        self.generator = generator;
        self
    }

    /// The same, reporting bus errors or not.
    #[must_use]
    pub const fn with_bus_errors(mut self, bus_errors: bool) -> Self {
        self.bus_errors = bus_errors;
        self
    }
}

/// A bus error the controller saw, from its last error code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum BusError {
    /// More than five equal bits in a row.
    Stuff,
    /// A fixed-form field held an illegal bit.
    Form,
    /// The frame it sent wasn't acknowledged.
    Acknowledgement,
    /// It sent a recessive bit and read back a dominant one.
    BitRecessive,
    /// It sent a dominant bit and read back a recessive one.
    BitDominant,
    /// A received frame's CRC didn't match.
    Crc,
}

impl BusError {
    /// The error a last error code (PSR's LEC) stands for, if any.
    pub(crate) const fn from_lec(lec: u32) -> Option<Self> {
        Some(match lec {
            1 => Self::Stuff,
            2 => Self::Form,
            3 => Self::Acknowledgement,
            4 => Self::BitRecessive,
            5 => Self::BitDominant,
            6 => Self::Crc,
            _ => return None,
        })
    }
}

/// The errors of this backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// A bit timing outside the controller's ranges, or one that gives no bit
    /// rate.
    InvalidTiming,
    /// The controller didn't enter or leave initialisation: its kernel clock
    /// may be off.
    InitTimeout,
    /// The [`State`] already serves a channel.
    StateInUse,
    /// The [`MessageRam`] is somewhere the controller can't reach.
    MessageRamUnreachable,
    /// A data phase ([`Config::with_data`]) with a [`ClassicMessageRam`],
    /// whose elements can't hold FD frames.
    FdNeedsFdRam,
    /// An FD frame on a classic channel.
    FdDisabled,
    /// More filters of one identifier kind than its list holds
    /// ([`Config::with_filter_lists`]): none were applied, and the channel
    /// accepts everything.
    TooManyFilters,
    /// Filter lists longer than the message RAM has room for.
    FilterListsTooLong,
    /// [`Config`]'s generator isn't running.
    NoClock,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidTiming => "bit timing out of the controller's range",
            Self::InitTimeout => "the controller didn't change mode",
            Self::StateInUse => "the state already serves a channel",
            Self::MessageRamUnreachable => "the message RAM is out of the controller's reach",
            Self::FdNeedsFdRam => "an FD channel needs FD message RAM",
            Self::FdDisabled => "an FD frame on a classic channel",
            Self::TooManyFilters => "more filters than the filter list holds",
            Self::FilterListsTooLong => "filter lists longer than the message RAM holds",
            Self::NoClock => "the kernel clock's generator isn't running",
        })
    }
}

impl core::error::Error for Error {}

/// Polls of the control register before giving up on a mode change: far
/// longer than synchronising the two clock domains takes.
const MODE_CHANGE_POLLS: u32 = 1_000_000;

/// The largest data-phase prescaler that uses transmitter delay compensation:
/// the fast phases (`CiA` 601-3), whose bits are too short for the transceiver's
/// loop delay.
const TDC_MAX_PRESCALER: u16 = 2;

/// Bit times from a bus error to the end of its error frame, and so to a
/// single-shot frame it failed being marked cancelled: up to 12 bits of error
/// flags, 8 of delimiter and 3 of intermission, with room to spare.
pub(crate) const ERROR_FRAME_BITS: u32 = 32;

/// An `M_CAN` channel with confirmed transmission.
///
/// Its events are read with [`ReceiveEvents`], or asynchronously with
/// [`next_event`](Self::next_event).
pub struct Can<C: Clock + 'static, const N: usize> {
    regs: Regs,
    state: &'static State<C, N>,
    /// The kernel clock's GCLK channel.
    gclk_channel: usize,
}

impl<C: Clock, const N: usize> Can<C, N> {
    /// Sets `can` up on `tx` and `rx` with `ram` as its message RAM, feeds
    /// its kernel clock from [`Config`]'s generator, binds it to `state` and
    /// enables its interrupt, leaving it in initialisation:
    /// [`start`](Self::start) it. Every identifier is received, into FIFO 0.
    ///
    /// Frames go on the bus in the order they are handed over (the transmit
    /// FIFO, not the identifier-ordered queue).
    ///
    /// # Errors
    ///
    /// [`Error::NoClock`] if the generator isn't running,
    /// [`Error::MessageRamUnreachable`] if the message RAM isn't in SRAM,
    /// [`Error::FdNeedsFdRam`] for a data phase with classic message RAM,
    /// [`Error::FilterListsTooLong`] for filter lists longer than the message
    /// RAM's, [`Error::InitTimeout`] if the controller doesn't enter
    /// initialisation,
    /// and [`Error::StateInUse`] if `state` already serves a channel.
    #[expect(clippy::needless_pass_by_value, reason = "taking the controller and pins is what makes the channel their only user")]
    pub fn new<T: Instance, const IOSET: u8>(
        can: Peri<'static, T>,
        _irqs: impl Binding<T::Interrupt, InterruptHandler<T>> + 'static,
        tx: Peri<'static, impl TxPin<T, IOSET>>,
        rx: Peri<'static, impl RxPin<T, IOSET>>,
        ram: &'static mut impl MessageRam,
        state: &'static State<C, N>,
        config: Config,
    ) -> Result<Self, Error>
    where
        InterruptHandler<T>: Handler<T::Interrupt>,
    {
        let _ = can;
        if crate::clocks::generator_hz(config.generator).is_none() {
            return Err(Error::NoClock);
        }
        let ram: &'static _ = ram;
        // SAFETY: `T`'s register block, which the `Peri` makes this
        // channel's alone, as `&mut` does the RAM.
        let regs = unsafe { Regs::new(T::REGISTERS, ram.layout()) };
        if !regs.ram().is_reachable() {
            return Err(Error::MessageRamUnreachable);
        }
        if config.data.is_some() && !regs.ram().holds_fd() {
            return Err(Error::FdNeedsFdRam);
        }
        crate::clocks::enable_channel(T::GCLK_CHANNEL, config.generator);
        let (tx_function, rx_function) = (tx.function(), rx.function());
        gpio::set_as_function(tx, tx_function);
        gpio::set_as_function(rx, rx_function);
        let can = Self::configure(regs, state, T::GCLK_CHANNEL, config)?;
        critical_section::with(|cs| T::slot().borrow(cs).set(Some(state)));
        T::Interrupt::unpend();
        // SAFETY: the handler touches only `state`, under its critical
        // section.
        unsafe { T::Interrupt::enable() };
        Ok(can)
    }

    /// Configures the controller and binds it to `state`.
    fn configure(regs: Regs, state: &'static State<C, N>, gclk_channel: usize, config: Config) -> Result<Self, Error> {
        let lists = filter_lists(regs.ram(), &config)?;
        let r = regs.get();
        enter_init(r)?;
        state.with(|inner| {
            if inner.regs.is_some() {
                return Err(Error::StateInUse);
            }
            inner.regs = Some(regs);
            inner.report_bus_errors = config.bus_errors;
            inner.single_shot = !config.retransmit;
            inner.filter_lists = lists;
            Ok(())
        })?;
        accept_all(regs.ram(), lists);
        configure(r, regs.ram(), config, lists);
        Ok(Self { regs, state, gclk_channel })
    }

    fn registers(&self) -> &RegisterBlock {
        self.regs.get()
    }

    /// The controller's core release register (CREL): the release, step
    /// and substep in its top twelve bits (`0x323` for 3.2.3), then the
    /// date of its synthesis.
    #[must_use]
    pub fn core_release(&self) -> u32 {
        self.registers().crel.read()
    }

    /// Stops the controller: cancels the transmissions outstanding, whose
    /// reports follow (`Aborted`, or `Sent` if a frame was already on its
    /// way; one that had already ended keeps its outcome), and enters
    /// initialisation, leaving the bus. [`start`](Self::start) rejoins it.
    ///
    /// # Errors
    ///
    /// [`Error::InitTimeout`] if the controller doesn't enter initialisation.
    pub fn stop(&mut self) -> Result<(), Error> {
        let regs = self.regs;
        self.state.with(|inner| {
            // What has finished already is reported as it ended, not as
            // aborted: a single-shot frame may have failed unreported.
            inner.handle(regs, &C::now());
            for mailbox in inner.outstanding.abort_unfinished() {
                regs.get().txbcr.write(1 << mailbox);
            }
        });
        // A buffer stops pending once its frame is cancelled, or sent.
        let r = self.registers();
        for _ in 0..MODE_CHANGE_POLLS {
            if r.txbrp.read() == 0 {
                break;
            }
            core::hint::spin_loop();
        }
        enter_init(r)?;
        self.state.with(|inner| inner.handle(regs, &C::now()));
        Ok(())
    }

    /// Stops the controller if it is running and sets it up afresh with
    /// `config`, leaving it stopped: then [`start`](Self::start) it.
    ///
    /// A channel set up afresh counts as error active. The error counters
    /// carry over, though, so if they still say otherwise, the first read of
    /// the state after this reports the change.
    ///
    /// # Errors
    ///
    /// [`Error::NoClock`] if [`Config`]'s generator isn't running,
    /// [`Error::FdNeedsFdRam`] for a data phase with classic message RAM,
    /// [`Error::FilterListsTooLong`] for filter lists longer than the message
    /// RAM's, [`Error::InitTimeout`] if the controller doesn't enter
    /// initialisation.
    pub fn reconfigure(&mut self, config: Config) -> Result<(), Error> {
        if crate::clocks::generator_hz(config.generator).is_none() {
            return Err(Error::NoClock);
        }
        if config.data.is_some() && !self.regs.ram().holds_fd() {
            return Err(Error::FdNeedsFdRam);
        }
        let lists = filter_lists(self.regs.ram(), &config)?;
        self.stop()?;
        crate::clocks::enable_channel(self.gclk_channel, config.generator);
        self.set_filter_lists(lists);
        configure(self.registers(), self.regs.ram(), config, lists);
        self.state.with(|inner| {
            inner.report_bus_errors = config.bus_errors;
            inner.single_shot = !config.retransmit;
            inner.watching = false;
            inner.bus_error = None;
            inner.bus_state = BusState::ErrorActive;
            inner.warning.forget();
        });
        Ok(())
    }

    /// Sets the controller up again as [`new`](Self::new) did, stopped, with
    /// `config`, and forgets everything before: frames not finished (they
    /// are not reported) and events not taken, received frames among them.
    /// Setting it up empties the receive FIFO (setting `CCCR.CCE` resets its
    /// status) and clears the message RAM. Statistics carry on, and transmit
    /// identifiers keep counting, so none from before can match a frame sent
    /// after.
    ///
    /// Setting `CCCR.INIT` leaves the error counters unchanged: for a start
    /// with nothing carried over, reset the controller first.
    ///
    /// # Errors
    ///
    /// [`Error::FdNeedsFdRam`] for a data phase with classic message RAM,
    /// [`Error::FilterListsTooLong`] for filter lists longer than the message
    /// RAM's, [`Error::InitTimeout`] if the controller doesn't enter
    /// initialisation.
    pub fn reinitialize(&mut self, config: Config) -> Result<(), Error> {
        if config.data.is_some() && !self.regs.ram().holds_fd() {
            return Err(Error::FdNeedsFdRam);
        }
        let lists = filter_lists(self.regs.ram(), &config)?;
        let r = self.registers();
        r.ie.write(0);
        enter_init(r)?;
        self.state.with(|inner| {
            inner.events = Queue::new();
            inner.outstanding = Outstanding::new();
            inner.report_bus_errors = config.bus_errors;
            inner.single_shot = !config.retransmit;
            inner.watching = false;
            inner.bus_error = None;
            inner.bus_state = BusState::ErrorActive;
            inner.warning.forget();
        });
        self.set_filter_lists(lists);
        configure(r, self.regs.ram(), config, lists);
        Ok(())
    }

    /// Records the filter list lengths for a configuration: the filters set
    /// stay if the lengths don't change, and are cleared (everything
    /// accepted) if they do.
    fn set_filter_lists(&self, lists: (usize, usize)) {
        let ram = self.regs.ram();
        self.state.with(|inner| {
            if inner.filter_lists != lists {
                inner.filter_lists = lists;
                accept_all(ram, lists);
            }
        });
    }

    /// Leaves initialisation. The controller joins the bus once it has seen
    /// it idle (11 recessive bits); in internal loopback, at once.
    ///
    /// # Errors
    ///
    /// [`Error::InitTimeout`] if the controller doesn't leave initialisation.
    pub fn start(&mut self) -> Result<(), Error> {
        leave_init(self.registers())
    }

    /// A read-only view of the channel's error state, to read it without
    /// the channel.
    #[must_use]
    pub const fn status(&self) -> Status<C, N> {
        Status { regs: self.regs, state: self.state }
    }

    /// Whether the controller has joined the bus since [`start`](Self::start):
    /// it is out of initialisation and no longer synchronising.
    #[must_use]
    pub fn is_synchronized(&self) -> bool {
        let regs = self.regs;
        regs.get().cccr.read() & CCCR_INIT == 0 && self.state.with(|inner| inner.update_bus_state(regs.get())) & PSR_ACT_MASK != 0
    }

    /// The next bus error, waiting for one, when the channel was configured
    /// to report them ([`Config::bus_errors`]); otherwise it waits for ever.
    /// Errors that come faster than they are read are counted
    /// ([`Statistics`]) but only the latest is kept.
    pub async fn next_bus_error(&self) -> BusError {
        poll_fn(|cx| {
            self.state.with(|inner| {
                if let Some(error) = inner.bus_error.take() {
                    Poll::Ready(error)
                } else {
                    inner.error_waker = Some(cx.waker().clone());
                    Poll::Pending
                }
            })
        })
        .await
    }

    /// Whether either error counter is at the warning level (96 or more)
    /// once that has changed since the last call, waiting for a change. It
    /// changes with the counters, so not in step with
    /// [`Event::BusStateChanged`]: it is still set when the channel becomes
    /// error passive or goes bus-off (can-hal's [`BusState`] has no warning
    /// state). Changes that come faster than they are read are merged, so a
    /// change and back may go unseen.
    pub async fn next_warning_change(&self) -> bool {
        let regs = self.regs;
        poll_fn(|cx| {
            self.state.with(|inner| {
                let _ = inner.update_bus_state(regs.get());
                inner.warning.poll_change(cx)
            })
        })
        .await
    }

    /// The next event, waiting for one.
    pub async fn next_event(&self) -> Event<Frame, C::Instant> {
        let regs = self.regs;
        poll_fn(|cx| {
            self.state.with(|inner| {
                let _ = inner.update_bus_state(regs.get());
                if let Some(event) = inner.events.pop() {
                    Poll::Ready(event)
                } else {
                    inner.waker = Some(cx.waker().clone());
                    Poll::Pending
                }
            })
        })
        .await
    }
}

/// Sets CCCR.INIT and waits for it to take.
fn enter_init(r: &RegisterBlock) -> Result<(), Error> {
    r.cccr.modify(|cccr| cccr | CCCR_INIT);
    wait_for_init(r, CCCR_INIT)
}

/// Clears CCCR.CCE then CCCR.INIT, and waits for it to take.
fn leave_init(r: &RegisterBlock) -> Result<(), Error> {
    r.cccr.modify(|cccr| cccr & !CCCR_CCE);
    r.cccr.modify(|cccr| cccr & !CCCR_INIT);
    wait_for_init(r, 0)
}

/// Polls CCCR until its INIT bit is `value`.
fn wait_for_init(r: &RegisterBlock, value: u32) -> Result<(), Error> {
    for _ in 0..MODE_CHANGE_POLLS {
        if r.cccr.read() & CCCR_INIT == value {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(Error::InitTimeout)
}

/// The interrupts `config` needs: Tx events, frames received, overruns
/// and message RAM access failures, cancellations finished, error warning,
/// error passive and bus-off changes, and each bus error if reported or if
/// frames are single-shot (see
/// [`Inner::watch_failures`](state::Inner::watch_failures)). The timeout
/// counter's is enabled only while it is needed.
const fn interrupts(config: Config) -> u32 {
    let base = IR_TEFN | IR_TEFL | IR_RF0N | IR_RF0L | IR_MRAF | IR_TCF | IR_EW | IR_EP | IR_BO;
    if config.bus_errors || !config.retransmit { base | IR_PEA | IR_PED } else { base }
}

/// Writes `config` to the controller, which must be in initialisation, and
/// lays out and clears its message RAM.
/// The filter list lengths `config` asks for, checked against the RAM.
fn filter_lists(ram: Ram, config: &Config) -> Result<(usize, usize), Error> {
    let capacity = ram.filter_capacity();
    let lists = config.filter_lists.unwrap_or(capacity);
    if lists.0 > capacity.0 || lists.1 > capacity.1 {
        return Err(Error::FilterListsTooLong);
    }
    Ok(lists)
}

fn configure(r: &RegisterBlock, ram: Ram, config: Config, lists: (usize, usize)) {
    r.cccr.modify(|cccr| cccr | CCCR_CCE);
    clear(ram);

    let dar = if config.retransmit { 0 } else { CCCR_DAR };
    let (mode, test) = match config.mode {
        Mode::Normal => (0, 0),
        Mode::Silent => (CCCR_MON, 0),
        Mode::Loopback => (CCCR_TEST, TEST_LBCK),
        Mode::SilentLoopback => (CCCR_TEST | CCCR_MON, TEST_LBCK),
    };
    // FD frames, with bit-rate switching, when there is a data phase.
    let fd = if config.data.is_some() { CCCR_FDOE | CCCR_BRSE } else { 0 };
    r.cccr.write(CCCR_INIT | CCCR_CCE | fd | dar | mode);
    r.test.write(test);
    r.nbtp.write(config.nominal.nbtp());
    // The data phase, with transmitter delay compensation when it is fast
    // enough to need it: the secondary sample point is then the data sample
    // point after the measured delay.
    if let Some(data) = config.data {
        if data.prescaler() <= TDC_MAX_PRESCALER {
            r.tdcr.write((data.sample_offset() & 0x7F) << 8);
            r.dbtp.write(data.dbtp() | DBTP_TDC);
        } else {
            r.tdcr.write(0);
            r.dbtp.write(data.dbtp());
        }
    }

    // The message RAM: its 64 KiB block, then each section's place in it.
    // The RAM's data field size everywhere: eight bytes in classic RAM, 64 in
    // FD RAM (RXESC's F0DS, F1DS and RBDS; TXESC's TBDS).
    r.mrcfg.write(ram.offset() << MRCFG_OFFSET_SHIFT);
    let esc = ram.esc();
    r.rxesc.write(esc | (esc << 4) | (esc << 8));
    r.txesc.write(esc);
    // The filter lists at the lengths configured, frames matching none
    // rejected: with no filters set, each list's first element accepts
    // everything (see `accept_all`), so changing them never needs
    // initialisation. A kind with no list isn't filtered: its frames are all
    // accepted. FIFO 0 keeps the oldest when full (F0OM 0, blocking).
    #[expect(clippy::cast_possible_truncation, reason = "at most 128")]
    let (standard, extended) = (lists.0 as u32, lists.1 as u32);
    r.sidfc.write(ram.standard_filters_start() | (standard << FILTERS_SIZE_SHIFT));
    r.xidfc.write(ram.extended_filters_start() | (extended << FILTERS_SIZE_SHIFT));
    r.xidam.write(0x1FFF_FFFF);
    let standard_policy = if lists.0 > 0 { GFC_ANFS_REJECT } else { 0 };
    let extended_policy = if lists.1 > 0 { GFC_ANFE_REJECT } else { 0 };
    r.gfc.write(standard_policy | extended_policy);
    r.rxf0c.write(ram.rx_fifo_start() | (COUNTS.2 << RXF0C_F0S_SHIFT));
    r.rxf1c.write(0);
    r.rxbc.write(0);
    r.txefc.write(ram.tx_events_start() | (COUNTS.1 << TXEFC_EFS_SHIFT));
    // The transmit FIFO, in hand-over order (TFQM 0), with no dedicated
    // buffers (NDTB 0).
    r.txbc.write(ram.tx_buffers_start() | (COUNTS.0 << TXBC_TFQS_SHIFT));

    // Stamps count bit times, and so does the timeout counter, which
    // single-shot frames need.
    r.tscc.write(TSCC_TSS_INTERNAL);
    r.tocc.write(if config.retransmit { 0 } else { (ERROR_FRAME_BITS << TOCC_TOP_SHIFT) | TOCC_ETOC });

    r.ie.write(interrupts(config));
    r.ils.write(0);
    r.ile.write(ILE_EINT0);
    r.txbtie.write(0);
    r.txbcie.write(u32::MAX >> (32 - COUNTS.0));
    r.ir.write(u32::MAX);
}

/// A standard filter element storing frames that match `filter` in FIFO 0.
fn standard_element(id: u16, mask: u32) -> u32 {
    SFT_CLASSIC | SFEC_FIFO0 | (u32::from(id & 0x7FF) << SFID1_SHIFT) | (mask & 0x7FF)
}

/// Writes the filter lists, `lists` long: `filters` in order, the rest
/// disabled.
fn write_filters(ram: Ram, lists: (usize, usize), filters: &[Filter]) {
    let mut standard = filters.iter().filter_map(|f| match f.id {
        CanId::Standard(id) => Some(standard_element(id, f.mask)),
        CanId::Extended(_) => None,
    });
    for index in 0..lists.0 {
        ram.standard_filter(index).write(standard.next().unwrap_or(0));
    }
    let mut extended = filters.iter().filter_map(|f| match f.id {
        CanId::Extended(id) => Some((EFEC_FIFO0 | (id & 0x1FFF_FFFF), EFT_CLASSIC | (f.mask & 0x1FFF_FFFF))),
        CanId::Standard(_) => None,
    });
    for index in 0..lists.1 {
        let (f0, f1) = extended.next().unwrap_or((0, 0));
        // The element's enable (EFEC) last on, first off.
        ram.extended_filter(index, 0).write(0);
        ram.extended_filter(index, 1).write(f1);
        ram.extended_filter(index, 0).write(f0);
    }
}

/// Filter lists that accept every frame: each list's first element matches
/// everything (a mask of 0), the rest are disabled.
fn accept_all(ram: Ram, lists: (usize, usize)) {
    write_filters(ram, lists, &[Filter::new(CanId::Standard(0), 0), Filter::new(CanId::Extended(0), 0)]);
}

/// Zeroes the message RAM's frame sections, which the controller reads
/// before it writes; the filter lists stay as set.
fn clear(ram: Ram) {
    for index in 0..registers::RX_FIFO {
        for word in 0..2 + ram.data_words() {
            ram.rx_word(index, word).write(0);
        }
    }
    for index in 0..registers::TX_BUFFERS {
        for word in 0..2 + ram.data_words() {
            ram.tx_word(index, word).write(0);
        }
    }
    for index in 0..registers::TX_EVENTS {
        ram.tx_event_word(index, 0).write(0);
        ram.tx_event_word(index, 1).write(0);
    }
}

impl<C: Clock, const N: usize> ConfirmedTransmit<Frame> for Can<C, N> {
    type Error = Error;

    fn capacity(&self) -> usize {
        MAILBOXES
    }

    fn try_transmit(&mut self, frame: &Frame) -> Result<TransmitId, TryTransmitError<Error>> {
        let regs = self.regs;
        let r = regs.get();
        if matches!(frame, Frame::Fd(_)) && r.cccr.read() & CCCR_FDOE == 0 {
            return Err(TryTransmitError::Backend(Error::FdDisabled));
        }
        let now = C::now();
        self.state.with(|inner| {
            // Take in what has finished, so that a buffer the controller has
            // freed has no transmission left unfinished in it.
            inner.handle(regs, &now);
            let txfqs = r.txfqs.read();
            if inner.outstanding.is_full() || txfqs & TXFQS_TFQF != 0 {
                return Err(TryTransmitError::Full);
            }
            let mailbox = ((txfqs >> TXFQS_TFQPI_SHIFT) & TXFQS_TFQPI_MASK) as u8;
            let mut element = frame::encode(frame);
            element.h1 |= T1_EFC | (u32::from(mailbox) << H1_MM_SHIFT);
            state::write_element(regs.ram(), usize::from(mailbox), &element);
            let id = inner.take_id();
            inner.outstanding.push(id, mailbox);
            r.txbar.write(1 << mailbox);
            Ok(id)
        })
    }

    fn abort(&mut self, id: TransmitId) -> Result<(), AbortError<Error>> {
        let regs = self.regs;
        self.state.with(|inner| {
            if let Some(mailbox) = inner.outstanding.request_abort(id) {
                regs.get().txbcr.write(1 << mailbox);
            }
        });
        Ok(())
    }
}

/// A classic channel's frames, through the [`Frame`] implementation.
impl<C: Clock, const N: usize> ConfirmedTransmit<CanFrame> for Can<C, N> {
    type Error = Error;

    fn capacity(&self) -> usize {
        MAILBOXES
    }

    fn try_transmit(&mut self, frame: &CanFrame) -> Result<TransmitId, TryTransmitError<Error>> {
        ConfirmedTransmit::<Frame>::try_transmit(self, &Frame::Can(frame.clone()))
    }

    fn abort(&mut self, id: TransmitId) -> Result<(), AbortError<Error>> {
        ConfirmedTransmit::<Frame>::abort(self, id)
    }
}

impl<C: Clock, const N: usize> ReceiveEvents<Frame> for Can<C, N> {
    type Error = Error;
    type Timestamp = C::Instant;

    fn receive_event(&mut self) -> Result<Event<Frame, C::Instant>, Error> {
        loop {
            if let Some(event) = self.try_receive_event()? {
                return Ok(event);
            }
            core::hint::spin_loop();
        }
    }

    fn try_receive_event(&mut self) -> Result<Option<Event<Frame, C::Instant>>, Error> {
        let regs = self.regs;
        Ok(self.state.with(|inner| {
            let _ = inner.update_bus_state(regs.get());
            inner.events.pop()
        }))
    }

    fn receive_event_timeout(&mut self, timeout: Duration) -> Result<Option<Event<Frame, C::Instant>>, Error> {
        let deadline = C::deadline(timeout);
        loop {
            if let Some(event) = self.try_receive_event()? {
                return Ok(Some(event));
            }
            if C::has_passed(&deadline) {
                return Ok(None);
            }
            core::hint::spin_loop();
        }
    }
}

/// Live acceptance filtering, exact or not at all: each [`Filter`] becomes
/// one classic (identifier and mask) filter element, up to the list lengths
/// configured ([`Config::with_filter_lists`]; by default all the message
/// RAM's room); more of a kind is [`Error::TooManyFilters`], never a merge
/// into something broader. A frame passes if it matches any filter, so with
/// filters of one kind only, frames of the other kind are all rejected
/// (unless that kind has no list, and so isn't filtered). Remote frames are
/// filtered like data frames, FD frames like classic ones.
///
/// The lists change in message RAM while the channel runs, so the channel
/// stays on the bus: frames arriving during the change meet the old filters
/// or the new. The list lengths themselves are set at configuration.
impl<C: Clock, const N: usize> Filterable for Can<C, N> {
    type Error = Error;

    fn set_filters(&mut self, filters: &[Filter]) -> Result<(), Error> {
        let ram = self.regs.ram();
        let lists = self.state.with(|inner| inner.filter_lists);
        let standard = filters.iter().filter(|f| matches!(f.id, CanId::Standard(_))).count();
        if filters.is_empty() {
            accept_all(ram, lists);
        } else if standard > lists.0 || filters.len() - standard > lists.1 {
            accept_all(ram, lists);
            return Err(Error::TooManyFilters);
        } else {
            write_filters(ram, lists, filters);
        }
        Ok(())
    }

    fn clear_filters(&mut self) -> Result<(), Error> {
        let lists = self.state.with(|inner| inner.filter_lists);
        accept_all(self.regs.ram(), lists);
        Ok(())
    }
}

impl<C: Clock, const N: usize> BusStatus for Can<C, N> {
    type Error = Error;

    fn bus_state(&self) -> Result<BusState, Error> {
        self.status().bus_state()
    }

    fn error_counters(&self) -> Result<ErrorCounters, Error> {
        self.status().error_counters()
    }
}

/// A read-only view of a channel's error state, for code that doesn't hold
/// the channel: a USB control handler answering a request for it, say. It
/// reads the protocol status through the channel's state, so that a bus
/// error the read clears is still reported.
pub struct Status<C: Clock + 'static, const N: usize> {
    regs: Regs,
    state: &'static State<C, N>,
}

impl<C: Clock, const N: usize> Clone for Status<C, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Clock, const N: usize> Copy for Status<C, N> {}

impl<C: Clock, const N: usize> BusStatus for Status<C, N> {
    type Error = Error;

    fn bus_state(&self) -> Result<BusState, Error> {
        let regs = self.regs;
        Ok(self.state.with(|inner| {
            let _ = inner.update_bus_state(regs.get());
            inner.bus_state
        }))
    }

    /// The receive counter is 128 while the controller is receive error
    /// passive: REC holds only seven bits (to 127), and ECR.RP says it has
    /// reached 128, as a passive receiver's counter does.
    fn error_counters(&self) -> Result<ErrorCounters, Error> {
        let ecr = self.regs.get().ecr.read();
        #[expect(clippy::cast_possible_truncation, reason = "the counters are seven- and eight-bit fields")]
        Ok(ErrorCounters {
            transmit: ecr as u8,
            receive: if ecr & ECR_RP != 0 { 128 } else { ((ecr >> 8) & 0x7F) as u8 },
        })
    }
}

impl<C: Clock, const N: usize> Recover for Can<C, N> {
    type Error = Error;

    /// Going bus-off puts the controller in initialisation; leaving it starts
    /// recovery, which ends after 129 runs of 11 recessive bits. If the
    /// controller isn't bus-off, it re-enters and leaves initialisation.
    fn recover(&mut self) -> Result<(), Error> {
        let r = self.registers();
        enter_init(r)?;
        leave_init(r)
    }
}

impl<C: Clock, const N: usize> Statistics for Can<C, N> {
    type Error = Error;

    fn statistics(&self) -> Result<BusStatistics, Error> {
        let counts = self.state.with(|inner| inner.counts);
        let mut statistics = BusStatistics::default();
        statistics.rx_frames = Some(u64::from(counts.rx_frames));
        statistics.tx_frames = Some(u64::from(counts.tx_frames));
        statistics.rx_dropped = Some(u64::from(counts.rx_dropped));
        statistics.rx_overruns = Some(u64::from(counts.rx_overruns));
        statistics.tx_failed = Some(u64::from(counts.tx_failed));
        statistics.bus_errors = Some(u64::from(counts.bus_errors));
        Ok(statistics)
    }
}
