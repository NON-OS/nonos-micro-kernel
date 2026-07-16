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

use crate::memory::addr::VirtAddr;

// Third breadcrumb segment (green), painted through the kernel's own
// mapping: proves the paging switch survived and the framebuffer mapped.
// The first two segments are painted by boot::entry_marker over the
// bootloader's tables before the switch.
pub(super) fn paint_mapped_marker(base_va: VirtAddr, offset: usize, stride: u32, width: u32) {
    const SEG_WIDTH: u32 = 180;
    const SEG_GAP: u32 = 20;
    const SEG_HEIGHT: u32 = 10;
    let x0 = 2 * (SEG_WIDTH + SEG_GAP);
    if x0 + SEG_WIDTH > width {
        return;
    }
    let row_px = (stride / 4) as u64;
    let base = (base_va.as_u64() + offset as u64) as *mut u32;
    for y in 0..SEG_HEIGHT as u64 {
        for x in 0..SEG_WIDTH as u64 {
            let off = y * row_px + x0 as u64 + x;
            // SAFETY: bounds checked against the handoff geometry; the
            // mapping was just created above with room for the full frame.
            unsafe { core::ptr::write_volatile(base.add(off as usize), 0xFF00_D060) };
        }
    }
}
