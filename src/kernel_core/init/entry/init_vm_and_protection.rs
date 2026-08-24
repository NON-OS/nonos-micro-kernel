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

//! Paging, then the ring-0 restrictions that only mean something once paging
//! is settled.

use super::fatal::fatal;
use crate::memory::mmu;

pub(super) fn init_vm_and_protection() {
    // VM/paging must be ready before any process creator runs. The process
    // subsystem only initializes its tables after this; the userspace init
    // process itself is created exactly once in `microkernel_main`.
    if let Err(e) = crate::memory::unified::init_unified_vm() {
        fatal("memory: init_unified_vm failed", e);
    }
    crate::sys::bench::mark(b"vm_ready");

    // Here rather than earlier: CR0.WP starts enforcing read-only kernel
    // mappings, so the tables have to be the ones the kernel will keep. SMAP
    // is safe on this design because the kernel reaches user memory only
    // through the directmap, whose leaves the bootloader builds without the
    // user bit, so a supervisor access to a user page never happens. A part
    // without execute-never is fatal: that same directmap is built NX, and
    // with EFER.NXE clear the whole window stays executable.
    if mmu::init_mmu().is_err() {
        fatal("memory: init_mmu failed", "no execute-never support");
    }
    match mmu::protection_flags() {
        Ok(flags) => mmu::report_protection(flags),
        Err(_) => fatal("memory: protection flags unreadable", "mmu not initialised"),
    }
}
