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
use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use super::ipv4::parse_ipv4;

const SERVICE: &[u8] = b"net.dns";
const MAGIC: u32 = 0x4E44_4E53;
const HDR_LEN: usize = 20;
const OP_RESOLVE_A: u16 = 2;
const TIMEOUT_MS: u64 = 2000;

pub fn resolve_host(host: &[u8]) -> Result<[u8; 4], &'static str> {
    if let Some(ip) = parse_ipv4(host) {
        return Ok(ip);
    }
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(
        SERVICE.as_ptr(),
        SERVICE.len(),
        &mut port as *mut u32,
        &mut pid as *mut u32,
    );
    if rc < 0 || port == 0 {
        return Err("pull: dns unavailable");
    }
    let mut tx: Vec<u8> = Vec::with_capacity(HDR_LEN + host.len());
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&1u16.to_le_bytes());
    tx.extend_from_slice(&OP_RESOLVE_A.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&1u32.to_le_bytes());
    tx.extend_from_slice(&(host.len() as u32).to_le_bytes());
    tx.extend_from_slice(host);
    let mut rx = [0u8; HDR_LEN + 4];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n < (HDR_LEN + 4) as i64 || u16::from_le_bytes([rx[8], rx[9]]) != 0 {
        return Err("pull: host not resolved");
    }
    Ok([rx[HDR_LEN], rx[HDR_LEN + 1], rx[HDR_LEN + 2], rx[HDR_LEN + 3]])
}
