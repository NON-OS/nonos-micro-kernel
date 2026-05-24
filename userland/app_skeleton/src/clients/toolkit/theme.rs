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

use alloc::vec::Vec;

use nonos_libc::mk_ipc_call;

const NOTK_MAGIC: u32 = 0x4E4F_544B;
const HDR_LEN: usize = 16;
const TOOLKIT_OP_THEME_GET: u16 = 0x0004;
const THEME_PAYLOAD_LEN: usize = 24;
const STATUS_OK: u16 = 0;

#[derive(Clone, Copy)]
pub struct Theme {
    pub background_argb: u32,
    pub surface_argb: u32,
    pub accent_argb: u32,
    pub text_argb: u32,
    pub border_argb: u32,
    pub revision: u32,
}

pub fn theme_get(port: u32, request_id: u32) -> Result<Theme, &'static str> {
    let mut request = Vec::with_capacity(HDR_LEN);
    request.extend_from_slice(&NOTK_MAGIC.to_le_bytes());
    request.extend_from_slice(&TOOLKIT_OP_THEME_GET.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&request_id.to_le_bytes());
    request.extend_from_slice(&0u32.to_le_bytes());
    let mut reply = [0u8; HDR_LEN + THEME_PAYLOAD_LEN];
    let rc = mk_ipc_call(port as u64, request.as_ptr(), request.len(), reply.as_mut_ptr(), reply.len());
    if rc < (HDR_LEN + THEME_PAYLOAD_LEN) as i64 {
        return Err("toolkit theme_get short reply");
    }
    let status = u16::from_le_bytes([reply[6], reply[7]]);
    if status != STATUS_OK {
        return Err("toolkit theme_get rejected");
    }
    let p = &reply[HDR_LEN..HDR_LEN + THEME_PAYLOAD_LEN];
    Ok(Theme {
        background_argb: u32_le(p, 0),
        surface_argb: u32_le(p, 4),
        accent_argb: u32_le(p, 8),
        text_argb: u32_le(p, 12),
        border_argb: u32_le(p, 16),
        revision: u32_le(p, 20),
    })
}

fn u32_le(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}
