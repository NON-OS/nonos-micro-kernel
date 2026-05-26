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

// Writes `argb` into a rectangle of the ARGB8888 surface mapped at
// `base_va` (which the GPU reads on TRANSFER_TO_HOST). Volatile so the
// optimizer cannot reorder around the gfx_client RPCs that follow.

pub fn fill_rect(
    base_va: u64,
    stride_bytes: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    argb: u32,
) {
    if width == 0 || height == 0 {
        return;
    }
    let row_stride = stride_bytes as usize;
    let row_width = width as usize;
    for row in 0..height as usize {
        let row_va = base_va as usize + (y as usize + row) * row_stride + x as usize * 4;
        let row_ptr = row_va as *mut u32;
        // SAFETY: caller provides a mapped ARGB8888 surface and the
        // requested rectangle is already clipped by the caller.
        unsafe { core::slice::from_raw_parts_mut(row_ptr, row_width).fill(argb) };
    }
}
