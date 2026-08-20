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

use super::field::Field;
use super::kind::{KIND_BOOL, KIND_I8, KIND_STR, KIND_U8};

pub fn kind_of(field: Field) -> u8 {
    match field {
        Field::Brightness
        | Field::MouseSensitivity
        | Field::Theme
        | Field::KeyboardLayout
        | Field::ScreenTimeout
        | Field::Language
        | Field::FontSize
        | Field::AutoLockTimeout
        | Field::CursorSize
        | Field::Wallpaper
        | Field::ProxyMode
        | Field::Volume
        | Field::AudioBalance => KIND_U8,
        Field::Timezone => KIND_I8,
        Field::Hostname | Field::DomainName => KIND_STR,
        _ => KIND_BOOL,
    }
}
