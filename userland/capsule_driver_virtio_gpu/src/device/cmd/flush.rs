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
use crate::constants::{VG_CMD_RESOURCE_FLUSH, VG_RESP_OK_NODATA};
use crate::device::virtqueue::ControlQueue;
use super::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use super::transfer_to_host_2d::Rect;
const BODY_LEN: usize = 24;
const REQ_LEN: usize = HDR_LEN + BODY_LEN;
pub fn resource_flush(
    q: &ControlQueue,
    fence_id: u64,
    resource_id: u32,
    rect: Rect,
) -> Result<(), &'static str> {
    if resource_id == 0 || rect.width == 0 || rect.height == 0 {
        return Err("virtio-gpu: invalid flush args");
    }
    let mut req = [0u8; REQ_LEN];
    Hdr::new(VG_CMD_RESOURCE_FLUSH, fence_id).write(&mut req[..HDR_LEN]);
    req[HDR_LEN..HDR_LEN + 4].copy_from_slice(&rect.x.to_le_bytes());
    req[HDR_LEN + 4..HDR_LEN + 8].copy_from_slice(&rect.y.to_le_bytes());
    req[HDR_LEN + 8..HDR_LEN + 12].copy_from_slice(&rect.width.to_le_bytes());
    req[HDR_LEN + 12..HDR_LEN + 16].copy_from_slice(&rect.height.to_le_bytes());
    req[HDR_LEN + 16..HDR_LEN + 20].copy_from_slice(&resource_id.to_le_bytes());
    req[HDR_LEN + 20..HDR_LEN + 24].fill(0);
    q.submit(&req, RESP_HDR_LEN as u32)?;
    let mut resp = [0u8; RESP_HDR_LEN];
    q.read_response(REQ_LEN, &mut resp);
    let hdr = Hdr::parse(&resp).ok_or("virtio-gpu: bad flush response")?;
    if hdr.type_ != VG_RESP_OK_NODATA {
        return Err("virtio-gpu: resource_flush rejected");
    }
    Ok(())
}