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
use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::{mk_ipc_call, mk_service_lookup};

const SERVICE: &[u8] = b"net.tcp";
const MAGIC: u32 = 0x4E54_4350;
const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

static REQ_ID: AtomicU32 = AtomicU32::new(1);

pub fn lookup() -> Option<u32> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    if rc < 0 || port == 0 {
        None
    } else {
        Some(port)
    }
}

pub fn call(port: u32, op: u16, body: &[u8], resp: &mut [u8]) -> Option<(u16, usize)> {
    let mut req = vec![0u8; HDR_LEN + body.len()];
    let rid = REQ_ID.fetch_add(1, Ordering::Relaxed);
    req[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    req[4..6].copy_from_slice(&VERSION.to_le_bytes());
    req[6..8].copy_from_slice(&op.to_le_bytes());
    req[12..16].copy_from_slice(&rid.to_le_bytes());
    req[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    req[HDR_LEN..].copy_from_slice(body);
    let n = mk_ipc_call(port as u64, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len());
    if n < HDR_LEN as i64 {
        return None;
    }
    let errno = u16::from_le_bytes([resp[8], resp[9]]);
    let payload_len = u32::from_le_bytes([resp[16], resp[17], resp[18], resp[19]]) as usize;
    Some((errno, payload_len))
}
