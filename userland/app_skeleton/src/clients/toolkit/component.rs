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
const TOOLKIT_OP_COMPONENT_RENDER: u16 = 0x0003;
const STATUS_OK: u16 = 0;

pub fn component_render(port: u32, request_id: u32, payload: &[u8]) -> Result<(), &'static str> {
    let mut request = Vec::with_capacity(HDR_LEN + payload.len());
    request.extend_from_slice(&NOTK_MAGIC.to_le_bytes());
    request.extend_from_slice(&TOOLKIT_OP_COMPONENT_RENDER.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&request_id.to_le_bytes());
    request.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    request.extend_from_slice(payload);

    let mut reply = [0u8; HDR_LEN];
    let rc =
        mk_ipc_call(port as u64, request.as_ptr(), request.len(), reply.as_mut_ptr(), reply.len());
    if rc < HDR_LEN as i64 {
        return Err("toolkit component_render short reply");
    }
    let status = u16::from_le_bytes([reply[6], reply[7]]);
    if status != STATUS_OK {
        return Err("toolkit component_render rejected");
    }
    Ok(())
}

pub fn ui_frame(
    port: u32,
    request_id: u32,
    surface_handle: u64,
    width: u32,
    height: u32,
) -> Result<(), &'static str> {
    let mut payload = [0u8; 28];
    payload[0..8].copy_from_slice(&surface_handle.to_le_bytes());
    payload[16..20].copy_from_slice(&width.to_le_bytes());
    payload[20..24].copy_from_slice(&height.to_le_bytes());
    component_render(port, request_id, &payload)
}
