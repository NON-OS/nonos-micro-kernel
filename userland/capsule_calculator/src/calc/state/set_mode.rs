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

use super::{ErrorKind, State};
use crate::calc::mode::Mode;

impl State {
    pub fn set_mode(&mut self, mode: Mode) {
        if mode == Mode::Programmer && self.mode != Mode::Programmer {
            self.enter_programmer();
        } else if self.mode == Mode::Programmer && mode != Mode::Programmer {
            self.leave_programmer();
        }
        self.mode = mode;
        self.hover = None;
        self.new_input = true;
        self.decimal_digits_typed = 0;
        self.error = ErrorKind::None;
    }
}
