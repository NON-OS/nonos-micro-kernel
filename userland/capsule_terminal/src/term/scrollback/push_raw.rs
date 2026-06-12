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

use super::role::Role;
use super::types::Scrollback;
use crate::term::dimensions::{COLS, SCROLLBACK_ROWS};

impl Scrollback {
    // Append one line to the visible ring, tagged with its render role, and
    // reset the scroll view to the bottom. Shared by push_line and push_error.
    pub(super) fn push_raw(&mut self, line: &[u8], role: Role) {
        let slot = (self.head + self.count) % SCROLLBACK_ROWS;
        let n = line.len().min(COLS);
        self.rows[slot][..n].copy_from_slice(&line[..n]);
        self.lengths[slot] = n as u16;
        self.roles[slot] = role;
        if self.count == SCROLLBACK_ROWS {
            self.head = (self.head + 1) % SCROLLBACK_ROWS;
        } else {
            self.count += 1;
        }
        self.view_offset = 0;
    }
}
