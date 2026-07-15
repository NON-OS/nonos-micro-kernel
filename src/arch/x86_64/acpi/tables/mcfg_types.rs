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

use super::sdt::{sdt_entry_count, SdtHeader};
use core::mem;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Mcfg {
    pub header: SdtHeader,
    pub reserved: u64,
}

impl Mcfg {
    pub fn entry_count(&self) -> usize {
        sdt_entry_count(
            self.header.length as usize,
            mem::size_of::<Self>(),
            mem::size_of::<McfgEntry>(),
        )
    }
    pub fn entries_offset(&self) -> usize {
        mem::size_of::<Self>()
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct McfgEntry {
    pub base_address: u64,
    pub segment_group: u16,
    pub start_bus: u8,
    pub end_bus: u8,
    pub reserved: u32,
}

impl McfgEntry {
    pub fn config_address(&self, bus: u8, device: u8, function: u8, offset: u16) -> Option<u64> {
        if bus < self.start_bus || bus > self.end_bus {
            return None;
        }
        if device >= 32 || function >= 8 || offset >= 4096 {
            return None;
        }
        Some(
            self.base_address
                + ((bus as u64) << 20)
                + ((device as u64) << 15)
                + ((function as u64) << 12)
                + (offset as u64),
        )
    }
    pub fn bus_count(&self) -> u16 {
        (self.end_bus as u16) - (self.start_bus as u16) + 1
    }
    pub fn contains_bus(&self, bus: u8) -> bool {
        bus >= self.start_bus && bus <= self.end_bus
    }
    pub fn memory_size(&self) -> u64 {
        (self.bus_count() as u64) << 20
    }
}
