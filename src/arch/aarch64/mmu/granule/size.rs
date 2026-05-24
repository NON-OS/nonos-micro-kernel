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
    pub const fn page_size(&self) -> usize {
        match self {
            Granule::G4K => 4096,
            Granule::G16K => 16384,
            Granule::G64K => 65536,
        }
    }

    pub const fn page_shift(&self) -> usize {
        match self {
            Granule::G4K => 12,
            Granule::G16K => 14,
            Granule::G64K => 16,
        }
    }

    pub const fn entries_per_table(&self) -> usize {
        match self {
            Granule::G4K => 512,
            Granule::G16K => 2048,
            Granule::G64K => 8192,
        }
    }

    pub const fn table_shift(&self) -> usize {
        match self {
            Granule::G4K => 9,
            Granule::G16K => 11,
            Granule::G64K => 13,
        }
    }

    pub const fn levels(&self) -> usize {
        match self {
            Granule::G4K => 4,
            Granule::G16K => 4,
            Granule::G64K => 3,
        }
    }
}
