// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Which image to boot, from the boot log, the images there are and why the part reset.
//!
//! The boot log is append-only: each [`Record`] names an image by its [`ImageId`] and says what
//! happened to it. The updater appends [`Record::Requested`] after writing a new image; the
//! bootloader appends [`Record::Started`] each time it boots one on trial, and
//! [`Record::Failed`] when a trial ended without a confirmation; the application appends
//! [`Record::Confirmed`] once it has shown its image works (or to go back to an older one).
//!
//! [`Policy::decide`] reads the log newest first, with the images the bootloader found and
//! checked good, and gives the record to append, if any, and the image to boot. The rules:
//!
//! 1. Only an image in the list, at or above the policy's minimum version, is ever booted.
//! 2. Log records for an image that is not in the list, or whose version is below the minimum
//!    version, are ignored.
//! 3. If no bootable image is found from the log, then the first image with a valid version
//!    whose trial has not failed is booted.
//! 4. The newest record determines which image to boot.
//! 5. A `Confirmed` log record boots its image.
//! 6. A `Failed` trial reverts to the previous `Confirmed` image, falling back to Rule 3.
//! 7. A trial may be `Requested` and each attempt to boot its image is recorded as `Started`.
//! 8. A `Started` trial that resets for a cause attributable to the application (such as crash or
//!    watchdog trigger) before it is `Confirmed` is recorded as `Failed`.
//! 9. A `Started` trial that resets for a cause not attributable to the application (such as
//!    by a power failure) may be re-attempted as determined by the Policy. If the retry limit is
//!    reached, the trial is recorded as `Failed`.
//!
//! If the list has the same image twice, the first in the list boots.
//! If no bootable image is found then the bootloader waits for a new image to be flashed by
//! external means.

/// An image's identity, typically derived from a content hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImageId(pub [u8; 16]);

/// An image the bootloader has found and validated, and where it is located: `at` is the caller's
/// identifier (a flash panel, an address), and [`Policy::decide`] never looks at it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Image<L> {
    pub id: ImageId,
    /// The image's version, for refusing old ones ([`Policy::min_version`]).
    pub version: u32,
    /// The location where the image is stored.
    pub at: L,
}

/// An entry in the boot log.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Record {
    /// The updater wrote this image and asks for it to be tried out.
    Requested(ImageId),
    /// The bootloader booted this image on trial.
    Started(ImageId),
    /// This image's trial ended without a confirmation.
    Failed(ImageId),
    /// The application confirmed this image: it runs from now on.
    Confirmed(ImageId),
}

impl Record {
    #[must_use]
    pub const fn id(self) -> ImageId {
        match self {
            Record::Requested(id) | Record::Started(id) | Record::Failed(id) | Record::Confirmed(id) => id,
        }
    }
}

/// Why the device reset and entered the bootloader.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetCause {
    /// Power came on.
    PowerOn,
    /// The supply dipped.
    BrownOut,
    /// The reset pin was activated.
    Pin,
    /// The watchdog fired.
    Watchdog,
    /// The software asked for it (or the core locked up).
    Software,
}

impl ResetCause {
    /// Whether this cause should end a trial.
    ///
    /// Power-related resets, such as initial power on or brownout, are not held against the
    /// trial image. Watchdog timeouts or crashes are.
    #[must_use]
    pub const fn ends_trial(self) -> bool {
        !matches!(self, ResetCause::PowerOn | ResetCause::BrownOut)
    }
}

/// The policy this bootloader uses to determine what can be booted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Policy {
    /// The lowest image version that is allowed to boot; may be used to mitigate rollbacks to a
    /// version with a known vulnerability.
    pub min_version: u32,
    /// How many times a trial may start, power failures included, before it is deemed to have
    /// failed.
    pub max_starts: u32,
}

/// The Policy's decision on which Image to boot, if any, and the record to append to the log, if
/// any.
#[derive(Debug, PartialEq, Eq)]
pub struct Decision<'a, L> {
    /// The image to boot, one of those passed in, or `None` to indicate there is nothing to run.
    pub boot: Option<&'a Image<L>>,
    /// A record to append to the log before booting, if any.
    pub append: Option<Record>,
}

