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
use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call, mk_service_lookup};

pub const HDR_LEN: usize = 20;
pub const VERSION: u16 = 1;

pub fn lookup_port(name: &[u8]) -> Option<u32> {
    let mut pid = 0u32;
    let mut port = 0u32;
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 || port == 0 { None } else { Some(port) }
}

pub fn lookup_pid(name: &[u8]) -> Option<u32> {
    let mut pid = 0u32;
    let mut port = 0u32;
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 { None } else { Some(pid) }
}

pub fn call(
    port: u32,
    magic: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    body: &mut [u8],
) -> Result<i32, &'static str> {
    let mut tx = Vec::with_capacity(HDR_LEN + payload.len());
    build(&mut tx, magic, op, request_id, payload);
    let mut rx = vec![0u8; HDR_LEN + 4 + body.len()];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < (HDR_LEN + 4 + body.len()) as i64 || u32_at(&rx, 0)? != magic {
        return Err("service call failed");
    }
    body.copy_from_slice(&rx[HDR_LEN + 4..HDR_LEN + 4 + body.len()]);
    Ok(i32::from_le_bytes(rx[HDR_LEN..HDR_LEN + 4].try_into().map_err(|_| "status missing")?))
}

fn build(out: &mut Vec<u8>, magic: u32, op: u16, request_id: u32, payload: &[u8]) {
    out.clear();
    out.extend_from_slice(&magic.to_le_bytes());
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&request_id.to_le_bytes());
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
}

pub fn u32_at(buf: &[u8], off: usize) -> Result<u32, &'static str> {
    let bytes = buf.get(off..off + 4).ok_or("u32 missing")?;
    Ok(u32::from_le_bytes(bytes.try_into().map_err(|_| "u32 missing")?))
}
