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

//! The proof set's stand-in for the kernel's entropy stack.

use core::sync::atomic::{AtomicU64, Ordering};

/*
 * A deterministic stream in place of the kernel's entropy stack, so runs repeat.
 * It is not zero: the enrolled-secret prover draws its nonces here, and a zero
 * nonce makes the nonce point the identity, which the verifier refuses.
 */
static STATE: AtomicU64 = AtomicU64::new(0x9e37_79b9_7f4a_7c15);

fn next() -> u64 {
    let mut z = STATE.fetch_add(0x9e37_79b9_7f4a_7c15, Ordering::Relaxed);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

pub fn get_random_bytes() -> [u8; 32] {
    let mut out = [0u8; 32];
    fill_random_bytes(&mut out);
    out
}

pub fn fill_random_bytes(buf: &mut [u8]) {
    for chunk in buf.chunks_mut(8) {
        let word = next().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
}
