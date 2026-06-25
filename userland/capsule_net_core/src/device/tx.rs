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

use nonos_libc::mk_ipc_call;

use crate::protocol::header::{parse_response, write_request};
use crate::protocol::ops::{HDR_LEN, OP_TX_PACKET};

static SEQ: AtomicU32 = AtomicU32::new(1);

fn next_rid() -> u32 {
    let v = SEQ.fetch_add(1, Ordering::Relaxed);
    if v == 0 { SEQ.fetch_add(1, Ordering::Relaxed) } else { v }
}

pub fn send_frame(port: u32, frame: &[u8]) -> bool {
    let total = HDR_LEN + frame.len();
    let mut req = vec![0u8; total];
    if write_request(&mut req, OP_TX_PACKET, next_rid(), frame.len() as u32).is_none() {
        return false;
    }
    req[HDR_LEN..total].copy_from_slice(frame);
    let mut resp = [0u8; HDR_LEN + 4];
    let n = mk_ipc_call(port as u64, req.as_ptr(), total, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return false;
    }
    let got = n as usize;
    if got < HDR_LEN + 4 {
        return false;
    }
    let view = &resp[..got.min(resp.len())];
    let (op, _, plen) = match parse_response(view) {
        Some(v) => v,
        None => return false,
    };
    if op != OP_TX_PACKET || plen as usize != 4 {
        return false;
    }
    let status = i32::from_le_bytes([view[HDR_LEN], view[HDR_LEN+1], view[HDR_LEN+2], view[HDR_LEN+3]]);
    status >= 0
}
