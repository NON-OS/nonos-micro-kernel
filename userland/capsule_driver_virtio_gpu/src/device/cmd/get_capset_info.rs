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

//! Query which 3D capability set the host renderer speaks. Capset 1 is the
//! classic VirGL (Gallium/OpenGL) set, 2 is VirGL2; the id and max_size shape
//! every later GET_CAPSET and command-stream decision.

use super::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use crate::constants::{VG_CMD_GET_CAPSET_INFO, VG_RESP_OK_CAPSET_INFO};
use crate::device::virtqueue::ControlQueue;
use crate::protocol::le_u32;

const REQ_LEN: usize = HDR_LEN + 8; // __le32 capset_index, __le32 padding
const RESP_LEN: usize = RESP_HDR_LEN + 16; // id, max_version, max_size, padding

#[derive(Clone, Copy, Default)]
pub struct CapsetInfo {
    pub capset_id: u32,
    pub max_version: u32,
    pub max_size: u32,
}

pub fn get_capset_info(q: &ControlQueue, index: u32) -> Result<CapsetInfo, &'static str> {
    let mut req = [0u8; REQ_LEN];
    Hdr::new(VG_CMD_GET_CAPSET_INFO, 0).write(&mut req);
    req[HDR_LEN..HDR_LEN + 4].copy_from_slice(&index.to_le_bytes());
    let out = q.submit(&req, RESP_LEN as u32)?;
    if (out.used_len as usize) < RESP_LEN {
        return Err("virtio-gpu: short capset info response");
    }
    let mut buf = [0u8; RESP_LEN];
    q.read_response(REQ_LEN, &mut buf);
    let hdr = Hdr::parse(&buf[..HDR_LEN]).ok_or("virtio-gpu: bad capset info header")?;
    if hdr.type_ != VG_RESP_OK_CAPSET_INFO {
        return Err("virtio-gpu: capset info rejected");
    }
    Ok(CapsetInfo {
        capset_id: le_u32(&buf, RESP_HDR_LEN).ok_or("virtio-gpu: bad capset info body")?,
        max_version: le_u32(&buf, RESP_HDR_LEN + 4).ok_or("virtio-gpu: bad capset info body")?,
        max_size: le_u32(&buf, RESP_HDR_LEN + 8).ok_or("virtio-gpu: bad capset info body")?,
    })
}
