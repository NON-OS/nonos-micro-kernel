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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct NvIndex {
    pub value: u32,
}

impl NvIndex {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn is_platform(&self) -> bool {
        (self.value & 0xFF000000) == 0x01C00000
    }

    pub fn is_owner(&self) -> bool {
        (self.value & 0xFF000000) == 0x01400000
    }

    pub fn is_endorsement(&self) -> bool {
        (self.value & 0xFF000000) == 0x01800000
    }
}

impl From<u32> for NvIndex {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<NvIndex> for u32 {
    fn from(index: NvIndex) -> u32 {
        index.value
    }
}
