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

use uefi::table::boot::BootServices;

use crate::loader::image::KernelImage;

use super::frame::alloc_pt_frame;
use super::map_directmap::map_directmap;
use super::map_identity::map_identity_low;
use super::map_kernel_text::map_kernel_text;
use super::table::PageTable;
use super::verify::verify_kernel_pml4;

// Build the kernel paging contract that NØNOS hands off to the
// kernel: a fresh PML4 with low identity over [0,
// IDENTITY_LOW_BYTES) (so bootloader text/data, handoff struct,
// stack, mmap area, and framebuffer stay reachable across the CR3
// swap even when firmware loads the image > 4 GiB), a 256 GiB
// linear directmap
// rooted at PML4[256] (the kernel's `phys_to_virt` window), and
// per-PT_LOAD phys -> virt mappings rooted at PML4[511] for the
// upper-half kernel image.
//
// Returns the new PML4's physical address. Caller switches CR3 to
// this value via `switch_to_kernel_pml4` after ExitBootServices has
// been called and the memory map is finalized.
pub fn build_kernel_pml4(bs: &BootServices, image: &KernelImage) -> Result<u64, &'static str> {
    let pml4_phys = alloc_pt_frame(bs)?;
    let pml4 = PageTable::from_phys(pml4_phys);

    map_identity_low(bs, pml4)?;
    map_directmap(bs, pml4)?;
    map_kernel_text(bs, pml4, image)?;
    verify_kernel_pml4(pml4, image.is_upper_half())?;

    Ok(pml4_phys)
}
