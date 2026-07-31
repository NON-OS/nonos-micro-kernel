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

use super::super::globals::is_present;
use super::super::globals::state::{DeviceBinding, STATE};
use super::super::types::{DomainId, VtdError, MAX_VTD_DOMAINS};
use super::bdf_to_source_id::bdf_to_source_id;

pub fn map_device(domain: DomainId, bus: u8, device: u8, function: u8) -> Result<(), VtdError> {
    if !is_present() {
        return Err(VtdError::NotPresent);
    }
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
    state.bindings.push(DeviceBinding { source, domain }).map_err(|_| VtdError::DomainTableFull)?;
    // Binding recorded, nothing programmed: confining the device needs a
    // context entry and translation enabled, and neither happens here.
    // Returning Ok claimed an isolation the device did not have.
    Err(VtdError::NotEnforcing)
}
