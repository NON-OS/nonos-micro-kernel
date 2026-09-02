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

mod count;
mod text;

use alloc::vec;
use alloc::vec::Vec;

// Apply one pipe-stage filter to the previous stage's output lines.
pub(super) fn apply(seg: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    match seg.first().copied().unwrap_or(b"") {
        b"grep" => text::grep(&seg[1..], input),
        b"sort" => text::sort(&seg[1..], input),
        b"uniq" => text::uniq(input),
        b"cut" => text::cut(&seg[1..], input),
        b"nl" => text::nl(input),
        b"wc" => count::wc(input),
        b"head" => count::head(seg.get(1), input),
        b"tail" => count::tail(seg.get(1), input),
        other => {
            let mut msg = Vec::new();
            msg.extend_from_slice(b"pipe: unknown filter ");
            msg.extend_from_slice(other);
            vec![msg]
        }
    }
}
