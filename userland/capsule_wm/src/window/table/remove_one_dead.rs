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

use crate::window::Window;

use super::WindowTable;

impl WindowTable {
    pub fn remove_one_dead(&mut self) -> Option<Window> {
        for slot in self.entries.iter_mut() {
            if slot.in_use && !nonos_libc::mk_pid_alive(slot.owner_pid) {
                let copy = *slot;
                *slot = Window::default();
                return Some(copy);
            }
        }
        None
    }
}
