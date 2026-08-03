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
//! Reading a remote back.

extern crate alloc;

use alloc::format;
use alloc::string::String;

use crate::storage::Storage;

use super::parse::walk;

/// The URL recorded for `name`, if there is one.
pub fn remote_url<S: Storage>(storage: &S, git_dir: &str, name: &str) -> Option<String> {
    let raw = storage.read(&format!("{git_dir}/config")).ok()?;
    let text = core::str::from_utf8(&raw).ok()?;
    let mut found = None;
    walk(text, |section, subsection, key, value| {
        if section == "remote" && subsection == name && key == "url" {
            found = Some(String::from(value));
        }
    });
    found
}
