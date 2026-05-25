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

//! DMA grant revocation. Three triggers:
//!
//!   * `MkDmaUnmap` — explicit holder request
//!   * `MkDeviceRelease` — drains every grant tied to the device
//!   * process exit — drains every grant the dying pid owns
//!
//! Revocation order: scrub the buffer, unmap user pages (when the
//! holder's CR3 is active so the unmap is in-context), free the
//! physical frame back to the allocator. The cross-pid teardown
//! path skips the unmap because dereferencing a foreign address
//! space would walk the wrong page tables; the AS reaper drops
//! those PTEs wholesale.

use super::records;
use super::pool;
use super::types::{DmaError, DmaGrant};
use crate::memory::addr::VirtAddr;
use crate::memory::layout::DIRECTMAP_BASE;
use crate::memory::phys::free_contiguous;

const PAGE_SIZE: u64 = 4096;

pub fn unmap_grant(pid: u32, grant_id: u64) -> Result<(), DmaError> {
    let g = records::remove(pid, grant_id)?;
    teardown(&g, true);
    Ok(())
}

pub fn release_for_device(pid: u32, device_id: u64) -> usize {
    let drained = records::drain_for_device(pid, device_id);
    for g in &drained {
        teardown(g, true);
    }
    drained.len()
}

pub fn release_all_for_pid(pid: u32, unmap_pages: bool) -> usize {
    let drained = records::drain_for_pid(pid);
    for g in &drained {
        teardown(g, unmap_pages);
    }
    drained.len()
}

fn teardown(g: &DmaGrant, unmap_pages: bool) {
    scrub_buffer(g.physical_start, g.length);
    if unmap_pages {
        let _ = crate::memory::paging::unmap_user_dma(VirtAddr::new(g.user_va), g.length as usize);
    }
    let pages = (g.length / PAGE_SIZE) as usize;
    if !pool::free(g.physical_start, pages) {
        let _ = free_contiguous(g.physical_start, pages);
    }
}

// Scrub the page through the kernel direct map before returning
// the frame to the global allocator. The next consumer of this
// frame must not see whatever the previous holder left there.
//
// SAFETY: eK@nonos.systems — `physical_start` came from
// `allocate_frame` and is only ever referenced through the broker
// grant table. The grant is removed from the records before this
// runs, so no other path can race on the same VA.
fn scrub_buffer(physical_start: u64, length: u64) {
    let kva = (DIRECTMAP_BASE + physical_start) as *mut u64;
    let words = (length / 8) as usize;
    unsafe {
        for i in 0..words {
            core::ptr::write_volatile(kva.add(i), 0);
        }
    }
}
