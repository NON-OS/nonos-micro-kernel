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

//! An object id: the 20-byte SHA-1 that names a git object.

extern crate alloc;

use alloc::string::String;

/// The name of a git object, its 20-byte SHA-1.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ObjectId {
    bytes: [u8; 20],
}

impl ObjectId {
    /// Wrap a raw 20-byte digest.
    pub const fn from_bytes(bytes: [u8; 20]) -> ObjectId {
        ObjectId { bytes }
    }

    pub const fn as_bytes(&self) -> &[u8; 20] {
        &self.bytes
    }

    /// The 40-character lowercase hex form git prints and uses in paths.
    pub fn to_hex(&self) -> String {
        let mut s = String::with_capacity(40);
        for b in &self.bytes {
            s.push(hex_digit(b >> 4));
            s.push(hex_digit(b & 0x0F));
        }
        s
    }

    /// Parse a 40-character hex id. Rejects any other length or a non-hex
    /// character, so a malformed ref name can never be read as an id.
    pub fn from_hex(s: &str) -> Option<ObjectId> {
        let bytes = s.as_bytes();
        if bytes.len() != 40 {
            return None;
        }
        let mut out = [0u8; 20];
        let mut i = 0;
        while i < 20 {
            let hi = from_hex_digit(bytes[i * 2])?;
            let lo = from_hex_digit(bytes[i * 2 + 1])?;
            out[i] = (hi << 4) | lo;
            i += 1;
        }
        Some(ObjectId { bytes: out })
    }

    /// The two-character directory and 38-character file name git splits an
    /// object into under `.git/objects`.
    pub fn loose_path(&self) -> (String, String) {
        let hex = self.to_hex();
        let (dir, file) = hex.split_at(2);
        (String::from(dir), String::from(file))
    }
}

fn hex_digit(v: u8) -> char {
    match v {
        0..=9 => (b'0' + v) as char,
        _ => (b'a' + (v - 10)) as char,
    }
}

fn from_hex_digit(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}
