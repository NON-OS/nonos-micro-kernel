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

use super::cursor_size_labels::CURSOR_SIZE_LABELS;
use super::field::Field;
use super::font_size_labels::FONT_SIZE_LABELS;
use super::keyboard_layout_labels::KEYBOARD_LAYOUT_LABELS;
use super::language_labels::LANGUAGE_LABELS;
use super::proxy_mode_labels::PROXY_MODE_LABELS;
use super::theme_labels::THEME_LABELS;
use super::wallpaper_labels::WALLPAPER_LABELS;

pub fn enum_table(field: Field) -> Option<&'static [&'static [u8]]> {
    Some(match field {
        Field::Theme => THEME_LABELS,
        Field::Wallpaper => WALLPAPER_LABELS,
        Field::KeyboardLayout => KEYBOARD_LAYOUT_LABELS,
        Field::Language => LANGUAGE_LABELS,
        Field::FontSize => FONT_SIZE_LABELS,
        Field::CursorSize => CURSOR_SIZE_LABELS,
        Field::ProxyMode => PROXY_MODE_LABELS,
        _ => return None,
    })
}
