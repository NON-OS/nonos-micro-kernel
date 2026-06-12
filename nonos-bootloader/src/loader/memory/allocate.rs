// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

extern crate alloc;

use super::table::AllocationTable;
use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::types::memory;
use uefi::table::boot::{AllocateType, BootServices, MemoryType};

pub fn allocate_at_address(
    bs: &BootServices,
    address: u64,
    pages: usize,
    table: &mut AllocationTable,
) -> LoaderResult<u64> {
    if address & (memory::PAGE_SIZE as u64 - 1) != 0 {
        return Err(LoaderError::AddressOutOfRange);
    }
    match bs.allocate_pages(AllocateType::Address(address), MemoryType::LOADER_DATA, pages) {
        Ok(addr) => {
            table.record(addr, pages)?;
            Ok(addr)
        }
        Err(e) => Err(LoaderError::AllocationFailed { addr: address, pages, status: e.status() }),
    }
}

pub fn allocate_anywhere(
    bs: &BootServices,
    pages: usize,
    table: &mut AllocationTable,
) -> LoaderResult<u64> {
    match bs.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages) {
        Ok(addr) => {
            table.record(addr, pages)?;
            Ok(addr)
        }
        Err(e) => Err(LoaderError::AllocationFailed { addr: 0, pages, status: e.status() }),
    }
}

pub fn allocate_below_4gb(
    bs: &BootServices,
    pages: usize,
    table: &mut AllocationTable,
) -> LoaderResult<u64> {
    match bs.allocate_pages(AllocateType::MaxAddress(0xFFFF_FFFF), MemoryType::LOADER_DATA, pages) {
        Ok(addr) => {
            table.record(addr, pages)?;
            Ok(addr)
        }
        Err(_) => allocate_anywhere(bs, pages, table),
    }
}
