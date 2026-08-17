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

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::list_paths;

use super::media::MediaItem;

pub const MAX_ENTRIES: usize = 256;

pub const ROOTS: [&[u8]; 5] = [b"/", b"/Movies", b"/Series", b"/Downloads", b"/Clips"];

pub fn scan(owner_pid: u32) -> Vec<MediaItem> {
    let mut out: Vec<MediaItem> = Vec::new();
    for root in ROOTS {
        collect(owner_pid, root, &mut out);
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out.truncate(MAX_ENTRIES);
    out
}

fn collect(owner_pid: u32, root: &[u8], out: &mut Vec<MediaItem>) {
    let Ok(paths) = list_paths(owner_pid, root) else {
        return;
    };
    for path in paths {
        if out.iter().any(|m| m.path == path) {
            continue;
        }
        if let Some(item) = MediaItem::from_path(&path) {
            out.push(item);
        }
    }
}
