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

//! Whether to put the unit in charge of DMA, and what to say about the
//! outcome. A kernel built without `nonos-iommu-enforce` reports the machine
//! and leaves the hardware alone; the DMA API refuses to issue device
//! addresses in that state, so nothing downstream mistakes it for protection.

use super::run::bring_up;
use super::verdict;
use crate::arch::x86_64::iommu::unit::report::probed;

/// Never panics, and never claims protection it did not achieve.
pub fn init() {
    if probed().is_none() {
        return;
    }
    if !cfg!(feature = "nonos-iommu-enforce") {
        verdict::not_built_in();
        return;
    }
    match bring_up() {
        Ok(assigned) => verdict::enabled(assigned),
        Err(e) => verdict::failed(e),
    }
}
