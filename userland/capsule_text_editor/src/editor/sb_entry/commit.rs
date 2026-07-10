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

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::clients::vfs;

use super::{EntryOp, EntryOutcome, SbEntry};
use crate::editor::tree::parent_of;

// Run the typed name against the vfs: create an empty file, create a folder,
// or rename `base` to a sibling with the new name.
pub(super) fn commit(entry: &SbEntry, owner_pid: u32) -> EntryOutcome {
    let name = entry.buf.trim();
    if name.is_empty() || name.contains('/') {
        return EntryOutcome::Failed("invalid name");
    }
    match entry.op {
        EntryOp::NewFile => {
            let path = join(&entry.base, name);
            match vfs::write_file(owner_pid, path.as_bytes(), &[]) {
                Ok(()) => EntryOutcome::Committed { renamed: None },
                Err(e) => EntryOutcome::Failed(e),
            }
        }
        EntryOp::NewFolder => {
            let path = join(&entry.base, name);
            match vfs::mkdir(owner_pid, path.as_bytes()) {
                Ok(()) => EntryOutcome::Committed { renamed: None },
                Err(e) => EntryOutcome::Failed(e),
            }
        }
        EntryOp::Rename => {
            let new_path = join(parent_of(&entry.base), name);
            if new_path == entry.base {
                return EntryOutcome::Cancelled;
            }
            match vfs::rename(owner_pid, entry.base.as_bytes(), new_path.as_bytes()) {
                Ok(()) => EntryOutcome::Committed { renamed: Some((entry.base.clone(), new_path)) },
                Err(e) => EntryOutcome::Failed(e),
            }
        }
    }
}

// "/" + name stays "/name"; "/src" + name becomes "/src/name".
fn join(dir: &str, name: &str) -> String {
    if dir == "/" {
        format!("/{name}")
    } else {
        format!("{dir}/{name}")
    }
}
