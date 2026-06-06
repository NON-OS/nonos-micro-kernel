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
use crate::constants::{VG_CMD_RESOURCE_ATTACH_BACKING, VG_RESP_OK_NODATA};
use crate::device::virtqueue::ControlQueue;
const ENTRY_LEN: usize = 16;
const BODY_LEN: usize = 8 + ENTRY_LEN;
const REQ_LEN: usize = HDR_LEN + BODY_LEN;
pub fn attach_backing(
    q: &ControlQueue,
    fence_id: u64,
    resource_id: u32,
    backing_addr: u64,
    backing_len: u32,
) -> Result<(), &'static str> {
    if resource_id == 0 || backing_addr == 0 || backing_len == 0 {
        return Err("virtio-gpu: invalid attach_backing args");
    }
    let mut req = [0u8; REQ_LEN];
    Hdr::new(VG_CMD_RESOURCE_ATTACH_BACKING, fence_id).write(&mut req[..HDR_LEN]);
    req[HDR_LEN..HDR_LEN + 4].copy_from_slice(&resource_id.to_le_bytes());
    req[HDR_LEN + 4..HDR_LEN + 8].copy_from_slice(&1u32.to_le_bytes());
    req[HDR_LEN + 8..HDR_LEN + 16].copy_from_slice(&backing_addr.to_le_bytes());
    req[HDR_LEN + 16..HDR_LEN + 20].copy_from_slice(&backing_len.to_le_bytes());
    req[HDR_LEN + 20..HDR_LEN + 24].fill(0);
    q.submit(&req, RESP_HDR_LEN as u32)?;
    let mut resp = [0u8; RESP_HDR_LEN];
    q.read_response(REQ_LEN, &mut resp);
    let hdr = Hdr::parse(&resp).ok_or("virtio-gpu: bad attach_backing response")?;
    if hdr.type_ != VG_RESP_OK_NODATA {
        return Err("virtio-gpu: attach_backing rejected");
    }
    Ok(())
}
