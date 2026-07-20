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

use crate::browser::manifest::WIDTH;
use crate::browser::paint::chrome::buttons::Btn;
use crate::browser::paint::chrome::constants;

pub fn toolbar_button_at(x: i32, y: i32) -> Option<Btn> {
    if y < constants::T || y >= constants::T + constants::TOOLBAR_H {
        return None;
    }
    if hit(x, constants::BACK_X) {
        return Some(Btn::Back);
    }
    if hit(x, constants::FWD_X) {
        return Some(Btn::Forward);
    }
    if hit(x, constants::RELOAD_X) {
        return Some(Btn::Reload);
    }
    if hit(x, constants::HOME_X) {
        return Some(Btn::Home);
    }
    if x >= WIDTH as i32 - 44 {
        return Some(Btn::Menu);
    }
    if x >= constants::PILL_L && x < WIDTH as i32 - 52 {
        return Some(Btn::Url);
    }
    None
}

fn hit(x: i32, base: i32) -> bool {
    x >= base && x < base + constants::BTN_W
}
