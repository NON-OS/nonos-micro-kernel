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

//! Second-level page table entries and the index math for walking them.
//!
//! A second-level table is 512 entries of 8 bytes over a 4 KiB page, indexed
//! nine bits at a time like the CPU's own tables. The permission bits differ:
//! there is no present bit, an entry counts as present when it grants read or
//! write, so a zero entry denies rather than faults open.

/// Read permission. Also what makes an entry present.
pub const SL_READ: u64 = 1 << 0;
/// Write permission.
pub const SL_WRITE: u64 = 1 << 1;
/// Snoop: force the access to be coherent with CPU caches.
pub const SL_SNOOP: u64 = 1 << 11;
/// Marks a leaf at a level above the last one, a large page.
pub const SL_LARGE: u64 = 1 << 7;

/// Physical address bits of an entry. Bits 51:12, so any entry is page aligned
/// by construction and the permission bits can never be read as address.
pub const SL_ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

/// Entries per table, and the bits one level consumes.
pub const ENTRIES: usize = 512;
pub const LEVEL_SHIFT: u32 = 9;
pub const PAGE_SHIFT: u32 = 12;

/// An entry grants access only if it permits read or write. Zero denies.
pub const fn is_present(entry: u64) -> bool {
    entry & (SL_READ | SL_WRITE) != 0
}

/// The frame an entry points at.
pub const fn entry_address(entry: u64) -> u64 {
    entry & SL_ADDR_MASK
}

/// Build a leaf entry. The address is masked rather than trusted, so a
/// misaligned frame cannot bleed into the permission bits.
pub const fn leaf(phys: u64, read: bool, write: bool, snoop: bool) -> u64 {
    let mut e = phys & SL_ADDR_MASK;
    if read {
        e |= SL_READ;
    }
    if write {
        e |= SL_WRITE;
    }
    if snoop {
        e |= SL_SNOOP;
    }
    e
}

/// Build an entry pointing at the next table down. Always readable and
/// writable: a table entry's own bits do not restrict, the leaf decides.
pub const fn table(phys: u64) -> u64 {
    (phys & SL_ADDR_MASK) | SL_READ | SL_WRITE
}

/// Index into the table at `level`, counting the leaf level as 1.
pub const fn index_for(addr: u64, level: u8) -> usize {
    let shift = PAGE_SHIFT + LEVEL_SHIFT * (level as u32 - 1);
    ((addr >> shift) as usize) & (ENTRIES - 1)
}

/// Bytes one entry at `level` spans.
pub const fn level_span(level: u8) -> u64 {
    1u64 << (PAGE_SHIFT + LEVEL_SHIFT * (level as u32 - 1))
}

/// Whether an address the caller asked to map fits the unit's input width.
pub const fn fits_address_width(addr: u64, width_bits: u8) -> bool {
    if width_bits >= 64 {
        true
    } else {
        addr < (1u64 << width_bits)
    }
}
