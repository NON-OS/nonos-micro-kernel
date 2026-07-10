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

//! Carry out a context-menu choice: creation and rename open the sidebar's
//! name entry; delete acts on the vfs immediately and reloads the tree.

use alloc::string::String;

use nonos_app_skeleton::clients::vfs;

use super::app::Editor;
use super::sb_entry::{EntryOp, SbEntry};
use super::sb_menu::MenuAction;

impl Editor {
    pub(super) fn apply_menu_action(&mut self, action: MenuAction, target: Option<usize>) {
        let node = target.and_then(|r| self.tree.visible.get(r)).map(|&ni| &self.tree.nodes[ni]);
        match action {
            MenuAction::NewFile | MenuAction::NewFolder => {
                // Create inside the clicked directory, or at the root when the
                // click was on the empty area.
                let base: String = match node {
                    Some(n) if n.is_dir => n.path.clone(),
                    _ => String::from("/"),
                };
                let op = if action == MenuAction::NewFile {
                    EntryOp::NewFile
                } else {
                    EntryOp::NewFolder
                };
                self.entry = Some(SbEntry { op, base, buf: String::new() });
            }
            MenuAction::Rename => {
                if let Some(n) = node {
                    self.entry = Some(SbEntry {
                        op: EntryOp::Rename,
                        base: n.path.clone(),
                        buf: n.name.clone(),
                    });
                }
            }
            MenuAction::Delete => {
                if let Some(n) = node {
                    let res = if n.is_dir {
                        vfs::rmdir(self.owner_pid, n.path.as_bytes(), true)
                    } else {
                        vfs::unlink(self.owner_pid, n.path.as_bytes())
                    };
                    match res {
                        Ok(()) => self.tree.reload(self.owner_pid),
                        Err(e) => self.tree.status = e,
                    }
                }
            }
        }
    }

    // A committed rename moves any open tab with it, so its next save writes
    // to the new path instead of resurrecting the old one.
    pub(super) fn follow_rename(&mut self, old: &str, new: &str) {
        if new.len() > 255 {
            return;
        }
        for d in self.docs.iter_mut() {
            let path = core::str::from_utf8(&d.path[..d.path_len]).unwrap_or("");
            if path == old {
                d.path[..new.len()].copy_from_slice(new.as_bytes());
                d.path_len = new.len();
            }
        }
    }
}