impl Policy {
    /// What to do after a reset for `cause`, given the boot `log` (newest record first) and a list
    /// of candidate `images`, in the caller's order of preference. See the module's rules.
    pub fn decide<L>(self, cause: ResetCause, log: impl IntoIterator<Item = Record>, images: &[Image<L>]) -> Decision<'_, L> {
        let usable = |id: ImageId| images.iter().find(|image| image.id == id && image.version >= self.min_version);
        let mut append = None; // By default, nothing gets appended.

        // Rule 1: If no images are supplied then there is nothing to boot.
        if images.is_empty() {
            return Decision { boot: None, append };
        }

        let mut failed = None;
        let mut attempts = 0;
        for record in log {
            // Verify this record refers to a usable image.
            let image_id = record.id();
            if let Some(image) = usable(image_id) {
                match record {
                    // Boot a confirmed image.
                    Record::Confirmed(_) => return Decision { boot: Some(image), append },

                    // If the trial failed, ignore previous attempts to start it.
                    _ if failed.is_some() => {}

                    // Remember that the trial failed so it is not attempted again.
                    Record::Failed(_) => failed = Some(image_id),

                    // A trial was started and has not yet failed.
                    Record::Started(_) => {
                        // If the reset end the trail then record the failure and continue
                        // looking for an image to boot.
                        if cause.ends_trial() {
                            failed = Some(image_id); // for us
                            append = Some(Record::Failed(image_id)); // for the next guy
                        } else {
                            attempts += 1;
                        }
                    }

                    // A request to trial new firmware.
                    Record::Requested(_) => {
                        // If we hit the limit on starts then treat this as a failure.
                        if attempts >= self.max_starts {
                            failed = Some(image_id); // for us
                            append = Some(Record::Failed(image_id)); // for the next guy
                            continue;
                        }

                        // Give the application another go.
                        append = Some(Record::Started(image_id));
                        return Decision { boot: Some(image), append };
                    }
                }
            }
        }

        // Rule 2: No entry found in the log, so boot the first image that passes the version check.
        // Never the image whose trial failed, or the failed-trial loop comes back.
        let boot = images.iter().find(|i| i.version >= self.min_version && failed != Some(i.id));
        Decision { boot, append }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use Record::{Confirmed, Failed, Requested, Started};
    use ResetCause::{BrownOut, Pin, PowerOn, Software, Watchdog};
    use std::{format, vec, vec::Vec};

    const POLICY: Policy = Policy { min_version: 1, max_starts: 3 };

    fn id(n: u8) -> ImageId {
        ImageId([n; 16])
    }

    /// Image `n`, version 1, in panel `at`.
    fn image(n: u8, at: char) -> Image<char> {
        Image { id: id(n), version: 1, at }
    }

    /// The panel booted and the record appended, as a list.
    fn run(policy: Policy, cause: ResetCause, log: &[Record], images: &[Image<char>]) -> (Option<char>, Vec<Record>) {
        let d = policy.decide(cause, log.iter().copied(), images);
        (d.boot.map(|i| i.at), d.append.into_iter().collect())
    }

    #[test]
    fn runs_the_confirmed_image() {
        let images = [image(1, 'A')];
        for cause in [PowerOn, Watchdog, Software] {
            assert_eq!(run(POLICY, cause, &[Confirmed(id(1))], &images), (Some('A'), vec![]), "{cause:?}");
        }
    }

    #[test]
    fn starts_a_requested_trial() {
        // Running image 1 in A; image 2 written to B, then a software reset to start it.
        let log = [Requested(id(2)), Confirmed(id(1))];
        let images = [image(1, 'A'), image(2, 'B')];
        assert_eq!(run(POLICY, Software, &log, &images), (Some('B'), vec![Started(id(2))]));
    }

    #[test]
    fn goes_back_when_a_trial_ends_unconfirmed() {
        let log = [Started(id(2)), Requested(id(2)), Confirmed(id(1))];
        let images = [image(1, 'A'), image(2, 'B')];
        for cause in [Pin, Watchdog, Software] {
            assert_eq!(run(POLICY, cause, &log, &images), (Some('A'), vec![Failed(id(2))]), "{cause:?}");
        }
    }

