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

//! The pane scrolls, so an item's screen position can be negative while part of
//! it is still on screen. The rounded-rect primitives take unsigned coordinates,
//! so a partly-scrolled item is drawn from the top edge with its height reduced.
//! Its top corners read square for the frames it straddles the edge.

pub fn visible(screen_y: i32, h: u32, view_h: u32) -> Option<(u32, u32)> {
    if h == 0 || screen_y >= view_h as i32 {
        return None;
    }
    let bottom = screen_y + h as i32;
    if bottom <= 0 {
        return None;
    }
    let top = screen_y.max(0) as u32;
    let height = (bottom.min(view_h as i32) as u32).saturating_sub(top);
    if height == 0 {
        None
    } else {
        Some((top, height))
    }
}
