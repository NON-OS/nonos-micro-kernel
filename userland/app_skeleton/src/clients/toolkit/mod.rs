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

extern crate alloc;

mod theme;

pub use theme::{theme_get, Theme};

use alloc::vec::Vec;

use nonos_libc::mk_ipc_call;

const NOTK_MAGIC: u32 = 0x4E4F_544B;
const HDR_LEN: usize = 16;
const TOOLKIT_ENDPOINT: u64 = 4610;
const TOOLKIT_OP_COMPONENT_RENDER: u16 = 0x0003;
const STATUS_OK: u16 = 0;
const KIND_PANEL: u16 = 0;
const KIND_LABEL: u16 = 2;
const CHROME_H: u32 = 28;
const LABEL_X: u32 = 10;
const LABEL_Y: u32 = 8;
const MAX_LABEL_BYTES: usize = 96;

pub fn ui_frame(
    port: u32,
    request_id: u32,
    surface_handle: u64,
    width: u32,
    title: &[u8],
) -> Result<(), &'static str> {
    render_component(port, request_id, surface_handle, 0, 0, width, CHROME_H, KIND_PANEL, b"")?;
    render_component(
        port,
        request_id.wrapping_add(1),
        surface_handle,
        LABEL_X,
        LABEL_Y,
        width.saturating_sub(LABEL_X.saturating_mul(2)),
        14,
        KIND_LABEL,
        title,
    )
}

fn render_component(
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
    let _ = TOOLKIT_ENDPOINT;
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
