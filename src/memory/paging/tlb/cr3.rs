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

//! The active page-table root, as the paging manager sees it. The register
//! itself belongs to the architecture; this layer only adds the `PhysAddr`
//! typing the manager works in.

use crate::memory::addr::PhysAddr;

/// Physical base of the table the CPU is translating through.
#[inline]
pub fn get_cr3() -> PhysAddr {
    PhysAddr::new(crate::arch::paging::read_root())
}

/// Point the CPU at `page_table_pa`, untagged.
///
/// The zero is the address-space id, and it is honest rather than incidental:
/// `AddressSpace` keeps the id in its own field and stores the raw page-table
/// frame here, so the value reaching this function is page aligned and carries
/// no id to pass on. Every switch therefore loads the table under id 0, which
/// `pcid::KERNEL_PCID` also names. That is correct, because writing the root
/// invalidates the entries tagged with the id being loaded, but it means
/// tagged invalidation buys nothing on this path even once `enable_pcid` has
/// turned it on. Wiring the id through is a behaviour change and wants a
/// measurement, not a guess.
#[inline]
pub fn set_cr3(page_table_pa: PhysAddr) {
    crate::arch::paging::write_root(page_table_pa.as_u64(), 0);
}
