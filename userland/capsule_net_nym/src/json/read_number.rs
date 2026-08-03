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

/// Read an unsigned integer starting at `at`.
pub fn read_u64(text: &[u8], mut at: usize) -> Option<u64> {
    while at < text.len() && (text[at] as char).is_whitespace() {
        at += 1;
    }
    let start = at;
    let mut value = 0u64;
    while at < text.len() && text[at].is_ascii_digit() {
        value = value.checked_mul(10)?.checked_add((text[at] - b'0') as u64)?;
        at += 1;
    }
    if at == start {
        return None;
    }
    Some(value)
}

/// Read a JSON array of byte-sized numbers.
pub fn read_bytes(text: &[u8], mut at: usize) -> Option<alloc::vec::Vec<u8>> {
    while at < text.len() && (text[at] as char).is_whitespace() {
        at += 1;
    }
    if text.get(at) != Some(&b'[') {
        return None;
    }
    at += 1;
    let mut out = alloc::vec::Vec::new();
    loop {
        while at < text.len() && ((text[at] as char).is_whitespace() || text[at] == b',') {
            at += 1;
        }
        if text.get(at) == Some(&b']') {
            return Some(out);
        }
        let start = at;
        let mut value = 0u32;
        while at < text.len() && text[at].is_ascii_digit() {
            value = value.checked_mul(10)?.checked_add((text[at] - b'0') as u32)?;
            at += 1;
        }
        if at == start || value > 255 {
            return None;
        }
        out.push(value as u8);
    }
}
