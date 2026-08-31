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
use super::ui::keypad_hit;

pub const PADDING: u32 = 12;

pub fn hit_test(mode: Mode, win_w: i32, win_h: i32, x: i32, y: i32) -> Option<(usize, usize)> {
    keypad_hit::at(mode, win_w, win_h, x, y)
}
