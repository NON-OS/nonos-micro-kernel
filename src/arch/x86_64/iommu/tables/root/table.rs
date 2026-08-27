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

use core::sync::atomic::Ordering;

use crate::arch::x86_64::iommu::globals::state::ROOT_TABLE;
use crate::arch::x86_64::iommu::tables::frame::allocate_table;
use crate::arch::x86_64::iommu::types::VtdError;

/// One root table, indexed by bus, shared by every unit. Created on first use.
pub fn root_table() -> Result<u64, VtdError> {
    let existing = ROOT_TABLE.load(Ordering::Acquire);
    if existing != 0 {
        return Ok(existing);
    }
    let table = allocate_table()?;
    // A loser keeps its frame unused rather than publishing it: two root
    // tables would leave devices behind one no unit points at.
    match ROOT_TABLE.compare_exchange(0, table, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => Ok(table),
        Err(winner) => Ok(winner),
    }
}
