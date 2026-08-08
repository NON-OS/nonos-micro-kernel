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

use crate::memory::addr::PhysAddr;
use core::sync::atomic::{AtomicU64, Ordering};

use super::pcid::enable_pcid;
use super::types::{pte_flags, AddressSpace};

static CURRENT_ADDRESS_SPACE: AtomicU64 = AtomicU64::new(0);

#[inline(always)]
pub fn switch_address_space(space: &AddressSpace) {
    let new_cr3 = space.cr3_value();
    // Compare table bases, not the packed register value: the boundary hands
    // back the base with the address-space id masked off, and the base is what
    // identifies the space. Skipping the write when it already matches avoids
    // the flush that installing a root costs.
    if crate::arch::paging::read_root() != new_cr3 & !0xFFF {
        crate::arch::paging::write_root(new_cr3, (new_cr3 & 0xFFF) as u16);
        CURRENT_ADDRESS_SPACE.store(space.pml4_phys.as_u64(), Ordering::SeqCst);
    }
}

pub fn current_address_space_phys() -> PhysAddr {
    PhysAddr::new(crate::arch::paging::read_root() & pte_flags::ADDR_MASK)
}

pub fn init() -> Result<(), &'static str> {
    crate::log::info!("[ADDR_SPACE] Initializing address space management...");
    enable_pcid();
    crate::log::info!("[ADDR_SPACE] Address space management initialized");
    Ok(())
}
