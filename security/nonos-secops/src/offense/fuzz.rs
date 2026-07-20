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

//! Fuzz the untrusted trailer parser. It must be total on adversarial input: for
//! any byte string, the deserializer returns rather than panicking or looping.
//! We drive it with random bytes and with mutations of a real trailer, the
//! values most likely to reach deep code paths, including absurd length
//! prefixes. One survivor short of the count is a boot-time denial of service.

use crate::attest::{enroll_kernel, proof_parser_is_total};
use crate::rng::Rng;

/// Fuzz the parser for `iterations` inputs. Returns false on the first input that
/// makes the parser panic, having reported which input and at which iteration.
pub fn fuzz(iterations: usize) -> bool {
    let seed = b"nonos-kernel corpus seed for the trailer fuzzer".to_vec();
    let (_root, corpus) = enroll_kernel(&seed);
    let mut rng = Rng::new(0x9e3779b97f4a7c15);
    let mut survived = 0usize;

    for i in 0..iterations {
        let mut buf: Vec<u8> = if i % 3 == 0 {
            // Wholly random bytes of a random length up to a trailer's size.
            let n = rng.below(corpus.len().max(1) + 64);
            (0..n).map(|_| rng.byte()).collect()
        } else {
            // A mutation of a real trailer: flip a handful of bytes, maybe truncate.
            let mut b = corpus.clone();
            let flips = 1 + rng.below(16);
            for _ in 0..flips {
                if b.is_empty() {
                    break;
                }
                let at = rng.below(b.len());
                b[at] ^= rng.byte();
            }
            if rng.below(4) == 0 && !b.is_empty() {
                b.truncate(rng.below(b.len()));
            }
            b
        };
        // Occasionally inject an absurd length prefix, the classic parser DoS.
        if !buf.is_empty() && rng.below(5) == 0 {
            let hi = buf.len().min(4);
            for byte in buf.iter_mut().take(hi) {
                *byte = 0xff;
            }
        }

        if !proof_parser_is_total(&buf) {
            println!("  [FAIL] parser panicked on a {}-byte input (iteration {i})", buf.len());
            return false;
        }
        survived += 1;
    }
    println!("  [PASS] parser stayed total over {survived} adversarial inputs");
    true
}
