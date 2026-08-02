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

//! Object framing: the `<type> <size>\0<content>` bytes git hashes and stores.

extern crate alloc;

use alloc::vec::Vec;

use crate::oid::ObjectId;
use crate::sha1::Sha1;

/// The four object kinds git stores. Tags are the annotated-tag object, not the
/// lightweight ref.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ObjectKind {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl ObjectKind {
    /// The ASCII name git writes into the object header.
    pub const fn name(self) -> &'static [u8] {
        match self {
            ObjectKind::Blob => b"blob",
            ObjectKind::Tree => b"tree",
            ObjectKind::Commit => b"commit",
            ObjectKind::Tag => b"tag",
        }
    }

    /// Parse the header name, rejecting anything git would not write.
    pub fn from_name(name: &[u8]) -> Option<ObjectKind> {
        match name {
            b"blob" => Some(ObjectKind::Blob),
            b"tree" => Some(ObjectKind::Tree),
            b"commit" => Some(ObjectKind::Commit),
            b"tag" => Some(ObjectKind::Tag),
            _ => None,
        }
    }
}

/// Frame `content` as git does before hashing or storing it: the kind, a space,
/// the decimal byte length, a NUL, then the content. Returns the framed bytes
/// and their object id, which is the SHA-1 of exactly those bytes.
pub fn frame(kind: ObjectKind, content: &[u8]) -> (Vec<u8>, ObjectId) {
    let mut framed = Vec::with_capacity(content.len() + 32);
    framed.extend_from_slice(kind.name());
    framed.push(b' ');
    push_decimal(&mut framed, content.len() as u64);
    framed.push(0);
    framed.extend_from_slice(content);

    let id = ObjectId::from_bytes(Sha1::digest(&framed));
    (framed, id)
}

/// Split framed bytes back into kind and content, validating the header. `None`
/// if the header is malformed or the stated length does not match the content,
/// so a corrupt object is never read as valid.
pub fn unframe(framed: &[u8]) -> Option<(ObjectKind, &[u8])> {
    let space = framed.iter().position(|b| *b == b' ')?;
    let nul = framed.iter().position(|b| *b == 0)?;
    if nul < space {
        return None;
    }
    let kind = ObjectKind::from_name(&framed[..space])?;
    let size = parse_decimal(&framed[space + 1..nul])?;
    let content = &framed[nul + 1..];
    if content.len() as u64 != size {
        return None;
    }
    Some((kind, content))
}

fn push_decimal(out: &mut Vec<u8>, mut v: u64) {
    if v == 0 {
        out.push(b'0');
        return;
    }
    let start = out.len();
    while v > 0 {
        out.push(b'0' + (v % 10) as u8);
        v /= 10;
    }
    out[start..].reverse();
}

fn parse_decimal(bytes: &[u8]) -> Option<u64> {
    if bytes.is_empty() {
        return None;
    }
    let mut v: u64 = 0;
    for b in bytes {
        if !b.is_ascii_digit() {
            return None;
        }
        v = v.checked_mul(10)?.checked_add((b - b'0') as u64)?;
    }
    Some(v)
}
