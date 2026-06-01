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

use super::wire::{call, read_status};

const OP: u16 = 0x0008;
const BODY_LEN: usize = 32;

pub fn transfer_to_host(
    gfx_port: u32,
    request_id: u32,
    resource_id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    offset: u64,
) -> Result<(), &'static str> {
    let mut body = [0u8; BODY_LEN];
    body[0..4].copy_from_slice(&resource_id.to_le_bytes());
    body[4..8].copy_from_slice(&x.to_le_bytes());
    body[8..12].copy_from_slice(&y.to_le_bytes());
    body[12..16].copy_from_slice(&width.to_le_bytes());
    body[16..20].copy_from_slice(&height.to_le_bytes());
    body[24..32].copy_from_slice(&offset.to_le_bytes());
    let mut rx = vec![0u8; super::wire::NVGP_HDR_LEN + 4];
    call(gfx_port, OP, request_id, &body, &mut rx)?;
    let status = read_status(&rx).ok_or("gfx transfer: short response")?;
    if status != 0 {
        return Err("gfx transfer: driver rejected");
    }
    Ok(())
}
