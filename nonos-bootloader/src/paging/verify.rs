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

use super::constants::{
    PML4_INDEX_DIRECTMAP, PML4_INDEX_KERNEL_TEXT, PML4_INDEX_LOW_IDENTITY, PTE_P,
};
use super::table::PageTable;

// Pre-CR3-swap sanity check. The bootloader builds the new PML4
// step by step; before flipping CR3 we confirm the contract entries
// are present. Failing here is preferable to faulting after the
// swap because the bootloader can still emit a serial line and halt
// cleanly.
//
// `expects_kernel_text` is true for upper-half kernels — they must
// have PML4[511] populated by `map_kernel_text`. Legacy low-half
// images execute through PML4[0]'s identity branch and do not.
pub fn verify_kernel_pml4(pml4: PageTable, expects_kernel_text: bool) -> Result<(), &'static str> {
    unsafe {
        let low = pml4.read_entry(PML4_INDEX_LOW_IDENTITY);
        if low & PTE_P == 0 {
            return Err("verify_kernel_pml4: low identity PML4 entry missing");
        }
        let direct = pml4.read_entry(PML4_INDEX_DIRECTMAP);
        if direct & PTE_P == 0 {
            return Err("verify_kernel_pml4: directmap PML4 entry missing");
        }
        if expects_kernel_text {
            let kt = pml4.read_entry(PML4_INDEX_KERNEL_TEXT);
            if kt & PTE_P == 0 {
                return Err("verify_kernel_pml4: kernel text PML4 entry missing");
            }
        }
    }
    Ok(())
}
