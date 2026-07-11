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

//! Create a host-side rendering context. Every 3D resource and every command
//! stream is scoped to a context id (carried in the command header), so this
//! is the first 3D command after negotiation. The debug name shows up in host
//! tooling when things go wrong.

use super::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use crate::constants::{VG_CMD_CTX_CREATE, VG_RESP_OK_NODATA};
use crate::device::virtqueue::ControlQueue;

const DEBUG_NAME_LEN: usize = 64;
const REQ_LEN: usize = HDR_LEN + 8 + DEBUG_NAME_LEN; // nlen, context_init, name[64]

pub fn ctx_create(q: &ControlQueue, ctx_id: u32, name: &[u8]) -> Result<(), &'static str> {
    if ctx_id == 0 {
        return Err("virtio-gpu: context id 0 is reserved");
    }
    let mut req = [0u8; REQ_LEN];
    let mut hdr = Hdr::new(VG_CMD_CTX_CREATE, 0);
    hdr.ctx_id = ctx_id;
    hdr.write(&mut req);
    let n = name.len().min(DEBUG_NAME_LEN);
    req[HDR_LEN..HDR_LEN + 4].copy_from_slice(&(n as u32).to_le_bytes());
    // context_init stays 0: plain VirGL context, no capset-specific init.
    req[HDR_LEN + 8..HDR_LEN + 8 + n].copy_from_slice(&name[..n]);
    let out = q.submit(&req, RESP_HDR_LEN as u32)?;
    if (out.used_len as usize) < RESP_HDR_LEN {
        return Err("virtio-gpu: short ctx create response");
    }
    let mut buf = [0u8; RESP_HDR_LEN];
    q.read_response(REQ_LEN, &mut buf);
    let hdr = Hdr::parse(&buf).ok_or("virtio-gpu: bad ctx create header")?;
    if hdr.type_ != VG_RESP_OK_NODATA {
        return Err("virtio-gpu: ctx create rejected");
    }
    Ok(())
}
