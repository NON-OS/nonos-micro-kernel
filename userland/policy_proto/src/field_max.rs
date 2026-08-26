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

use super::enum_table::enum_table;
use super::field::Field;

pub fn max_of(field: Field) -> u8 {
    if let Some(table) = enum_table(field) {
        return (table.len() as u8).saturating_sub(1);
    }
    match field {
        Field::Brightness => 100,
        Field::MouseSensitivity => 4,
        Field::ScreenTimeout => 240,
        Field::AutoLockTimeout => 240,
        Field::Volume => 100,
        Field::AudioBalance => 100,
        _ => 0,
    }
}
