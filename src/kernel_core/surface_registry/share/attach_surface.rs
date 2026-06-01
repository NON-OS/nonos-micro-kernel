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
use crate::kernel_core::surface_registry::types::{
    decode_handle, RegistryError, SurfaceDescriptor, SurfaceHandle,
};
use crate::memory::paging::manager::api::{lookup_asid_for_process, map_page_in_asid};
use crate::memory::paging::types::PagePermissions;
use crate::process::current_process;

pub fn attach_surface(
    receiver_pid: u32,
    handle: SurfaceHandle,
    out_desc: &mut SurfaceDescriptor,
) -> Result<u64, RegistryError> {
    if let Some((base_va, byte_len)) = super::super::attach_map::lookup(receiver_pid, handle) {
        *out_desc = super::descriptor::descriptor(handle)?;
        out_desc.base_va = base_va;
        out_desc.byte_len = byte_len;
        return Ok(base_va);
    }
    let (idx, epoch) = decode_handle(handle);
    let frames = {
        let mut slots = SLOTS.lock();
        let slot = slots
            .get_mut(idx as usize)
            .and_then(|s| s.as_mut())
            .ok_or(RegistryError::BadHandle)?;
        if slot.epoch != epoch {
            return Err(RegistryError::BadHandle);
        }
        slot.refcount = slot.refcount.checked_add(1).ok_or(RegistryError::InvalidArg)?;
        slot.frames.clone()
    };
    let mut desc = super::descriptor::descriptor(handle)?;
    let asid = lookup_asid_for_process(receiver_pid).ok_or(RegistryError::MapFailed)?;
    let proc = current_process().ok_or(RegistryError::NoProc)?;
    let base = proc
        .reserve_vma(frames.len().saturating_mul(4096))
        .map_err(|_| RegistryError::MapFailed)?;
    let perms = PagePermissions::user_rw();
    for (i, frame) in frames.iter().enumerate() {
        let va = crate::memory::addr::VirtAddr::new(base.as_u64() + (i as u64) * 4096);
        map_page_in_asid(asid, va, *frame, perms).map_err(|_| RegistryError::MapFailed)?;
    }
    desc.base_va = base.as_u64();
    *out_desc = desc;
    super::super::attach_map::record(receiver_pid, handle, base.as_u64(), out_desc.byte_len);
    Ok(base.as_u64())
}
