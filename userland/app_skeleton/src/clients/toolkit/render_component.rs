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

use super::constants::{
    HDR_LEN, MAX_LABEL_BYTES, NOTK_MAGIC, STATUS_OK, TOOLKIT_OP_COMPONENT_RENDER,
};

pub(super) fn render_component(
    port: u32,
    request_id: u32,
    surface_handle: u64,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    kind: u16,
    label: &[u8],
) -> Result<(), &'static str> {
    let label_len = label.len().min(MAX_LABEL_BYTES);
    let mut body = Vec::with_capacity(28 + label_len);
    body.extend_from_slice(&surface_handle.to_le_bytes());
    body.extend_from_slice(&x.to_le_bytes());
    body.extend_from_slice(&y.to_le_bytes());
    body.extend_from_slice(&width.to_le_bytes());
    body.extend_from_slice(&height.to_le_bytes());
    body.extend_from_slice(&kind.to_le_bytes());
    body.extend_from_slice(&(label_len as u16).to_le_bytes());
    body.extend_from_slice(&label[..label_len]);
    let mut request = Vec::with_capacity(HDR_LEN + body.len());
    request.extend_from_slice(&NOTK_MAGIC.to_le_bytes());
    request.extend_from_slice(&TOOLKIT_OP_COMPONENT_RENDER.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&request_id.to_le_bytes());
    request.extend_from_slice(&(body.len() as u32).to_le_bytes());
    request.extend_from_slice(&body);
    let mut reply = [0u8; HDR_LEN];
    let rc =
        mk_ipc_call(port as u64, request.as_ptr(), request.len(), reply.as_mut_ptr(), reply.len());
    if rc < HDR_LEN as i64 {
        return Err("toolkit ui route failed");
    }
    if u32::from_le_bytes(reply[0..4].try_into().unwrap_or([0u8; 4])) != NOTK_MAGIC {
        return Err("toolkit reply magic mismatch");
    }
    let status = u16::from_le_bytes(reply[6..8].try_into().unwrap_or([0u8; 2]));
    if status != STATUS_OK {
        return Err("toolkit rejected ui frame");
    }
    Ok(())
}
