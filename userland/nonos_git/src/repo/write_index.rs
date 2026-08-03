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

//! Storing the index.

extern crate alloc;

use alloc::format;

use crate::index::{encode, IndexEntry};
use crate::storage::Storage;

use super::error::RepoError;

/// Write the index file.
pub fn write_index<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    entries: &[IndexEntry],
) -> Result<(), RepoError> {
    storage.write(&format!("{git_dir}/index"), &encode(entries))?;
    Ok(())
}
