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

use nonos_app_skeleton::PaintBuffer;

/// The disclosure mark after an inline choice. Drawn rather than typed so it
/// keeps its weight at any size the row is laid out at.
pub fn draw(fb: &mut PaintBuffer, right_x: u32, cy: i32, argb: u32) {
    let w = 9i32;
    let h = 5i32;
    let x = right_x as i32 - w;
    let top = cy - h / 2;
    fb.polyline_aa(&[(x, top), (x + w / 2, top + h), (x + w, top)], argb);
}
