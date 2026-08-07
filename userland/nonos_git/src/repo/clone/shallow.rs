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
//! Recording that a clone stopped short of the full history.

extern crate alloc;

use alloc::format;
use alloc::string::String;

use crate::oid::ObjectId;
use crate::storage::Storage;

use super::super::error::RepoError;

/// Write `.git/shallow`, the list of commits whose parents were not fetched.
///
/// Without this file git treats the missing parents as corruption and `fsck`
/// reports broken links. With it, git knows the history is cut at these
/// commits and works normally.
pub(super) fn mark_shallow<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    tips: &[ObjectId],
) -> Result<(), RepoError> {
    let mut body = String::new();
    for id in tips {
        body.push_str(&id.to_hex());
        body.push('\n');
    }
    storage.write(&format!("{git_dir}/shallow"), body.as_bytes())?;
    Ok(())
}
