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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainId(u16);

impl DomainId {
    pub const fn new(id: u16) -> Self {
        Self(id)
    }

    pub const fn as_u16(&self) -> u16 {
        self.0
    }
}

/// A PCI requester id, bus in the high byte and device and function below it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceId(u16);

impl SourceId {
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    pub const fn as_u16(&self) -> u16 {
        self.0
    }

    pub const fn bus(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    pub const fn device(&self) -> u8 {
        ((self.0 >> 3) & 0x1F) as u8
    }

    pub const fn function(&self) -> u8 {
        (self.0 & 0x7) as u8
    }
}
