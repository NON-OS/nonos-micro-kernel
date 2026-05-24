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
use crate::constants::{VG_CMD_GET_DISPLAY_INFO, VG_MAX_SCANOUTS, VG_RESP_OK_DISPLAY_INFO};
use crate::device::virtqueue::ControlQueue;
use crate::protocol::le_u32;
use super::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
const DISPLAY_ONE_LEN: usize = 24;
const RESP_LEN: usize = RESP_HDR_LEN + DISPLAY_ONE_LEN * VG_MAX_SCANOUTS;
#[derive(Clone, Copy, Default)]
pub struct DisplayInfo {
    pub scanouts: [Scanout; VG_MAX_SCANOUTS],
    pub count: u8,
}
#[derive(Clone, Copy, Default)]
pub struct Scanout {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub enabled: u32,
}
pub fn get_display_info(q: &ControlQueue, fence_id: u64) -> Result<DisplayInfo, &'static str> {
    let mut req = [0u8; HDR_LEN];
    Hdr::new(VG_CMD_GET_DISPLAY_INFO, fence_id).write(&mut req);
    let out = q.submit(&req, RESP_LEN as u32)?;
    if (out.used_len as usize) < RESP_LEN {
        return Err("virtio-gpu: short display info response");
    }
    let mut buf = [0u8; RESP_LEN];
    q.read_response(HDR_LEN, &mut buf);
    let hdr = Hdr::parse(&buf[..HDR_LEN]).ok_or("virtio-gpu: bad display info header")?;
    if hdr.type_ != VG_RESP_OK_DISPLAY_INFO {
        return Err("virtio-gpu: display info rejected");
    }
    let mut info = DisplayInfo::default();
    for i in 0..VG_MAX_SCANOUTS {
        let base = RESP_HDR_LEN + i * DISPLAY_ONE_LEN;
        let s = Scanout {
            x: le_u32(&buf, base).ok_or("virtio-gpu: bad display info entry")?,
            y: le_u32(&buf, base + 4).ok_or("virtio-gpu: bad display info entry")?,
            width: le_u32(&buf, base + 8).ok_or("virtio-gpu: bad display info entry")?,
            height: le_u32(&buf, base + 12).ok_or("virtio-gpu: bad display info entry")?,
            enabled: le_u32(&buf, base + 16).ok_or("virtio-gpu: bad display info entry")?,
        };
        info.scanouts[i] = s;
        if s.enabled != 0 && info.count < VG_MAX_SCANOUTS as u8 {
            info.count += 1;
        }
    }
    Ok(info)
}