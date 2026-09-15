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

use alloc::string::String;
use alloc::vec::Vec;

use super::encode::{SIDECAR_MAGIC, SIDECAR_VERSION};

fn take_u32(buf: &[u8], off: &mut usize) -> Option<u32> {
    if *off + 4 > buf.len() {
        return None;
    }
    let v = u32::from_le_bytes([buf[*off], buf[*off + 1], buf[*off + 2], buf[*off + 3]]);
    *off += 4;
    Some(v)
}

fn take_str(buf: &[u8], off: &mut usize) -> Option<String> {
    let n = take_u32(buf, off)? as usize;
    if *off + n > buf.len() {
        return None;
    }
    let s = core::str::from_utf8(&buf[*off..*off + n]).ok()?;
    *off += n;
    Some(String::from(s))
}

// Parse a sidecar blob. Anything unrecognised, truncated, or from a newer
// format version yields an empty map: metadata is never load-bearing, so a bad
// blob must degrade to "no tags" rather than fail the app.
pub fn decode(buf: &[u8]) -> Vec<(String, String)> {
    let mut off = 0usize;
    let (Some(magic), Some(version), Some(count)) =
        (take_u32(buf, &mut off), take_u32(buf, &mut off), take_u32(buf, &mut off))
    else {
        return Vec::new();
    };
    if magic != SIDECAR_MAGIC || version != SIDECAR_VERSION {
        return Vec::new();
    }
    let mut out = Vec::new();
    for _ in 0..count {
        let (Some(key), Some(val)) = (take_str(buf, &mut off), take_str(buf, &mut off)) else {
            return Vec::new();
        };
        out.push((key, val));
    }
    out
}
