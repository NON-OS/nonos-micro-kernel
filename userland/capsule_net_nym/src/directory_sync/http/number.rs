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

pub fn decimal(s: &[u8]) -> Option<usize> {
    let mut n = 0usize;
    let mut seen = false;
    for b in s {
        if !b.is_ascii_digit() {
            break;
        }
        seen = true;
        n = n.checked_mul(10)?.checked_add((*b - b'0') as usize)?;
    }
    if seen {
        Some(n)
    } else {
        None
    }
}

pub fn trim_left(mut s: &[u8]) -> &[u8] {
    while s.first() == Some(&b' ') {
        s = &s[1..];
    }
    s
}
