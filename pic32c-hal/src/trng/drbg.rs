// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `HMAC_DRBG` with SHA-256, as NIST SP 800-90A Rev. 1 section 10.1.2 defines
//! it: instantiate, reseed and generate, without additional input (the
//! driver passes none).
//!
//! The driver instantiates one per call from fresh TRNG words and drops it
//! afterwards, so every output is backed by new entropy (what prediction
//! resistance asks for) and no state outlives a call. `reseed` is only for the
//! known-answer tests.

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// SHA-256's output length, the size of `K` and `V`.
const OUTLEN: usize = 32;

/// The most one generate call may return: 2¹⁹ bits (SP 800-90A table 2).
pub(super) const MAX_REQUEST: usize = 1 << 16;

pub(super) struct Drbg {
    k: [u8; OUTLEN],
    v: [u8; OUTLEN],
}

impl Drbg {
    /// `HMAC_DRBG_Instantiate_algorithm` (section 10.1.2.3).
    pub(super) fn instantiate(entropy: &[u8], nonce: &[u8], personalization: &[u8]) -> Self {
        let mut drbg = Self { k: [0x00; OUTLEN], v: [0x01; OUTLEN] };
        drbg.update(&[entropy, nonce, personalization]);
        drbg
    }

    /// `HMAC_DRBG_Reseed_algorithm` (section 10.1.2.4), without additional
    /// input.
    #[cfg(test)]
    pub(super) fn reseed(&mut self, entropy: &[u8]) {
        self.update(&[entropy]);
    }

    /// `HMAC_DRBG_Generate_algorithm` (section 10.1.2.5), without additional
    /// input. `out` holds at most [`MAX_REQUEST`] bytes.
    pub(super) fn generate(&mut self, out: &mut [u8]) {
        assert!(out.len() <= MAX_REQUEST, "an HMAC_DRBG request of more than 2^19 bits");
        let keyed = self.keyed();
        for chunk in out.chunks_mut(OUTLEN) {
            let v = keyed.clone().chain_update(self.v).finalize().into_bytes();
            self.v.copy_from_slice(&v);
            chunk.copy_from_slice(&self.v[..chunk.len()]);
        }
        self.update(&[]);
    }

    /// `HMAC_DRBG_Update` (section 10.1.2.2), `provided_data` the
    /// concatenation of `data`.
    fn update(&mut self, data: &[&[u8]]) {
        for (round, provided) in [(0x00_u8, true), (0x01, data.iter().any(|d| !d.is_empty()))] {
            if !provided {
                return;
            }
            let mut mac = self.keyed().chain_update(self.v).chain_update([round]);
            for d in data {
                mac.update(d);
            }
            self.k.copy_from_slice(&mac.finalize().into_bytes());
            let v = self.keyed().chain_update(self.v).finalize().into_bytes();
            self.v.copy_from_slice(&v);
        }
    }

    fn keyed(&self) -> HmacSha256 {
        HmacSha256::new_from_slice(&self.k).expect("HMAC takes a key of any length")
    }
}

