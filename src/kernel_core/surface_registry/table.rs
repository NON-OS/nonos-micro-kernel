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

use alloc::vec::Vec;
use spin::Mutex;

use crate::memory::addr::PhysAddr;

use super::types::{
    encode_handle, RegistryError, SurfaceDescriptor, SurfaceHandle, SurfaceSid, FMT_ARGB8888,
    MAX_PAGES_PER_SURFACE, PIXEL_BYTES, SLOT_CAP,
};

pub(super) struct Slot {
    pub(super) owner_pid: u32,
    pub(super) epoch: u32,
    pub(super) refcount: u32,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) stride: u32,
    pub(super) format: u32,
    pub(super) flags: u64,
    pub(super) byte_len: u64,
    pub(super) frames: Vec<PhysAddr>,
}

pub(super) static SLOTS: Mutex<[Option<Slot>; SLOT_CAP]> = Mutex::new([const { None }; SLOT_CAP]);

pub fn register_surface(
    owner_pid: u32,
    desc: &SurfaceDescriptor,
    frames: Vec<PhysAddr>,
) -> Result<(SurfaceSid, SurfaceHandle), RegistryError> {
    if desc.format != FMT_ARGB8888 || desc.width == 0 || desc.height == 0 {
        return Err(RegistryError::InvalidArg);
    }
    let min_stride = desc
        .width
        .checked_mul(PIXEL_BYTES)
        .ok_or(RegistryError::InvalidArg)?;
    if desc.stride < min_stride {
        return Err(RegistryError::InvalidArg);
    }
    if frames.is_empty() || frames.len() > MAX_PAGES_PER_SURFACE {
        return Err(RegistryError::InvalidArg);
    }
    let mut slots = SLOTS.lock();
    for (i, entry) in slots.iter_mut().enumerate() {
        if entry.is_none() {
            let epoch = (i as u32).wrapping_add(1);
            *entry = Some(Slot {
                owner_pid,
                epoch,
                refcount: 1,
                width: desc.width,
                height: desc.height,
                stride: desc.stride,
                format: desc.format,
                flags: desc.flags,
                byte_len: desc.byte_len,
                frames,
            });
            return Ok((i as u64, encode_handle(i as u32, epoch)));
        }
    }
    Err(RegistryError::OutOfSlots)
}

pub fn lookup_owned(owner_pid: u32, sid: SurfaceSid) -> Result<SurfaceHandle, RegistryError> {
    let slots = SLOTS.lock();
    let idx = sid as usize;
    let slot = slots
        .get(idx)
        .and_then(|s| s.as_ref())
        .ok_or(RegistryError::NotFound)?;
    if slot.owner_pid != owner_pid {
        return Err(RegistryError::NotOwner);
    }
    Ok(encode_handle(idx as u32, slot.epoch))
}