    #[test]
    fn a_power_failure_restarts_a_trial_up_to_the_limit() {
        let images = [image(1, 'A'), image(2, 'B')];
        let mut log = vec![Started(id(2)), Requested(id(2)), Confirmed(id(1))];
        for cause in [PowerOn, BrownOut] {
            assert_eq!(run(POLICY, cause, &log, &images), (Some('B'), vec![Started(id(2))]), "{cause:?}");
        }
        log.insert(0, Started(id(2)));
        assert_eq!(run(POLICY, PowerOn, &log, &images), (Some('B'), vec![Started(id(2))]));
        // Three starts: the third power failure ends it.
        log.insert(0, Started(id(2)));
        assert_eq!(run(POLICY, PowerOn, &log, &images), (Some('A'), vec![Failed(id(2))]));
    }

    #[test]
    fn a_confirmed_update_stays() {
        let log = [Confirmed(id(2)), Started(id(2)), Requested(id(2)), Confirmed(id(1))];
        let images = [image(1, 'A'), image(2, 'B')];
        assert_eq!(run(POLICY, Watchdog, &log, &images), (Some('B'), vec![]));
    }

    #[test]
    fn going_back_to_an_older_confirmed_image() {
        // The application confirms image 1 again, over the newer image 2.
        let log = [Confirmed(id(1)), Confirmed(id(2)), Confirmed(id(1))];
        let images = [image(1, 'A'), image(2, 'B')];
        assert_eq!(run(POLICY, PowerOn, &log, &images), (Some('A'), vec![]));
    }

    #[test]
    fn a_failed_trial_stays_failed() {
        let log = [Failed(id(2)), Started(id(2)), Requested(id(2)), Confirmed(id(1))];
        let images = [image(1, 'A'), image(2, 'B')];
        assert_eq!(run(POLICY, PowerOn, &log, &images), (Some('A'), vec![]));
        // Requested again, it's a new trial.
        let mut again = log.to_vec();
        again.insert(0, Requested(id(2)));
        assert_eq!(run(POLICY, Software, &again, &images), (Some('B'), vec![Started(id(2))]));
    }

    #[test]
    fn skips_an_image_thats_gone() {
        // B was written over, and the request for its new image hasn't been appended yet.
        let log = [Confirmed(id(2)), Confirmed(id(1))];
        let images = [image(1, 'A'), image(3, 'B')];
        assert_eq!(run(POLICY, PowerOn, &log, &images), (Some('A'), vec![]));
        // A trial whose image is gone isn't failed: there's nothing to fail.
        let log = [Started(id(3)), Requested(id(3)), Confirmed(id(1))];
        let images = [image(1, 'A')];
        assert_eq!(run(POLICY, Watchdog, &log, &images), (Some('A'), vec![]));
    }

    #[test]
    fn refuses_old_versions() {
        let policy = Policy { min_version: 2, ..POLICY };
        let old = Image { id: id(1), version: 1, at: 'A' };
        let new = Image { id: id(2), version: 2, at: 'B' };
        assert_eq!(run(policy, PowerOn, &[Confirmed(id(1))], &[old]), (None, vec![]));
        assert_eq!(run(policy, PowerOn, &[Confirmed(id(1)), Confirmed(id(2))], &[old, new]), (Some('B'), vec![]));
        assert_eq!(run(policy, PowerOn, &[], &[old]), (None, vec![]));
    }

    #[test]
    fn a_first_image_is_a_trial() {
        let images = [image(1, 'A'), image(2, 'B')];
        assert_eq!(run(POLICY, PowerOn, &[], &images), (Some('A'), vec![]));
        assert_eq!(run(POLICY, PowerOn, &[], &[]), (None, vec![]));
    }

