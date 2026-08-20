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

//! Taking a device back out of its domain.
//!
//! The context entry is cleared and the caches dropped before the binding is
//! forgotten, so at no point does the kernel believe a device is detached
//! while the hardware still translates for it. If the invalidation fails the
//! binding is kept, because a device whose stale translations may still be
//! live has not actually been detached.

use super::super::globals::is_enforcing;
use super::super::globals::state::STATE;
use super::super::tables::root::clear_context;
use super::super::types::VtdError;
use super::super::unit::invalidate::invalidate_all;
use super::super::unit::report::probed;
use super::bdf_to_source_id::bdf_to_source_id;

pub fn unmap_device(bus: u8, device: u8, function: u8) -> Result<(), VtdError> {
    if !is_enforcing() {
        return Err(VtdError::NotEnforcing);
    }
    let info = probed().ok_or(VtdError::NotPresent)?;
    let source = bdf_to_source_id(bus, device, function);

    let mut state = STATE.lock();
    if !state.bindings.iter().any(|binding| binding.source == source) {
        return Err(VtdError::DeviceNotAttached);
    }
    clear_context(source)?;
    invalidate_all(&info.unit, info.ecap)?;
    state.bindings.retain(|binding| binding.source != source);
    Ok(())
}
