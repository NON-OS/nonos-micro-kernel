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

// Map one 4 KiB page at a page-aligned target VA, zero it, then copy
// up to (PAGE - dst_off) bytes from `src` starting at `dst_off`
// inside the new frame. Used by `load_segment` to honour a
// `p_vaddr` whose intra-page offset is non-zero: the segment's
// first page lays its data down at offset (p_vaddr & 0xFFF), every
// later page starts at offset 0.
pub(super) fn populate_page(
    target_asid: u32,
    page_va: VirtAddr,
    perms: PagePermissions,
    dst_off: usize,
    src: &[u8],
) -> Result<(), ElfError> {
    debug_assert!(dst_off < PAGE);
    let frame = frame_alloc::allocate_frame().ok_or(ElfError::MemoryAllocationFailed)?;
    if map_page_in_asid(target_asid, page_va, frame, perms).is_err() {
        frame_alloc::deallocate_frame(frame).map_err(|_| ElfError::MemoryAllocationFailed)?;
        return Err(ElfError::MemoryAllocationFailed);
    }

    let dst = (DIRECTMAP_BASE + frame.as_u64()) as *mut u8;

    // SAFETY: eK@nonos.systems — frame phys came from frame_alloc, so
    // DIRECTMAP_BASE + phys is a fresh 4 KiB page we own.
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
