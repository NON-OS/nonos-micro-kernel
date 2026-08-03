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

//! Writing a commit's content bytes.

extern crate alloc;

use alloc::vec::Vec;

use super::types::Commit;

/// Encode a commit as git does: `tree <hex>`, a `parent <hex>` line per parent
/// in order, `author`, `committer`, a blank line, then the message.
///
/// The object ids appear as 40-character hex here, unlike a tree, which stores
/// them raw. That asymmetry is git's, and it is why the two encoders are
/// separate rather than sharing an id writer.
pub fn encode(commit: &Commit) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend_from_slice(b"tree ");
    out.extend_from_slice(commit.tree.to_hex().as_bytes());
    out.push(b'\n');

    for parent in &commit.parents {
        out.extend_from_slice(b"parent ");
        out.extend_from_slice(parent.to_hex().as_bytes());
        out.push(b'\n');
    }

    out.extend_from_slice(b"author ");
    commit.author.write(&mut out);
    out.push(b'\n');

    out.extend_from_slice(b"committer ");
    commit.committer.write(&mut out);
    out.push(b'\n');

    out.push(b'\n');
    out.extend_from_slice(commit.message.as_bytes());
    out
}
