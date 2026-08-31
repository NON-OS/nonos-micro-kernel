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

use nonos_policy_proto::{max_of, Field};

use super::state::STORE;

pub fn set(field: Field, value: u8) -> bool {
    let max = max_of(field);
    if max > 0 && value > max {
        return false;
    }
    let mut s = STORE.lock();
    match field {
        Field::Brightness => s.brightness = value,
        Field::MouseSensitivity => s.mouse_sensitivity = value,
        Field::Theme => s.theme = value,
        Field::KeyboardLayout => s.keyboard_layout = value,
        Field::ScreenTimeout => s.screen_timeout = value,
        Field::Language => s.language = value,
        Field::FontSize => s.font_size = value,
        Field::AutoLockTimeout => s.auto_lock_timeout = value,
        Field::CursorSize => s.cursor_size = value,
        Field::Wallpaper => s.wallpaper = value,
        Field::ProxyMode => s.proxy_mode = value,
        Field::Volume => s.volume = value,
        Field::AudioBalance => s.audio_balance = value,

        _ => return false,
    }
    true
}
