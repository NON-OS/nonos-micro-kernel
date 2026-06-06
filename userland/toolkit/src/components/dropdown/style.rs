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
use crate::components::list::ListStyle;
use crate::design::color::Argb;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DropdownStyle {
    pub bg: Argb,
    pub fg: Argb,
    pub list: ListStyle,
}

impl Default for DropdownStyle {
    fn default() -> Self {
        Self {
            bg: Argb::from_channels(0xFF, 0x1A, 0x22, 0x30),
            fg: Argb::from_channels(0xFF, 0xEA, 0xEE, 0xF3),
            list: ListStyle::default(),
        }
    }
}
