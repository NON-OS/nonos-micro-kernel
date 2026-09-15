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

use super::types::Store;

// A path is under `prefix` only if `prefix` is a full path segment of it, not
// merely a string prefix: `/docs` must not match a sibling like `/docs2/x`.
fn in_subtree(name: &str, prefix: &str) -> bool {
    if !name.starts_with(prefix) || name.len() == prefix.len() {
        return false;
    }
    prefix == "/" || name.as_bytes()[prefix.len()] == b'/'
}

impl Store {
    // Recursive occupancy under `prefix`: file count, directory count, and the
    // summed size of the files. Stops after `max_nodes` entries and reports
    // truncation so a caller can render a lower bound rather than a wrong total.
    pub fn dirstat(&self, prefix: &str, max_nodes: usize) -> (u32, u32, u64, bool) {
        let mut files = 0u32;
        let mut dirs = 0u32;
        let mut bytes = 0u64;
        let mut seen = 0usize;
        for f in self.files.iter() {
            if !in_subtree(&f.name, prefix) {
                continue;
            }
            if seen >= max_nodes {
                return (files, dirs, bytes, true);
            }
            seen += 1;
            if f.is_dir {
                dirs += 1;
            } else {
                files += 1;
                bytes += f.data.len() as u64;
            }
        }
        (files, dirs, bytes, false)
    }
}
