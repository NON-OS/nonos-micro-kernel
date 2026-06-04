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
use super::state::Primary;
use super::{descriptor, dma, geometry, prime, resource};
use crate::constants::VG_FORMAT_B8G8R8A8_UNORM;
use crate::device::cmd;
use crate::device::virtqueue::ControlQueue;
use crate::state::{FenceCounter, ResourceTable, Scanout};
use nonos_libc::{mk_surface_register, mk_surface_share};
pub fn create(
    device_id: u64,
    claim_epoch: u64,
    q: &ControlQueue,
    fences: &FenceCounter,
    resources: &ResourceTable,
    scanout: Scanout,
) -> Result<Option<Primary>, &'static str> {
    let geom = match geometry::derive(scanout)? {
        Some(g) => g,
        None => return Ok(None),
    };
    let dma = dma::map(device_id, claim_epoch, geom.byte_len)?;
    let resource_id = resources.alloc_id();
    cmd::create_resource_2d(
        q,
        fences.issue(),
        resource_id,
        VG_FORMAT_B8G8R8A8_UNORM,
        scanout.width,
        scanout.height,
    )?;
    cmd::attach_backing(q, fences.issue(), resource_id, dma.device_addr, geom.byte_len as u32)?;
    prime::display(q, fences, resource_id, scanout)?;
    let desc = descriptor::build(scanout, geom, dma.user_va);
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        dma::rollback(dma.grant_id, "virtio-gpu: primary dma rollback failed")?;
        return Err("virtio-gpu: surface register rejected");
    }
    let handle = mk_surface_share(sid as u64);
    if handle < 0 {
        dma::rollback(dma.grant_id, "virtio-gpu: primary dma rollback failed")?;
        return Err("virtio-gpu: surface share rejected");
    }
    if resource::insert(resources, resource_id, scanout, dma.device_addr, geom.byte_len as u32)
        .is_err()
    {
        dma::rollback(dma.grant_id, "virtio-gpu: primary dma rollback failed")?;
        return Err("virtio-gpu: resource table full");
    }
    Ok(Some(Primary {
        handle: handle as u64,
        resource_id,
        width: scanout.width,
        height: scanout.height,
        stride: geom.stride,
    }))
}
