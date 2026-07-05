// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Runnable robustness proofs: exhaustive over tiny inputs and structured
//! random over the interesting shapes, asserting the security-critical parsers
//! never panic and never violate their invariants. A panic or a broken
//! invariant fails the test. The matching `kani_proofs` module proves the same
//! properties over bounded inputs with a model checker.

use alloc::vec;
use alloc::vec::Vec;

use crate::vfs_protocol::{decode_request, MAGIC, VERSION};
use crate::{normalize, split_caller};

// Deterministic xorshift so a failure is reproducible.
fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

// Every buffer of length `len`, invoking `f` on each. Only used for tiny `len`.
fn exhaustive(len: usize, mut f: impl FnMut(&[u8])) {
    let total: u64 = 1u64 << (8 * len);
    let mut buf = vec![0u8; len];
    for n in 0..total {
        for (i, b) in buf.iter_mut().enumerate() {
            *b = (n >> (8 * i)) as u8;
        }
        f(&buf);
    }
}

#[test]
fn decode_request_never_panics() {
    // Exhaustive over all buffers up to 2 bytes (the short-buffer path).
    for len in 0..=2usize {
        exhaustive(len, |buf| {
            let _ = decode_request(buf);
        });
    }
    // Structured: a valid-looking header with an arbitrary declared length and
    // an independently sized payload, to hammer the length-mismatch branches.
    let mut rng = 0x1234_5678_9abc_def0u64;
    for _ in 0..1_000_000 {
        let mut buf = Vec::new();
        buf.extend_from_slice(&MAGIC.to_le_bytes());
        buf.extend_from_slice(&VERSION.to_le_bytes());
        buf.extend_from_slice(&(next(&mut rng) as u16).to_le_bytes()); // op
        buf.extend_from_slice(&(next(&mut rng) as u16).to_le_bytes()); // flags
        buf.extend_from_slice(&(next(&mut rng) as u16).to_le_bytes()); // reserved
        buf.extend_from_slice(&(next(&mut rng) as u32).to_le_bytes()); // request_id
        buf.extend_from_slice(&(next(&mut rng) as u32).to_le_bytes()); // declared len
        let plen = (next(&mut rng) % 48) as usize;
        for _ in 0..plen {
            buf.push(next(&mut rng) as u8);
        }
        let _ = decode_request(&buf);
    }
    // Pure random bytes of random length.
    for _ in 0..1_000_000 {
        let len = (next(&mut rng) % 64) as usize;
        let buf: Vec<u8> = (0..len).map(|_| next(&mut rng) as u8).collect();
        let _ = decode_request(&buf);
    }
}

#[test]
fn split_caller_never_lets_userspace_impersonate() {
    let mut rng = 0xdead_beef_0bad_f00du64;
    for _ in 0..3_000_000 {
        let len = (next(&mut rng) % 20) as usize;
        let buf: Vec<u8> = (0..len).map(|_| next(&mut rng) as u8).collect();
        let sender = next(&mut rng) as u32;
        if let Ok((pid, _rest)) = split_caller(&buf, sender) {
            // The core authority theorem: a non-TCB sender can only ever be
            // attested as its own pid, never another's.
            if sender != 0 {
                assert_eq!(pid, sender, "userspace impersonation slipped through");
            }
        }
    }
}

#[test]
fn normalize_never_panics_and_stays_absolute() {
    // Exhaustive over short ascii-ish strings plus random, asserting normalize
    // always yields a rooted, slash-clean path.
    let mut rng = 0x0123_4567_89ab_cdefu64;
    let alphabet = b"/.abc";
    for _ in 0..2_000_000 {
        let len = (next(&mut rng) % 24) as usize;
        let s: Vec<u8> =
            (0..len).map(|_| alphabet[(next(&mut rng) as usize) % alphabet.len()]).collect();
        let text = core::str::from_utf8(&s).unwrap();
        let out = normalize(text);
        // Invariants that must hold for every input.
        assert!(out.starts_with('/'), "not rooted: {out:?}");
        assert!(!out.contains("//"), "double slash: {out:?}");
        assert!(!out.contains("/./"), "dot component: {out:?}");
        assert!(!out.contains("/../"), "dotdot component: {out:?}");
        assert!(out == "/" || !out.ends_with('/'), "trailing slash: {out:?}");
    }
}
