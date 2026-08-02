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

//! Following `HEAD` to a commit.

extern crate alloc;

use alloc::format;

use crate::oid::ObjectId;
use crate::storage::Storage;

use super::head::Head;
use super::read_head::read_head;

/// The commit `HEAD` names, or `None` on an unborn branch.
pub fn resolve_head<S: Storage>(storage: &S, git_dir: &str) -> Option<ObjectId> {
    match read_head(storage, git_dir)? {
        Head::Detached(id) => Some(id),
        Head::Branch(branch) => {
            let raw = storage.read(&format!("{git_dir}/refs/heads/{branch}")).ok()?;
            ObjectId::from_hex(core::str::from_utf8(&raw).ok()?.trim())
        }
    }
}
