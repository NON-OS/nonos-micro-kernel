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

use crate::memory::paging::unmap_page;
use crate::memory::VirtAddr;

use super::super::consts::PAGE_SIZE;

pub(crate) fn rollback_mapped_pages(base_va: u64, installed: usize) {
    for j in 0..installed {
        let va = VirtAddr::new(base_va + (j * PAGE_SIZE) as u64);
        if let Ok(phys) = unmap_page(va) {
            if crate::memory::frame_alloc::deallocate_frame(phys).is_err() {
                crate::sys::serial::println(b"[MMAP] rollback_frame_release_failed");
            }
        }
    }
}
