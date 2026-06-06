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
use super::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use super::transfer_to_host_2d::Rect;
use crate::constants::{VG_CMD_SET_SCANOUT, VG_MAX_SCANOUTS, VG_RESP_OK_NODATA};
use crate::device::virtqueue::ControlQueue;
const BODY_LEN: usize = 24;
const REQ_LEN: usize = HDR_LEN + BODY_LEN;
pub fn set_scanout(
    q: &ControlQueue,
    fence_id: u64,
    scanout_id: u32,
    resource_id: u32,
    rect: Rect,
) -> Result<(), &'static str> {
    if scanout_id as usize >= VG_MAX_SCANOUTS || rect.width == 0 || rect.height == 0 {
        return Err("virtio-gpu: invalid set_scanout args");
    }
    let mut req = [0u8; REQ_LEN];
    Hdr::new(VG_CMD_SET_SCANOUT, fence_id).write(&mut req[..HDR_LEN]);
    req[HDR_LEN..HDR_LEN + 4].copy_from_slice(&rect.x.to_le_bytes());
    req[HDR_LEN + 4..HDR_LEN + 8].copy_from_slice(&rect.y.to_le_bytes());
    req[HDR_LEN + 8..HDR_LEN + 12].copy_from_slice(&rect.width.to_le_bytes());
    req[HDR_LEN + 12..HDR_LEN + 16].copy_from_slice(&rect.height.to_le_bytes());
    req[HDR_LEN + 16..HDR_LEN + 20].copy_from_slice(&scanout_id.to_le_bytes());
    req[HDR_LEN + 20..HDR_LEN + 24].copy_from_slice(&resource_id.to_le_bytes());
    q.submit(&req, RESP_HDR_LEN as u32)?;
    let mut resp = [0u8; RESP_HDR_LEN];
    q.read_response(REQ_LEN, &mut resp);
    let hdr = Hdr::parse(&resp).ok_or("virtio-gpu: bad set_scanout response")?;
    if hdr.type_ != VG_RESP_OK_NODATA {
        return Err("virtio-gpu: set_scanout rejected");
    }
    Ok(())
}
