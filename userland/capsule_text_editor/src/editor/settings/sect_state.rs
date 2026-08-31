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

//! Persisted switch state for the six non-General sections, one bitfield per
//! nav index so a flip survives leaving the section and coming back. Index 0 is
//! unused: General keeps its own switches in `state`.

use core::sync::atomic::{AtomicU32, Ordering};

static SECT_BITS: [AtomicU32; 7] = [
    AtomicU32::new(0),
    AtomicU32::new(0b1_1101),
    AtomicU32::new(0b011),
    AtomicU32::new(0b01),
    AtomicU32::new(0b1101),
    AtomicU32::new(0b0111),
    AtomicU32::new(0b1011),
];

pub(super) fn sect_on(nav: usize, bit: u32) -> bool {
    match SECT_BITS.get(nav) {
        Some(cell) => cell.load(Ordering::Relaxed) & (1 << bit) != 0,
        None => false,
    }
}

pub(super) fn flip_sect(nav: usize, bit: u32) {
    if let Some(cell) = SECT_BITS.get(nav) {
        cell.fetch_xor(1 << bit, Ordering::Relaxed);
    }
}
