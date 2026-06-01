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

use alloc::vec;

use super::wire::{call_boot, payload_slice, read_status};

// driver_virtio_gpu::OP_GET_PRIMARY_SURFACE
const OP: u16 = 0x000C;
const RESP_LEN: usize = 32;

#[derive(Clone, Copy)]
pub struct PrimaryReply {
    pub handle: u64,
    pub resource_id: u32,
    pub width: u32,
    pub height: u32,
    pub format: u32,
}

pub fn get_primary_surface(gfx_port: u32, request_id: u32) -> Result<PrimaryReply, &'static str> {
    let mut rx = vec![0u8; super::wire::NVGP_HDR_LEN + 4 + RESP_LEN];
    let n = call_boot(gfx_port, OP, request_id, &[], &mut rx)?;
    let rx = &rx[..n];
    let status = read_status(rx).ok_or("gfx primary: short response")?;
    if status != 0 {
        return Err("gfx primary: driver rejected");
    }
    let body = payload_slice(&rx);
    if body.len() < RESP_LEN {
        return Err("gfx primary: body too short");
    }
    Ok(PrimaryReply {
        handle: u64_at(body, 0).ok_or("gfx primary: body too short")?,
        resource_id: u32_at(body, 8).ok_or("gfx primary: body too short")?,
        width: u32_at(body, 12).ok_or("gfx primary: body too short")?,
        height: u32_at(body, 16).ok_or("gfx primary: body too short")?,
        format: u32_at(body, 24).ok_or("gfx primary: body too short")?,
    })
}

fn u32_at(buf: &[u8], off: usize) -> Option<u32> {
    let bytes = buf.get(off..off + 4)?;
    Some(u32::from_le_bytes(bytes.try_into().ok()?))
}

fn u64_at(buf: &[u8], off: usize) -> Option<u64> {
    let bytes = buf.get(off..off + 8)?;
    Some(u64::from_le_bytes(bytes.try_into().ok()?))
}
