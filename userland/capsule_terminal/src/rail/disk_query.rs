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

use nonos_libc::{mk_getpid, mk_ipc_call_timeout, mk_service_lookup};

use super::disk::Disk;
use super::disk_decode::{decode_usage, HDR_LEN, REPLY_LEN};

const SERVICE: &[u8] = b"vfs_pool";
const MAGIC: u32 = 0x4E4F_5646;
const OP_USAGE: u16 = 14;
const PAYLOAD_LEN: u32 = 4;
const TIMEOUT_MS: u64 = 64;

/// One store-occupancy round trip against the VFS. The rail throttles this well
/// below its own poll rate, so a whole timeout costs a fraction of a second in
/// the worst case and the previous answer stands in between.
pub fn query() -> Disk {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(
        SERVICE.as_ptr(),
        SERVICE.len(),
        &mut port as *mut u32,
        &mut pid as *mut u32,
    );
    if rc < 0 || port == 0 {
        return Disk::UNKNOWN;
    }
    let mut tx = [0u8; HDR_LEN + PAYLOAD_LEN as usize];
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&OP_USAGE.to_le_bytes());
    tx[16..20].copy_from_slice(&PAYLOAD_LEN.to_le_bytes());
    tx[20..24].copy_from_slice(&mk_getpid().to_le_bytes());
    let mut rx = [0u8; REPLY_LEN];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n < REPLY_LEN as i64 {
        return Disk::UNKNOWN;
    }
    decode_usage(&rx)
}
