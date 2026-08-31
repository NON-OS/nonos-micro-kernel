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

//! Whether a point falls on the closed dropdown control.

pub(in crate::editor) fn dropdown_hit(rect: (u32, u32, u32, u32), mx: i32, my: i32) -> bool {
    let (x, y, w, h) = rect;
    mx >= x as i32 && my >= y as i32 && mx < (x + w) as i32 && my < (y + h) as i32
}
