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
    pub fn request(&mut self, pid: u32, kind_mask: u32) -> bool {
        if kind_mask & 0b0000_0011 != 0 {
            if self.keyboard.holder_pid != 0 && self.keyboard.holder_pid != pid {
                return false;
            }
            self.keyboard = Grab { holder_pid: pid, kind_mask };
        }
        if kind_mask & 0b1111_1100 != 0 {
            if self.pointer.holder_pid != 0 && self.pointer.holder_pid != pid {
                return false;
            }
            self.pointer = Grab { holder_pid: pid, kind_mask };
        }
        true
    }
}
