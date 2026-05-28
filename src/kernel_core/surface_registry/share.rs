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

use crate::memory::addr::PhysAddr;
use crate::memory::paging::manager::api::{lookup_asid_for_process, map_page_in_asid};
use crate::memory::paging::types::PagePermissions;
use crate::process::current_process;

use super::table::SLOTS;
use super::types::{decode_handle, RegistryError, SurfaceDescriptor, SurfaceHandle};

pub fn share_surface(
    owner_pid: u32,
    handle: SurfaceHandle,
) -> Result<SurfaceHandle, RegistryError> {
    let (idx, epoch) = decode_handle(handle);
    let mut slots = SLOTS.lock();
    let slot = slots
        .get_mut(idx as usize)
        .and_then(|s| s.as_mut())
        .ok_or(RegistryError::BadHandle)?;
    if slot.epoch != epoch {
        return Err(RegistryError::BadHandle);
    }
    if slot.owner_pid != owner_pid {
        return Err(RegistryError::NotOwner);
    }
    slot.refcount = slot.refcount.checked_add(1).ok_or(RegistryError::InvalidArg)?;
    Ok(handle)
}

pub fn attach_surface(
    receiver_pid: u32,
    handle: SurfaceHandle,
    out_desc: &mut SurfaceDescriptor,
) -> Result<u64, RegistryError> {
    let (idx, epoch) = decode_handle(handle);
    let (frames, descriptor) = {
        let mut slots = SLOTS.lock();
        let slot = slots
            .get_mut(idx as usize)
            .and_then(|s| s.as_mut())
            .ok_or(RegistryError::BadHandle)?;
        if slot.epoch != epoch {
            return Err(RegistryError::BadHandle);
        }
        slot.refcount = slot.refcount.checked_add(1).ok_or(RegistryError::InvalidArg)?;
        let desc = SurfaceDescriptor {
            width: slot.width,
            height: slot.height,
            stride: slot.stride,
            format: slot.format,
            byte_len: slot.byte_len,
            base_va: 0,
            flags: slot.flags,
        };
        (slot.frames.clone(), desc)
    };

    let asid = lookup_asid_for_process(receiver_pid).ok_or(RegistryError::MapFailed)?;
    let proc = current_process().ok_or(RegistryError::NoProc)?;
    let length = frames.len().saturating_mul(4096);
    let base = proc.reserve_vma(length).map_err(|_| RegistryError::MapFailed)?;
    let perms = PagePermissions::user_rw();
    for (i, frame) in frames.iter().enumerate() {
        let va = crate::memory::addr::VirtAddr::new(base.as_u64() + (i as u64) * 4096);
        map_page_in_asid(asid, va, *frame, perms).map_err(|_| RegistryError::MapFailed)?;
    }
    *out_desc = descriptor;
    out_desc.base_va = base.as_u64();
    super::attach_map::record(receiver_pid, handle, base.as_u64(), out_desc.byte_len);
    Ok(base.as_u64())
}

#[allow(dead_code)]
pub(super) fn _phys_anchor() -> PhysAddr {
    PhysAddr::new(0)
}
