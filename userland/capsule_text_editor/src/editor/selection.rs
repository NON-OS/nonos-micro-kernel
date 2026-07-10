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

//! Selection helpers on the editor state: the ordered byte range, starting and
//! clearing a selection, and deleting it as a unit.

use super::state::State;

impl State {
    /// The selected byte range as (start, end), or None when nothing is
    /// selected (anchor absent or collapsed onto the caret).
    pub fn sel_range(&self) -> Option<(usize, usize)> {
        let a = self.sel_anchor?;
        let (s, e) = (a.min(self.caret), a.max(self.caret));
        if s == e {
            None
        } else {
            Some((s, e))
        }
    }

    /// Drop the anchor onto the caret so subsequent movement starts a fresh
    /// selection from here.
    pub fn begin_sel(&mut self) {
        if self.sel_anchor.is_none() {
            self.sel_anchor = Some(self.caret);
        }
    }

    pub fn clear_sel(&mut self) {
        self.sel_anchor = None;
    }

    /// Delete the selected range (undoable) and place the caret where it was.
    /// Returns true when something was removed.
    pub fn delete_sel(&mut self) -> bool {
        let Some((s, e)) = self.sel_range() else {
            return false;
        };
        self.sel_anchor = None;
        self.apply_edit(s, e - s, &[])
    }
}
