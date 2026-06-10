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
pub struct TmpHandle {
    pub value: u32,
}

impl TmpHandle {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn is_valid(&self) -> bool {
        self.value != 0 && self.value != 0xFFFFFFFF
    }

    pub fn is_persistent(&self) -> bool {
        (self.value & 0xFF000000) == 0x81000000
    }

    pub fn is_transient(&self) -> bool {
        (self.value & 0xFF000000) == 0x80000000
    }
}

impl From<u32> for TmpHandle {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<TmpHandle> for u32 {
    fn from(handle: TmpHandle) -> u32 {
        handle.value
    }
}
