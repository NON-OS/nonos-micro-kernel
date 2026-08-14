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

//! Read a scanout's raw EDID block from the device. Bytes 21 and 22 of the
//! base block carry the panel's physical size in centimetres. Every byte here
//! crosses the trust boundary, so the magic header, the declared length, and
//! the length the queue reported are checked before any of it is believed.

use super::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use crate::constants::{VG_CMD_GET_EDID, VG_RESP_OK_EDID};
use crate::device::virtqueue::ControlQueue;
use crate::protocol::le_u32;

const REQ_LEN: usize = HDR_LEN + 8; // __le32 scanout, __le32 padding
const BODY_OFFSET: usize = RESP_HDR_LEN + 8; // __le32 size, __le32 padding
const EDID_MAX_LEN: usize = 1024;
const EDID_BLOCK_LEN: usize = 128;
const RESP_LEN: usize = BODY_OFFSET + EDID_MAX_LEN;
const PARSE_LEN: usize = BODY_OFFSET + EDID_BLOCK_LEN;
const EDID_MAGIC: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];
const OFF_WIDTH_CM: usize = 21;
const OFF_HEIGHT_CM: usize = 22;

#[derive(Clone, Copy, Default)]
pub struct PhysicalSize {
    pub width_cm: u32,
    pub height_cm: u32,
}

pub fn get_edid(q: &ControlQueue, scanout: u32) -> Result<Option<PhysicalSize>, &'static str> {
    let mut req = [0u8; REQ_LEN];
    Hdr::new(VG_CMD_GET_EDID, 0).write(&mut req);
    req[HDR_LEN..HDR_LEN + 4].copy_from_slice(&scanout.to_le_bytes());
    let out = q.submit(&req, RESP_LEN as u32)?;
    let used = out.used_len as usize;
    if used < BODY_OFFSET {
        return Err("virtio-gpu: short edid response");
    }
    let mut buf = [0u8; PARSE_LEN];
    q.read_response(REQ_LEN, &mut buf);
    let hdr = Hdr::parse(&buf[..HDR_LEN]).ok_or("virtio-gpu: bad edid header")?;
    if hdr.type_ != VG_RESP_OK_EDID {
        return Err("virtio-gpu: edid rejected");
    }
    let size = le_u32(&buf, RESP_HDR_LEN).ok_or("virtio-gpu: bad edid body")? as usize;
    if size < EDID_BLOCK_LEN || size > EDID_MAX_LEN || size > used - BODY_OFFSET {
        return Err("virtio-gpu: edid size out of range");
    }
    let block = &buf[BODY_OFFSET..PARSE_LEN];
    if block[..EDID_MAGIC.len()] != EDID_MAGIC {
        return Err("virtio-gpu: edid magic mismatch");
    }
    let physical = PhysicalSize {
        width_cm: block[OFF_WIDTH_CM] as u32,
        height_cm: block[OFF_HEIGHT_CM] as u32,
    };
    if physical.width_cm == 0 || physical.height_cm == 0 {
        return Ok(None);
    }
    Ok(Some(physical))
}