impl Drop for Drbg {
    fn drop(&mut self) {
        // The working state is secret; don't leave it on the stack.
        for byte in self.k.iter_mut().chain(self.v.iter_mut()) {
            // SAFETY: a byte of `self`, valid and aligned.
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }

    /// RFC 6979 section A.1's worked example: `HMAC_DRBG` with SHA-256 drawing
    /// the nonce for ECDSA over K-163, instantiated with the private key as
    /// entropy and the reduced message hash as nonce, no personalization; its
    /// first 21 bytes are the first candidate `T`.
    #[test]
    fn rfc6979_a1() {
        let mut drbg = Drbg::instantiate(&hex("009A4D6792295A7F730FC3F2B49CBC0F62E862272F"), &hex("01795EDF0D54DB760F156D0DAC04C0322B3A204224"), &[]);
        let mut out = [0; 21];
        drbg.generate(&mut out);
        assert_eq!(out.as_slice(), hex("9305A46DE7FF8EB107194DEBD3FD48AA20D5E7656C"));
    }

    /// Within one call, output is the V chain: a shorter request is a prefix
    /// of a longer one from the same state.
    #[test]
    fn a_short_request_is_a_prefix() {
        let seed = [0x5A; 48];
        let mut long = [0; 100];
        Drbg::instantiate(&seed, &[1; 16], b"unit").generate(&mut long);
        for len in [0, 1, 31, 32, 33, 64, 99] {
            let mut short = vec![0; len];
            Drbg::instantiate(&seed, &[1; 16], b"unit").generate(&mut short);
            assert_eq!(short, long[..len], "{len}");
        }
    }

    /// Every input changes the output, and a reseed or a second generate
    /// moves the state on.
    #[test]
    fn inputs_and_state_matter() {
        let out = |entropy: &[u8], nonce: &[u8], personalization: &[u8]| {
            let mut out = [0; 32];
            Drbg::instantiate(entropy, nonce, personalization).generate(&mut out);
            out
        };
        let base = out(&[1; 32], &[2; 16], &[3; 16]);
        assert_ne!(base, out(&[9; 32], &[2; 16], &[3; 16]));
        assert_ne!(base, out(&[1; 32], &[9; 16], &[3; 16]));
        assert_ne!(base, out(&[1; 32], &[2; 16], &[9; 16]));

        let mut drbg = Drbg::instantiate(&[1; 32], &[2; 16], &[3; 16]);
        let (mut first, mut second) = ([0; 32], [0; 32]);
        drbg.generate(&mut first);
        drbg.generate(&mut second);
        assert_eq!(first, base);
        assert_ne!(first, second);

        let mut reseeded = Drbg::instantiate(&[1; 32], &[2; 16], &[3; 16]);
        reseeded.reseed(&[4; 32]);
        let mut third = [0; 32];
        reseeded.generate(&mut third);
        assert_ne!(third, base);
    }

    #[test]
    #[should_panic(expected = "2^19 bits")]
    fn an_oversized_request_panics() {
        let mut out = vec![0; MAX_REQUEST + 1];
        Drbg::instantiate(&[1; 32], &[], &[]).generate(&mut out);
    }

    /// A CAVP vector as hex: file, entropy, nonce, personalization string,
    /// reseed entropy, returned bits.
    type CavpVector = (&'static str, &'static str, &'static str, &'static str, &'static [&'static str], &'static str);

    /// One CAVP vector: the inputs, and what to do with them.
    #[derive(Debug, Default)]
    struct Vector {
        entropy: Vec<u8>,
        nonce: Vec<u8>,
        personalization: Vec<u8>,
        /// `EntropyInputReseed` (`pr_false`), or the two `EntropyInputPR` (`pr_true`).
        reseeds: Vec<Vec<u8>>,
        returned: Vec<u8>,
    }

    /// Runs a vector as CAVP's harness does: instantiate, then two generate
    /// calls, the second's output the answer. `pr_false` reseeds once after
    /// instantiating; `pr_true` reseeds before each generate.
    fn run(file: &str, v: &Vector) -> Vec<u8> {
        let mut drbg = Drbg::instantiate(&v.entropy, &v.nonce, &v.personalization);
        let mut out = vec![0; v.returned.len()];
        match file {
            "no_reseed" => {}
            "pr_false" | "pr_true" => drbg.reseed(&v.reseeds[0]),
            _ => unreachable!(),
        }
        drbg.generate(&mut out);
        if file == "pr_true" {
            drbg.reseed(&v.reseeds[1]);
        }
        drbg.generate(&mut out);
        out
    }

    /// The `[SHA-256]` vectors without additional input from one of NIST's
    /// CAVP `HMAC_DRBG.rsp` files (the case the driver uses: no additional
    /// input), any personalization string length.
    fn parse(rsp: &str) -> Vec<Vector> {
        let mut vectors = Vec::new();
        let mut section_ok = false;
        let mut sha256 = false;
        let mut current: Option<Vector> = None;
        for line in rsp.lines().map(str::trim) {
            if line.starts_with("[SHA-") {
                sha256 = line == "[SHA-256]";
                section_ok = sha256;
            } else if let Some(len) = line.strip_prefix("[AdditionalInputLen = ") {
                section_ok = sha256 && len == "0]";
            } else if line.starts_with('[') {
                // Another section parameter.
            } else if let Some((key, value)) = line.split_once(" = ").or_else(|| line.strip_suffix(" =").map(|k| (k, ""))) {
                if !section_ok || key == "COUNT" {
                    if section_ok {
                        current = Some(Vector::default());
                    }
                    continue;
                }
                let bytes = hex(value);
                match key {
                    "EntropyInput" => current.as_mut().unwrap().entropy = bytes,
                    "Nonce" => current.as_mut().unwrap().nonce = bytes,
                    "PersonalizationString" => current.as_mut().unwrap().personalization = bytes,
                    "EntropyInputReseed" | "EntropyInputPR" => current.as_mut().unwrap().reseeds.push(bytes),
                    "AdditionalInput" | "AdditionalInputReseed" => assert_eq!(bytes, []),
                    "ReturnedBits" => {
                        let mut v = current.take().unwrap();
                        v.returned = bytes;
                        vectors.push(v);
                    }
                    _ => {}
                }
            }
        }
        vectors
    }

    /// The first vector (`COUNT = 0`) of each `[SHA-256]` section without
    /// additional input, with and without a personalization string, in each
    /// of NIST's three CAVP `HMAC_DRBG.rsp` files (`drbgtestvectors.zip`),
    /// the returned bits the second generate's 1024. `cavp_all` runs all
    /// 360.
    const CAVP: &[CavpVector] = &[
        (
            "no_reseed",
            "ca851911349384bffe89de1cbdc46e6831e44d34a4fb935ee285dd14b71a7488",
            "659ba96c601dc69fc902940805ec0ca8",
            "",
            &[],
            "e528e9abf2dece54d47c7e75e5fe302149f817ea9fb4bee6f4199697d04d5b89d54fbb978a15b5c443c9ec21036d2460b6f73ebad0dc2aba6e624abf07745bc107694bb7547bb0995f70de25d6b29e2d3011bb19d27676c07162c8b5ccde0668961df86803482cb37ed6d5c0bb8d50cf1f50d476aa0458bdaba806f48be9dcb8",
        ),
        (
            "no_reseed",
            "5cacc68165a2e2ee20812f35ec73a79dbf30fd475476ac0c44fc6174cdac2b55",
            "6f885496c1e63af620becd9e71ecb824",
            "e72dd8590d4ed5295515c35ed6199e9d211b8f069b3058caa6670b96ef1208d0",
            &[],
            "f1012cf543f94533df27fedfbf58e5b79a3dc517a9c402bdbfc9a0c0f721f9d53faf4aafdc4b8f7a1b580fcaa52338d4bd95f58966a243cdcd3f446ed4bc546d9f607b190dd69954450d16cd0e2d6437067d8b44d19a6af7a7cfa8794e5fbd728e8fb2f2e8db5dd4ff1aa275f35886098e80ff844886060da8b1e7137846b23b",
        ),
        (
            "pr_false",
            "06032cd5eed33f39265f49ecb142c511da9aff2af71203bffaf34a9ca5bd9c0d",
            "0e66f71edc43e42a45ad3c6fc6cdc4df",
            "",
            &["01920a4e669ed3a85ae8a33b35a74ad7fb2a6bb4cf395ce00334a9c9a5a5d552"],
            "76fc79fe9b50beccc991a11b5635783a83536add03c157fb30645e611c2898bb2b1bc215000209208cd506cb28da2a51bdb03826aaf2bd2335d576d519160842e7158ad0949d1a9ec3e66ea1b1a064b005de914eac2e9d4f2d72a8616a80225422918250ff66a41bd2f864a6a38cc5b6499dc43f7f2bd09e1e0f8f5885935124",
        ),
        (
            "pr_false",
            "fa0ee1fe39c7c390aa94159d0de97564342b591777f3e5f6a4ba2aea342ec840",
            "dd0820655cb2ffdb0da9e9310a67c9e5",
            "f2e58fe60a3afc59dad37595415ffd318ccf69d67780f6fa0797dc9aa43e144c",
            &["e0629b6d7975ddfa96a399648740e60f1f9557dc58b3d7415f9ba9d4dbb501f6"],
            "f92d4cf99a535b20222a52a68db04c5af6f5ffc7b66a473a37a256bd8d298f9b4aa4af7e8d181e02367903f93bdb744c6c2f3f3472626b40ce9bd6a70e7b8f93992a16a76fab6b5f162568e08ee6c3e804aefd952ddd3acb791c50f2ad69e9a04028a06a9c01d3a62aca2aaf6efe69ed97a016213a2dd642b4886764072d9cbe",
        ),
        (
            "pr_true",
            "9969e54b4703ff31785b879a7e5c0eae0d3e309559e9fe96b0676d49d591ea4d",
            "07d20d46d064757d3023cac2376127ab",
            "",
            &["c60f2999100f738c10f74792676a3fc4a262d13721798046e29a295181569f54", "c11d4524c9071bd3096015fcf7bc24a607f22fa065c937658a2a77a8699089f4"],
            "abc015856094803a938dffd20da94843870ef935b82cfec17706b8f551b8385044235dd44b599f94b39be78dd476e0cf11309c995a7334e0a78b37bc9586235086fa3b637ba91cf8fb65efa22a589c137531aa7b2d4e2607aac27292b01c698e6e01ae679eb87c01a89c7422d4372d6d754ababb4bf896fcb1cd09d692d0283f",
        ),
        (
            "pr_true",
            "f7b90c797a4a376cdd9f5c435f5985e77f36ec1df1145a12072cbb2a0da378fc",
            "d95202986d45896e9f4a65f2f353fa35",
            "61535c5c045e784267fd0d85f2861778fa53c8e8586af67cf5c9f21a28ebb656",
            &["130ab64f41a5d49d6a241e0260b4bb8a46a16c6ac9e234c84b5b26cdb518d459", "f7670e817ac061ac60439be60982492000dc5da8bc6636bdac8b1cab03198dfd"],
            "8df4e349f9ea43cc509ecb2b1124358cda2de1f5cc9315edca63610a413478d68b8bb49c2814c82ce571f6e0a6780fa21c4b570610ee0c04d3edb92124f580f962d741330200c19885ca716502223247b728d66fbbeb7c6cc25cfe9866b1450b346227c7663074c8b15d189f1c6edba172a53c733d67c1c69bd7aca7e62013cd",
        ),
    ];

    #[test]
    fn cavp_subset() {
        for (file, entropy, nonce, personalization, reseeds, returned) in CAVP {
            let v = Vector {
                entropy: hex(entropy),
                nonce: hex(nonce),
                personalization: hex(personalization),
                reseeds: reseeds.iter().map(|r| hex(r)).collect(),
                returned: hex(returned),
            };
            assert_eq!(run(file, &v), v.returned, "{file}, personalization {}", personalization.len() * 4);
        }
    }

    /// Every such vector in NIST's CAVP DRBG test vectors
    /// (<https://csrc.nist.gov/projects/cryptographic-algorithm-validation-program/random-number-generators>,
    /// `drbgtestvectors.zip`): set `CAVP_HMAC_DRBG_DIR` to the folder holding
    /// its three sets unzipped (`drbgvectors_no_reseed/`, `drbgvectors_pr_false/`,
    /// `drbgvectors_pr_true/`) and run with `--ignored`.
    #[test]
    #[ignore = "needs NIST's CAVP vectors: set CAVP_HMAC_DRBG_DIR"]
    fn cavp_all() {
        let dir = std::env::var("CAVP_HMAC_DRBG_DIR").expect("CAVP_HMAC_DRBG_DIR");
        for file in ["no_reseed", "pr_false", "pr_true"] {
            let rsp = std::fs::read_to_string(format!("{dir}/drbgvectors_{file}/HMAC_DRBG.rsp")).unwrap();
            let vectors = parse(&rsp);
            assert!(!vectors.is_empty(), "{file}: no vectors");
            for (i, v) in vectors.iter().enumerate() {
                assert_eq!(run(file, v), v.returned, "{file} vector {i}");
            }
            println!("{file}: {} vectors passed", vectors.len());
        }
    }
}
