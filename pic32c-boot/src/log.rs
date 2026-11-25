// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The boot log in flash: two pages of 32-byte blocks, each block written once.
//!
//! - A block holds a [`Record`]: its kind in byte 0, the image's [`ImageId`] in bytes 1 to 16, a
//!   16-bit sequence number in bytes 17 and 18, and a CRC-32 over bytes 0 to 27 in bytes 28 to
//!   31. An erased block is all `0xFF`. A block whose CRC doesn't match (a write the power cut
//!   short, or what an erase cut short leaves) is skipped, and never written over.
//! - Each record's sequence number is one more than the last one's, wrapping, and the records
//!   fill a page from block 0. The current page is the one whose last record is newest; the
//!   other holds the records before it.
//! - When the current page is full, the log flips: the other page is erased and the records go
//!   on there. Nothing is copied, so a record is never seen twice or out of order, and the page
//!   before stays readable until the next flip.
//! - The log is the unbroken run of sequence numbers back from the newest record. The order
//!   comes from the records themselves, so no power cut needs repairing: what an erase cut short
//!   leaves behind either fails its CRC or breaks the run, and isn't read. A torn write leaves no
//!   gap: the append that failed didn't use its number.
//!
//! Sequence numbers are compared by serial number arithmetic (newer by less than half the
//! range), which holds as the two pages never hold more than a few hundred records.
//!
//! The history kept is a page back. A `Confirmed` more than a page of records old is erased at
//! the next flip, and the decision's fallback then applies.
//!
//! [`Flash`] is the storage, two pages of [`Flash::BLOCKS`] blocks, so the log runs on the host
//! against a simulated flash as on the part. [`Log`] drives it blocking, for a bootloader;
//! [`AsyncLog`] drives an [`AsyncFlash`] for an application on an executor, so an erase (some
//! 20 ms on the PIC32CK) waits without holding the executor. Both read and write the same log.

use crate::decide::{ImageId, Record};

/// The size of a block: the smallest unit the flash writes (the PIC32CK's quad double word).
pub const BLOCK: usize = 32;

const ERASED: [u8; BLOCK] = [0xFF; BLOCK];
const REQUESTED: u8 = 1;
const STARTED: u8 = 2;
const FAILED: u8 = 3;
const CONFIRMED: u8 = 4;

/// Two pages of flash, each of [`Flash::BLOCKS`] blocks of [`BLOCK`] bytes.
pub trait Flash {
    type Error;
    /// Blocks in a page: at least 1, and no more than 16,000 (so the two pages' sequence
    /// numbers stay within half their range).
    const BLOCKS: usize;
    /// Block `block` of page `page` (0 or 1).
    fn read(&mut self, page: usize, block: usize) -> [u8; BLOCK];
    /// Write block `block` of page `page`, which is erased.
    ///
    /// # Errors
    ///
    /// If the flash fails.
    fn write(&mut self, page: usize, block: usize, data: &[u8; BLOCK]) -> Result<(), Self::Error>;
    /// Erase page `page`.
    ///
    /// # Errors
    ///
    /// If the flash fails.
    fn erase(&mut self, page: usize) -> Result<(), Self::Error>;
}

/// A borrowed flash is a flash, so a log can be opened on one and the flash looked at after.
impl<T: Flash> Flash for &mut T {
    type Error = T::Error;
    const BLOCKS: usize = T::BLOCKS;
    fn read(&mut self, page: usize, block: usize) -> [u8; BLOCK] {
        T::read(self, page, block)
    }
    fn write(&mut self, page: usize, block: usize, data: &[u8; BLOCK]) -> Result<(), Self::Error> {
        T::write(self, page, block, data)
    }
    fn erase(&mut self, page: usize) -> Result<(), Self::Error> {
        T::erase(self, page)
    }
}

