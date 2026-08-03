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

pub fn der_tlv(buf: &[u8], off: usize) -> Option<(u8, usize, usize)> {
    if off + 2 > buf.len() {
        return None;
    }
    let tag = buf[off];
    let first = buf[off + 1];
    if first < 128 {
        let val = off + 2;
        return Some((tag, val, val.checked_add(first as usize)?));
    }
    let n = (first & 0x7f) as usize;
    if n == 0 || n > 4 || off + 2 + n > buf.len() {
        return None;
    }
    let mut len = 0usize;
    for b in &buf[off + 2..off + 2 + n] {
        len = len.checked_shl(8)?.checked_add(*b as usize)?;
    }
    let val = off + 2 + n;
    let end = val.checked_add(len)?;
    if end <= buf.len() {
        Some((tag, val, end))
    } else {
        None
    }
}
