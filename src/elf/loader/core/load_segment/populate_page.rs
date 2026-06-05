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

use core::ptr;

use crate::elf::errors::ElfError;
use crate::memory::addr::VirtAddr;
use crate::memory::frame_alloc;
use crate::memory::layout::DIRECTMAP_BASE;
use crate::memory::paging::manager::map_page_in_asid;
use crate::memory::paging::types::PagePermissions;

const PAGE: usize = 4096;

pub(super) fn populate_page(
    target_asid: u32,
    page_va: VirtAddr,
    perms: PagePermissions,
    dst_off: usize,
    src: &[u8],
) -> Result<(), ElfError> {
    if dst_off >= PAGE {
        return Err(ElfError::MemoryAllocationFailed);
    }
    let frame = frame_alloc::allocate_frame().ok_or(ElfError::MemoryAllocationFailed)?;
    if map_page_in_asid(target_asid, page_va, frame, perms).is_err() {
        frame_alloc::deallocate_frame(frame).map_err(|_| ElfError::MemoryAllocationFailed)?;
        return Err(ElfError::MemoryAllocationFailed);
    }

    let dst = (DIRECTMAP_BASE + frame.as_u64()) as *mut u8;

    unsafe {
        ptr::write_bytes(dst, 0, PAGE);
        if !src.is_empty() {
            let space = PAGE - dst_off;
            let len = if src.len() < space { src.len() } else { space };
            ptr::copy_nonoverlapping(src.as_ptr(), dst.add(dst_off), len);
        }
    }
    Ok(())
}
