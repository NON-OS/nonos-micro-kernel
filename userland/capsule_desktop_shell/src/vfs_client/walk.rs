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

//! Walk the length-prefixed name list in a LIST reply, handing each raw name to
//! `classify` and keeping what it accepts. This reads bytes the vfs service
//! sent us, so every step is bounds checked: a truncated length, a length that
//! runs past the reply, or a flood of entries can only ever yield fewer
//! results, never an out-of-range read or an unbounded allocation.

use alloc::vec::Vec;

/// Ceiling on entries so a malformed reply cannot drive us to allocate without
/// bound. The real root holds a handful of items.
const MAX_ENTRIES: usize = 4096;

pub(super) fn walk<T, F>(rx: &[u8], start: usize, end: usize, mut classify: F) -> Vec<T>
where
    F: FnMut(&str) -> Option<T>,
{
    let end = end.min(rx.len());
    let mut out = Vec::new();
    let mut off = start;
    while off < end && out.len() < MAX_ENTRIES {
        let n = rx[off] as usize;
        off += 1;
        if off + n > end {
            break;
        }
        if let Ok(raw) = core::str::from_utf8(&rx[off..off + n]) {
            if let Some(item) = classify(raw) {
                out.push(item);
            }
        }
        off += n;
    }
    out
}