    #[test]
    fn nothing_to_run() {
        // The only image failed its trial, with nothing confirmed behind it: it doesn't get to
        // be the fallback.
        let images = [image(1, 'A')];
        assert_eq!(run(POLICY, Watchdog, &[Started(id(1)), Requested(id(1))], &images), (None, vec![Failed(id(1))]));
        assert_eq!(run(POLICY, PowerOn, &[Failed(id(1)), Started(id(1)), Requested(id(1))], &images), (None, vec![]));
        // With another image there, that one runs instead.
        let images = [image(1, 'A'), image(2, 'B')];
        assert_eq!(run(POLICY, Watchdog, &[Started(id(1)), Requested(id(1))], &images), (Some('B'), vec![Failed(id(1))]));
    }

    #[test]
    fn fallback_to_factory_if_nothing_to_run() {
        let images = [image(1, 'A'), image(2, 'B')];
        assert_eq!(run(POLICY, PowerOn, &[Confirmed(id(3))], &images), (Some('A'), vec![]));
    }

    #[test]
    fn the_first_of_two_copies_boots() {
        let images = [image(1, 'B'), image(1, 'A')];
        assert_eq!(run(POLICY, PowerOn, &[Confirmed(id(1))], &images), (Some('B'), vec![]));
    }

    #[test]
    fn the_reference_is_into_the_list() {
        let images = [image(1, 'A'), image(1, 'A')];
        let d = POLICY.decide(PowerOn, [Confirmed(id(1))], &images);
        assert!(core::ptr::eq(d.boot.unwrap(), &raw const images[0]));
    }

    /// An update's life, as a part would live it: the bootloader appends what each decision
    /// says, and the application confirms or doesn't.
    #[test]
    fn an_update_end_to_end() {
        let mut log: Vec<Record> = Vec::new();
        let mut images = vec![image(1, 'A')];
        let boot = |log: &mut Vec<Record>, images: &[Image<char>], cause| {
            let d = POLICY.decide(cause, log.iter().rev().copied(), images);
            log.extend(d.append);
            d.boot.map(|i| i.at)
        };

        // As it left the programmer, then confirmed.
        assert_eq!(boot(&mut log, &images, PowerOn), Some('A'));
        log.push(Confirmed(id(1)));
        // Image 2 written to B and requested; it hangs, and the watchdog resets the part.
        images.push(image(2, 'B'));
        log.push(Requested(id(2)));
        assert_eq!(boot(&mut log, &images, Software), Some('B'));
        assert_eq!(boot(&mut log, &images, Watchdog), Some('A'));
        assert_eq!(boot(&mut log, &images, Watchdog), Some('A'));
        // Image 3 written over B; the power fails once during its trial, then it confirms.
        images[1] = image(3, 'B');
        log.push(Requested(id(3)));
        assert_eq!(boot(&mut log, &images, Software), Some('B'));
        assert_eq!(boot(&mut log, &images, PowerOn), Some('B'));
        log.push(Confirmed(id(3)));
        assert_eq!(boot(&mut log, &images, Pin), Some('B'));
        // Back to image 1 by choice.
        log.push(Confirmed(id(1)));
        assert_eq!(boot(&mut log, &images, Software), Some('A'));
        assert_eq!(log.iter().filter(|r| matches!(r, Failed(_))).count(), 1);
    }

    /// The rules written another way, over the log's records for usable images as a slice.
    fn oracle(policy: Policy, cause: ResetCause, log: &[Record], images: &[Image<char>]) -> (Option<usize>, Vec<Record>) {
        let usable = |id: ImageId| images.iter().position(|i| i.id == id && i.version >= policy.min_version);
        let recs: Vec<Record> = log.iter().copied().filter(|r| usable(r.id()).is_some()).collect();
        let confirmed_in = |rs: &[Record]| rs.iter().find_map(|r| if let Confirmed(id) = r { usable(*id) } else { None });
        let fallback = |failed: Option<ImageId>| images.iter().position(|i| i.version >= policy.min_version && Some(i.id) != failed);
        let after = |from: usize, failed: Option<ImageId>| recs.get(from..).and_then(confirmed_in).or_else(|| fallback(failed));

        // A Confirmed ahead of any trial record boots.
        match recs.iter().position(|r| !matches!(r, Confirmed(_))) {
            None => return (after(0, None), vec![]),
            Some(t) if t > 0 => return (confirmed_in(&recs[..t]), vec![]),
            Some(_) => {}
        }
        match recs[0] {
            Failed(x) => (after(1, Some(x)), vec![]),
            Started(x) if cause.ends_trial() => (after(1, Some(x)), vec![Failed(x)]),
            // A request, after any starts cut short by power failures.
            _ => {
                let starts = recs.iter().take_while(|r| matches!(r, Started(_))).count();
                match recs.get(starts) {
                    Some(Requested(y)) if u32::try_from(starts).unwrap() < policy.max_starts => (usable(*y), vec![Started(*y)]),
                    Some(Requested(y)) => (after(starts + 1, Some(*y)), vec![Failed(*y)]),
                    Some(Failed(z)) => (after(starts + 1, Some(*z)), vec![]),
                    Some(Confirmed(_)) => (confirmed_in(&recs[starts..]), vec![]),
                    Some(Started(_)) => unreachable!("past the starts"),
                    None => (fallback(None), vec![]),
                }
            }
        }
    }

