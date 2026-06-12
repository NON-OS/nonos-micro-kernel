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

use crate::kernel_core::surface_registry::table::SLOTS;
use crate::kernel_core::surface_registry::types::{decode_handle, RegistryError, SurfaceHandle};

pub fn release_surface(handle: SurfaceHandle) -> Result<u32, RegistryError> {
    let (idx, epoch) = decode_handle(handle);
    let mut slots = SLOTS.lock();
    let entry = slots.get_mut(idx as usize).ok_or(RegistryError::BadHandle)?;
    let new_count = {
        let slot = entry.as_mut().ok_or(RegistryError::BadHandle)?;
        if slot.epoch != epoch {
            return Err(RegistryError::BadHandle);
        }
        slot.refcount = slot.refcount.checked_sub(1).ok_or(RegistryError::InvalidArg)?;
        slot.refcount
    };
    if new_count == 0 {
        *entry = None;
    }
    Ok(new_count)
}
