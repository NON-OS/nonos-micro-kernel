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

use super::state::ExceptionFrame;

impl ExceptionFrame {
    pub fn set_return_value(&mut self, value: u64) {
        self.x0 = value;
    }

    pub fn syscall_number(&self) -> u64 {
        self.x8
    }

    pub fn syscall_arg(&self, n: usize) -> u64 {
        match n {
            0 => self.x0,
            1 => self.x1,
            2 => self.x2,
            3 => self.x3,
            4 => self.x4,
            5 => self.x5,
            _ => 0,
        }
    }
}
