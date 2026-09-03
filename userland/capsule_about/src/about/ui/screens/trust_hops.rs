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

use crate::about::data::trust;

// The chain pills are split out of the same string the card quotes rather than
// typed again, so the rail can never name a hop the trust data does not.
pub fn nodes(dst: &mut [&'static [u8]; 4]) -> usize {
    let mut n = 0usize;
    let mut rest: &'static [u8] = trust::SIGNING_CHAIN;
    while n < dst.len() {
        match rest.windows(2).position(|w| w == b"->") {
            Some(at) => {
                dst[n] = strip(&rest[..at]);
                rest = &rest[at + 2..];
            }
            None => {
                dst[n] = strip(rest);
                return n + 1;
            }
        }
        n += 1;
    }
    n
}

fn strip(b: &'static [u8]) -> &'static [u8] {
    let mut start = 0;
    let mut end = b.len();
    while start < end && b[start] == b' ' {
        start += 1;
    }
    while end > start && b[end - 1] == b' ' {
        end -= 1;
    }
    &b[start..end]
}
