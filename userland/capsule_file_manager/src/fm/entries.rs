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
}

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
        if name.is_empty() || out.iter().any(|entry: &Entry| entry.label.as_str() == name) {
            continue;
        }
        let mut label = String::from(name);
        let full_path = if is_dir {
            label.push('/');
            alloc::format!("{prefix}{name}/")
        } else {
            alloc::format!("{prefix}{name}")
        };
        out.push(Entry { label, full_path, is_dir });
    }
    out
}
