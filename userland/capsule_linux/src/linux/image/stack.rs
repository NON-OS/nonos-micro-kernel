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

use alloc::vec::Vec;

use super::auxv::pairs;
use super::loaded::Loaded;
use super::stack_guard::{random, RANDOM_LEN};
use super::stack_strings::place;
use super::stack_words::words;
use crate::linux::guest::Guest;

pub fn build(
    guest: &Guest,
    top: u64,
    image: &Loaded,
    interp_base: u64,
    argv: &[Vec<u8>],
    envp: &[Vec<u8>],
) -> Option<u64> {
    let mut all = argv.to_vec();
    all.extend_from_slice(envp);
    let placed = place(guest, top, &all)?;
    let random_at = (placed.floor - RANDOM_LEN) & !0x0F;

    let aux = pairs(image, interp_base, random_at, *placed.at.first()?);
    let words = words(argv.len(), &placed.at, aux)?;

    /*
     * System V wants rsp itself sixteen-byte aligned at the entry point, so
     * the block is placed from an aligned base.
     */
    let rsp = (random_at - words.len() as u64 * 8) & !0x0F;
    if guest.write(random_at, &random()?) < 0 {
        return None;
    }
    let mut blob: Vec<u8> = Vec::with_capacity(words.len() * 8);
    for word in &words {
        blob.extend_from_slice(&word.to_le_bytes());
    }
    match guest.write(rsp, &blob) {
        n if n < 0 => None,
        _ => Some(rsp),
    }
}
