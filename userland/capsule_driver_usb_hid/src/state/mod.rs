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

use crate::hid::{Keyboard, Mouse, Tablet};

pub struct State {
    pub keyboard: Keyboard,
    pub mouse: Mouse,
    pub tablet: Tablet,
    pub configs_probed: u64,
    pub key_reports: u64,
    pub mouse_reports: u64,
}

impl State {
    pub fn new() -> Self {
        Self {
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
            tablet: Tablet::new(),
            configs_probed: 0,
            key_reports: 0,
            mouse_reports: 0,
        }
    }
}
