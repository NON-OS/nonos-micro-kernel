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

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::js::value::Value;

const MAX_DEPTH: u32 = 32;
const MAX_ITEMS: usize = 4096;

// One JSON value at `pos`, advancing past it. Malformed input rejects the
// whole document; the caller sees None and hands undefined to the script.
pub(super) fn json_value(b: &[u8], pos: &mut usize, depth: u32) -> Option<Value> {
    if depth > MAX_DEPTH {
        return None;
    }
    while *pos < b.len() && b[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
    let c = *b.get(*pos)?;
    if b[*pos..].starts_with(b"null") {
        *pos += 4;
        return Some(Value::Null);
    }
    if b[*pos..].starts_with(b"true") {
        *pos += 4;
        return Some(Value::Bool(true));
    }
    if b[*pos..].starts_with(b"false") {
        *pos += 5;
        return Some(Value::Bool(false));
    }
    if c == b'"' {
        *pos += 1;
        let mut s = String::new();
        loop {
            let ch = *b.get(*pos)?;
            *pos += 1;
            match ch {
                b'"' => return Some(Value::Str(Rc::new(s))),
                b'\\' => {
                    let esc = *b.get(*pos)?;
                    *pos += 1;
                    match esc {
                        b'n' => s.push('\n'),
                        b't' => s.push('\t'),
                        b'r' => s.push('\r'),
                        b'u' => {
                            let hex = b.get(*pos..*pos + 4)?;
                            *pos += 4;
                            let n =
                                u32::from_str_radix(core::str::from_utf8(hex).ok()?, 16).ok()?;
                            s.push(char::from_u32(n).unwrap_or('\u{fffd}'));
                        }
                        other => s.push(other as char),
                    }
                }
                other if other < 0x80 => s.push(other as char),
                other => {
                    // Re-borrow the full utf-8 sequence starting here.
                    let start = *pos - 1;
                    let len = match other {
                        0xC0..=0xDF => 2,
                        0xE0..=0xEF => 3,
                        _ => 4,
                    };
                    let chunk = b.get(start..start + len)?;
                    s.push_str(core::str::from_utf8(chunk).ok()?);
                    *pos = start + len;
                }
            }
            if s.len() > 65_536 {
                return None;
            }
        }
    }
    if c == b'[' {
        *pos += 1;
        let mut items: Vec<Value> = Vec::new();
        loop {
            while *pos < b.len() && b[*pos].is_ascii_whitespace() {
                *pos += 1;
            }
            if *b.get(*pos)? == b']' {
                *pos += 1;
                return Some(Value::Array(Rc::new(RefCell::new(items))));
            }
            if items.len() >= MAX_ITEMS {
                return None;
            }
            items.push(json_value(b, pos, depth + 1)?);
            while *pos < b.len() && b[*pos].is_ascii_whitespace() {
                *pos += 1;
            }
            match *b.get(*pos)? {
                b',' => *pos += 1,
                b']' => {}
                _ => return None,
            }
        }
    }
    if c == b'{' {
        *pos += 1;
        let mut map: BTreeMap<String, Value> = BTreeMap::new();
        loop {
            while *pos < b.len() && b[*pos].is_ascii_whitespace() {
                *pos += 1;
            }
            if *b.get(*pos)? == b'}' {
                *pos += 1;
                return Some(Value::Object(Rc::new(RefCell::new(map))));
            }
            if map.len() >= MAX_ITEMS {
                return None;
            }
            let key = match json_value(b, pos, depth + 1)? {
                Value::Str(s) => (*s).clone(),
                _ => return None,
            };
            while *pos < b.len() && b[*pos].is_ascii_whitespace() {
                *pos += 1;
            }
            if *b.get(*pos)? != b':' {
                return None;
            }
            *pos += 1;
            map.insert(key, json_value(b, pos, depth + 1)?);
            while *pos < b.len() && b[*pos].is_ascii_whitespace() {
                *pos += 1;
            }
            match *b.get(*pos)? {
                b',' => *pos += 1,
                b'}' => {}
                _ => return None,
            }
        }
    }
    // Number: consume the token and let the float parser judge it.
    let start = *pos;
    while *pos < b.len() && matches!(b[*pos], b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E') {
        *pos += 1;
    }
    if start == *pos {
        return None;
    }
    let n = core::str::from_utf8(&b[start..*pos]).ok()?.parse::<f64>().ok()?;
    if n.is_finite() {
        Some(Value::Num(n))
    } else {
        None
    }
}
