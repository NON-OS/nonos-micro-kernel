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


//! One round trip to net.sockets.
//!
//! Sockets there are keyed by the caller's pid, so every handle belongs
//! to this capsule and a guest can only reach one through a descriptor
//! this capsule gave it.

use alloc::{vec, vec::Vec};

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::{build_request, HDR_LEN};
use nonos_libc::mk_ipc_call;

/// Keep in sync with CAPSULE_SERVICE_ENDPOINT in
/// userland/capsule_net_sockets/Capsule.mk.
const FIXED_PORT: u32 = 4460;
const NAME: &[u8] = b"net.sockets";
const MAGIC: u32 = 0x4E53_4B54;

/// Status is a u16 at offset 8 of the reply header, payload at 20.
const STATUS_AT: usize = 8;

pub fn call(op: u16, body: &[u8], want: usize) -> Option<(u16, Vec<u8>)> {
    let port = match lookup_service(NAME) {
        Some(peer) if peer.port != 0 => peer.port,
        _ => FIXED_PORT,
    };
    let tx = build_request(MAGIC, op, 1, body);
    let mut rx = vec![0u8; HDR_LEN + want];
    let n = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if n < HDR_LEN as i64 {
        return None;
    }
    let status = u16::from_le_bytes([rx[STATUS_AT], rx[STATUS_AT + 1]]);
    Some((status, rx[HDR_LEN..n as usize].to_vec()))
}
