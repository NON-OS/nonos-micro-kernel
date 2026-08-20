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
use super::region::Box3d;
use crate::constants::VG_RESP_OK_NODATA;
use crate::device::cmd::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use crate::device::virtqueue::ControlQueue;

const REQ_LEN: usize = HDR_LEN + 24 + 8 + 16;

#[allow(clippy::too_many_arguments)]
pub(super) fn transfer(
    q: &ControlQueue,
    type_: u32,
    ctx_id: u32,
    fence_id: u64,
    resource_id: u32,
    region: &Box3d,
    offset: u64,
) -> Result<(), &'static str> {
    if ctx_id == 0 {
        return Err("virtio-gpu: context id 0 is reserved");
    }
    if resource_id == 0 {
        return Err("virtio-gpu: resource id 0 is reserved");
    }
    if region.is_empty() {
        return Err("virtio-gpu: empty transfer box");
    }

    let mut req = [0u8; REQ_LEN];
    let mut hdr = Hdr::new(type_, fence_id);
    hdr.ctx_id = ctx_id;
    hdr.write(&mut req[..HDR_LEN]);
    for (i, value) in region.words().iter().enumerate() {
        let at = HDR_LEN + i * 4;
        req[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    let at = HDR_LEN + 24;
    req[at..at + 8].copy_from_slice(&offset.to_le_bytes());
    req[at + 8..at + 12].copy_from_slice(&resource_id.to_le_bytes());

    q.submit(&req, RESP_HDR_LEN as u32)?;
    let mut resp = [0u8; RESP_HDR_LEN];
    q.read_response(REQ_LEN, &mut resp);
    let parsed = Hdr::parse(&resp).ok_or("virtio-gpu: bad 3d transfer response")?;
    if parsed.type_ != VG_RESP_OK_NODATA {
        return Err("virtio-gpu: 3d transfer rejected");
    }
    Ok(())
}
