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

use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU64};
use spin::Mutex;

use super::super::types::{DomainId, SourceId, MAX_VTD_DOMAINS};

pub(crate) const FIRST_DYNAMIC_DOMAIN_ID: u64 = 1;

pub(crate) struct DomainSlot {
    pub used: bool,
    /// Physical address of this domain's second-level root table. Zero
    /// while the slot is free: a domain with no root confines nothing,
    /// so the two are set and cleared together.
    pub root: u64,
}

pub(crate) struct DeviceBinding {
    pub source: SourceId,
    pub domain: DomainId,
}

pub(crate) struct VtdState {
    pub domains: [DomainSlot; MAX_VTD_DOMAINS],
    pub bindings: heapless::Vec<DeviceBinding, { super::super::types::MAX_VTD_DEVICES }>,
}

impl VtdState {
    const fn new() -> Self {
        const SLOT: DomainSlot = DomainSlot { used: false, root: 0 };
        Self { domains: [SLOT; MAX_VTD_DOMAINS], bindings: heapless::Vec::new() }
    }
}

pub(crate) static DMAR_PRESENT: AtomicBool = AtomicBool::new(false);
/// Second-level page table depth every domain is built to. Zero until a
/// unit has been probed, and a map cannot proceed without it: guessing the
/// depth would build a table the hardware walks at the wrong offsets.
/// Set only once a unit has confirmed Translation Enable. `DMAR_PRESENT`
/// means the firmware described an IOMMU; this means devices are actually
/// being confined by it, which is a different and much stronger claim.
pub(crate) static ENFORCING: AtomicBool = AtomicBool::new(false);
pub(crate) static PAGE_LEVELS: AtomicU8 = AtomicU8::new(0);
/// Physical address of the one root table every unit is pointed at, or zero
/// before it exists. Shared rather than per-unit: a device appears behind
/// exactly one unit, so one table indexed by bus and function describes them
/// all, and a single table is one thing to invalidate.
pub(crate) static ROOT_TABLE: AtomicU64 = AtomicU64::new(0);
pub(crate) static NEXT_DOMAIN_ID: AtomicU64 = AtomicU64::new(FIRST_DYNAMIC_DOMAIN_ID);
pub(crate) static STATE: Mutex<VtdState> = Mutex::new(VtdState::new());
