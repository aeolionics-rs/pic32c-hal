// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Driver modes, as embassy's HALs have them: a driver generic over `M`
//! waits on its peripheral's flags in [`Blocking`] mode and on its
//! interrupt in [`Async`] mode.

/// Blocking mode: the driver waits on the flags.
pub struct Blocking;
/// Async mode: the driver waits on the interrupts.
pub struct Async;
