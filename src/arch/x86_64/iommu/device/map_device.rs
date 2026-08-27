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

//! Assigning one PCI function to a domain.
//!
//! The context entry is written before the binding is recorded as complete and
//! the caches are dropped immediately after, so the window in which the
//! hardware disagrees with the kernel's bookkeeping is closed before this
//! returns. Until the entry exists the device is denied, which is the safe
//! direction to fail in.

use super::super::globals::state::{DeviceBinding, STATE};
use super::super::globals::is_enforcing;
use super::super::tables::root::set_context;
use super::super::types::{DomainId, VtdError, MAX_VTD_DOMAINS};
use super::super::unit::invalidate::invalidate_all;
use super::super::unit::report::probed;
use super::bdf_to_source_id::bdf_to_source_id;

pub fn map_device(domain: DomainId, bus: u8, device: u8, function: u8) -> Result<(), VtdError> {
    // Not `is_present`: a described unit that is not translating would let
    // this write a context entry no hardware reads, and report success.
    if !is_enforcing() {
        return Err(VtdError::NotEnforcing);
    }
    let info = probed().ok_or(VtdError::NotPresent)?;
    let index = domain.as_u16() as usize;
    if index >= MAX_VTD_DOMAINS {
        return Err(VtdError::DomainNotFound);
    }
    let source = bdf_to_source_id(bus, device, function);

    let mut state = STATE.lock();
    if !state.domains[index].used {
        return Err(VtdError::DomainNotFound);
    }
    if state.bindings.iter().any(|binding| binding.source == source) {
        return Err(VtdError::DeviceAlreadyAttached);
    }
    let sl_root = state.domains[index].root;

    // Space for the binding is taken first. Programming the hardware and then
    // finding nowhere to record it would leave a device assigned that the
    // kernel cannot later detach.
    state.bindings.push(DeviceBinding { source, domain }).map_err(|_| VtdError::DomainTableFull)?;

    let programmed = set_context(source, sl_root, domain, info.levels.context_aw())
        .and_then(|()| invalidate_all(&info.unit, info.ecap));
    if let Err(e) = programmed {
        state.bindings.retain(|binding| binding.source != source);
        return Err(e);
    }
    Ok(())
}
