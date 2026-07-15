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

// Architecture-neutral seam over the CR3 write. The actual `mov
// cr3` lives at the arch boundary (`arch::x86_64::paging::load_cr3`);
// this shim is what `handoff::exit::orchestrate` calls so the
// orchestration code stays free of inline asm.

use crate::arch::x86_64::paging::{enable_nxe, load_cr3};

// SAFETY: see `arch::x86_64::paging::load_cr3`.
pub unsafe fn switch_to_kernel_pml4(pml4_phys: u64) {
    // The PML4 carries NX bits (directmap, kernel data). With EFER.NXE
    // clear those are reserved bits and the first access through them
    // triple-faults; firmware is not guaranteed to have set it.
    enable_nxe();
    load_cr3(pml4_phys);
}
