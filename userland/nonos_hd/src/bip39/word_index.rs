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

use crate::wordlist::ENGLISH_WORDLIST;

/// Look up a typed word in the BIP39 list. The list is lexicographically
/// sorted, so binary search resolves in eleven comparisons. Case-insensitive
/// over ASCII, since users type recovery phrases in whatever case.
pub fn word_index(word: &[u8]) -> Option<u16> {
    let mut lo = 0usize;
    let mut hi = ENGLISH_WORDLIST.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match cmp_ascii_ci(word, ENGLISH_WORDLIST[mid].as_bytes()) {
            core::cmp::Ordering::Equal => return Some(mid as u16),
            core::cmp::Ordering::Less => hi = mid,
            core::cmp::Ordering::Greater => lo = mid + 1,
        }
    }
    None
}

fn cmp_ascii_ci(a: &[u8], b: &[u8]) -> core::cmp::Ordering {
    let mut i = 0;
    loop {
        match (a.get(i), b.get(i)) {
            (None, None) => return core::cmp::Ordering::Equal,
            (None, Some(_)) => return core::cmp::Ordering::Less,
            (Some(_), None) => return core::cmp::Ordering::Greater,
            (Some(&x), Some(&y)) => {
                let (x, y) = (x.to_ascii_lowercase(), y.to_ascii_lowercase());
                if x != y {
                    return x.cmp(&y);
                }
            }
        }
        i += 1;
    }
}
