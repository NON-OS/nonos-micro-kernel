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

use crate::browser::paint::home_page::{constants, shortcut, shortcut_data};

pub fn shortcut_at(x: i32, y: i32) -> Option<&'static str> {
    if x < 0 || y < 0 {
        return None;
    }
    let (xu, yu) = (x as u32, y as u32);
    for i in 0..shortcut_data::SHORTCUTS.len() as u32 {
        let bx = shortcut::badge_x(i);
        if xu >= bx && xu < bx + constants::BADGE && yu >= constants::BADGE_Y && yu < constants::BADGE_Y + constants::BADGE {
            return Some(shortcut_data::SHORTCUTS[i as usize].url);
        }
    }
    None
}

pub fn search_bar_hit(x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    let (xu, yu) = (x as u32, y as u32);
    xu >= constants::PILL_X && xu < constants::PILL_X + constants::PILL_W && yu >= constants::PILL_Y && yu < constants::PILL_Y + constants::PILL_H
}
