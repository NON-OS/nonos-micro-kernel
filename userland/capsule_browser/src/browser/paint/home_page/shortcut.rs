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

use crate::browser::paint::home_page::{constants, shortcut_data};

pub fn shortcut(fb: &mut PaintBuffer, i: u32) {
    let s = &shortcut_data::SHORTCUTS[i as usize];
    let cx = center_x(fb.width, i);
    let bx = cx.saturating_sub(constants::BADGE / 2);
    fb.fill_rect(bx, constants::BADGE_Y, constants::BADGE, constants::BADGE, s.color);
    fb.fill_rect(bx, constants::BADGE_Y, 6, 6, constants::PAGE_BG);
    fb.fill_rect(bx + constants::BADGE - 6, constants::BADGE_Y, 6, 6, constants::PAGE_BG);
    fb.fill_rect(bx, constants::BADGE_Y + constants::BADGE - 6, 6, 6, constants::PAGE_BG);
    fb.fill_rect(
        bx + constants::BADGE - 6,
        constants::BADGE_Y + constants::BADGE - 6,
        6,
        6,
        constants::PAGE_BG,
    );
    let badge = core::str::from_utf8(s.badge).unwrap_or("");
    let bw = fb.measure_ttf(badge, 26.0);
    fb.text_ttf(
        cx as i32 - bw / 2,
        (constants::BADGE_Y + 14) as i32,
        badge,
        constants::WHITE,
        26.0,
    );
    let label = core::str::from_utf8(s.label).unwrap_or("");
    let lw = fb.measure_ttf(label, 15.0);
    fb.text_ttf(cx as i32 - lw / 2, 366, label, constants::FG, 15.0);
}

pub fn center_x(width: u32, i: u32) -> u32 {
    let row = constants::COUNT.saturating_mul(constants::CELL_W);
    width.saturating_sub(row) / 2 + i * constants::CELL_W + constants::CELL_W / 2
}
