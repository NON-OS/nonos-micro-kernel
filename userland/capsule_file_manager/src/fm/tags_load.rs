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

use super::tags::{valid_tag, TagMap, TAG_MAX_ASSIGNMENTS, TAG_MAX_PER_PATH};

impl TagMap {
    // Blobs are untrusted disk state: re-derive every invariant `add` would
    // have enforced rather than trusting the sidecar's records verbatim.
    pub fn from_blob(buf: &[u8]) -> TagMap {
        let mut map = TagMap::default();
        for (path, joined) in sidecar::decode(buf) {
            if map.entries.len() >= TAG_MAX_ASSIGNMENTS {
                break;
            }
            if map.entries.iter().any(|(p, _)| *p == path) {
                continue;
            }
            let mut tags: Vec<String> = Vec::new();
            for raw in joined.split(',').filter(|t| !t.is_empty()) {
                let tag = raw.to_ascii_lowercase();
                if !valid_tag(&tag) || tags.contains(&tag) {
                    continue;
                }
                tags.push(tag);
                if tags.len() >= TAG_MAX_PER_PATH {
                    break;
                }
            }
            if !tags.is_empty() {
                map.entries.push((path, tags));
            }
        }
        map
    }
}
