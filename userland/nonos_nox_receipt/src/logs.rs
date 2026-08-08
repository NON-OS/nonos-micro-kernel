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

//! Walking the `logs` array of a receipt without a JSON library.
//!
//! A receipt is a small, machine-written document with a known shape, so this
//! reads exactly what the decision needs: the `logs` array, each log object
//! inside it, and within a log its `address`, `topics` and `data`. It tracks
//! string and brace depth so a brace or bracket inside a quoted value cannot
//! desync the walk, and it yields nothing it cannot delimit, so a malformed
//! receipt produces no logs rather than a misread one.

/// A borrowed view of one log entry's raw fields, still hex strings.
pub struct RawLog<'a> {
    pub address: &'a [u8],
    /// Up to four topic hex strings; `topic_count` says how many are set.
    pub topics: [&'a [u8]; 4],
    pub topic_count: usize,
    pub data: &'a [u8],
}

/// Find the `"status"` field and report whether it is `0x1`. `None` means the
/// field was absent, which a caller must treat as not-confirmed.
pub fn status_ok(json: &[u8]) -> Option<bool> {
    let v = string_field(json, b"\"status\":")?;
    Some(v == b"0x1" || v == b"0x01")
}

/// Run `f` on each log object in the receipt's `logs` array until one returns
/// `true`. Returns whether any did. Stops early on the first accepting log.
pub fn for_each_log(json: &[u8], mut f: impl FnMut(&RawLog<'_>) -> bool) -> bool {
    let Some(arr) = array_after(json, b"\"logs\":") else {
        return false;
    };
    let mut i = 0;
    while i < arr.len() {
        match arr[i] {
            b'{' => {
                let Some(end) = object_end(&arr[i..]) else {
                    return false;
                };
                let obj = &arr[i..i + end];
                if let Some(log) = parse_log(obj) {
                    if f(&log) {
                        return true;
                    }
                }
                i += end;
            }
            _ => i += 1,
        }
    }
    false
}

/// Extract the fields of one log object.
fn parse_log(obj: &[u8]) -> Option<RawLog<'_>> {
    let address = string_field(obj, b"\"address\":")?;
    let data = string_field(obj, b"\"data\":")?;
    let topics_arr = array_after(obj, b"\"topics\":")?;

    let mut topics: [&[u8]; 4] = [b""; 4];
    let mut topic_count = 0;
    let mut i = 0;
    while i < topics_arr.len() && topic_count < 4 {
        if topics_arr[i] == b'"' {
            let Some(rel_end) = find(&topics_arr[i + 1..], b'"') else {
                break;
            };
            topics[topic_count] = &topics_arr[i + 1..i + 1 + rel_end];
            topic_count += 1;
            i += 1 + rel_end + 1;
        } else if topics_arr[i] == b']' {
            break;
        } else {
            i += 1;
        }
    }
    Some(RawLog { address, topics, topic_count, data })
}

/// The quoted string value that follows `key`, e.g. for `"data":"0x.."` and
/// key `"data":` returns `0x..`. `None` if the key or its string is absent.
fn string_field<'a>(hay: &'a [u8], key: &[u8]) -> Option<&'a [u8]> {
    let at = find_sub(hay, key)?;
    let mut i = at + key.len();
    // Skip whitespace to the opening quote.
    while i < hay.len() && (hay[i] == b' ' || hay[i] == b'\n' || hay[i] == b'\t' || hay[i] == b'\r')
    {
        i += 1;
    }
    if i >= hay.len() || hay[i] != b'"' {
        return None;
    }
    i += 1;
    let end = find(&hay[i..], b'"')?;
    Some(&hay[i..i + end])
}

/// The bytes inside the array that follows `key`, without the brackets. Tracks
/// bracket depth so a nested array ends at the matching `]`.
fn array_after<'a>(hay: &'a [u8], key: &[u8]) -> Option<&'a [u8]> {
    let at = find_sub(hay, key)?;
    let mut i = at + key.len();
    while i < hay.len() && hay[i] != b'[' {
        // Only whitespace may sit between the key and its array.
        if hay[i] != b' ' && hay[i] != b'\n' && hay[i] != b'\t' && hay[i] != b'\r' {
            return None;
        }
        i += 1;
    }
    if i >= hay.len() {
        return None;
    }
    let open = i;
    let mut depth = 0i32;
    let mut in_str = false;
    while i < hay.len() {
        let c = hay[i];
        if in_str {
            if c == b'"' {
                in_str = false;
            }
        } else {
            match c {
                b'"' => in_str = true,
                b'[' => depth += 1,
                b']' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(&hay[open + 1..i]);
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    None
}

/// Length from the opening `{` of `s` through its matching `}`, inclusive.
fn object_end(s: &[u8]) -> Option<usize> {
    let mut depth = 0i32;
    let mut in_str = false;
    let mut i = 0;
    while i < s.len() {
        let c = s[i];
        if in_str {
            if c == b'"' {
                in_str = false;
            }
        } else {
            match c {
                b'"' => in_str = true,
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i + 1);
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    None
}

/// Index of the first `needle` byte in `hay`, or `None`.
fn find(hay: &[u8], needle: u8) -> Option<usize> {
    hay.iter().position(|b| *b == needle)
}

/// Index of the first occurrence of `sub` in `hay`, or `None`.
fn find_sub(hay: &[u8], sub: &[u8]) -> Option<usize> {
    if sub.is_empty() || sub.len() > hay.len() {
        return None;
    }
    hay.windows(sub.len()).position(|w| w == sub)
}
