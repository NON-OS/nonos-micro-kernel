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

use alloc::sync::Arc;
use core::sync::atomic::Ordering;

use crate::process::core::ProcessControlBlock;

pub fn release(pcb: &Arc<ProcessControlBlock>) {
    let mut mem = pcb.memory.lock();
    mem.vmas.clear();
    mem.resident_pages.store(0, Ordering::Release);
    drop(mem);
    // Reclaim the user frames and page tables through the dying process's own
    // ASID. The per-VMA unmap that used to live here walked the active page
    // table, so it freed whatever address space happened to be current; the
    // ASID-scoped teardown frees the leaf frames as well, so it is the only
    // path that touches the right tables.
    if let Some(asid) = crate::memory::paging::manager::lookup_asid_for_process(pcb.pid) {
        if crate::memory::paging::manager::cleanup_address_space(asid).is_err() {
            crate::sys::serial::println(b"[EXIT] address_space_cleanup_failed");
        }
    }
}
