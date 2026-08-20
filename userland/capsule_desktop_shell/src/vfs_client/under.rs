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

//! Count what a folder holds from the LIST reply the desktop already has. The
//! server answers with the whole subtree, so a folder's direct children are
//! on the wire and cost no second round trip.

use alloc::string::String;

pub(super) fn relative(prefix: &str, raw: &str) -> Option<String> {
    let trimmed = raw.trim_end_matches('/');
    let rel = if let Some(stripped) = trimmed.strip_prefix(prefix) {
        stripped.trim_start_matches('/')
    } else if !trimmed.starts_with('/') {
        trimmed.trim_start_matches('/')
    } else {
        return None;
    };
    if rel.is_empty() {
        return None;
    }
    Some(String::from(rel))
}

pub(super) fn count_under(rels: &[String], name: &str) -> u32 {
    rels.iter().filter(|rel| is_direct_child(rel, name)).count() as u32
}

fn is_direct_child(rel: &str, name: &str) -> bool {
    match rel.strip_prefix(name).and_then(|tail| tail.strip_prefix('/')) {
        Some(leaf) => !leaf.is_empty() && !leaf.contains('/'),
        None => false,
    }
}
