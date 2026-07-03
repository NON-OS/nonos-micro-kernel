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

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_ipc_call;

use crate::protocol::header::{parse_response, write_request};
use crate::protocol::ops::{HDR_LEN, OP_MAC_ADDRESS};

static SEQ: AtomicU32 = AtomicU32::new(1);

pub(super) fn next_rid() -> u32 {
    let v = SEQ.fetch_add(1, Ordering::Relaxed);
    if v == 0 { SEQ.fetch_add(1, Ordering::Relaxed) } else { v }
}

pub fn read_mac(port: u32) -> Option<[u8; 6]> {
    let mut req = [0u8; HDR_LEN];
    if write_request(&mut req, OP_MAC_ADDRESS, next_rid(), 0).is_none() {
        return None;
    }
    let mut resp = [0u8; HDR_LEN + 4 + 6];
    let n = mk_ipc_call(port as u64, req.as_ptr(), HDR_LEN, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return None;
    }
    let got = n as usize;
    if got > resp.len() {
        return None;
    }
    let view = &resp[..got];
    let (op, _, plen) = parse_response(view)?;
    if op != OP_MAC_ADDRESS || plen as usize != 4 + 6 {
        return None;
    }
    if view.len() < HDR_LEN + 4 + 6 {
        return None;
    }
    let status = i32::from_le_bytes([
        view[HDR_LEN],
        view[HDR_LEN + 1],
        view[HDR_LEN + 2],
        view[HDR_LEN + 3],
    ]);
    if status != 0 {
        return None;
    }
    let mac_start = HDR_LEN + 4;
    let mut mac = [0u8; 6];
    mac.copy_from_slice(&view[mac_start..mac_start + 6]);
    Some(mac)
}
