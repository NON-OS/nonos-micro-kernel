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

use super::entry::Entry;

fn lower(b: u8) -> u8 {
    if b.is_ascii_uppercase() {
        b + 32
    } else {
        b
    }
}

/// Case-insensitive substring, because a reader types what they remember of a
/// label rather than how it was capitalised. An empty needle matches, so an
/// unfiltered palette shows the index in its natural order.
pub fn matches(label: &str, needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    let hay = label.as_bytes();
    if needle.len() > hay.len() {
        return false;
    }
    hay.windows(needle.len()).any(|w| w.iter().zip(needle).all(|(a, b)| lower(*a) == lower(*b)))
}

/// Positions of the matching entries, written into `out` and capped by its
/// length. The index is built source by source with history newest first, so
/// keeping that order ranks by source and then by recency in one pass.
pub fn filter(items: &[Entry], needle: &[u8], out: &mut [usize]) -> usize {
    let mut n = 0;
    for (i, e) in items.iter().enumerate() {
        if n == out.len() {
            break;
        }
        if matches(e.label, needle) {
            out[n] = i;
            n += 1;
        }
    }
    n
}
