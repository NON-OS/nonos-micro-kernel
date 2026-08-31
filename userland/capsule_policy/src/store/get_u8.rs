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

use nonos_policy_proto::Field;

use super::state::STORE;

pub fn get(field: Field) -> Option<u8> {
    let s = STORE.lock();
    Some(match field {
        Field::Brightness => s.brightness,
        Field::MouseSensitivity => s.mouse_sensitivity,
        Field::Theme => s.theme,
        Field::KeyboardLayout => s.keyboard_layout,
        Field::ScreenTimeout => s.screen_timeout,
        Field::Language => s.language,
        Field::FontSize => s.font_size,
        Field::AutoLockTimeout => s.auto_lock_timeout,
        Field::CursorSize => s.cursor_size,
        Field::Wallpaper => s.wallpaper,
        Field::ProxyMode => s.proxy_mode,
        Field::Volume => s.volume,
        Field::AudioBalance => s.audio_balance,

        _ => return None,
    })
}
