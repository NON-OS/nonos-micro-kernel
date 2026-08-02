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

//! One staged path.

extern crate alloc;

use alloc::string::String;

use crate::oid::ObjectId;
use crate::tree::Mode;

/// A file staged for the next commit.
///
/// Git also records the stat data it saw when staging, so it can skip hashing
/// files whose metadata is unchanged. That is a cache, not a fact about the
/// commit, and this writes it as zeros: git treats a zeroed stat as "cannot
/// trust, compare the content", which is correct, just without the shortcut.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IndexEntry {
    /// Path relative to the work tree root, with `/` separators.
    pub path: String,
    pub mode: Mode,
    pub id: ObjectId,
    /// Size of the staged content, which git carries in the entry.
    pub size: u32,
}
