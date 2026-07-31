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

#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub base: u64,
    pub size: u64,
    pub region_type: MemoryType,
}

impl MemoryRegion {
    /// A zero-length entry, for padding fixed storage before it is filled.
    pub const EMPTY: Self =
        Self { base: 0, size: 0, region_type: MemoryType::Reserved };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    Available,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    Unusable,
    Kernel,
    DeviceMemory,
}
