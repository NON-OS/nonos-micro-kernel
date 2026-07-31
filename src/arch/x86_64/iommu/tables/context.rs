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

//! Root and context entries: what turns a PCI source id into a page table.
//!
//! The root table is indexed by bus and points at a context table; the context
//! table is indexed by the device and function bits and names the domain and
//! its second-level root. Both entries are 128 bits, low half first. An entry
//! with the present bit clear denies the device, so a zeroed table blocks
//! everything rather than passing it through.

/// Present bit, in the low half of both entry kinds.
pub const PRESENT: u64 = 1 << 0;

/// Address bits of the pointer each entry carries.
pub const ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

/// Entries per table. The root table is indexed by an 8-bit bus number; the
/// context table by the 8-bit device and function pair.
pub const ROOT_ENTRIES: usize = 256;
pub const CONTEXT_ENTRIES: usize = 256;

/// Context entry translation type: use the second-level tables.
pub const TT_MULTI_LEVEL: u64 = 0b00;

/// Low half of a root entry pointing at a context table.
pub const fn root_low(context_table_phys: u64) -> u64 {
    (context_table_phys & ADDR_MASK) | PRESENT
}

/// Low half of a context entry: the second-level table root, plus the
/// translation type and the present bit.
pub const fn context_low(sl_root_phys: u64) -> u64 {
    (sl_root_phys & ADDR_MASK) | (TT_MULTI_LEVEL << 2) | PRESENT
}

/// High half of a context entry: the address width and the domain id.
pub const fn context_high(domain_id: u16, address_width: u8) -> u64 {
    ((address_width as u64) & 0x7) | ((domain_id as u64) << 8)
}

/// Index of a device within its context table, from the device and function.
pub const fn context_index(device: u8, function: u8) -> usize {
    (((device & 0x1F) as usize) << 3) | ((function & 0x7) as usize)
}

/// An entry is only honoured when present.
pub const fn is_present(low: u64) -> bool {
    low & PRESENT != 0
}

/// The table or page an entry points at.
pub const fn entry_address(low: u64) -> u64 {
    low & ADDR_MASK
}

/// Domain id recorded in a context entry.
pub const fn context_domain(high: u64) -> u16 {
    ((high >> 8) & 0xFFFF) as u16
}
