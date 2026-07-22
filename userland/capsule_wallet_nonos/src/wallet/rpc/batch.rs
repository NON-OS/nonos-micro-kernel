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

// JSON-RPC 2.0 batching: wrap several request objects in one array so a whole
// account refresh is a single HTTP POST over a single TLS connection instead
// of one handshake per field. Responses come back as an array of objects in
// any order, so each is located by its request id and handed to the existing
// single-response parsers unchanged.

use alloc::vec::Vec;

/// Join complete JSON-RPC request objects into one batch array body.
pub fn request_batch(parts: &[&[u8]]) -> Vec<u8> {
    let cap = parts.iter().map(|p| p.len() + 1).sum::<usize>() + 2;
    let mut out = Vec::with_capacity(cap);
    out.push(b'[');
    for (i, p) in parts.iter().enumerate() {
        if i > 0 {
            out.push(b',');
        }
        out.extend_from_slice(p);
    }
    out.push(b']');
    out
}

/// Return the top-level response object whose `id` matches, so the ordinary
/// `parse_*` functions can extract its `result`. String contents are skipped so
/// a brace or quote inside a value never confuses the object boundaries.
pub fn object_for_id(resp: &[u8], id: u64) -> Option<&[u8]> {
    let mut in_string = false;
    let mut escaped = false;
    let mut depth = 0u32;
    let mut start = 0usize;
    for (i, &b) in resp.iter().enumerate() {
        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'{' => {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            }
            b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let obj = &resp[start..=i];
                    if object_has_id(obj, id) {
                        return Some(obj);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Whether a response object carries `"id":<id>`, matching the whole number so
/// id 2 is not mistaken for the 2 in id 20.
fn object_has_id(obj: &[u8], id: u64) -> bool {
    let pat = b"\"id\":";
    let mut base = 0usize;
    while let Some(pos) = obj[base..].windows(pat.len()).position(|w| w == pat) {
        let mut j = base + pos + pat.len();
        while j < obj.len() && obj[j] == b' ' {
            j += 1;
        }
        let mut val = 0u64;
        let mut any = false;
        while j < obj.len() && obj[j].is_ascii_digit() {
            val = val.wrapping_mul(10).wrapping_add((obj[j] - b'0') as u64);
            any = true;
            j += 1;
        }
        if any && val == id {
            return true;
        }
        base += pos + pat.len();
    }
    false
}