    /// Every log of up to three records over three images, against every list of up to two
    /// images (two versions each), every reset cause and two policies: decide agrees with the
    /// oracle, and keeps its promises.
    #[test]
    fn every_short_log() {
        let records: Vec<Record> = (1..=3).flat_map(|n| [Requested(id(n)), Started(id(n)), Failed(id(n)), Confirmed(id(n))]).collect();
        let mut logs: Vec<Vec<Record>> = vec![vec![]];
        for _ in 0..3 {
            let longer: Vec<Vec<Record>> = logs.iter().filter(|l| l.len() == logs.last().unwrap().len()).flat_map(|l| records.iter().map(move |r| [l.clone(), vec![*r]].concat())).collect();
            logs.extend(longer);
        }
        let one: Vec<Image<char>> = (1..=3).flat_map(|n| [1, 2].map(|version| Image { id: id(n), version, at: char::from(b'0' + n) })).collect();
        let mut lists: Vec<Vec<Image<char>>> = vec![vec![]];
        lists.extend(one.iter().map(|i| vec![*i]));
        lists.extend(one.iter().flat_map(|a| one.iter().map(move |b| vec![*a, *b])));

        let mut cases = 0;
        for policy in [Policy { min_version: 1, max_starts: 2 }, Policy { min_version: 2, max_starts: 1 }] {
            for cause in [PowerOn, BrownOut, Pin, Watchdog, Software] {
                for log in &logs {
                    for images in &lists {
                        let d = policy.decide(cause, log.iter().copied(), images);
                        let index = d.boot.map(|b| images.iter().position(|i| core::ptr::eq(i, b)).expect("a reference into the list"));
                        let append: Vec<Record> = d.append.into_iter().collect();
                        let ctx = format!("{policy:?} {cause:?} {log:?} {images:?}");
                        assert_eq!((index, append.clone()), oracle(policy, cause, log, images), "{ctx}");
                        let usable_confirmed = log.iter().any(|r| matches!(r, Confirmed(c) if images.iter().any(|i| i.id == *c && i.version >= policy.min_version)));
                        if let Some(b) = d.boot {
                            // Only a usable image boots: confirmed, a trial with its Started, or
                            // the fallback when nothing confirmed can run.
                            assert!(b.version >= policy.min_version, "{ctx}");
                            assert!(log.contains(&Confirmed(b.id)) || append.contains(&Started(b.id)) || !usable_confirmed, "{ctx}");
                            // The fallback is never the image whose trial failed just now (a
                            // confirmed one can be: confirmed means known good).
                            if !log.contains(&Confirmed(b.id)) {
                                assert!(!append.contains(&Failed(b.id)), "{ctx}");
                            }
                        }
                        // A Failed is only ever appended once a trial had started.
                        if append.iter().any(|r| matches!(r, Failed(_))) {
                            assert!(log.iter().any(|r| matches!(r, Started(_))), "{ctx}");
                        }
                        cases += 1;
                    }
                }
            }
        }
        assert_eq!(cases, 2 * 5 * (1 + 12 + 144 + 1728) * (1 + 6 + 36));
    }
}
