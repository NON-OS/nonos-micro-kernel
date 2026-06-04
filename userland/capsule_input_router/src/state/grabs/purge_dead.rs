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

use super::{Grab, GrabTable};

impl GrabTable {
    pub fn purge_dead(&mut self) -> u32 {
        let mut purged = 0u32;
        if self.keyboard.holder_pid != 0 && !nonos_libc::mk_pid_alive(self.keyboard.holder_pid) {
            self.keyboard = Grab::default();
            purged += 1;
        }
        if self.pointer.holder_pid != 0 && !nonos_libc::mk_pid_alive(self.pointer.holder_pid) {
            self.pointer = Grab::default();
            purged += 1;
        }
        purged
    }
}
