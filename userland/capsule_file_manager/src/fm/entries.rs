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

extern crate alloc;

use alloc::{string::String, vec::Vec};

#[derive(Clone)]
pub struct Entry {
    pub label: String,
    pub full_path: String,
    pub is_dir: bool,
    // Byte size for files, filled in by a stat pass after the listing is built;
    // None until known or for directories.
    pub size: Option<u64>,
    // Last-modified time in unix milliseconds, 0 when unknown.
    pub mtime: u64,
    // Whether the entry can be written; false marks a read-only file.
    pub writable: bool,
}

// Turn the vfs path listing into one entry per immediate child of `prefix`,
// de-duplicated. Ordering and filtering are applied later by the view layer.
pub fn build_entries(prefix: &str, paths: &[String]) -> Vec<Entry> {
    let mut out = Vec::new();
    for path in paths {
        let Some(rest) = path.strip_prefix(prefix) else { continue };
        if rest.is_empty() {
            continue;
        }
        let cut = rest.find('/').unwrap_or(rest.len());
        let name = &rest[..cut];
        let is_dir = cut < rest.len();
        // Compare against the bare name: a directory's label carries a trailing
        // slash, so without trimming it a folder with several children would be
        // listed once per child instead of once.
        if name.is_empty()
            || out.iter().any(|entry: &Entry| entry.label.trim_end_matches('/') == name)
        {
            continue;
        }
        let mut label = String::from(name);
        let full_path = if is_dir {
            label.push('/');
            alloc::format!("{prefix}{name}/")
        } else {
            alloc::format!("{prefix}{name}")
        };
        out.push(Entry { label, full_path, is_dir, size: None, mtime: 0, writable: true });
    }
    out
}
