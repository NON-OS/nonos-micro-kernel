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

//! Hash cost, which is most of what attesting a capsule spends its time on.
//!
//! Every spawn hashes the binary before checking a signature over that hash, so
//! this is the part of the guarantee that scales with how much code is being
//! admitted rather than with the signature scheme. Measured at one page,
//! because that is the unit the loader works in and it makes the per-byte cost
//! comparable against published figures for the same primitives.

use crate::crypto::hash::{blake3_hash, keccak256, sha256, sha3_256, sha512};
use crate::sys::bench::measure::{measure, report};

/// One page, the granularity the loader hashes in.
const BLOCK: usize = 4096;

/// Enough runs that `min` settles, few enough that boot is not delayed.
const ITERATIONS: u32 = 64;

/// Time each hash over a page and print the spread.
///
/// The input is filled rather than zeroed. A zero page can be handled faster by
/// implementations that special-case it, and a benchmark that quietly measures
/// the easy path is worse than no benchmark at all.
pub(super) fn run() {
    let mut block = [0u8; BLOCK];
    for (i, byte) in block.iter_mut().enumerate() {
        *byte = (i % 251) as u8;
    }

    report(
        b"blake3_4k",
        &measure(ITERATIONS, || {
            core::hint::black_box(blake3_hash(core::hint::black_box(&block)));
        }),
    );
    report(
        b"sha256_4k",
        &measure(ITERATIONS, || {
            core::hint::black_box(sha256(core::hint::black_box(&block)));
        }),
    );
    report(
        b"sha512_4k",
        &measure(ITERATIONS, || {
            core::hint::black_box(sha512(core::hint::black_box(&block)));
        }),
    );
    report(
        b"sha3_256_4k",
        &measure(ITERATIONS, || {
            core::hint::black_box(sha3_256(core::hint::black_box(&block)));
        }),
    );
    report(
        b"keccak256_4k",
        &measure(ITERATIONS, || {
            core::hint::black_box(keccak256(core::hint::black_box(&block)));
        }),
    );
}