/// CRC-32 (IEEE 802.3, reflected): small and slow, for a few blocks per boot.
fn crc32(data: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

/// Whether sequence number `a` is newer than `b`.
const fn newer(a: u16, b: u16) -> bool {
    a != b && a.wrapping_sub(b) < 0x8000
}

/// `record`, numbered `seq`, as a block.
#[must_use]
pub fn encode(record: Record, seq: u16) -> [u8; BLOCK] {
    let (kind, id) = match record {
        Record::Requested(id) => (REQUESTED, id),
        Record::Started(id) => (STARTED, id),
        Record::Failed(id) => (FAILED, id),
        Record::Confirmed(id) => (CONFIRMED, id),
    };
    let mut block = [0; BLOCK];
    block[0] = kind;
    block[1..17].copy_from_slice(&id.0);
    block[17..19].copy_from_slice(&seq.to_le_bytes());
    let crc = crc32(&block[..BLOCK - 4]);
    block[BLOCK - 4..].copy_from_slice(&crc.to_le_bytes());
    block
}

/// The record in `block` and its sequence number, if it holds a whole one.
#[must_use]
pub fn decode(block: &[u8; BLOCK]) -> Option<(Record, u16)> {
    if block[BLOCK - 4..] != crc32(&block[..BLOCK - 4]).to_le_bytes() {
        return None;
    }
    let mut id = [0; 16];
    id.copy_from_slice(&block[1..17]);
    let id = ImageId(id);
    let seq = u16::from_le_bytes([block[17], block[18]]);
    let record = match block[0] {
        REQUESTED => Record::Requested(id),
        STARTED => Record::Started(id),
        FAILED => Record::Failed(id),
        CONFIRMED => Record::Confirmed(id),
        _ => return None,
    };
    Some((record, seq))
}

/// A page as found: where its records end, and its newest record's sequence number.
struct Page {
    /// After the last block written: a torn one isn't written over.
    end: usize,
    newest: Option<u16>,
}

/// Where the log is, whichever way its flash is driven: [`Log`] and [`AsyncLog`] share it.
#[derive(Clone, Copy, Debug)]
struct Cursor {
    /// The current page.
    current: usize,
    /// The next free block in the current page.
    next: usize,
    /// The end of the other page's records.
    older: usize,
    /// The next record's sequence number.
    seq: u16,
}

impl Cursor {
    /// The log as `pages` were found: the current page is the one whose newest record is newer.
    fn new(pages: &[Page; 2]) -> Self {
        let current = match (pages[0].newest, pages[1].newest) {
            (Some(a), Some(b)) => usize::from(newer(b, a)),
            (None, Some(_)) => 1,
            _ => 0,
        };
        let seq = pages[current].newest.map_or(0, |s| s.wrapping_add(1));
        Self {
            current,
            next: pages[current].end,
            older: pages[1 - current].end,
            seq,
        }
    }

    /// The blocks to read, newest first: the current page's, then the other's.
    fn blocks(self) -> impl Iterator<Item = (usize, usize)> {
        let (page, next, older) = (self.current, self.next, self.older);
        (0..next).rev().map(move |b| (page, b)).chain((0..older).rev().map(move |b| (1 - page, b)))
    }

    /// Where the next record goes in pages of `blocks`: its page and block, and whether that
    /// page has to be erased first (a flip).
    fn place(self, blocks: usize) -> (usize, usize, bool) {
        if self.next == blocks { (1 - self.current, 0, true) } else { (self.current, self.next, false) }
    }

    /// The record placed by [`place`](Self::place) is written (or `flipped` and written).
    fn advance(&mut self, blocks: usize, flipped: bool) {
        if flipped {
            self.current = 1 - self.current;
            self.older = blocks;
            self.next = 0;
        }
        self.next += 1;
        self.seq = self.seq.wrapping_add(1);
    }

    /// The flip is done but the write after it failed: the record isn't there, its number not
    /// used.
    fn flipped(&mut self, blocks: usize) {
        self.current = 1 - self.current;
        self.older = blocks;
        // The block the write was cut short in isn't written over.
        self.next = 1;
    }
}

/// The unbroken run of sequence numbers back from the newest record: a gap means what an erase
/// left behind, and ends the log.
#[derive(Default)]
struct Run {
    expect: Option<u16>,
}

impl Run {
    /// The record in `block`, if it's whole and continues the run; `Err` once the run ends.
    fn take(&mut self, block: &[u8; BLOCK]) -> Result<Option<Record>, ()> {
        let Some((record, seq)) = decode(block) else { return Ok(None) };
        if self.expect.is_some_and(|e| e != seq) {
            return Err(());
        }
        self.expect = Some(seq.wrapping_sub(1));
        Ok(Some(record))
    }
}

fn scan<F: Flash>(flash: &mut F, page: usize) -> Page {
    let end = (0..F::BLOCKS).rev().find(|&b| flash.read(page, b) != ERASED).map_or(0, |b| b + 1);
    let newest = (0..end).rev().find_map(|b| decode(&flash.read(page, b))).map(|(_, seq)| seq);
    Page { end, newest }
}

/// The boot log on `F`, blocking: for a bootloader.
pub struct Log<F> {
    flash: F,
    at: Cursor,
}

impl<F: Flash> Log<F> {
    /// The log on `flash`, as the last power cut left it.
    pub fn open(mut flash: F) -> Self {
        let pages = [scan(&mut flash, 0), scan(&mut flash, 1)];
        Self { at: Cursor::new(&pages), flash }
    }

    /// The flash, back.
    pub fn into_flash(self) -> F {
        self.flash
    }

    /// The records, newest first.
    pub fn newest_first(&mut self) -> impl Iterator<Item = Record> + '_ {
        let flash = &mut self.flash;
        let mut run = Run::default();
        self.at.blocks().map(move |(page, block)| run.take(&flash.read(page, block))).map_while(Result::ok).flatten()
    }

    /// Append `record`, flipping to the other page first if this one is full.
    ///
    /// # Errors
    ///
    /// If the flash fails. The log is then as it was, or flipped without `record`.
    pub fn append(&mut self, record: Record) -> Result<(), F::Error> {
        let (page, block, flip) = self.at.place(F::BLOCKS);
        if flip {
            self.flash.erase(page)?;
        }
        if let Err(e) = self.flash.write(page, block, &encode(record, self.at.seq)) {
            if flip {
                self.at.flipped(F::BLOCKS);
            } else {
                self.at.next += 1;
            }
            return Err(e);
        }
        self.at.advance(F::BLOCKS, flip);
        Ok(())
    }
}

