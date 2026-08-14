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
use super::spec::Resource3d;
use crate::constants::{VG_CMD_RESOURCE_CREATE_3D, VG_RESP_OK_NODATA};
use crate::device::cmd::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use crate::device::virtqueue::ControlQueue;

const REQ_LEN: usize = HDR_LEN + 48;

pub fn resource_create_3d(
    q: &ControlQueue,
    fence_id: u64,
    res: &Resource3d,
) -> Result<(), &'static str> {
    if res.resource_id == 0 {
        return Err("virtio-gpu: resource id 0 is reserved");
    }
    if res.width == 0 || res.height == 0 || res.depth == 0 || res.array_size == 0 {
        return Err("virtio-gpu: zero extent in 3d resource");
    }
    if res.bind == 0 {
        return Err("virtio-gpu: 3d resource with no bind flags");
    }

    let mut req = [0u8; REQ_LEN];
    Hdr::new(VG_CMD_RESOURCE_CREATE_3D, fence_id).write(&mut req[..HDR_LEN]);
    for (i, value) in res.fields().iter().enumerate() {
        let at = HDR_LEN + i * 4;
        req[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }

    q.submit(&req, RESP_HDR_LEN as u32)?;
    let mut resp = [0u8; RESP_HDR_LEN];
    q.read_response(REQ_LEN, &mut resp);
    let hdr = Hdr::parse(&resp).ok_or("virtio-gpu: bad resource_create_3d response")?;
    if hdr.type_ != VG_RESP_OK_NODATA {
        return Err("virtio-gpu: resource_create_3d rejected");
    }
    Ok(())
}
