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

//! Decoding a PCI bridge's `ranges` property.
//!
//! Each entry is a child address, then a parent address, then a size. The
//! child address is three cells because PCI addresses carry a space code: the
//! top cell's bits 25:24 say whether the range is configuration space, I/O
//! space, 32-bit memory or 64-bit memory. The parent address is however many
//! cells the enclosing node uses, and the size is two cells.

use crate::arch::fdt::endian::{be_cells, be_u32};
use crate::arch::fdt::error::FdtError;

/// Cells in a PCI child address: `phys.hi`, `phys.mid`, `phys.lo`.
const PCI_ADDRESS_CELLS: u32 = 3;
/// Cells in a PCI range size.
const PCI_SIZE_CELLS: u32 = 2;
/// `phys.hi` bits 25:24 hold the space code.
const SPACE_SHIFT: u32 = 24;
const SPACE_MASK: u32 = 0b11;
/// The space code for I/O.
pub const SPACE_IO: u32 = 0b01;

pub struct Range {
    pub space: u32,
    /// The PCI-side address, from `phys.mid` and `phys.lo`.
    pub child: u64,
    /// The CPU-side address the range is mapped at.
    pub parent: u64,
    pub size: u64,
}

/// Walk `data` as a `ranges` property whose parent node uses
/// `parent_address_cells` cells for an address.
pub fn iter(data: &[u8], parent_address_cells: u32) -> impl Iterator<Item = Range> + '_ {
    let entry_cells = PCI_ADDRESS_CELLS + parent_address_cells + PCI_SIZE_CELLS;
    let stride = entry_cells as usize * 4;
    (0..)
        .map(move |i| i * stride)
        .take_while(move |offset| offset + stride <= data.len())
        .filter_map(move |offset| decode(data, offset, parent_address_cells).ok())
}

fn decode(data: &[u8], offset: usize, parent_address_cells: u32) -> Result<Range, FdtError> {
    let hi = be_u32(data, offset)?;
    // `phys.mid` and `phys.lo` together are the 64-bit PCI address.
    let child = be_cells(data, offset + 4, 2)?;
    let parent_offset = offset + PCI_ADDRESS_CELLS as usize * 4;
    let parent = be_cells(data, parent_offset, parent_address_cells)?;
    let size_offset = parent_offset + parent_address_cells as usize * 4;
    let size = be_cells(data, size_offset, PCI_SIZE_CELLS)?;
    Ok(Range { space: (hi >> SPACE_SHIFT) & SPACE_MASK, child, parent, size })
}