/// [`Flash`], with writes and erases that wait without blocking: for an application on an
/// executor. Reads are as quick as the flash's (memory-mapped on the PIC32CK) but async too, as
/// `embedded-storage-async`'s are.
pub trait AsyncFlash {
    type Error;
    /// As [`Flash::BLOCKS`].
    const BLOCKS: usize;
    /// Block `block` of page `page` (0 or 1).
    fn read(&mut self, page: usize, block: usize) -> impl Future<Output = [u8; BLOCK]>;
    /// Write block `block` of page `page`, which is erased.
    fn write(&mut self, page: usize, block: usize, data: &[u8; BLOCK]) -> impl Future<Output = Result<(), Self::Error>>;
    /// Erase page `page`.
    fn erase(&mut self, page: usize) -> impl Future<Output = Result<(), Self::Error>>;
}

impl<T: AsyncFlash> AsyncFlash for &mut T {
    type Error = T::Error;
    const BLOCKS: usize = T::BLOCKS;
    fn read(&mut self, page: usize, block: usize) -> impl Future<Output = [u8; BLOCK]> {
        T::read(self, page, block)
    }
    fn write(&mut self, page: usize, block: usize, data: &[u8; BLOCK]) -> impl Future<Output = Result<(), Self::Error>> {
        T::write(self, page, block, data)
    }
    fn erase(&mut self, page: usize) -> impl Future<Output = Result<(), Self::Error>> {
        T::erase(self, page)
    }
}

async fn scan_async<F: AsyncFlash>(flash: &mut F, page: usize) -> Page {
    let mut end = 0;
    for b in (0..F::BLOCKS).rev() {
        if flash.read(page, b).await != ERASED {
            end = b + 1;
            break;
        }
    }
    let mut newest = None;
    for b in (0..end).rev() {
        if let Some((_, seq)) = decode(&flash.read(page, b).await) {
            newest = Some(seq);
            break;
        }
    }
    Page { end, newest }
}

/// The boot log on `F`, async: [`Log`] for an application, the same log in the same flash.
pub struct AsyncLog<F> {
    flash: F,
    at: Cursor,
}

impl<F: AsyncFlash> AsyncLog<F> {
    /// The log on `flash`, as the last power cut left it.
    pub async fn open(mut flash: F) -> Self {
        let pages = [scan_async(&mut flash, 0).await, scan_async(&mut flash, 1).await];
        Self { at: Cursor::new(&pages), flash }
    }

    /// The flash, back.
    pub fn into_flash(self) -> F {
        self.flash
    }

    /// The newest record that `want` picks, reading newest first.
    pub async fn find(&mut self, mut want: impl FnMut(Record) -> bool) -> Option<Record> {
        let mut run = Run::default();
        for (page, block) in self.at.blocks() {
            match run.take(&self.flash.read(page, block).await) {
                Ok(Some(record)) if want(record) => return Some(record),
                Ok(_) => {}
                Err(()) => break,
            }
        }
        None
    }

