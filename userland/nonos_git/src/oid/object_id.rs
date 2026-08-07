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

//! The object id itself.

extern crate alloc;

use alloc::string::String;

use super::hex;

/// The name of a git object, its 20-byte SHA-1.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
            s.push(hex::digit(b >> 4));
            s.push(hex::digit(*b));
        }
        s
    }

    /// Parse a 40-character hex id. Rejects any other length or a non-hex
    /// character, so a malformed ref can never be read as an id.
    pub fn from_hex(s: &str) -> Option<ObjectId> {
        let bytes = s.as_bytes();
        if bytes.len() != 40 {
            return None;
        }
        let mut out = [0u8; 20];
        for (i, slot) in out.iter_mut().enumerate() {
            let hi = hex::value(bytes[i * 2])?;
            let lo = hex::value(bytes[i * 2 + 1])?;
            *slot = (hi << 4) | lo;
        }
        Some(ObjectId { bytes: out })
    }

    /// The two-character directory and 38-character file name git splits an
    /// object into under `objects`.
    pub fn loose_path(&self) -> (String, String) {
        let hex = self.to_hex();
        let (dir, file) = hex.split_at(2);
        (String::from(dir), String::from(file))
    }
}
