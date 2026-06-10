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

use uefi::table::boot::{AllocateType, BootServices, MemoryType};

use super::constants::PAGE_SIZE;

// Allocate one 4-KiB page-table frame from UEFI Boot Services
// memory. Frames must be allocated below 4 GiB so the kernel can
// reach them through the eventual directmap; AnyPages on UEFI
// typically returns low memory but the explicit ceiling makes the
// invariant observable. Returns the physical address; UEFI
// identity-mapping means it doubles as a writable VA while Boot
// Services are still active.
pub fn alloc_pt_frame(bs: &BootServices) -> Result<u64, &'static str> {
    let r = bs.allocate_pages(AllocateType::MaxAddress(0xFFFF_FFFF), MemoryType::LOADER_DATA, 1);
    let addr = r.map_err(|_| "alloc_pt_frame: UEFI allocate_pages failed")?;
    if addr & (PAGE_SIZE - 1) != 0 {
        return Err("alloc_pt_frame: UEFI returned misaligned frame");
    }
    let table = unsafe { &mut *(addr as *mut [u64; 512]) };
    for entry in table.iter_mut() {
        *entry = 0;
    }
    Ok(addr)
}
