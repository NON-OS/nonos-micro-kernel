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

// A path carries at most this many tags, the whole map at most this many
// paths, so a sidecar blob stays inside one vfs write.
pub(crate) const TAG_MAX_PER_PATH: usize = 8;
pub(crate) const TAG_MAX_ASSIGNMENTS: usize = 4096;
const TAG_NAME_MAX: usize = 24;

pub(crate) fn valid_tag(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= TAG_NAME_MAX
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

#[derive(Default)]
pub struct TagMap {
    pub(crate) entries: Vec<(String, Vec<String>)>,
}

impl TagMap {
    pub fn to_blob(&self) -> Vec<u8> {
        let records: Vec<(String, String)> =
            self.entries.iter().map(|(p, t)| (p.clone(), t.join(","))).collect();
        sidecar::encode(&records)
    }
}
