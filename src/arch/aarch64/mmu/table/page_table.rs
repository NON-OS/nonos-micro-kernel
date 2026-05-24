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

#[repr(C, align(4096))]
#[derive(Clone, Copy)]
pub struct PageTable {
    pub(super) entries: [u64; 512],
}

impl PageTable {
    pub const fn new() -> Self {
        Self { entries: [0; 512] }
    }

    pub fn as_ptr(&self) -> *const u64 {
        self.entries.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u64 {
        self.entries.as_mut_ptr()
    }

    pub fn physical_address(&self) -> u64 {
        self.entries.as_ptr() as u64
    }
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}