    /// Append `record`, flipping to the other page first if this one is full.
    ///
    /// # Errors
    ///
    /// As [`Log::append`].
    pub async fn append(&mut self, record: Record) -> Result<(), F::Error> {
        let (page, block, flip) = self.at.place(F::BLOCKS);
        if flip {
            self.flash.erase(page).await?;
        }
        if let Err(e) = self.flash.write(page, block, &encode(record, self.at.seq)).await {
            if flip {
                self.at.flipped(F::BLOCKS);
            } else {
                self.at.next += 1;
            }
            return Err(e);
        }
        self.at.advance(F::BLOCKS, flip);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use super::*;
    use crate::decide::{Image, Policy, ResetCause};
    use Record::{Confirmed, Failed, Requested, Started};

    fn id(n: u8) -> ImageId {
        ImageId([n; 16])
    }

    /// Flash in RAM, with a power cut after `budget` operations: the write then cut short leaves
    /// a torn block, the erase cut short a page part erased, part garbage, part as it was.
    #[derive(Clone)]
    struct Sim<const N: usize> {
        pages: [[[u8; BLOCK]; N]; 2],
        budget: Option<usize>,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct PowerCut;

    impl<const N: usize> Sim<N> {
        fn new() -> Self {
            Self { pages: [[ERASED; N]; 2], budget: None }
        }

        fn cut_after(mut self, budget: usize) -> Self {
            self.budget = Some(budget);
            self
        }

        fn powered(mut self) -> Self {
            self.budget = None;
            self
        }

        fn spend(&mut self) -> bool {
            match &mut self.budget {
                Some(0) => true,
                Some(n) => {
                    *n -= 1;
                    false
                }
                None => false,
            }
        }
    }

    impl<const N: usize> Flash for Sim<N> {
        type Error = PowerCut;
        const BLOCKS: usize = N;

        fn read(&mut self, page: usize, block: usize) -> [u8; BLOCK] {
            self.pages[page][block]
        }

        fn write(&mut self, page: usize, block: usize, data: &[u8; BLOCK]) -> Result<(), PowerCut> {
            assert_eq!(self.pages[page][block], ERASED, "a block written twice");
            if self.spend() {
                // Half the block made it, the rest still erased.
                self.pages[page][block][..BLOCK / 2].copy_from_slice(&data[..BLOCK / 2]);
                return Err(PowerCut);
            }
            self.pages[page][block] = *data;
            Ok(())
        }

        fn erase(&mut self, page: usize) -> Result<(), PowerCut> {
            if self.spend() {
                for (i, block) in self.pages[page].iter_mut().enumerate() {
                    match i % 3 {
                        0 => *block = ERASED,
                        1 => block[3] ^= 0x5A,
                        // Left as it was, an old record still whole.
                        _ => {}
                    }
                }
                return Err(PowerCut);
            }
            self.pages[page] = [ERASED; N];
            Ok(())
        }
    }

    #[expect(clippy::unused_async_trait_impl, reason = "the async form of a flash that never waits")]
    impl<const N: usize> AsyncFlash for Sim<N> {
        type Error = PowerCut;
        const BLOCKS: usize = N;

        async fn read(&mut self, page: usize, block: usize) -> [u8; BLOCK] {
            Flash::read(self, page, block)
        }

        async fn write(&mut self, page: usize, block: usize, data: &[u8; BLOCK]) -> Result<(), PowerCut> {
            Flash::write(self, page, block, data)
        }

        async fn erase(&mut self, page: usize) -> Result<(), PowerCut> {
            Flash::erase(self, page)
        }
    }

    /// `future` run to its end: the simulated flash is never pending, so polling once does it.
    fn block_on<T>(future: impl Future<Output = T>) -> T {
        let mut future = core::pin::pin!(future);
        let mut cx = core::task::Context::from_waker(core::task::Waker::noop());
        match future.as_mut().poll(&mut cx) {
            core::task::Poll::Ready(value) => value,
            core::task::Poll::Pending => panic!("the simulated flash never waits"),
        }
    }

    /// The records an [`AsyncLog`] finds, newest first.
    fn async_records<F: AsyncFlash>(log: &mut AsyncLog<F>) -> Vec<Record> {
        let mut out = Vec::new();
        block_on(log.find(|r| {
            out.push(r);
            false
        }));
        out
    }

    fn records<F: Flash>(log: &mut Log<F>) -> Vec<Record> {
        log.newest_first().collect()
    }

    /// A log on four-block pages holding `history`, oldest first.
    fn log_of(history: &[Record]) -> Sim<4> {
        let mut log = Log::open(Sim::<4>::new());
        for r in history {
            log.append(*r).unwrap();
        }
        log.into_flash()
    }

    #[test]
    fn blocks_round_trip() {
        for (record, seq) in [(Requested(id(1)), 0), (Started(id(2)), 1), (Failed(id(3)), 0x8000), (Confirmed(id(4)), 0xFFFF)] {
            assert_eq!(decode(&encode(record, seq)), Some((record, seq)));
        }
        assert_eq!(decode(&ERASED), None);
        assert_eq!(decode(&[0; BLOCK]), None);
        let mut torn = encode(Confirmed(id(1)), 7);
        torn[5] ^= 1;
        assert_eq!(decode(&torn), None);
    }

    #[test]
    fn crc32_check_value() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn serial_number_order() {
        assert!(newer(1, 0) && !newer(0, 1) && !newer(5, 5));
        assert!(newer(0, 0xFFFF) && newer(3, 0xFFF0) && !newer(0xFFF0, 3));
    }

    #[test]
    fn appends_and_reopens() {
        let mut log = Log::open(Sim::<8>::new());
        assert_eq!(records(&mut log), vec![]);
        for r in [Confirmed(id(1)), Requested(id(2)), Started(id(2))] {
            log.append(r).unwrap();
        }
        let mut log = Log::open(log.into_flash());
        assert_eq!(records(&mut log), vec![Started(id(2)), Requested(id(2)), Confirmed(id(1))]);
    }

    #[test]
    fn a_torn_record_is_skipped_and_not_written_over() {
        let mut log = Log::open(log_of(&[Confirmed(id(1))]).cut_after(0));
        assert_eq!(log.append(Requested(id(2))), Err(PowerCut));
        let mut log = Log::open(log.into_flash().powered());
        assert_eq!(records(&mut log), vec![Confirmed(id(1))]);
        log.append(Requested(id(3))).unwrap();
        assert_eq!(records(&mut Log::open(log.into_flash())), vec![Requested(id(3)), Confirmed(id(1))]);
    }

    #[test]
    fn a_flip_keeps_the_page_before() {
        let history = [Confirmed(id(1)), Requested(id(2)), Started(id(2)), Confirmed(id(2)), Requested(id(3))];
        let mut log = Log::open(log_of(&history));
        let mut expect = history.to_vec();
        expect.reverse();
        assert_eq!(records(&mut log), expect);
        assert_eq!(log.at.current, 1);
    }

    #[test]
    fn the_next_flip_drops_the_oldest_page() {
        let history: Vec<Record> = (1..=9).map(|n| Confirmed(id(n))).collect();
        let mut log = Log::open(log_of(&history));
        // Page 0 had 1 to 4 and holds 9 now; page 1 holds 5 to 8.
        assert_eq!(records(&mut log), [9, 8, 7, 6, 5].map(|n| Confirmed(id(n))));
    }

    #[test]
    fn across_the_wrap() {
        // Page 0 numbered up to 0xFFFF, page 1 from 0 on: page 1 is current.
        let mut flash = Sim::<4>::new();
        for (b, seq) in [0xFFFCu16, 0xFFFD, 0xFFFE, 0xFFFF].into_iter().enumerate() {
            flash.pages[0][b] = encode(Confirmed(id(u8::try_from(b).unwrap())), seq);
        }
        flash.pages[1][0] = encode(Requested(id(9)), 0);
        let mut log = Log::open(flash);
        assert_eq!((log.at.current, log.at.seq), (1, 1));
        assert_eq!(records(&mut log), vec![Requested(id(9)), Confirmed(id(3)), Confirmed(id(2)), Confirmed(id(1)), Confirmed(id(0))]);
    }

    /// The power cut short at every step of a flip: reopened, the records are as they were,
    /// maybe with the new one, never out of order, and the decision from them the same.
    #[test]
    fn a_power_cut_anywhere_in_a_flip() {
        // Two pages' worth: page 0 then page 1 full, so the next append flips into page 0.
        let history = [Confirmed(id(1)), Requested(id(2)), Started(id(2)), Confirmed(id(2)), Requested(id(3)), Started(id(3)), Started(id(3)), Confirmed(id(3))];
        let full = log_of(&history);
        let before = records(&mut Log::open(full.clone()));
        let images = [Image { id: id(2), version: 1, at: () }, Image { id: id(3), version: 1, at: () }];
        let policy = Policy { min_version: 1, max_starts: 3 };
        let decide = |log: &[Record]| {
            let d = policy.decide(ResetCause::Watchdog, log.iter().copied(), &images);
            (d.boot.map(|i| i.id), d.append)
        };
        let new = Requested(id(4));
        // A flip: an erase and the record.
        for cut in 0..2 {
            let mut log = Log::open(full.clone().cut_after(cut));
            assert_eq!(log.append(new), Err(PowerCut), "cut {cut}");
            let after = records(&mut Log::open(log.into_flash().powered()));
            // The newest of the records before, in order, and nothing an erase left behind.
            assert_eq!(after[..], before[..after.len()], "cut {cut}");
            assert!(after.len() >= 4, "cut {cut}: the current page's records at least");
            assert_eq!(decide(&after), decide(&before), "cut {cut}: {after:?}");
        }
    }

    /// Many records, each appended after a power cut at a different point: reopening always
    /// gives the newest of what was appended, in order.
    #[test]
    fn a_long_history_through_power_cuts() {
        let mut flash = Sim::<4>::new();
        let mut appended: Vec<Record> = Vec::new();
        for n in 0..90u8 {
            let record = [Requested(id(n)), Started(id(n)), Confirmed(id(n))][usize::from(n % 3)];
            flash = flash.cut_after(usize::from(n % 3));
            if Log::open(&mut flash).append(record).is_ok() {
                appended.push(record);
            }
            flash = flash.powered();
            let read = records(&mut Log::open(&mut flash));
            let newest: Vec<Record> = appended.iter().rev().take(read.len()).copied().collect();
            assert_eq!(read, newest, "after {n}");
        }
    }

    /// The blocking and async logs on one flash, taking turns over three flips: both read back
    /// what both appended, in order.
    #[test]
    fn async_and_blocking_logs_are_one_log() {
        let mut flash = Sim::<4>::new();
        let mut appended: Vec<Record> = Vec::new();
        for n in 0..14u8 {
            let record = [Requested(id(n)), Started(id(n)), Confirmed(id(n))][usize::from(n % 3)];
            if n % 2 == 0 {
                Log::open(&mut flash).append(record).unwrap();
            } else {
                block_on(async { AsyncLog::open(&mut flash).await.append(record).await }).unwrap();
            }
            appended.push(record);
            let blocking = records(&mut Log::open(&mut flash));
            let mut log = block_on(AsyncLog::open(&mut flash));
            let read = async_records(&mut log);
            assert_eq!(read, blocking, "after {n}");
            let newest: Vec<Record> = appended.iter().rev().take(read.len()).copied().collect();
            assert_eq!(read, newest, "after {n}");
        }
    }

    #[test]
    fn async_find_stops_at_the_newest_match() {
        let mut flash = Sim::<8>::new();
        for r in [Confirmed(id(1)), Requested(id(2)), Started(id(2)), Confirmed(id(2))] {
            Log::open(&mut flash).append(r).unwrap();
        }
        let mut log = block_on(AsyncLog::open(&mut flash));
        assert_eq!(block_on(log.find(|r| r.id() == id(2))), Some(Confirmed(id(2))));
        assert_eq!(block_on(log.find(|r| matches!(r, Requested(_)))), Some(Requested(id(2))));
        assert_eq!(block_on(log.find(|r| r.id() == id(3))), None);
    }

    /// As [`a_power_cut_anywhere_in_a_flip`], through the async log.
    #[test]
    fn a_power_cut_anywhere_in_an_async_flip() {
        let history = [Confirmed(id(1)), Requested(id(2)), Started(id(2)), Confirmed(id(2)), Requested(id(3)), Started(id(3)), Started(id(3)), Confirmed(id(3))];
        let full = log_of(&history);
        let before = records(&mut Log::open(full.clone()));
        for cut in 0..2 {
            let mut flash = full.clone().cut_after(cut);
            let result = block_on(async { AsyncLog::open(&mut flash).await.append(Requested(id(4))).await });
            assert_eq!(result, Err(PowerCut), "cut {cut}");
            let after = async_records(&mut block_on(AsyncLog::open(flash.powered())));
            assert_eq!(after[..], before[..after.len()], "cut {cut}");
            assert!(after.len() >= 4, "cut {cut}");
        }
    }
}
