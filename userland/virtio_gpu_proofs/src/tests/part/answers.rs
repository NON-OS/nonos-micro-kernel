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

//! What the part answers, from the virtio-gpu response layouts.

use super::bodies::{capset_info, display_info, edid};
use super::Answer;
use crate::constants::{
    VG_CMD_GET_CAPSET_INFO, VG_CMD_GET_DISPLAY_INFO, VG_CMD_GET_EDID, VG_CMD_SUBMIT_3D,
    VG_FLAG_FENCE, VG_RESP_OK_CAPSET_INFO, VG_RESP_OK_DISPLAY_INFO, VG_RESP_OK_EDID,
    VG_RESP_OK_NODATA,
};

pub const RESP_ERR_UNSPEC: u32 = 0x1200;
pub const PANEL: (u32, u32) = (1024, 768);
pub const PANEL_CM: (u8, u8) = (52, 29);
pub const CAPSET: (u32, u32, u32) = (1, 1, 308);

/// A response header: type, flags, fence, context, padding.
pub fn header(type_: u32, flags: u32, fence_id: u64) -> Vec<u8> {
    let mut h = Vec::with_capacity(24);
    h.extend_from_slice(&type_.to_le_bytes());
    h.extend_from_slice(&flags.to_le_bytes());
    h.extend_from_slice(&fence_id.to_le_bytes());
    h.extend_from_slice(&[0u8; 8]);
    h
}

/// The answer to `request`, cut to the buffer the driver offered.
pub fn build(request: &[u8], room: usize, answer: Answer) -> Vec<u8> {
    let type_ = u32::from_le_bytes([request[0], request[1], request[2], request[3]]);
    let fence = u64::from_le_bytes(request[8..16].try_into().unwrap_or([0; 8]));
    let mut out = match (answer, type_) {
        (Answer::Reject, _) => header(RESP_ERR_UNSPEC, 0, 0),
        (_, t) if t == VG_CMD_GET_DISPLAY_INFO => {
            display_info(header(VG_RESP_OK_DISPLAY_INFO, 0, 0))
        }
        (_, t) if t == VG_CMD_GET_EDID => {
            edid(header(VG_RESP_OK_EDID, 0, 0), answer == Answer::BadEdidMagic)
        }
        (_, t) if t == VG_CMD_GET_CAPSET_INFO => capset_info(header(VG_RESP_OK_CAPSET_INFO, 0, 0)),
        (_, t) if t == VG_CMD_SUBMIT_3D => {
            let echoed = if answer == Answer::WrongFence { fence + 1 } else { fence };
            header(VG_RESP_OK_NODATA, VG_FLAG_FENCE, echoed)
        }
        _ => header(VG_RESP_OK_NODATA, 0, 0),
    };
    out.truncate(room);
    out
}
