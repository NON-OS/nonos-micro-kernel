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

//! The commit record.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::oid::ObjectId;

use super::sig::Signature;

/// A commit: the tree it snapshots, the commits it follows, who wrote it and
/// the message.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Commit {
    pub tree: ObjectId,
    /// Empty for a root commit, one for an ordinary commit, more for a merge.
    /// Order matters: the first parent is the branch the merge was made on, and
    /// it is hashed into the id.
    pub parents: Vec<ObjectId>,
    pub author: Signature,
    pub committer: Signature,
    pub message: String,
}
