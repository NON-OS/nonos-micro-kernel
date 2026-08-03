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
//! Recording a remote.

extern crate alloc;

use alloc::format;
use alloc::string::String;

use crate::storage::{Storage, StorageError};

use super::write::remote_section;

/// Record `name` as pointing at `url`, replacing any URL already there.
///
/// The file is rebuilt rather than appended to, so setting a remote twice
/// leaves one section rather than two that disagree.
pub fn set_remote<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    name: &str,
    url: &str,
) -> Result<(), StorageError> {
    let path = format!("{git_dir}/config");
    let raw = storage.read(&path).unwrap_or_default();
    let text = core::str::from_utf8(&raw).unwrap_or("");

    let mut out = String::new();
    let mut in_target = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_target = trimmed.starts_with(&format!("[remote \"{name}\""));
        }
        if !in_target {
            out.push_str(line);
            out.push('\n');
        }
    }
    out.push_str(&remote_section(name, url));
    storage.write(&path, out.as_bytes())
}
