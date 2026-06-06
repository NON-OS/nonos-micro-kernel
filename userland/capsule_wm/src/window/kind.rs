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

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Normal = 0,
    Dialog = 1,
    Tooltip = 2,
    Popup = 3,
}

impl Kind {
    pub fn focusable(self) -> bool {
        matches!(self, Self::Normal | Self::Dialog | Self::Popup)
    }
}

pub fn from_u32(raw: u32) -> Option<Kind> {
    match raw {
        0 => Some(Kind::Normal),
        1 => Some(Kind::Dialog),
        2 => Some(Kind::Tooltip),
        3 => Some(Kind::Popup),
        _ => None,
    }
}
