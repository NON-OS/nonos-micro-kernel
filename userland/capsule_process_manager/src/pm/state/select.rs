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

use super::State;

impl State {
    // A click hands back an index into `filtered()`, which reorders and shrinks
    // under sort and filter. Resolve it to a pid at click time: the pid is the
    // only handle that still names the same process one refresh later.
    pub fn select_visible(&mut self, index: usize) {
        let Some(pid) = self.filtered().get(index).map(|r| r.pid) else {
            return;
        };
        self.disarm();
        self.selected_pid = pid;
    }
}
