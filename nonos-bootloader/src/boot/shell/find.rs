// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::paths::SHELL_PATHS;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::SearchType;
use uefi::Identify;

pub fn find_shell(bs: &BootServices) -> Option<&'static uefi::CStr16> {
    let handles = bs.locate_handle_buffer(SearchType::ByProtocol(&SimpleFileSystem::GUID)).ok()?;
    for handle in handles.iter() {
        if let Ok(mut fs) = bs.open_protocol_exclusive::<SimpleFileSystem>(*handle) {
            if let Ok(mut root) = fs.open_volume() {
                for path in &SHELL_PATHS {
                    if root.open(path, FileMode::Read, FileAttribute::empty()).is_ok() {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}
