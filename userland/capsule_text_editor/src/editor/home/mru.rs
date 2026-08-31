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

//! The session most-recently-opened list. There is no mtime in the VFS, so the
//! only ordering the editor can honestly claim is the order it opened things.

use alloc::string::ToString;

use super::super::app::Editor;

const MRU_CAP: usize = 8;

impl Editor {
    pub(in crate::editor) fn mru_note(&mut self, path: &str) {
        if path.is_empty() || path.len() > 255 {
            return;
        }
        if let Some(i) = self.mru.iter().position(|p| p == path) {
            self.mru.remove(i);
        }
        self.mru.insert(0, path.to_string());
        self.mru.truncate(MRU_CAP);
    }
}
