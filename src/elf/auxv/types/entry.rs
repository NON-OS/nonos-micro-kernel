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

use super::aux_type;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AuxEntry {
    pub a_type: u64,
    pub a_val: u64,
}

impl AuxEntry {
    pub const SIZE: usize = 16;

    pub const fn new(a_type: u64, a_val: u64) -> Self {
        Self { a_type, a_val }
    }

    pub const fn null() -> Self {
        Self { a_type: aux_type::AT_NULL, a_val: 0 }
    }

    pub fn is_null(&self) -> bool {
        self.a_type == aux_type::AT_NULL
    }
}

impl Default for AuxEntry {
    fn default() -> Self {
        Self::null()
    }
}
