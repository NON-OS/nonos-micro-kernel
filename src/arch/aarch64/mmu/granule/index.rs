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

use super::kind::Granule;

impl Granule {
    pub const fn block_size(&self, level: usize) -> Option<usize> {
        match (self, level) {
            (Granule::G4K, 1) => Some(1 << 30),
            (Granule::G4K, 2) => Some(1 << 21),
            (Granule::G16K, 2) => Some(1 << 25),
            (Granule::G64K, 2) => Some(1 << 29),
            _ => None,
        }
    }

    pub const fn tcr_granule_bits(&self) -> u64 {
        match self {
            Granule::G4K => 0b00,
            Granule::G16K => 0b10,
            Granule::G64K => 0b01,
        }
    }

    pub fn index_at_level(&self, addr: u64, level: usize) -> usize {
        let shift = self.page_shift() + self.table_shift() * (self.levels() - 1 - level);
        ((addr >> shift) as usize) & (self.entries_per_table() - 1)
    }
}
