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

use alloc::vec::Vec;

use crate::json::{find_key, read_string, read_u64};

/// A string field, as bytes. An array of strings reads as its first element,
/// which is how a node that answers on several addresses is handled: a hop is
/// reached at one of them or not at all.
pub fn string_field(obj: &[u8], key: &str) -> Option<Vec<u8>> {
    let at = find_key(obj, key)?;
    let start = obj[at..].iter().position(|&b| b == b'"')? + at;
    Some(read_string(obj, start)?.into_bytes())
}

/// A numeric field. The API sends some of these quoted, so a string is read
/// as a number rather than treated as the wrong type.
pub fn u64_field(obj: &[u8], key: &str) -> Option<u64> {
    let at = find_key(obj, key)?;
    if let Some(n) = read_u64(obj, at) {
        return Some(n);
    }
    let text = read_string(obj, at)?;
    let mut acc: u64 = 0;
    for b in text.bytes() {
        if !b.is_ascii_digit() {
            return None;
        }
        acc = acc.checked_mul(10)?.checked_add((b - b'0') as u64)?;
    }
    Some(acc)
}

/// A number held inside a nested object, named by the key that opens it.
///
/// `find_key` refuses to match inside a nested object on purpose, so a key
/// meant for one level is never read from another. The role a node plays is
/// written as `"role":{"Mixnode":{"layer":3}}`, so the layer is looked for
/// within the span that key opens rather than across the whole record.
pub fn nested_u64(obj: &[u8], outer: &str, inner: &str) -> Option<u64> {
    let at = find_key(obj, outer)?;
    let span = &obj[at..];
    let needle = inner.as_bytes();
    let mut i = 0usize;
    while i + needle.len() + 2 < span.len() {
        if span[i] == b'"'
            && span[i + 1..].starts_with(needle)
            && span[i + 1 + needle.len()] == b'"'
        {
            // Step past the closing quote and the colon: the number reader
            // skips whitespace but a colon is not whitespace.
            let mut at = i + needle.len() + 2;
            while at < span.len() && (span[at] as char).is_whitespace() {
                at += 1;
            }
            if span.get(at) != Some(&b':') {
                return None;
            }
            return read_u64(span, at + 1);
        }
        i += 1;
    }
    None
}

/// Dotted-quad to four octets. A host given as a name is refused here: this
/// runs per node, and resolving each one would announce the whole route to
/// the local resolver.
pub fn ipv4_field(text: &[u8]) -> Option<[u8; 4]> {
    let mut out = [0u8; 4];
    let mut octet: u16 = 0;
    let mut digits = 0;
    let mut index = 0;
    for &b in text {
        match b {
            b'0'..=b'9' => {
                octet = octet * 10 + (b - b'0') as u16;
                digits += 1;
                if octet > 255 || digits > 3 {
                    return None;
                }
            }
            b'.' => {
                if digits == 0 || index >= 3 {
                    return None;
                }
                out[index] = octet as u8;
                index += 1;
                octet = 0;
                digits = 0;
            }
            _ => return None,
        }
    }
    if digits == 0 || index != 3 {
        return None;
    }
    out[3] = octet as u8;
    Some(out)
}
