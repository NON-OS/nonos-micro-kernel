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
//! A URL split into what a request needs.

extern crate alloc;

use alloc::string::String;

/// An `https://host/path` URL, already checked as safe to put in a request.
pub struct Url {
    pub host: String,
    /// The path, with no trailing slash.
    pub path: String,
}

impl Url {
    /// The last path segment, with any `.git` removed.
    ///
    /// This is what a clone names the directory it creates, so a segment that
    /// would climb out of the working directory, or name it, is refused. A
    /// url ending in `/..` would otherwise have a clone write over whatever
    /// sits beside where it was run.
    pub fn last_segment(&self) -> Option<&str> {
        let tail = self.path.rsplit('/').next()?;
        let name = tail.strip_suffix(".git").unwrap_or(tail);
        if name.is_empty() || name == "." || name == ".." {
            return None;
        }
        Some(name)
    }
}
