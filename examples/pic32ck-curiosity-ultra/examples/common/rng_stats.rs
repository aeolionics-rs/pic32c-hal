// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Sanity statistics over 2 Mbit (65,536 words) of random output, as `trng`
//! and `crypto_rng` check it: the share of ones, a chi-square over byte
//! values (255 degrees of freedom), the longest run of equal bits, and
//! repeated consecutive words. These catch a stuck or biased source, not a
//! subtly weak one.

use core::fmt::Write;

/// How many words a check takes.
pub const WORDS: u32 = 65_536;

/// Running statistics over the words as they come.
pub struct Stats {
    ones: u64,
    bytes: [u32; 256],
    run: u32,
    longest: u32,
    last_bit: bool,
    last_word: Option<u32>,
    repeats: u32,
}

impl Stats {
    pub const fn new() -> Self {
        Self {
            ones: 0,
            bytes: [0; 256],
            run: 0,
            longest: 0,
            last_bit: false,
            last_word: None,
            repeats: 0,
        }
    }

    pub fn add(&mut self, word: u32) {
        self.ones += u64::from(word.count_ones());
        for byte in word.to_le_bytes() {
            self.bytes[usize::from(byte)] += 1;
        }
        for i in 0..32 {
            let bit = word >> i & 1 != 0;
            if bit == self.last_bit {
                self.run += 1;
            } else {
                self.run = 1;
                self.last_bit = bit;
            }
            self.longest = self.longest.max(self.run);
        }
        if self.last_word == Some(word) {
            self.repeats += 1;
        }
        self.last_word = Some(word);
    }

    /// Chi-square of the byte counts against a uniform spread.
    fn chi_square(&self) -> f32 {
        #[expect(clippy::cast_precision_loss, reason = "counts up to 2^18, exact in f32")]
        let expected = (WORDS * 4) as f32 / 256.0;
        self.bytes
            .iter()
            .map(|&n| {
                #[expect(clippy::cast_precision_loss, reason = "as above")]
                let d = n as f32 - expected;
                d * d / expected
            })
            .sum()
    }

    /// Reports the statistics after [`WORDS`] words, and returns how many
    /// checks failed.
    pub fn report(&self, log: &mut impl Write) -> u32 {
        let bits = u64::from(WORDS) * 32;
        #[expect(clippy::cast_precision_loss, reason = "2^21 bits, exact in f32")]
        let share = self.ones as f32 / bits as f32;
        let chi = self.chi_square();
        // Bounds well past chance for a good source: the share's sd is 0.00035,
        // the chi-square's 99.9th percentile is ~330, a run past 40 in 2^21
        // bits is under one in a million, and a repeat one in 2^32 per word.
        let checks = [
            ("share of ones", (0.497..0.503).contains(&share)),
            ("byte chi-square", chi < 330.0),
            ("longest run", self.longest <= 40),
            ("repeated words", self.repeats == 0),
        ];
        let _ = writeln!(log, "2 Mbit: ones {share:.5}, chi-square {chi:.1}, longest run {}, repeated words {}", self.longest, self.repeats);
        let mut failures = 0;
        for (name, good) in checks {
            if !good {
                let _ = writeln!(log, "{name}: FAILED");
                failures += 1;
            }
        }
        failures
    }
}
