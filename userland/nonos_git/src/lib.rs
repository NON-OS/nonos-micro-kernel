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

//! Git's object model for the NONOS terminal, from scratch and `no_std`.
//!
//! A git repository is a content-addressed store: every blob, tree and commit
//! is named by the SHA-1 of its framed bytes, `<type> <size>\0<content>`. This
//! crate is that core, the part that must agree with real git bit for bit so a
//! repository written here is one `git` can read and one written by `git` reads
//! here. It owns the hash, the object framing, and the object id; the store on
//! disk and the higher commands build on top.
//!
//! Everything is pure and deterministic, so it is proven on the host against
//! the hashes real `git` produces for the same content.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod object;
mod oid;
mod sha1;
mod zlib;

pub use object::{frame, unframe, ObjectKind};
pub use oid::ObjectId;
pub use sha1::Sha1;
pub use zlib::{compress, decompress, InflateError};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod zlib_tests;
