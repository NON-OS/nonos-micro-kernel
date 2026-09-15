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

use nonos_app_skeleton::sidecar;

// Sidebar pins. Bounded so the list always fits the sidebar without scrolling
// machinery of its own.
const FAVORITES_MAX: usize = 32;

#[derive(Default)]
pub struct Favorites {
    paths: Vec<String>,
}

impl Favorites {
    // Ordinal keys are zero-padded so the sidecar's sort-by-key preserves the
    // order the user pinned them in rather than sorting the paths. A
    // corrupted blob can carry the same path under two ordinals, so dedup
    // while capping rather than trusting the record count.
    pub fn from_blob(buf: &[u8]) -> Favorites {
        let mut paths: Vec<String> = Vec::new();
        for (_, p) in sidecar::decode(buf) {
            if paths.len() >= FAVORITES_MAX {
                break;
            }
            if paths.contains(&p) {
                continue;
            }
            paths.push(p);
        }
        Favorites { paths }
    }

    pub fn to_blob(&self) -> Vec<u8> {
        let records: Vec<(String, String)> = self
            .paths
            .iter()
            .enumerate()
            .map(|(i, p)| (alloc::format!("{i:04}"), p.clone()))
            .collect();
        sidecar::encode(&records)
    }

    pub fn add(&mut self, path: &str) -> bool {
        if self.paths.len() >= FAVORITES_MAX || self.paths.iter().any(|p| p == path) {
            return false;
        }
        self.paths.push(String::from(path));
        true
    }

    pub fn remove(&mut self, path: &str) {
        self.paths.retain(|p| p != path);
    }

    pub fn list(&self) -> Vec<&str> {
        self.paths.iter().map(|p| p.as_str()).collect()
    }
}
