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
use crate::constants::VG_RESP_OK_NODATA;
use crate::device::cmd::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use crate::device::virtqueue::ControlQueue;

const REQ_LEN: usize = HDR_LEN + 8;

pub(super) fn submit(
    q: &ControlQueue,
    type_: u32,
    ctx_id: u32,
    resource_id: u32,
    rejected: &'static str,
) -> Result<(), &'static str> {
    if ctx_id == 0 {
        return Err("virtio-gpu: context id 0 is reserved");
    }
    if resource_id == 0 {
        return Err("virtio-gpu: resource id 0 is reserved");
    }
    let mut req = [0u8; REQ_LEN];
    let mut hdr = Hdr::new(type_, 0);
    hdr.ctx_id = ctx_id;
    hdr.write(&mut req[..HDR_LEN]);
    req[HDR_LEN..HDR_LEN + 4].copy_from_slice(&resource_id.to_le_bytes());

    q.submit(&req, RESP_HDR_LEN as u32)?;
    let mut resp = [0u8; RESP_HDR_LEN];
    q.read_response(REQ_LEN, &mut resp);
    let hdr = Hdr::parse(&resp).ok_or("virtio-gpu: bad ctx resource response")?;
    if hdr.type_ != VG_RESP_OK_NODATA {
        return Err(rejected);
    }
    Ok(())
}
