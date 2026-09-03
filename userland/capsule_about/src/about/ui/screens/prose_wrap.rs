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

use super::super::metrics::BODY_PX;
use super::super::text;

// Longest word-boundary prefix of `b` that measures within `w`, and where the
// remainder starts. Splitting on a space keeps a wrapped sentence readable; a run
// with no space left in it falls back to the measured character cut so a single
// long token still lands instead of looping forever.
pub fn split(b: &[u8], w: u32) -> (usize, usize) {
    let mut end = 0usize;
    let mut i = 0usize;
    while i < b.len() {
        let next = match b[i..].iter().position(|c| *c == b' ') {
            Some(p) => i + p,
            None => b.len(),
        };
        if text::width_of(&b[..next], BODY_PX) > w {
            break;
        }
        end = next;
        i = next + 1;
    }
    if end == 0 {
        let cut = fallback(b, w);
        return (cut, cut);
    }
    (end, (end + 1).min(b.len()))
}

fn fallback(b: &[u8], w: u32) -> usize {
    let s = core::str::from_utf8(b).unwrap_or("");
    let mut end = s.len();
    while end > 1 && text::width_of(&b[..end], BODY_PX) > w {
        end -= 1;
        while end > 1 && !s.is_char_boundary(end) {
            end -= 1;
        }
    }
    end.max(1)
}
