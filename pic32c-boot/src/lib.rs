// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A bootloader for Microchip's PIC32C (spike). So far only its decision: which image to run,
//! and what to append to the boot log first ([`Policy::decide`]). It's a pure function of the
//! log, the images the bootloader found and checked, and why the part reset, tested on the host,
//! so that the part with the rules has no flash, registers or jumps in it. Reading and appending
//! the log, finding and checking the images, swapping the panels and jumping come later, around
//! it.
//!
//! The scheme, on a part with two program flash panels (the PIC32CK):
//!
//! - The application is always linked to run from the start of program flash. The bootloader
//!   maps the panel it chose there (the PIC32CK's `PFSWAP`, set afresh at every boot), so
//!   updating never copies an image: the updater writes the new one into the other panel, which
//!   the part can do while running from this one.
//! - What happened to each image is in an append-only boot log of records, each written once
//!   (flash with ECC takes one write per block): requested by the updater, started and failed by
//!   the bootloader, confirmed by the application. The newest record that still applies decides.
//! - A new image runs on trial. A reset other than a power failure before the application
//!   confirms it, or too many power failures, and the bootloader goes back to the newest
//!   confirmed image. So an update that doesn't come up well enough to confirm itself undoes
//!   itself, and going back to an older confirmed image on purpose is one more record.
#![no_std]

mod decide;

pub use decide::{Decision, Image, ImageId, Policy, Record, ResetCause};
