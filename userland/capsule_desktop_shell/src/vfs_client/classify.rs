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

//! Decide whether a raw absolute path names a direct child of `prefix`, and if
//! so hand back its basename and directory flag. The server answers a LIST with
//! full paths for the whole subtree, so this is what keeps the desktop showing
//! only the top level. It touches no I/O, which is why the host proofs can pin
//! its behaviour down without booting.

use alloc::string::String;

pub(super) fn classify(prefix: &str, raw: &str) -> Option<(String, bool)> {
    let is_dir = raw.as_bytes().last() == Some(&b'/');
    let trimmed = raw.trim_end_matches('/');
    // Reduce the entry to a path relative to the queried prefix.
    let rel = if let Some(stripped) = trimmed.strip_prefix(prefix) {
        stripped.trim_start_matches('/')
    } else if !trimmed.starts_with('/') {
        // The server answered with a name already relative to the prefix.
        trimmed.trim_start_matches('/')
    } else {
        // An absolute path that is not under the prefix, or the prefix folder
        // itself once its own trailing slash is gone. Neither is a child.
        return None;
    };
    if rel.is_empty() || rel.contains('/') {
        return None;
    }
    Some((String::from(rel), is_dir))
}
