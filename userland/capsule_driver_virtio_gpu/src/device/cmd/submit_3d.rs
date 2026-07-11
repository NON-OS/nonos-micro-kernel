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

//! Hand a Gallium command stream to the host renderer. The guest never
//! executes these dwords: virglrenderer decodes them into real GL/Vulkan
//! calls on the host GPU. Completion is asynchronous, so every submit carries
//! a fence id; the host raises the control-queue interrupt when execution is
//! done and the response echoes the fence.

use alloc::vec::Vec;

use super::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use crate::constants::{VG_CMD_SUBMIT_3D, VG_FLAG_FENCE, VG_RESP_OK_NODATA};
use crate::device::virtqueue::ControlQueue;

// One control-queue buffer bounds a submission; larger streams split into
// multiple submits at Gallium command boundaries.
pub const MAX_STREAM_BYTES: usize = 65536;

pub fn submit_3d(
    q: &ControlQueue,
    ctx_id: u32,
    fence_id: u64,
    stream: &[u8],
) -> Result<(), &'static str> {
    if ctx_id == 0 {
        return Err("virtio-gpu: context id 0 is reserved");
    }
    if stream.is_empty() || !stream.len().is_multiple_of(4) || stream.len() > MAX_STREAM_BYTES {
        // The stream is a dword-tokenised Gallium buffer; a ragged length
        // means the encoder is broken, not something to round silently.
        return Err("virtio-gpu: malformed 3d stream");
    }
    let mut req = Vec::with_capacity(HDR_LEN + 4 + 4 + stream.len());
    let mut hdr = Hdr::new(VG_CMD_SUBMIT_3D, fence_id);
    hdr.ctx_id = ctx_id;
    hdr.flags = VG_FLAG_FENCE;
    let mut hdr_bytes = [0u8; HDR_LEN];
    hdr.write(&mut hdr_bytes);
    req.extend_from_slice(&hdr_bytes);
    req.extend_from_slice(&(stream.len() as u32).to_le_bytes());
    req.extend_from_slice(&0u32.to_le_bytes());
    req.extend_from_slice(stream);

    let out = q.submit(&req, RESP_HDR_LEN as u32)?;
    if (out.used_len as usize) < RESP_HDR_LEN {
        return Err("virtio-gpu: short submit 3d response");
    }
    let mut buf = [0u8; RESP_HDR_LEN];
    q.read_response(req.len(), &mut buf);
    let resp = Hdr::parse(&buf).ok_or("virtio-gpu: bad submit 3d header")?;
    if resp.type_ != VG_RESP_OK_NODATA {
        return Err("virtio-gpu: submit 3d rejected");
    }
    if resp.flags & VG_FLAG_FENCE != 0 && resp.fence_id != fence_id {
        return Err("virtio-gpu: submit 3d fence mismatch");
    }
    Ok(())
}
