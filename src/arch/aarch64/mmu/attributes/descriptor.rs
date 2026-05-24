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

use super::page::PageAttributes;

impl PageAttributes {
    pub fn to_descriptor_bits(&self) -> u64 {
        let mut bits = self.memory_type.attr_index() << 2;
        if !self.user {
            bits |= 1 << 6;
        }
        if !self.write {
            bits |= 1 << 7;
        }
        bits |= 0b11 << 8;
        if self.accessed {
            bits |= 1 << 10;
        }
        if !self.global {
            bits |= 1 << 11;
        }
        if self.contiguous {
            bits |= 1 << 52;
        }
        if !self.execute && self.user {
            bits |= 1 << 53;
        }
        if !self.execute && !self.user {
            bits |= 1 << 54;
        }
        bits
    }
}
