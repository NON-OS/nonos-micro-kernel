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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyField {
    KernelPreempt = 0x0001,
    TimezoneOffset = 0x0002,
    Hostname = 0x0003,
    DomainName = 0x0004,
}

impl PolicyField {
    pub const fn from_u32(value: u32) -> Option<Self> {
        match value {
            0x0001 => Some(Self::KernelPreempt),
            0x0002 => Some(Self::TimezoneOffset),
            0x0003 => Some(Self::Hostname),
            0x0004 => Some(Self::DomainName),
            _ => None,
        }
    }
}
