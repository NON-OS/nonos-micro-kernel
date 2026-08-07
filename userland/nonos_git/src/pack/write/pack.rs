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
//! Writing whole objects into a version 2 pack.

extern crate alloc;

use alloc::vec::Vec;

use crate::object::ObjectKind;
use crate::sha1::Sha1;
use crate::zlib::compress;

use super::entry_header;

/// Build a pack holding `objects`, each stored whole.
///
/// Deltas would make it smaller, and nothing about the format requires them:
/// a receiver reconstructs whole objects the same way either side sent them.
/// The trailing SHA-1 covers everything before it, which is how the receiver
/// knows the stream arrived intact.
pub fn write_pack(objects: &[(ObjectKind, Vec<u8>)]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"PACK");
    out.extend_from_slice(&2u32.to_be_bytes());
    out.extend_from_slice(&(objects.len() as u32).to_be_bytes());

    for (kind, content) in objects {
        entry_header::write(*kind, content.len(), &mut out);
        out.extend_from_slice(&compress(content));
    }

    let trailer = Sha1::digest(&out);
    out.extend_from_slice(&trailer);
    out
}
