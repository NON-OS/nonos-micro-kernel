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

//! User-VA allocator for DMA grants. The region sits clear of the
//! MMIO grant region (`USER_MMIO_*`) and the rest of the user
//! address-space layout. Each grant is followed by a 4 KiB guard
//! page so an out-of-bounds access cannot silently spill into the
//! next grant.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::memory::addr::VirtAddr;

pub(super) const USER_DMA_BASE: u64 = 0x0000_00A0_0000_0000;
pub(super) const USER_DMA_END: u64 = 0x0000_00B0_0000_0000;
const PAGE_SIZE: u64 = 4096;

static NEXT_USER_DMA_VA: AtomicU64 = AtomicU64::new(USER_DMA_BASE);

pub(super) fn reserve(pages: u64) -> Option<VirtAddr> {
    let bytes = pages.checked_mul(PAGE_SIZE)?;
    let with_guard = bytes.checked_add(PAGE_SIZE)?;
    let base = NEXT_USER_DMA_VA.fetch_add(with_guard, Ordering::SeqCst);
    let end = base.checked_add(bytes)?;
    if end > USER_DMA_END {
        return None;
    }
    Some(VirtAddr::new(base))
}
