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

use alloc::string::String;

use super::tags::TagMap;

// A path is under `prefix` only if `prefix` is a full path segment of it, not
// merely a string prefix: `/docs` must not match a sibling like `/docs2/x`.
// `State::prefix` carries a trailing slash, so the separator is trimmed before
// the segment check rather than being counted twice.
fn in_subtree(path: &str, prefix: &str) -> bool {
    let base = prefix.trim_end_matches('/');
    if !path.starts_with(base) || path.len() == base.len() {
        return false;
    }
    base.is_empty() || path.as_bytes()[base.len()] == b'/'
}

// Drop tag assignments whose path no longer exists. Scoped to `prefix` because
// `live` only lists the directory currently loaded, so a global sweep would
// delete every tag outside the view.
pub fn reconcile(map: &mut TagMap, prefix: &str, live: &[String]) {
    let stale: alloc::vec::Vec<String> = map
        .tagged_paths()
        .into_iter()
        .filter(|p| in_subtree(p, prefix) && !live.iter().any(|l| l == p))
        .map(String::from)
        .collect();
    for path in stale {
        map.drop_path(&path);
    }
}
