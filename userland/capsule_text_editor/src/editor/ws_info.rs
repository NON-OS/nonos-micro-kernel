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

//! New tabs the menu bar can open: a blank document, and the two reference
//! sheets Help offers. Each is a real editable tab backed by the same State as
//! any file, so the text is searchable, exportable, and saveable.

use super::app::Editor;
use super::info_text::{ABOUT, SHORTCUTS};
use super::state::State;

impl Editor {
    pub(super) fn new_tab(&mut self) {
        let mut d = State::new();
        d.owner_pid = self.owner_pid;
        d.path_len = 0;
        self.docs.push(d);
        self.active = self.docs.len() - 1;
    }

    pub(super) fn info_tab(&mut self, which: usize) {
        let (name, body) = match which {
            0 => ("/shortcuts.md", SHORTCUTS),
            _ => ("/about.md", ABOUT),
        };
        let key = name.as_bytes();
        if let Some(i) = self.docs.iter().position(|d| &d.path[..d.path_len] == key) {
            self.active = i;
            return;
        }
        let mut d = State::new();
        d.owner_pid = self.owner_pid;
        d.path[..name.len()].copy_from_slice(name.as_bytes());
        d.path_len = name.len();
        let n = body.len().min(d.buf.len());
        d.buf[..n].copy_from_slice(&body.as_bytes()[..n]);
        d.len = n;
        d.mode = super::mode::mode_for_path(name);
        d.reflow();
        self.docs.push(d);
        self.active = self.docs.len() - 1;
    }
}
