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

pub(crate) const MAGIC: u32 = 0x4E43_4D50;
pub(crate) const VERSION: u16 = 1;
pub(crate) const HDR_LEN: usize = 20;
const OP_HEALTHCHECK: u16 = 0x0001;

pub(crate) fn header(op: u16, request_id: u32, payload_len: u32) -> Vec<u8> {
    let mut tx = Vec::with_capacity(HDR_LEN + payload_len as usize);
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&request_id.to_le_bytes());
    tx.extend_from_slice(&payload_len.to_le_bytes());
    tx
}

pub(crate) fn call_status(port: u32, tx: &[u8]) -> Result<(), i32> {
    let mut rx = vec![0u8; HDR_LEN + 4];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < (HDR_LEN + 4) as i64 {
        return Err(-11);
    }
    let status = i32::from_le_bytes(rx[HDR_LEN..HDR_LEN + 4].try_into().map_err(|_| -11)?);
    if status == 0 {
        Ok(())
    } else {
        Err(status)
    }
}

pub(crate) fn lookup_compositor() -> Option<u32> {
    let name: &[u8] = b"compositor";
    let mut pid: u32 = 0;
    let mut port: u32 = 0;
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 || port == 0 {
        return None;
    }
    Some(port)
}

pub(crate) fn healthcheck(port: u32, rid: u32) -> Result<(), i32> {
    call_status(port, &header(OP_HEALTHCHECK, rid, 0))
}
