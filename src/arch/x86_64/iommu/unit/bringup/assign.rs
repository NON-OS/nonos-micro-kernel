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

use crate::arch::x86_64::iommu::device::bdf_to_source_id;
use crate::arch::x86_64::iommu::globals::state::{DeviceBinding, STATE};
use crate::arch::x86_64::iommu::tables::root::set_context;
use crate::arch::x86_64::iommu::types::{DomainId, VtdError};

/// Give every enumerated PCI function a context entry, and report how many.
pub(super) fn assign_enumerated(
    root: u64,
    domain: DomainId,
    address_width: u8,
) -> Result<usize, VtdError> {
    let devices = crate::bus::pci::enumerate_devices();
    let mut state = STATE.lock();
    let mut count = 0;
    for dev in devices.iter() {
        let source = bdf_to_source_id(dev.bus, dev.device, dev.function);
        if state.bindings.iter().any(|binding| binding.source == source) {
            continue;
        }
        // Bookkeeping first: a device programmed but unrecorded could never be
        // detached again.
        if state.bindings.push(DeviceBinding { source, domain }).is_err() {
            return Err(VtdError::DomainTableFull);
        }
        set_context(source, root, domain, address_width)?;
        count += 1;
    }
    Ok(count)
}
