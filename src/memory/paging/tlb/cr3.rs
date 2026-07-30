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

/// Install `root` and drop the translations the previous table left behind.
#[inline]
pub fn flush_address_space(root: PhysAddr) {
    crate::arch::paging::write_root(root.as_u64(), 0);
}

/// Physical base of the table the CPU is translating through.
#[inline]
pub fn get_cr3() -> PhysAddr {
    PhysAddr::new(crate::arch::paging::read_root())
}

/// Point the CPU at `page_table_pa`.
#[inline]
pub fn set_cr3(page_table_pa: PhysAddr) {
    crate::arch::paging::write_root(page_table_pa.as_u64(), 0);
}
