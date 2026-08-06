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
//! What a clone was asked to lay down.

use crate::oid::ObjectId;

/// Where a clone writes and what it fetched.
pub struct CloneRequest<'a> {
    /// The repository directory, usually `.git` inside the work tree.
    pub git_dir: &'a str,
    /// Prefix the work tree files are written under. Empty for the root.
    pub work_tree: &'a str,
    /// The commit the branch is set to.
    pub head: ObjectId,
    /// Short branch name, without `refs/heads/`.
    pub branch: &'a str,
    /// True when the fetch asked for a bounded depth, so the parents of `head`
    /// were not sent and git has to be told the history is cut here.
    pub shallow: bool,
}
