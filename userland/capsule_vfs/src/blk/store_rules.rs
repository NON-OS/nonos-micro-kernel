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

//! When an existing record may be overwritten.
//!
//! Kept apart from the write itself, and free of the block transport, so the
//! decision can be driven on the host. It is a security boundary: the persist
//! op is open to any capsule that can reach the server, so this is what stands
//! between a caller and a staged capsule payload.

use super::error::BlkError;

/// The read-only tree. Mirrors the prefix `artifact_path` whitelists, and is
/// checked here as well because that gate guards the install ops rather than
/// the container underneath them.
pub const CAPSULE_TREE: &str = "/capsules/";

/// Whether the record named `name`, currently `entry_len` bytes, may be
/// overwritten with `data_len` bytes.
///
/// Two rules, both refusing with `Exists` so a caller cannot tell one from the
/// other, and both deliberately narrow. The container has no allocator, so a
/// different length would have to move an extent. And the capsule tree stays
/// exactly as immutable as it was when every name was written once.
pub fn permitted(name: &str, entry_len: u64, data_len: usize) -> Result<(), BlkError> {
    if entry_len != data_len as u64 {
        return Err(BlkError::Exists);
    }
    if in_capsule_tree(name) {
        return Err(BlkError::Exists);
    }
    Ok(())
}

/// Prefix match on the normalised name the table carries. The table is written
/// with normalised absolute paths, so there is no `..` or `//` to fold here;
/// a name that is not normalised does not match the tree and is refused by
/// the length rule or written outside it, never treated as inside it.
pub fn in_capsule_tree(name: &str) -> bool {
    name.starts_with(CAPSULE_TREE)
}
