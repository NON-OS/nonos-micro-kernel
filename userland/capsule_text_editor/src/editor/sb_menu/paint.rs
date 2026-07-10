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

use super::{SbMenu, MENU_ROW_H, MENU_W};
use crate::editor::layout::CHROME_PX;
use crate::editor::theme::{FOREGROUND, LINE, TAB_INACTIVE_BG};

// Draw the context menu over whatever is beneath it, one row per action.
pub(in crate::editor) fn paint_menu(fb: &mut PaintBuffer, menu: &SbMenu) {
    let h = menu.items.len() as u32 * MENU_ROW_H + 2;
    fb.fill_rect(menu.x, menu.y, MENU_W, h, TAB_INACTIVE_BG);
    fb.fill_rect(menu.x, menu.y, MENU_W, 1, LINE);
    fb.fill_rect(menu.x, menu.y + h - 1, MENU_W, 1, LINE);
    fb.fill_rect(menu.x, menu.y, 1, h, LINE);
    fb.fill_rect(menu.x + MENU_W - 1, menu.y, 1, h, LINE);
    for (i, item) in menu.items.iter().enumerate() {
        let iy = menu.y + 1 + i as u32 * MENU_ROW_H;
        let _ =
            fb.text_ttf((menu.x + 14) as i32, (iy + 6) as i32, item.label(), FOREGROUND, CHROME_PX);
    }
}
