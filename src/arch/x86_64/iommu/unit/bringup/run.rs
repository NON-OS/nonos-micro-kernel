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

use super::assign::assign_enumerated;
use super::domain::identity_domain;
use crate::arch::x86_64::iommu::globals::{set_enforcing, set_page_levels};
use crate::arch::x86_64::iommu::tables::root::root_table;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::enable::bring_into_service;
use crate::arch::x86_64::iommu::unit::report::probed;

/// Program the tables, assign the enumerated devices, then enable. The order
/// is the safety argument and it is the reverse of the obvious one: every
/// device the kernel knows about has a context entry before translation
/// starts, so the disk and the network keep working. What changes is that
/// anything not enumerated, including a card plugged in later, is denied.
pub fn bring_up() -> Result<usize, VtdError> {
    let info = probed().ok_or(VtdError::NotPresent)?;
    let levels = info.levels.page_table_levels();

    set_page_levels(levels);
    let root = root_table()?;

    let (domain, sl_root) = identity_domain(levels, info.cap)?;
    let assigned = assign_enumerated(sl_root, domain, info.levels.context_aw())?;

    // SAFETY: eK@nonos.systems - `root` is a table this module allocated and
    // never frees, so the unit may walk it by physical address for the life of
    // the kernel. Every enumerated device already has a context entry in it.
    unsafe {
        bring_into_service(&info.unit, root, info.ecap)?;
    }
    set_enforcing();
    Ok(assigned)
}
