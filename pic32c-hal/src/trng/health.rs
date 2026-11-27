// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Continuous health tests on the TRNG's words, after NIST SP 800-90B section
//! 4.4: the repetition count test and the adaptive proportion test, with a
//! word (32 bits) as the sample.
//!
//! The cutoffs follow the standard's formulas for a false-positive rate of
//! α = 2⁻⁴⁰ per test, from an **assumed** min-entropy of [`ASSUMED_BITS`] per
//! word: the datasheet gives no rating, so this is a deliberately low guess
//! (a quarter of a bit per bit), which keeps false alarms negligible while a
//! stuck or nearly stuck source still fails within a few words. An SP 800-90B
//! entropy assessment of the part would replace it with a measured value.

use core::fmt;

/// The min-entropy assumed per 32-bit word, in bits.
pub(super) const ASSUMED_BITS: u32 = 8;

/// The repetition count test's cutoff: `1 + ⌈40 / H⌉` identical words in a
/// row fail (SP 800-90B section 4.4.1).
pub(super) const RCT_CUTOFF: u32 = 1 + 40_u32.div_ceil(ASSUMED_BITS);

/// The adaptive proportion test's window, for non-binary samples (SP 800-90B
/// section 4.4.2).
pub(super) const APT_WINDOW: u32 = 512;

/// The adaptive proportion test's cutoff: `1 + CRITBINOM(512, 2⁻⁸, 1 − 2⁻⁴⁰)`
/// occurrences of a window's first word in the window fail. Computed exactly
/// (rational arithmetic) offline: CRITBINOM is 18.
pub(super) const APT_CUTOFF: u32 = 19;

/// How many words the start-up test runs both tests over before the first
/// word is used (SP 800-90B section 4.3).
#[cfg(feature = "embassy-crypto-rng")]
pub(super) const STARTUP_WORDS: u32 = 1024;

/// Which test failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fault {
    /// The same word [`RCT_CUTOFF`] times in a row.
    Repetition,
    /// A window's first word [`APT_CUTOFF`] times in its window.
    Proportion,
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Repetition => "repetition count test",
            Self::Proportion => "adaptive proportion test",
        })
    }
}

/// Both tests' state.
#[derive(Debug)]
pub(super) struct Health {
    /// The repetition count test's last word, and how many times in a row.
    last: u32,
    run: u32,
    /// The adaptive proportion test's window: its first word, how often it
    /// has occurred, and how many words the window has seen.
    first: u32,
    occurrences: u32,
    seen: u32,
}

impl Health {
    pub(super) const fn new() -> Self {
        Self {
            last: 0,
            run: 0,
            first: 0,
            occurrences: 0,
            seen: 0,
        }
    }

    /// Checks the next word.
    pub(super) fn check(&mut self, word: u32) -> Result<(), Fault> {
        if self.run > 0 && word == self.last {
            self.run += 1;
            if self.run >= RCT_CUTOFF {
                return Err(Fault::Repetition);
            }
        } else {
            self.last = word;
            self.run = 1;
        }

        if self.seen == 0 {
            self.first = word;
            self.occurrences = 1;
        } else if word == self.first {
            self.occurrences += 1;
            if self.occurrences >= APT_CUTOFF {
                return Err(Fault::Proportion);
            }
        }
        self.seen += 1;
        if self.seen == APT_WINDOW {
            self.seen = 0;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A sequence with no repeats: a Weyl sequence through all 2³² words.
    fn distinct(n: u32) -> impl Iterator<Item = u32> {
        (0..n).map(|i| i.wrapping_mul(0x9E37_79B9))
    }

    #[test]
    fn cutoffs() {
        assert_eq!(RCT_CUTOFF, 6);
        assert_eq!(APT_CUTOFF, 19);
    }

    #[test]
    fn distinct_words_pass() {
        let mut health = Health::new();
        for word in distinct(100_000) {
            assert_eq!(health.check(word), Ok(()));
        }
    }

    #[test]
    fn a_stuck_word_fails_the_repetition_test() {
        let mut health = Health::new();
        for _ in 1..RCT_CUTOFF {
            assert_eq!(health.check(0xDEAD_BEEF), Ok(()));
        }
        assert_eq!(health.check(0xDEAD_BEEF), Err(Fault::Repetition));
    }

    #[test]
    fn a_run_one_short_of_the_cutoff_passes() {
        // A new word for each run, so the proportion test has nothing to see.
        let mut health = Health::new();
        for word in distinct(1000) {
            for _ in 1..RCT_CUTOFF {
                assert_eq!(health.check(word), Ok(()));
            }
        }
    }

    #[test]
    fn two_alternating_words_fail_the_proportion_test() {
        let mut health = Health::new();
        let mut result = Ok(());
        let mut words = 0;
        for word in [1, 2].into_iter().cycle() {
            words += 1;
            result = health.check(word);
            if result.is_err() {
                break;
            }
        }
        assert_eq!(result, Err(Fault::Proportion));
        // The first word, then every second one: its nineteenth occurrence.
        assert_eq!(words, 2 * (APT_CUTOFF - 1) + 1);
    }

    #[test]
    fn the_proportion_test_starts_a_new_window() {
        let mut health = Health::new();
        // Its first word occurs APT_CUTOFF - 1 times in each window, spread
        // out: passes, window after window.
        for _ in 0..10 {
            let mut others = distinct(APT_WINDOW).map(|w| w | 1);
            for i in 0..APT_WINDOW {
                let word = if i % 16 == 0 && i / 16 < APT_CUTOFF - 1 { 0 } else { others.next().unwrap_or(1) };
                assert_eq!(health.check(word), Ok(()), "word {i}");
            }
        }
    }
}
