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

//! Framing content into the bytes git hashes.

extern crate alloc;

use alloc::vec::Vec;

use crate::oid::ObjectId;
use crate::sha1::Sha1;

use super::decimal;
use super::kind::ObjectKind;

/// Frame `content` as git does before hashing or storing it: the kind, a
/// space, the decimal byte length, a NUL, then the content. Returns the framed
/// bytes and their object id, which is the SHA-1 of exactly those bytes.
pub fn frame(kind: ObjectKind, content: &[u8]) -> (Vec<u8>, ObjectId) {
    let mut framed = Vec::with_capacity(content.len() + 32);
    framed.extend_from_slice(kind.name());
    framed.push(b' ');
    decimal::push(&mut framed, content.len() as u64);
    framed.push(0);
    framed.extend_from_slice(content);

    let id = ObjectId::from_bytes(Sha1::digest(&framed));
    (framed, id)
}
