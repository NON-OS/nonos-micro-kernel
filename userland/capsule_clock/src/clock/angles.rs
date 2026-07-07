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

use crate::clock::fixed::{cos_deg, sin_deg, SCALE};

pub fn hour_angle(h: u8, m: u8) -> i32 {
    (h as i32 % 12) * 30 + m as i32 / 2
}

pub fn minute_angle(m: u8, s: u8) -> i32 {
    m as i32 * 6 + s as i32 / 10
}

pub fn second_angle(s: u8) -> i32 {
    s as i32 * 6
}

pub fn hand_end(cx: i32, cy: i32, angle: i32, len: i32) -> (i32, i32) {
    let x = cx + len * sin_deg(angle) / SCALE;
    let y = cy - len * cos_deg(angle) / SCALE;
    (x, y)
}
