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

use crate::browser::html::parse::{read_to_gt, tag_name};

pub fn skip_until_close(chars: &mut core::iter::Peekable<core::str::CharIndices>, name: &str) {
    let mut scanned = 0u32;
    while let Some((_, c)) = chars.next() {
        scanned = scanned.saturating_add(1);
        if scanned > 4_000_000 {
            break;
        }
        if c == '<' && chars.peek().map(|&(_, n)| n) == Some('/') {
            let raw = read_to_gt::read_to_gt(chars);
            if tag_name::tag_name(&raw) == name {
                break;
            }
        }
    }
}
