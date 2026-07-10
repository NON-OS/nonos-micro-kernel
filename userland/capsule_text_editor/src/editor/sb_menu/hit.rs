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

use super::{MenuAction, SbMenu, MENU_ROW_H, MENU_W};

// The action under a click, or None when the click falls outside the menu.
pub(in crate::editor) fn menu_hit(menu: &SbMenu, x: i32, y: i32) -> Option<MenuAction> {
    if x < 0 || y < 0 {
        return None;
    }
    let (x, y) = (x as u32, y as u32);
    let h = menu.items.len() as u32 * MENU_ROW_H + 2;
    if x < menu.x || x >= menu.x + MENU_W || y < menu.y + 1 || y >= menu.y + h - 1 {
        return None;
    }
    let idx = ((y - menu.y - 1) / MENU_ROW_H) as usize;
    menu.items.get(idx).copied()
}
