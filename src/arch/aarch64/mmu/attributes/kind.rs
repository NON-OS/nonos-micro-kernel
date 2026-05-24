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
pub enum MemoryType {
    DeviceNGnRnE,
    DeviceNGnRE,
    DeviceNGRE,
    DeviceGRE,
    NormalNC,
    NormalWT,
    NormalWB,
}

impl MemoryType {
    pub const fn attr_index(&self) -> u64 {
        match self {
            Self::DeviceNGnRnE => 0,
            Self::DeviceNGnRE => 1,
            Self::DeviceNGRE => 2,
            Self::DeviceGRE => 3,
            Self::NormalNC => 4,
            Self::NormalWT => 5,
            Self::NormalWB => 6,
        }
    }
}
