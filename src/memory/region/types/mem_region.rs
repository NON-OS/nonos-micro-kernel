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

use super::region_flags::RegionFlags;
use super::region_type::RegionType;
use crate::memory::addr::VirtAddr;

#[derive(Debug, Clone, Copy)]
pub struct MemRegion {
    pub start: u64,
    pub size: usize,
    pub region_type: RegionType,
    pub flags: u32,
    pub creation_time: u64,
    pub access_count: u64,
}

impl MemRegion {
    pub const fn new(start: u64, size: usize, region_type: RegionType) -> Self {
        Self { start, size, region_type, flags: 0, creation_time: 0, access_count: 0 }
    }
    pub fn start_addr(&self) -> VirtAddr {
        VirtAddr::new(self.start)
    }
    pub const fn end(&self) -> u64 {
        self.start + self.size as u64
    }
    pub fn end_addr(&self) -> VirtAddr {
        VirtAddr::new(self.end())
    }
    pub const fn size_bytes(&self) -> u64 {
        self.size as u64
    }
    pub const fn contains(&self, addr: u64) -> bool {
        super::super::overlap::contains(self.start, self.end(), addr)
    }
    pub const fn contains_range(&self, other: &MemRegion) -> bool {
        super::super::overlap::contains_range(self.start, self.end(), other.start, other.end())
    }
    pub const fn overlaps(&self, other: &MemRegion) -> bool {
        super::super::overlap::overlaps(self.start, self.end(), other.start, other.end())
    }
    pub fn has_flag(&self, flag: RegionFlags) -> bool {
        (self.flags & flag.bit()) != 0
    }
    pub fn set_flag(&mut self, flag: RegionFlags) {
        self.flags |= flag.bit();
    }
    pub fn clear_flag(&mut self, flag: RegionFlags) {
        self.flags &= !flag.bit();
    }
    pub const fn is_valid(&self) -> bool {
        self.size > 0 && self.start < self.end()
    }
    pub const fn is_available(&self) -> bool {
        matches!(self.region_type, RegionType::Available)
    }
}

impl Default for MemRegion {
    fn default() -> Self {
        Self::new(0, 0, RegionType::Available)
    }
}
