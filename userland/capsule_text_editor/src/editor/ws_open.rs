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

//! Open a tree path into a document tab (reusing an existing tab for the same
//! file) and close tabs, always leaving at least one document open.

use super::app::Editor;
use super::ctrl_open::ctrl_open;
use super::state::State;

fn doc_path(d: &State) -> &str {
    core::str::from_utf8(&d.path[..d.path_len]).unwrap_or("")
}

impl Editor {
    pub(super) fn open_path(&mut self, path: &str) {
        if path.len() > 255 {
            return;
        }
        self.mru_note(path);
        if let Some(i) = self.docs.iter().position(|d| doc_path(d) == path) {
            self.active = i;
            return;
        }
        let mut d = State::new();
        d.path[..path.len()].copy_from_slice(path.as_bytes());
        d.path_len = path.len();
        d.owner_pid = self.owner_pid;
        ctrl_open(&mut d);
        self.docs.push(d);
        self.active = self.docs.len() - 1;
    }

    pub(super) fn close_tab(&mut self, idx: usize) {
        if idx >= self.docs.len() {
            return;
        }
        self.docs.remove(idx);
        if self.docs.is_empty() {
            self.docs.push(State::new());
            self.active = 0;
            return;
        }
        if self.active > idx || self.active >= self.docs.len() {
            self.active = self.active.saturating_sub(1).min(self.docs.len() - 1);
        }
    }
}
