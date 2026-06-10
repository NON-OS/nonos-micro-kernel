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

// Implicit pointer grab armed by a button press inside a window. The
// origin is the window's screen position frozen at press time, so the
// grab holder receives motion in a coordinate frame where deltas equal
// screen deltas even while it moves itself.
#[derive(Clone, Copy)]
pub struct Press {
    pub pid: u32,
    pub origin_x: i32,
    pub origin_y: i32,
}
