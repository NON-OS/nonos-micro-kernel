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

const KEYBOARD_BITS: u32 = 0b0000_0011;
const POINTER_BITS: u32 = 0b1111_1100;

impl GrabTable {
    pub fn request(&mut self, pid: u32, kind_mask: u32) -> bool {
        if kind_mask & KEYBOARD_BITS != 0 {
            if self.keyboard.holder_pid != 0 && self.keyboard.holder_pid != pid {
                return false;
            }
            // Store only the keyboard bits so holder_for cannot cross-match a
            // pointer kind against this grab if the caller passed a mixed mask.
            self.keyboard = Grab { holder_pid: pid, kind_mask: kind_mask & KEYBOARD_BITS };
        }
        if kind_mask & POINTER_BITS != 0 {
            if self.pointer.holder_pid != 0 && self.pointer.holder_pid != pid {
                return false;
            }
            self.pointer = Grab { holder_pid: pid, kind_mask: kind_mask & POINTER_BITS };
        }
        true
    }
}
