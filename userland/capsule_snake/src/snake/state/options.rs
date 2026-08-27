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

use super::mode::Mode;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Options {
    pub obstacles: bool,
    pub wrap: bool,
    pub powerups: bool,
}

impl Options {
    pub fn new() -> Self {
        Options { obstacles: true, wrap: false, powerups: true }
    }

    // Zen forces wrapping on and Classic forces it off; otherwise the toggle
    // is the whole story. Nothing else is coupled.
    pub fn wraps(&self, mode: Mode) -> bool {
        if mode.forces_wrap() {
            return true;
        }
        if mode.hard_walls() {
            return false;
        }
        self.wrap
    }
}
