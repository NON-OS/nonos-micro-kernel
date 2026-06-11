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
pub enum NotifyLevel {
    Info = 0,
    Warn = 1,
    Error = 2,
}

impl NotifyLevel {
    pub fn from_u32(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Info),
            1 => Some(Self::Warn),
            2 => Some(Self::Error),
            _ => None,
        }
    }

    pub fn tint(self) -> u32 {
        match self {
            Self::Info => 0xFF2A_5FAF,
            Self::Warn => 0xFFB8_8A20,
            Self::Error => 0xFFB8_2030,
        }
    }
}
