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

use core::sync::atomic::{AtomicU32, Ordering};

use alloc::string::ToString;

use nonos_app_skeleton::clients::vfs::list_paths;

use super::{FileTree, Node};

const RETRY_FRAMES: u32 = 120;

static RETRY_GATE: AtomicU32 = AtomicU32::new(0);

impl FileTree {
    // (Re)read the whole store and rebuild the visible rows, keeping whatever
    // directories were expanded before.
    pub fn reload(&mut self, owner_pid: u32) {
        let waiting = RETRY_GATE.load(Ordering::Relaxed);
        if waiting > 0 {
            RETRY_GATE.store(waiting - 1, Ordering::Relaxed);
            return;
        }
        match list_paths(owner_pid, b"/") {
            Ok(paths) => {
                self.nodes.clear();
                for p in paths {
                    let is_dir = p.ends_with('/');
                    let canon = if is_dir { p.trim_end_matches('/') } else { p.as_str() };
                    if canon.is_empty() {
                        continue;
                    }
                    let name = canon.rsplit('/').next().unwrap_or(canon).to_string();
                    let depth = canon.matches('/').count() as u16;
                    self.nodes.push(Node { path: canon.to_string(), name, is_dir, depth });
                }
                self.loaded = true;
                self.status = "";
                self.rebuild_visible();
            }
            Err(e) => {
                self.status = e;
                self.loaded = false;
                RETRY_GATE.store(RETRY_FRAMES, Ordering::Relaxed);
            }
        }
    }
}
