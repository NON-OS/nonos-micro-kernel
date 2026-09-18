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


//! The stack a Linux program wakes up on: argc, then argv, then the
//! environment, then the auxiliary vector, each list ended by a null.
//! A C runtime reads all four before `main` and crashes without them.
//! The layout is fixed by the System V supplement, not by us.

use alloc::vec::Vec;

use super::auxv::pairs;
use super::loaded::Loaded;
use crate::linux::guest::Guest;

/// Sixteen bytes a C runtime turns into its stack guard.
const RANDOM_LEN: u64 = 16;

pub fn build(
    guest: &Guest,
    top: u64,
    image: &Loaded,
    interp_base: u64,
    argv0: &[u8],
) -> Option<u64> {
    let name_at = top - (argv0.len() as u64 + 1);
    let random_at = (name_at - RANDOM_LEN) & !0x0F;
    let mut words: Vec<u64> = alloc::vec![1, name_at, 0, 0];
    words.extend(pairs(image, interp_base, random_at, name_at));
    let bytes = words.len() as u64 * 8;
    let rsp = (random_at - bytes) & !0x0F;

    let mut name = argv0.to_vec();
    name.push(0);
    if guest.write(name_at, &name) < 0 {
        return None;
    }
    if guest.write(random_at, &random()?) < 0 {
        return None;
    }
    let mut blob: Vec<u8> = Vec::with_capacity(bytes as usize);
    for word in &words {
        blob.extend_from_slice(&word.to_le_bytes());
    }
    if guest.write(rsp, &blob) < 0 {
        return None;
    }
    Some(rsp)
}

/// Entropy for the guard comes from the system, never from a constant: a
/// fixed value here would make every guest's stack guard the same and
/// the protection worth nothing.
fn random() -> Option<[u8; RANDOM_LEN as usize]> {
    let mut out = [0u8; RANDOM_LEN as usize];
    match nonos_libc::crypto_random(out.as_mut_ptr(), out.len()) {
        n if n < 0 => None,
        _ => Some(out),
    }
}
