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

pub fn share_surface(
    owner_pid: u32,
    handle: SurfaceHandle,
) -> Result<SurfaceHandle, RegistryError> {
    let (idx, epoch) = decode_handle(handle);
    let mut slots = SLOTS.lock();
    let slot =
        slots.get_mut(idx as usize).and_then(|s| s.as_mut()).ok_or(RegistryError::BadHandle)?;
    if slot.epoch != epoch {
        return Err(RegistryError::BadHandle);
    }
    if slot.owner_pid != owner_pid {
        return Err(RegistryError::NotOwner);
    }
    slot.refcount = slot.refcount.checked_add(1).ok_or(RegistryError::InvalidArg)?;
    Ok(handle)
}
