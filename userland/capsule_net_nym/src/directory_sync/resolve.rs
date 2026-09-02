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
use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

const SERVICE: &[u8] = b"net.dns";
const MAGIC_NDNS: u32 = 0x4E44_4E53;
const OP_RESOLVE_A: u16 = 2;
const HDR: usize = 20;
/// A lookup runs on the idle directory tick, so it can afford to wait out a
/// full recursive resolve. This must stay above the resolver's own per-query
/// deadline (3s in net.core); at 2s the client gave up first and a cold lookup
/// of the directory host aborted every sync with a false "unresolvable", so the
/// gateway list never installed.
const TIMEOUT_MS: u64 = 6_000;

/// Resolve `host` to one IPv4 address through `net.dns`.
///
/// The directory is named rather than pinned to an address, because an
/// address compiled into an image outlives whatever it pointed at and leaves
/// no way to notice.
pub fn resolve(host: &[u8]) -> Option<[u8; 4]> {
    let mut port = 0u32;
    let mut pid = 0u32;
    if mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid) < 0 || port == 0 {
        return None;
    }
    let mut tx = vec![0u8; HDR + host.len()];
    tx[0..4].copy_from_slice(&MAGIC_NDNS.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&OP_RESOLVE_A.to_le_bytes());
    tx[12..16].copy_from_slice(&1u32.to_le_bytes());
    tx[16..20].copy_from_slice(&(host.len() as u32).to_le_bytes());
    tx[HDR..].copy_from_slice(host);

    let mut rx = [0u8; HDR + 4];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n < (HDR + 4) as i64 || u16::from_le_bytes([rx[8], rx[9]]) != 0 {
        return None;
    }
    Some([rx[HDR], rx[HDR + 1], rx[HDR + 2], rx[HDR + 3]])
}
