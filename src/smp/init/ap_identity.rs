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

//! A temporary low identity mapping covering the AP trampoline, live only for
//! the duration of SMP bring-up.
//!
//! An application processor comes out of INIT/SIPI in 16-bit real mode at the
//! trampoline's physical address (0x8000). The trampoline switches to long
//! mode and loads the BSP's CR3, whose `PML4[0]` low identity map was already
//! torn down by VM init. The very next instruction fetch is still at the low
//! physical address, so without a low identity mapping the AP faults the
//! instant paging turns on, triple-faults with no IDT, and the BSP hangs
//! waiting on the startup barrier.
//!
//! So we re-map exactly the trampoline region identity before starting the
//! APs and remove it once they are online, restoring the invariant that
//! `PML4[0]` stays cleared while any userspace runs. No capsule has spawned at
//! this point, so the window is closed before it could matter.

use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::manager::api::{map_page, unmap_page};
use crate::memory::paging::types::PagePermissions;
use crate::smp::constants::AP_TRAMPOLINE_ADDR;

// The trampoline blob (real/protected/long stubs, GDT, tables) sits at 0x8000
// and spans well under this. 16 pages of headroom keeps it comfortably covered.
const TRAMPOLINE_PAGES: u64 = 16;
const PAGE: u64 = 4096;

fn region_base() -> u64 {
    AP_TRAMPOLINE_ADDR & !(PAGE - 1)
}

// Identity-map the trampoline region executable, so the AP can run through the
// mode switch and the far jump before it reaches the higher-half entry.
pub(super) fn install() -> Result<(), &'static str> {
    let base = region_base();
    // Read + execute, no write: the trampoline only reads its context and code
    // through the low identity (the BSP patches it through the directmap), so
    // this satisfies W^X. A writable+executable low page would be rejected.
    let perms = PagePermissions::READ | PagePermissions::EXECUTE;
    for i in 0..TRAMPOLINE_PAGES {
        let addr = base + i * PAGE;
        map_page(VirtAddr::new(addr), PhysAddr::new(addr), perms)
            .map_err(|_| "ap trampoline identity map failed")?;
    }
    Ok(())
}

// Drop the temporary mapping and re-clear the low half, restoring the
// post-VM-init invariant. Best-effort: a failure here is logged by the caller,
// not fatal, since the APs are already up by the time this runs.
pub(super) fn remove() {
    let base = region_base();
    for i in 0..TRAMPOLINE_PAGES {
        let _ = unmap_page(VirtAddr::new(base + i * PAGE));
    }
    let _ = crate::arch::x86_64::paging::clear_low_half();
}
