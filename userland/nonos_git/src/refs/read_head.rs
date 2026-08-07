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

//! Reading `HEAD`.

extern crate alloc;

use alloc::format;
use alloc::string::String;

use crate::oid::ObjectId;
use crate::storage::Storage;

use super::head::Head;
use super::name::is_valid_ref_name;

/// Read `HEAD`. `None` if it is missing or holds something we will not follow.
pub fn read_head<S: Storage>(storage: &S, git_dir: &str) -> Option<Head> {
    let raw = storage.read(&format!("{git_dir}/HEAD")).ok()?;
    let text = core::str::from_utf8(&raw).ok()?.trim();

    if let Some(target) = text.strip_prefix("ref: ") {
        let branch = target.strip_prefix("refs/heads/")?;
        // A HEAD naming an invalid ref is refused rather than followed, since
        // the name becomes a path.
        if !is_valid_ref_name(branch) {
            return None;
        }
        return Some(Head::Branch(String::from(branch)));
    }

    ObjectId::from_hex(text).map(Head::Detached)
}
