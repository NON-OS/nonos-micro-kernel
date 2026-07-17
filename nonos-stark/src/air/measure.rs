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

//! Measuring a capsule or kernel image to a Poseidon leaf. The bytes are absorbed
//! into a Poseidon sponge seven at a time (each block a canonical field element) and
//! the rate lanes are squeezed to a digest. The length is bound first so images of
//! different size cannot collide. That digest is the enrolled leaf: a policy root
//! commits to exactly the measured images, and an attestation proves membership of a
//! real measurement rather than an arbitrary secret.

use super::super::field::Fp;
use super::poseidon::{Poseidon, RATE, WIDTH};

/// The Poseidon measurement of `image`: bind the length, absorb the bytes in
/// seven-byte little-endian blocks with one permutation per rate group, then squeeze
/// the rate lanes.
pub fn measure_capsule(hasher: &Poseidon, image: &[u8]) -> [Fp; RATE] {
    let mut state = [Fp::ZERO; WIDTH];
    state[0] = state[0] + Fp::from_u64(image.len() as u64);
    state = hasher.permute(state);

    let mut lane = 0usize;
    let mut i = 0usize;
    while i < image.len() {
        let take = core::cmp::min(7, image.len() - i);
        let mut buf = [0u8; 8];
        buf[..take].copy_from_slice(&image[i..i + take]);
        state[lane] = state[lane] + Fp::from_u64(u64::from_le_bytes(buf));
        lane += 1;
        if lane == RATE {
            state = hasher.permute(state);
            lane = 0;
        }
        i += 7;
    }
    if lane != 0 {
        state = hasher.permute(state);
    }

    let mut digest = [Fp::ZERO; RATE];
    digest.copy_from_slice(&state[..RATE]);
    digest
}
