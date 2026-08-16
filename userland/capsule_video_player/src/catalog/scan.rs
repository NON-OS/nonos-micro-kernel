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

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::list_paths;

use super::entry::is_playable;

pub const MAX_ENTRIES: usize = 256;

pub fn scan(owner_pid: u32) -> Vec<String> {
    let mut out: Vec<String> = match list_paths(owner_pid, b"/") {
        Ok(paths) => paths.into_iter().filter(|p| is_playable(p)).collect(),
        Err(_) => Vec::new(),
    };
    out.sort();
    out.truncate(MAX_ENTRIES);
    out
}
